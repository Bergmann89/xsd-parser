#[derive(Debug, Clone)]
pub struct Schema {
    pub target_namespace: Option<String>,
    pub version: Option<String>,
    pub final_default: FullDerivationSetType,
    pub block_default: BlockSetType,
    pub attribute_form_default: FormChoiceType,
    pub element_form_default: FormChoiceType,
    pub default_attributes: Option<String>,
    pub xpath_default_namespace: XpathDefaultNamespaceType,
    pub id: Option<String>,
    pub lang: Option<String>,
    pub content: Vec<SchemaContent>,
}
#[derive(Debug, Clone)]
pub enum SchemaContent {
    Include(Include),
    Import(Import),
    Redefine(Redefine),
    Override(Override),
    Annotation(Annotation),
    DefaultOpenContent(DefaultOpenContent),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
    Element(ElementType),
    Attribute(AttributeType),
    Notation(Notation),
}
impl xsd_parser::WithNamespace for Schema {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl Schema {
    #[must_use]
    pub fn default_final_default() -> FullDerivationSetType {
        FullDerivationSetType::TypeDerivationControlList(TypeDerivationControlList(Vec::new()))
    }
    #[must_use]
    pub fn default_block_default() -> BlockSetType {
        BlockSetType::BlockSetItemList(BlockSetItemList(Vec::new()))
    }
    #[must_use]
    pub fn default_attribute_form_default() -> FormChoiceType {
        FormChoiceType::Unqualified
    }
    #[must_use]
    pub fn default_element_form_default() -> FormChoiceType {
        FormChoiceType::Unqualified
    }
    #[must_use]
    pub fn default_xpath_default_namespace() -> XpathDefaultNamespaceType {
        XpathDefaultNamespaceType::String(String::from("##local"))
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Schema {
    type Deserializer = quick_xml_deserialize::SchemaDeserializer;
}
#[derive(Debug, Clone)]
pub struct Include {
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for Include {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Include {
    type Deserializer = quick_xml_deserialize::IncludeDeserializer;
}
#[derive(Debug, Clone)]
pub struct Import {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for Import {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Import {
    type Deserializer = quick_xml_deserialize::ImportDeserializer;
}
#[derive(Debug, Clone)]
pub struct Redefine {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<RedefineContent>,
}
#[derive(Debug, Clone)]
pub enum RedefineContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}
impl xsd_parser::WithNamespace for Redefine {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Redefine {
    type Deserializer = quick_xml_deserialize::RedefineDeserializer;
}
#[derive(Debug, Clone)]
pub struct Override {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<OverrideContent>,
}
#[derive(Debug, Clone)]
pub enum OverrideContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
    Element(ElementType),
    Attribute(AttributeType),
    Notation(Notation),
}
impl xsd_parser::WithNamespace for Override {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Override {
    type Deserializer = quick_xml_deserialize::OverrideDeserializer;
}
#[derive(Debug, Clone)]
pub struct Annotation {
    pub id: Option<String>,
    pub content: Vec<AnnotationContent>,
}
#[derive(Debug, Clone)]
pub enum AnnotationContent {
    Appinfo(Appinfo),
    Documentation(Documentation),
}
impl xsd_parser::WithNamespace for Annotation {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Annotation {
    type Deserializer = quick_xml_deserialize::AnnotationDeserializer;
}
#[derive(Debug, Clone)]
pub struct DefaultOpenContent {
    pub id: Option<String>,
    pub applies_to_empty: bool,
    pub mode: DefaultOpenContentModeType,
    pub annotation: Option<Annotation>,
    pub any: WildcardType,
}
impl xsd_parser::WithNamespace for DefaultOpenContent {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl DefaultOpenContent {
    #[must_use]
    pub fn default_applies_to_empty() -> bool {
        false
    }
    #[must_use]
    pub fn default_mode() -> DefaultOpenContentModeType {
        DefaultOpenContentModeType::Interleave
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DefaultOpenContent {
    type Deserializer = quick_xml_deserialize::DefaultOpenContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct SimpleBaseType {
    pub id: Option<String>,
    pub final_: Option<SimpleDerivationSetType>,
    pub name: Option<String>,
    pub content: Vec<SimpleBaseTypeContent>,
}
#[derive(Debug, Clone)]
pub enum SimpleBaseTypeContent {
    Annotation(Annotation),
    Restriction(Restriction),
    List(List),
    Union(Union),
}
impl xsd_parser::WithNamespace for SimpleBaseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SimpleBaseType {
    type Deserializer = quick_xml_deserialize::SimpleBaseTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ComplexBaseType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mixed: Option<bool>,
    pub abstract_: bool,
    pub final_: Option<DerivationSetType>,
    pub block: Option<DerivationSetType>,
    pub default_attributes_apply: bool,
    pub content: Vec<ComplexBaseTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ComplexBaseTypeContent {
    Annotation(Annotation),
    SimpleContent(SimpleContent),
    ComplexContent(ComplexContent),
    OpenContent(OpenContent),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttribute),
    Assert(AssertionType),
}
impl xsd_parser::WithNamespace for ComplexBaseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl ComplexBaseType {
    #[must_use]
    pub fn default_abstract_() -> bool {
        false
    }
    #[must_use]
    pub fn default_default_attributes_apply() -> bool {
        true
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ComplexBaseType {
    type Deserializer = quick_xml_deserialize::ComplexBaseTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct GroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub min_occurs: usize,
    pub max_occurs: AllNNIType,
    pub content: Vec<GroupTypeContent>,
}
#[derive(Debug, Clone)]
pub enum GroupTypeContent {
    Annotation(Annotation),
    Element(ElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Any(Any),
}
impl xsd_parser::WithNamespace for GroupType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl GroupType {
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> AllNNIType {
        AllNNIType::Usize(1usize)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for GroupType {
    type Deserializer = quick_xml_deserialize::GroupTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct AttributeGroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub content: Vec<AttributeGroupTypeContent>,
}
#[derive(Debug, Clone)]
pub enum AttributeGroupTypeContent {
    Annotation(Annotation),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttribute),
}
impl xsd_parser::WithNamespace for AttributeGroupType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for AttributeGroupType {
    type Deserializer = quick_xml_deserialize::AttributeGroupTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ElementType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub type_: Option<String>,
    pub substitution_group: Option<ElementSubstitutionGroupType>,
    pub min_occurs: usize,
    pub max_occurs: AllNNIType,
    pub default: Option<String>,
    pub fixed: Option<String>,
    pub nillable: Option<bool>,
    pub abstract_: bool,
    pub final_: Option<DerivationSetType>,
    pub block: Option<BlockSetType>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<String>,
    pub content: Vec<ElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ElementTypeContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Alternative(AltType),
    Unique(KeybaseType),
    Key(KeybaseType),
    Keyref(Keyref),
}
impl xsd_parser::WithNamespace for ElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl ElementType {
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> AllNNIType {
        AllNNIType::Usize(1usize)
    }
    #[must_use]
    pub fn default_abstract_() -> bool {
        false
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ElementType {
    type Deserializer = quick_xml_deserialize::ElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct AttributeType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub type_: Option<String>,
    pub use_: AttributeUseType,
    pub default: Option<String>,
    pub fixed: Option<String>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<String>,
    pub inheritable: Option<bool>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<SimpleBaseType>,
}
impl xsd_parser::WithNamespace for AttributeType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl AttributeType {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}
impl xsd_parser::quick_xml::WithDeserializer for AttributeType {
    type Deserializer = quick_xml_deserialize::AttributeTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct Notation {
    pub id: Option<String>,
    pub name: String,
    pub public: Option<String>,
    pub system: Option<String>,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for Notation {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Notation {
    type Deserializer = quick_xml_deserialize::NotationDeserializer;
}
#[derive(Debug, Clone)]
pub enum FullDerivationSetType {
    All,
    TypeDerivationControlList(TypeDerivationControlList),
}
impl xsd_parser::WithNamespace for FullDerivationSetType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for FullDerivationSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::TypeDerivationControlList(
                TypeDerivationControlList::deserialize_bytes(reader, x)?,
            )),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct TypeDerivationControlList(pub Vec<TypeDerivationControlType>);
impl xsd_parser::WithNamespace for TypeDerivationControlList {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for TypeDerivationControlList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| TypeDerivationControlType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum BlockSetType {
    All,
    BlockSetItemList(BlockSetItemList),
}
impl xsd_parser::WithNamespace for BlockSetType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for BlockSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::BlockSetItemList(BlockSetItemList::deserialize_bytes(
                reader, x,
            )?)),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct BlockSetItemList(pub Vec<BlockSetItemType>);
impl xsd_parser::WithNamespace for BlockSetItemList {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for BlockSetItemList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| BlockSetItemType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum FormChoiceType {
    Qualified,
    Unqualified,
}
impl xsd_parser::WithNamespace for FormChoiceType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for FormChoiceType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"qualified" => Ok(Self::Qualified),
            b"unqualified" => Ok(Self::Unqualified),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum XpathDefaultNamespaceType {
    String(String),
    DefaultNamespace,
    TargetNamespace,
    Local,
}
impl xsd_parser::WithNamespace for XpathDefaultNamespaceType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for XpathDefaultNamespaceType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"##defaultNamespace" => Ok(Self::DefaultNamespace),
            b"##targetNamespace" => Ok(Self::TargetNamespace),
            b"##local" => Ok(Self::Local),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Appinfo {
    pub source: Option<String>,
}
impl xsd_parser::WithNamespace for Appinfo {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Appinfo {
    type Deserializer = quick_xml_deserialize::AppinfoDeserializer;
}
#[derive(Debug, Clone)]
pub struct Documentation {
    pub source: Option<String>,
    pub lang: Option<String>,
}
impl xsd_parser::WithNamespace for Documentation {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Documentation {
    type Deserializer = quick_xml_deserialize::DocumentationDeserializer;
}
#[derive(Debug, Clone)]
pub struct WildcardType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for WildcardType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl WildcardType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
impl xsd_parser::quick_xml::WithDeserializer for WildcardType {
    type Deserializer = quick_xml_deserialize::WildcardTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum DefaultOpenContentModeType {
    Interleave,
    Suffix,
}
impl xsd_parser::WithNamespace for DefaultOpenContentModeType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for DefaultOpenContentModeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"interleave" => Ok(Self::Interleave),
            b"suffix" => Ok(Self::Suffix),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Restriction {
    pub id: Option<String>,
    pub base: Option<String>,
    pub content: Vec<RestrictionContent>,
}
#[derive(Debug, Clone)]
pub enum RestrictionContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    Facet(Facet),
}
impl xsd_parser::WithNamespace for Restriction {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Restriction {
    type Deserializer = quick_xml_deserialize::RestrictionDeserializer;
}
#[derive(Debug, Clone)]
pub struct List {
    pub id: Option<String>,
    pub item_type: Option<String>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<SimpleBaseType>,
}
impl xsd_parser::WithNamespace for List {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for List {
    type Deserializer = quick_xml_deserialize::ListDeserializer;
}
#[derive(Debug, Clone)]
pub struct Union {
    pub id: Option<String>,
    pub member_types: Option<UnionMemberTypesType>,
    pub annotation: Option<Annotation>,
    pub simple_type: Vec<SimpleBaseType>,
}
impl xsd_parser::WithNamespace for Union {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Union {
    type Deserializer = quick_xml_deserialize::UnionDeserializer;
}
#[derive(Debug, Clone)]
pub enum SimpleDerivationSetType {
    All,
    SimpleDerivationSetItemList(SimpleDerivationSetItemList),
}
impl xsd_parser::WithNamespace for SimpleDerivationSetType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for SimpleDerivationSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::SimpleDerivationSetItemList(
                SimpleDerivationSetItemList::deserialize_bytes(reader, x)?,
            )),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SimpleContent {
    pub id: Option<String>,
    pub content: Vec<SimpleContentContent>,
}
#[derive(Debug, Clone)]
pub enum SimpleContentContent {
    Annotation(Annotation),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl xsd_parser::WithNamespace for SimpleContent {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SimpleContent {
    type Deserializer = quick_xml_deserialize::SimpleContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct ComplexContent {
    pub id: Option<String>,
    pub mixed: Option<bool>,
    pub content: Vec<ComplexContentContent>,
}
#[derive(Debug, Clone)]
pub enum ComplexContentContent {
    Annotation(Annotation),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl xsd_parser::WithNamespace for ComplexContent {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ComplexContent {
    type Deserializer = quick_xml_deserialize::ComplexContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct OpenContent {
    pub id: Option<String>,
    pub mode: OpenContentModeType,
    pub annotation: Option<Annotation>,
    pub any: Option<WildcardType>,
}
impl xsd_parser::WithNamespace for OpenContent {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl OpenContent {
    #[must_use]
    pub fn default_mode() -> OpenContentModeType {
        OpenContentModeType::Interleave
    }
}
impl xsd_parser::quick_xml::WithDeserializer for OpenContent {
    type Deserializer = quick_xml_deserialize::OpenContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct AnyAttribute {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListAType>,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for AnyAttribute {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl AnyAttribute {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
impl xsd_parser::quick_xml::WithDeserializer for AnyAttribute {
    type Deserializer = quick_xml_deserialize::AnyAttributeDeserializer;
}
#[derive(Debug, Clone)]
pub struct AssertionType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for AssertionType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for AssertionType {
    type Deserializer = quick_xml_deserialize::AssertionTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum DerivationSetType {
    All,
    ReducedDerivationControlList(ReducedDerivationControlList),
}
impl xsd_parser::WithNamespace for DerivationSetType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for DerivationSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::ReducedDerivationControlList(
                ReducedDerivationControlList::deserialize_bytes(reader, x)?,
            )),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Any {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListType>,
    pub min_occurs: usize,
    pub max_occurs: AllNNIType,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for Any {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl Any {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> AllNNIType {
        AllNNIType::Usize(1usize)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Any {
    type Deserializer = quick_xml_deserialize::AnyDeserializer;
}
#[derive(Debug, Clone)]
pub enum AllNNIType {
    Usize(usize),
    Unbounded,
}
impl xsd_parser::WithNamespace for AllNNIType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for AllNNIType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"unbounded" => Ok(Self::Unbounded),
            x => Ok(Self::Usize(usize::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AltType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub type_: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub content: Vec<AltTypeContent>,
}
#[derive(Debug, Clone)]
pub enum AltTypeContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
}
impl xsd_parser::WithNamespace for AltType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for AltType {
    type Deserializer = quick_xml_deserialize::AltTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct KeybaseType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub annotation: Option<Annotation>,
    pub selector: Field,
    pub field: Vec<Field>,
}
impl xsd_parser::WithNamespace for KeybaseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for KeybaseType {
    type Deserializer = quick_xml_deserialize::KeybaseTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct Keyref {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub refer: Option<String>,
    pub annotation: Option<Annotation>,
    pub selector: Field,
    pub field: Vec<Field>,
}
impl xsd_parser::WithNamespace for Keyref {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Keyref {
    type Deserializer = quick_xml_deserialize::KeyrefDeserializer;
}
#[derive(Debug, Clone, Default)]
pub struct ElementSubstitutionGroupType(pub Vec<QnameType>);
impl xsd_parser::WithNamespace for ElementSubstitutionGroupType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for ElementSubstitutionGroupType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| QnameType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum AttributeUseType {
    Prohibited,
    Optional,
    Required,
}
impl xsd_parser::WithNamespace for AttributeUseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for AttributeUseType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"prohibited" => Ok(Self::Prohibited),
            b"optional" => Ok(Self::Optional),
            b"required" => Ok(Self::Required),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
pub type TypeDerivationControlType = DerivationControlType;
pub type BlockSetItemType = DerivationControlType;
#[derive(Debug, Clone)]
pub enum NamespaceListType {
    Any,
    Other,
    BasicNamespaceList(BasicNamespaceListType),
}
impl xsd_parser::WithNamespace for NamespaceListType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for NamespaceListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"##any" => Ok(Self::Any),
            b"##other" => Ok(Self::Other),
            x => Ok(Self::BasicNamespaceList(
                BasicNamespaceListType::deserialize_bytes(reader, x)?,
            )),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct NotNamespaceType(pub Vec<BasicNamespaceListItemType>);
impl xsd_parser::WithNamespace for NotNamespaceType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for NotNamespaceType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| BasicNamespaceListItemType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum ProcessContentsType {
    Skip,
    Lax,
    Strict,
}
impl xsd_parser::WithNamespace for ProcessContentsType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for ProcessContentsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"skip" => Ok(Self::Skip),
            b"lax" => Ok(Self::Lax),
            b"strict" => Ok(Self::Strict),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum Facet {
    MinExclusive(FacetType),
    MinInclusive(FacetType),
    MaxExclusive(FacetType),
    MaxInclusive(FacetType),
    TotalDigits(FacetType),
    FractionDigits(FacetType),
    Length(FacetType),
    MinLength(FacetType),
    MaxLength(FacetType),
    Enumeration(FacetType),
    WhiteSpace(FacetType),
    Pattern(FacetType),
    Assertion(AssertionType),
    ExplicitTimezone(FacetType),
}
impl xsd_parser::WithNamespace for Facet {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Facet {
    type Deserializer = quick_xml_deserialize::FacetDeserializer;
}
#[derive(Debug, Clone, Default)]
pub struct UnionMemberTypesType(pub Vec<QnameType>);
impl xsd_parser::WithNamespace for UnionMemberTypesType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for UnionMemberTypesType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| QnameType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone, Default)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
impl xsd_parser::WithNamespace for SimpleDerivationSetItemList {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for SimpleDerivationSetItemList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| SimpleDerivationSetItemType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub struct RestrictionType {
    pub id: Option<String>,
    pub base: String,
    pub content: Vec<RestrictionTypeContent>,
}
#[derive(Debug, Clone)]
pub enum RestrictionTypeContent {
    Annotation(Annotation),
    OpenContent(OpenContent),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    SimpleType(SimpleBaseType),
    Facet(Facet),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttribute),
    Assert(AssertionType),
}
impl xsd_parser::WithNamespace for RestrictionType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for RestrictionType {
    type Deserializer = quick_xml_deserialize::RestrictionTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ExtensionType {
    pub id: Option<String>,
    pub base: String,
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ExtensionTypeContent {
    Annotation(Annotation),
    OpenContent(OpenContent),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttribute),
    Assert(AssertionType),
}
impl xsd_parser::WithNamespace for ExtensionType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ExtensionType {
    type Deserializer = quick_xml_deserialize::ExtensionTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum OpenContentModeType {
    None,
    Interleave,
    Suffix,
}
impl xsd_parser::WithNamespace for OpenContentModeType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for OpenContentModeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"none" => Ok(Self::None),
            b"interleave" => Ok(Self::Interleave),
            b"suffix" => Ok(Self::Suffix),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct QnameListAType(pub Vec<QnameListAItemType>);
impl xsd_parser::WithNamespace for QnameListAType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for QnameListAType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| QnameListAItemType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone, Default)]
pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
impl xsd_parser::WithNamespace for ReducedDerivationControlList {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for ReducedDerivationControlList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| ReducedDerivationControlType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone, Default)]
pub struct QnameListType(pub Vec<QnameListItemType>);
impl xsd_parser::WithNamespace for QnameListType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for QnameListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| QnameListItemType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub struct Field {
    pub id: Option<String>,
    pub xpath: String,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for Field {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Field {
    type Deserializer = quick_xml_deserialize::FieldDeserializer;
}
pub type QnameType = String;
#[derive(Debug, Clone)]
pub enum DerivationControlType {
    Substitution,
    Extension,
    Restriction,
    List,
    Union,
}
impl xsd_parser::WithNamespace for DerivationControlType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for DerivationControlType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"substitution" => Ok(Self::Substitution),
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            b"list" => Ok(Self::List),
            b"union" => Ok(Self::Union),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
impl xsd_parser::WithNamespace for BasicNamespaceListType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for BasicNamespaceListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| BasicNamespaceListItemType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum BasicNamespaceListItemType {
    String(String),
    TargetNamespace,
    Local,
}
impl xsd_parser::WithNamespace for BasicNamespaceListItemType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for BasicNamespaceListItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"##targetNamespace" => Ok(Self::TargetNamespace),
            b"##local" => Ok(Self::Local),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FacetType {
    pub id: Option<String>,
    pub value: String,
    pub fixed: bool,
    pub annotation: Option<Annotation>,
}
impl xsd_parser::WithNamespace for FacetType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl FacetType {
    #[must_use]
    pub fn default_fixed() -> bool {
        false
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FacetType {
    type Deserializer = quick_xml_deserialize::FacetTypeDeserializer;
}
pub type SimpleDerivationSetItemType = DerivationControlType;
#[derive(Debug, Clone)]
pub enum QnameListAItemType {
    String(String),
    Defined,
}
impl xsd_parser::WithNamespace for QnameListAItemType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for QnameListAItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
pub type ReducedDerivationControlType = DerivationControlType;
#[derive(Debug, Clone)]
pub enum QnameListItemType {
    String(String),
    Defined,
    DefinedSibling,
}
impl xsd_parser::WithNamespace for QnameListItemType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> &'static str {
        "http://www.w3.org/2001/XMLSchema"
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for QnameListItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            b"##definedSibling" => Ok(Self::DefinedSibling),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct SchemaDeserializer {
        target_namespace: Option<String>,
        version: Option<String>,
        final_default: super::FullDerivationSetType,
        block_default: super::BlockSetType,
        attribute_form_default: super::FormChoiceType,
        element_form_default: super::FormChoiceType,
        default_attributes: Option<String>,
        xpath_default_namespace: super::XpathDefaultNamespaceType,
        id: Option<String>,
        lang: Option<String>,
        content: Vec<super::SchemaContent>,
        state: Box<SchemaDeserializerState>,
    }
    #[derive(Debug)]
    enum SchemaDeserializerState {
        Next__,
        Include(<Include as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Import(<Import as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Redefine(<Redefine as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Override(<Override as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        DefaultOpenContent(
            <DefaultOpenContent as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ComplexType(<ComplexBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        Element(<ElementType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Attribute(<AttributeType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Notation(<Notation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl SchemaDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            const NS_XML: &[u8] = b"http://www.w3.org/XML/1998/namespace";
            let mut target_namespace: Option<String> = None;
            let mut version: Option<String> = None;
            let mut final_default: Option<FullDerivationSetType> = None;
            let mut block_default: Option<BlockSetType> = None;
            let mut attribute_form_default: Option<FormChoiceType> = None;
            let mut element_form_default: Option<FormChoiceType> = None;
            let mut default_attributes: Option<String> = None;
            let mut xpath_default_namespace: Option<XpathDefaultNamespaceType> = None;
            let mut id: Option<String> = None;
            let mut lang: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"targetNamespace")
                ) {
                    reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"version")
                ) {
                    reader.read_attrib(&mut version, b"version", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"finalDefault")
                ) {
                    reader.read_attrib(&mut final_default, b"finalDefault", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"blockDefault")
                ) {
                    reader.read_attrib(&mut block_default, b"blockDefault", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"attributeFormDefault")
                ) {
                    reader.read_attrib(
                        &mut attribute_form_default,
                        b"attributeFormDefault",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"elementFormDefault")
                ) {
                    reader.read_attrib(
                        &mut element_form_default,
                        b"elementFormDefault",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"defaultAttributes")
                ) {
                    reader.read_attrib(
                        &mut default_attributes,
                        b"defaultAttributes",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    reader.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XML), Some(b"lang")) {
                    reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
                }
            }
            Ok(Self {
                target_namespace: target_namespace,
                version: version,
                final_default: final_default.unwrap_or_else(super::Schema::default_final_default),
                block_default: block_default.unwrap_or_else(super::Schema::default_block_default),
                attribute_form_default: attribute_form_default
                    .unwrap_or_else(super::Schema::default_attribute_form_default),
                element_form_default: element_form_default
                    .unwrap_or_else(super::Schema::default_element_form_default),
                default_attributes: default_attributes,
                xpath_default_namespace: xpath_default_namespace
                    .unwrap_or_else(super::Schema::default_xpath_default_namespace),
                id: id,
                lang: lang,
                content: Vec::new(),
                state: Box::new(SchemaDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Schema> for SchemaDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Schema, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Schema, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, SchemaDeserializerState::Next__),
                &event,
            ) {
                (SchemaDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"include")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Include as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Include(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Include(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"import"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Import as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Import(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Import(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"redefine")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Redefine as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Redefine(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Redefine(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"override")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Override as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Override(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Override(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"defaultOpenContent")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <DefaultOpenContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::DefaultOpenContent(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::DefaultOpenContent(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"complexType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ComplexBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::ComplexType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::ComplexType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::AttributeGroup(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"element"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Element(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Element(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Attribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Attribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"notation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Notation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SchemaContent::Notation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SchemaDeserializerState::Notation(deserializer);
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
                (SchemaDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (SchemaDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (SchemaDeserializerState::Include(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Include(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Include(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Import(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Import(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Import(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Redefine(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Redefine(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Redefine(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Override(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Override(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Override(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::DefaultOpenContent(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::DefaultOpenContent(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::DefaultOpenContent(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::ComplexType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::ComplexType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::ComplexType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::AttributeGroup(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Element(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Element(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Element(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Attribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Attribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Attribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SchemaDeserializerState::Notation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SchemaContent::Notation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SchemaDeserializerState::Notation(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::Schema, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Schema {
                target_namespace: self.target_namespace,
                version: self.version,
                final_default: self.final_default,
                block_default: self.block_default,
                attribute_form_default: self.attribute_form_default,
                element_form_default: self.element_form_default,
                default_attributes: self.default_attributes,
                xpath_default_namespace: self.xpath_default_namespace,
                id: self.id,
                lang: self.lang,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct IncludeDeserializer {
        id: Option<String>,
        schema_location: String,
        annotation: Option<super::Annotation>,
        state: Box<IncludeDeserializerState>,
    }
    #[derive(Debug)]
    enum IncludeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl IncludeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut schema_location: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                schema_location: schema_location
                    .ok_or(ErrorKind::MissingAttribute("schemaLocation".into()))?,
                annotation: None,
                state: Box::new(IncludeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Include> for IncludeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Include, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Include, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, IncludeDeserializerState::Done__),
                    event,
                ) {
                    (IncludeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = IncludeDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(IncludeDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = IncludeDeserializerState::Annotation(None);
                        event
                    }
                    (IncludeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = IncludeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = IncludeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            IncludeDeserializerState::Annotation(None),
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
                            *self.state = IncludeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(IncludeDeserializerState::Annotation(None));
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
                            *self.state = IncludeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (IncludeDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Include, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Include {
                id: self.id,
                schema_location: self.schema_location,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct ImportDeserializer {
        id: Option<String>,
        namespace: Option<String>,
        schema_location: Option<String>,
        annotation: Option<super::Annotation>,
        state: Box<ImportDeserializerState>,
    }
    #[derive(Debug)]
    enum ImportDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ImportDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut namespace: Option<String> = None;
            let mut schema_location: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                namespace: namespace,
                schema_location: schema_location,
                annotation: None,
                state: Box::new(ImportDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Import> for ImportDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Import, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Import, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ImportDeserializerState::Done__),
                    event,
                ) {
                    (ImportDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ImportDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(ImportDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = ImportDeserializerState::Annotation(None);
                        event
                    }
                    (ImportDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = ImportDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ImportDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ImportDeserializerState::Annotation(None),
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
                            *self.state = ImportDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(ImportDeserializerState::Annotation(None));
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
                            *self.state = ImportDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ImportDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Import, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Import {
                id: self.id,
                namespace: self.namespace,
                schema_location: self.schema_location,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct RedefineDeserializer {
        schema_location: String,
        id: Option<String>,
        content: Vec<super::RedefineContent>,
        state: Box<RedefineDeserializerState>,
    }
    #[derive(Debug)]
    enum RedefineDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ComplexType(<ComplexBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
    }
    impl RedefineDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut schema_location: Option<String> = None;
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                schema_location: schema_location
                    .ok_or(ErrorKind::MissingAttribute("schemaLocation".into()))?,
                id: id,
                content: Vec::new(),
                state: Box::new(RedefineDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Redefine> for RedefineDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Redefine, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Redefine, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, RedefineDeserializerState::Next__),
                &event,
            ) {
                (RedefineDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RedefineContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RedefineDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(RedefineContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RedefineDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"complexType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ComplexBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(RedefineContent::ComplexType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RedefineDeserializerState::ComplexType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RedefineContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RedefineDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(RedefineContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RedefineDeserializerState::AttributeGroup(deserializer);
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
                (RedefineDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (RedefineDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (RedefineDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RedefineContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RedefineDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RedefineDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RedefineContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RedefineDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RedefineDeserializerState::ComplexType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RedefineContent::ComplexType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RedefineDeserializerState::ComplexType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RedefineDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RedefineContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RedefineDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RedefineDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RedefineContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RedefineDeserializerState::AttributeGroup(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::Redefine, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Redefine {
                schema_location: self.schema_location,
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct OverrideDeserializer {
        schema_location: String,
        id: Option<String>,
        content: Vec<super::OverrideContent>,
        state: Box<OverrideDeserializerState>,
    }
    #[derive(Debug)]
    enum OverrideDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ComplexType(<ComplexBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        Element(<ElementType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Attribute(<AttributeType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Notation(<Notation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl OverrideDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut schema_location: Option<String> = None;
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                schema_location: schema_location
                    .ok_or(ErrorKind::MissingAttribute("schemaLocation".into()))?,
                id: id,
                content: Vec::new(),
                state: Box::new(OverrideDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Override> for OverrideDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Override, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Override, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, OverrideDeserializerState::Next__),
                &event,
            ) {
                (OverrideDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"complexType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ComplexBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::ComplexType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::ComplexType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::AttributeGroup(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"element"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::Element(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::Element(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::Attribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::Attribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"notation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Notation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(OverrideContent::Notation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = OverrideDeserializerState::Notation(deserializer);
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
                (OverrideDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (OverrideDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (OverrideDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::ComplexType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::ComplexType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::ComplexType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::AttributeGroup(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::Element(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::Element(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::Element(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::Attribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::Attribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::Attribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (OverrideDeserializerState::Notation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(OverrideContent::Notation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = OverrideDeserializerState::Notation(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::Override, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Override {
                schema_location: self.schema_location,
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnnotationDeserializer {
        id: Option<String>,
        content: Vec<super::AnnotationContent>,
        state: Box<AnnotationDeserializerState>,
    }
    #[derive(Debug)]
    enum AnnotationDeserializerState {
        Next__,
        Appinfo(<Appinfo as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Documentation(<Documentation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl AnnotationDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state: Box::new(AnnotationDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Annotation> for AnnotationDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Annotation, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Annotation, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, AnnotationDeserializerState::Next__),
                &event,
            ) {
                (AnnotationDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"appinfo")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Appinfo as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(AnnotationContent::Appinfo(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = AnnotationDeserializerState::Appinfo(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"documentation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Documentation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(AnnotationContent::Documentation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = AnnotationDeserializerState::Documentation(deserializer);
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
                (AnnotationDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (AnnotationDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (AnnotationDeserializerState::Appinfo(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(AnnotationContent::Appinfo(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AnnotationDeserializerState::Appinfo(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (AnnotationDeserializerState::Documentation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(AnnotationContent::Documentation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AnnotationDeserializerState::Documentation(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::Annotation, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Annotation {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct DefaultOpenContentDeserializer {
        id: Option<String>,
        applies_to_empty: bool,
        mode: super::DefaultOpenContentModeType,
        annotation: Option<super::Annotation>,
        any: Option<super::WildcardType>,
        state: Box<DefaultOpenContentDeserializerState>,
    }
    #[derive(Debug)]
    enum DefaultOpenContentDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Any(Option<<WildcardType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl DefaultOpenContentDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut applies_to_empty: Option<bool> = None;
            let mut mode: Option<DefaultOpenContentModeType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"appliesToEmpty")
                ) {
                    reader.read_attrib(&mut applies_to_empty, b"appliesToEmpty", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"mode")) {
                    reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                applies_to_empty: applies_to_empty
                    .unwrap_or_else(super::DefaultOpenContent::default_applies_to_empty),
                mode: mode.unwrap_or_else(super::DefaultOpenContent::default_mode),
                annotation: None,
                any: None,
                state: Box::new(DefaultOpenContentDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DefaultOpenContent>
        for DefaultOpenContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DefaultOpenContent, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DefaultOpenContent, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        DefaultOpenContentDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        DefaultOpenContentDeserializerState::Annotation(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DefaultOpenContentDeserializerState::Annotation(deserializer);
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
                                DefaultOpenContentDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = DefaultOpenContentDeserializerState::Annotation(None);
                        event
                    }
                    (DefaultOpenContentDeserializerState::Annotation(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_XS),
                                    Some(b"annotation")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <Annotation as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.annotation.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"annotation",
                                        )))?;
                                    }
                                    self.annotation = Some(data);
                                }
                                *self.state =
                                    DefaultOpenContentDeserializerState::Annotation(deserializer);
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            DefaultOpenContentDeserializerState::Any(None);
                                        if allow_any {
                                            allow_any_fallback.get_or_insert(
                                                DefaultOpenContentDeserializerState::Annotation(
                                                    None,
                                                ),
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
                                *self.state = DefaultOpenContentDeserializerState::Any(None);
                                allow_any_fallback.get_or_insert(
                                    DefaultOpenContentDeserializerState::Annotation(None),
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
                                *self.state = DefaultOpenContentDeserializerState::Annotation(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (DefaultOpenContentDeserializerState::Any(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.any.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
                            }
                            self.any = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DefaultOpenContentDeserializerState::Any(deserializer);
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
                                DefaultOpenContentDeserializerState::Any(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.any.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
                            }
                            self.any = Some(data);
                        }
                        *self.state = DefaultOpenContentDeserializerState::Any(None);
                        event
                    }
                    (DefaultOpenContentDeserializerState::Any(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"any")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <WildcardType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.any.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"any",
                                    )))?;
                                }
                                self.any = Some(data);
                            }
                            *self.state = DefaultOpenContentDeserializerState::Any(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DefaultOpenContentDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DefaultOpenContentDeserializerState::Any(None),
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
                            *self.state = DefaultOpenContentDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(DefaultOpenContentDeserializerState::Any(None));
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
                            *self.state = DefaultOpenContentDeserializerState::Any(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DefaultOpenContentDeserializerState::Done__, event) => {
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
        ) -> Result<super::DefaultOpenContent, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::DefaultOpenContent {
                id: self.id,
                applies_to_empty: self.applies_to_empty,
                mode: self.mode,
                annotation: self.annotation,
                any: self
                    .any
                    .ok_or_else(|| ErrorKind::MissingElement("any".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleBaseTypeDeserializer {
        id: Option<String>,
        final_: Option<super::SimpleDerivationSetType>,
        name: Option<String>,
        content: Vec<super::SimpleBaseTypeContent>,
        state: Box<SimpleBaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleBaseTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Restriction(<Restriction as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        List(<List as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Union(<Union as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl SimpleBaseTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut final_: Option<SimpleDerivationSetType> = None;
            let mut name: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"final")) {
                    reader.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                final_: final_,
                name: name,
                content: Vec::new(),
                state: Box::new(SimpleBaseTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SimpleBaseType>
        for SimpleBaseTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SimpleBaseType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SimpleBaseType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, SimpleBaseTypeDeserializerState::Next__),
                &event,
            ) {
                (SimpleBaseTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SimpleBaseTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SimpleBaseTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"restriction")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Restriction as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SimpleBaseTypeContent::Restriction(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                SimpleBaseTypeDeserializerState::Restriction(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"list")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <List as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SimpleBaseTypeContent::List(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SimpleBaseTypeDeserializerState::List(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"union")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Union as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SimpleBaseTypeContent::Union(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SimpleBaseTypeDeserializerState::Union(deserializer);
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
                (SimpleBaseTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (SimpleBaseTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (SimpleBaseTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleBaseTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleBaseTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SimpleBaseTypeDeserializerState::Restriction(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleBaseTypeContent::Restriction(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleBaseTypeDeserializerState::Restriction(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SimpleBaseTypeDeserializerState::List(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleBaseTypeContent::List(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleBaseTypeDeserializerState::List(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SimpleBaseTypeDeserializerState::Union(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleBaseTypeContent::Union(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleBaseTypeDeserializerState::Union(deserializer);
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
        ) -> Result<super::SimpleBaseType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::SimpleBaseType {
                id: self.id,
                final_: self.final_,
                name: self.name,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ComplexBaseTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        mixed: Option<bool>,
        abstract_: bool,
        final_: Option<super::DerivationSetType>,
        block: Option<super::DerivationSetType>,
        default_attributes_apply: bool,
        content: Vec<super::ComplexBaseTypeContent>,
        state: Box<ComplexBaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ComplexBaseTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleContent(<SimpleContent as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ComplexContent(<ComplexContent as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        OpenContent(<OpenContent as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        All(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Choice(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Sequence(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Attribute(<AttributeType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        AnyAttribute(<AnyAttribute as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Assert(<AssertionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl ComplexBaseTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut mixed: Option<bool> = None;
            let mut abstract_: Option<bool> = None;
            let mut final_: Option<DerivationSetType> = None;
            let mut block: Option<DerivationSetType> = None;
            let mut default_attributes_apply: Option<bool> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"mixed")) {
                    reader.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"abstract")
                ) {
                    reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"final")) {
                    reader.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"block")) {
                    reader.read_attrib(&mut block, b"block", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"defaultAttributesApply")
                ) {
                    reader.read_attrib(
                        &mut default_attributes_apply,
                        b"defaultAttributesApply",
                        &attrib.value,
                    )?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                mixed: mixed,
                abstract_: abstract_.unwrap_or_else(super::ComplexBaseType::default_abstract_),
                final_: final_,
                block: block,
                default_attributes_apply: default_attributes_apply
                    .unwrap_or_else(super::ComplexBaseType::default_default_attributes_apply),
                content: Vec::new(),
                state: Box::new(ComplexBaseTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ComplexBaseType>
        for ComplexBaseTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ComplexBaseType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ComplexBaseType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, ComplexBaseTypeDeserializerState::Next__),
                &event,
            ) {
                (ComplexBaseTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexBaseTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleContent")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleContent as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content
                                .push(ComplexBaseTypeContent::SimpleContent(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexBaseTypeDeserializerState::SimpleContent(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"complexContent")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ComplexContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content
                                .push(ComplexBaseTypeContent::ComplexContent(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexBaseTypeDeserializerState::ComplexContent(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"openContent")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <OpenContent as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::OpenContent(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexBaseTypeDeserializerState::OpenContent(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexBaseTypeDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"all")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::All(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexBaseTypeDeserializerState::All(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"choice"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::Choice(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexBaseTypeDeserializerState::Choice(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"sequence")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::Sequence(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexBaseTypeDeserializerState::Sequence(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::Attribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexBaseTypeDeserializerState::Attribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content
                                .push(ComplexBaseTypeContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexBaseTypeDeserializerState::AttributeGroup(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"anyAttribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content
                                .push(ComplexBaseTypeContent::AnyAttribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexBaseTypeDeserializerState::AnyAttribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"assert"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexBaseTypeContent::Assert(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexBaseTypeDeserializerState::Assert(deserializer);
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
                (ComplexBaseTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (ComplexBaseTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (ComplexBaseTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::SimpleContent(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(ComplexBaseTypeContent::SimpleContent(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::SimpleContent(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::ComplexContent(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(ComplexBaseTypeContent::ComplexContent(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            ComplexBaseTypeDeserializerState::ComplexContent(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::OpenContent(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::OpenContent(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::OpenContent(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::All(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::All(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::All(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::Choice(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::Choice(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::Choice(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::Sequence(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::Sequence(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::Sequence(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::Attribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::Attribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::Attribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(ComplexBaseTypeContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            ComplexBaseTypeDeserializerState::AttributeGroup(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::AnyAttribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(ComplexBaseTypeContent::AnyAttribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::AnyAttribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexBaseTypeDeserializerState::Assert(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexBaseTypeContent::Assert(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexBaseTypeDeserializerState::Assert(deserializer);
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
        ) -> Result<super::ComplexBaseType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ComplexBaseType {
                id: self.id,
                name: self.name,
                mixed: self.mixed,
                abstract_: self.abstract_,
                final_: self.final_,
                block: self.block,
                default_attributes_apply: self.default_attributes_apply,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct GroupTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        min_occurs: usize,
        max_occurs: super::AllNNIType,
        content: Vec<super::GroupTypeContent>,
        state: Box<GroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GroupTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Element(<ElementType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        All(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Choice(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Sequence(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Any(<Any as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl GroupTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<AllNNIType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"ref")) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"minOccurs")
                ) {
                    reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"maxOccurs")
                ) {
                    reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                min_occurs: min_occurs.unwrap_or_else(super::GroupType::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::GroupType::default_max_occurs),
                content: Vec::new(),
                state: Box::new(GroupTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::GroupType> for GroupTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::GroupType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::GroupType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, GroupTypeDeserializerState::Next__),
                &event,
            ) {
                (GroupTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"element"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::Element(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::Element(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"all")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::All(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::All(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"choice"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::Choice(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::Choice(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"sequence")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::Sequence(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::Sequence(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"any")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Any as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(GroupTypeContent::Any(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = GroupTypeDeserializerState::Any(deserializer);
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
                (GroupTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (GroupTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (GroupTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (GroupTypeDeserializerState::Element(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::Element(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::Element(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (GroupTypeDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (GroupTypeDeserializerState::All(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::All(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::All(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (GroupTypeDeserializerState::Choice(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::Choice(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::Choice(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (GroupTypeDeserializerState::Sequence(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::Sequence(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::Sequence(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (GroupTypeDeserializerState::Any(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(GroupTypeContent::Any(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = GroupTypeDeserializerState::Any(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::GroupType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::GroupType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                min_occurs: self.min_occurs,
                max_occurs: self.max_occurs,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct AttributeGroupTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        content: Vec<super::AttributeGroupTypeContent>,
        state: Box<AttributeGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AttributeGroupTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Attribute(<AttributeType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        AnyAttribute(<AnyAttribute as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl AttributeGroupTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"ref")) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                content: Vec::new(),
                state: Box::new(AttributeGroupTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::AttributeGroupType>
        for AttributeGroupTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AttributeGroupType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AttributeGroupType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(
                    &mut *self.state,
                    AttributeGroupTypeDeserializerState::Next__,
                ),
                &event,
            ) {
                (
                    AttributeGroupTypeDeserializerState::Next__,
                    Event::Start(x) | Event::Empty(x),
                ) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content
                                .push(AttributeGroupTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                AttributeGroupTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content
                                .push(AttributeGroupTypeContent::Attribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                AttributeGroupTypeDeserializerState::Attribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content
                                .push(AttributeGroupTypeContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                AttributeGroupTypeDeserializerState::AttributeGroup(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"anyAttribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content
                                .push(AttributeGroupTypeContent::AnyAttribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                AttributeGroupTypeDeserializerState::AnyAttribute(deserializer);
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
                (AttributeGroupTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (AttributeGroupTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (AttributeGroupTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(AttributeGroupTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AttributeGroupTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (AttributeGroupTypeDeserializerState::Attribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(AttributeGroupTypeContent::Attribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AttributeGroupTypeDeserializerState::Attribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (AttributeGroupTypeDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(AttributeGroupTypeContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            AttributeGroupTypeDeserializerState::AttributeGroup(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (AttributeGroupTypeDeserializerState::AnyAttribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(AttributeGroupTypeContent::AnyAttribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            AttributeGroupTypeDeserializerState::AnyAttribute(deserializer);
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
        ) -> Result<super::AttributeGroupType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::AttributeGroupType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ElementTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        type_: Option<String>,
        substitution_group: Option<super::ElementSubstitutionGroupType>,
        min_occurs: usize,
        max_occurs: super::AllNNIType,
        default: Option<String>,
        fixed: Option<String>,
        nillable: Option<bool>,
        abstract_: bool,
        final_: Option<super::DerivationSetType>,
        block: Option<super::BlockSetType>,
        form: Option<super::FormChoiceType>,
        target_namespace: Option<String>,
        content: Vec<super::ElementTypeContent>,
        state: Box<ElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ElementTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ComplexType(<ComplexBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Alternative(<AltType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Unique(<KeybaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Key(<KeybaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Keyref(<Keyref as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl ElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut substitution_group: Option<ElementSubstitutionGroupType> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<AllNNIType> = None;
            let mut default: Option<String> = None;
            let mut fixed: Option<String> = None;
            let mut nillable: Option<bool> = None;
            let mut abstract_: Option<bool> = None;
            let mut final_: Option<DerivationSetType> = None;
            let mut block: Option<BlockSetType> = None;
            let mut form: Option<FormChoiceType> = None;
            let mut target_namespace: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"ref")) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"type")) {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"substitutionGroup")
                ) {
                    reader.read_attrib(
                        &mut substitution_group,
                        b"substitutionGroup",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"minOccurs")
                ) {
                    reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"maxOccurs")
                ) {
                    reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"default")
                ) {
                    reader.read_attrib(&mut default, b"default", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"fixed")) {
                    reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"nillable")
                ) {
                    reader.read_attrib(&mut nillable, b"nillable", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"abstract")
                ) {
                    reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"final")) {
                    reader.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"block")) {
                    reader.read_attrib(&mut block, b"block", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"form")) {
                    reader.read_attrib(&mut form, b"form", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"targetNamespace")
                ) {
                    reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                type_: type_,
                substitution_group: substitution_group,
                min_occurs: min_occurs.unwrap_or_else(super::ElementType::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::ElementType::default_max_occurs),
                default: default,
                fixed: fixed,
                nillable: nillable,
                abstract_: abstract_.unwrap_or_else(super::ElementType::default_abstract_),
                final_: final_,
                block: block,
                form: form,
                target_namespace: target_namespace,
                content: Vec::new(),
                state: Box::new(ElementTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ElementType> for ElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ElementType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ElementType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, ElementTypeDeserializerState::Next__),
                &event,
            ) {
                (ElementTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"complexType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ComplexBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::ComplexType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::ComplexType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"alternative")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AltType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::Alternative(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::Alternative(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"unique"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <KeybaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::Unique(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::Unique(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"key")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <KeybaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::Key(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::Key(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"keyref"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Keyref as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ElementTypeContent::Keyref(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ElementTypeDeserializerState::Keyref(deserializer);
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
                (ElementTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (ElementTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (ElementTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ElementTypeDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ElementTypeDeserializerState::ComplexType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::ComplexType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::ComplexType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ElementTypeDeserializerState::Alternative(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::Alternative(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::Alternative(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ElementTypeDeserializerState::Unique(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::Unique(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::Unique(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ElementTypeDeserializerState::Key(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::Key(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::Key(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ElementTypeDeserializerState::Keyref(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ElementTypeContent::Keyref(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ElementTypeDeserializerState::Keyref(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::ElementType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ElementType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                type_: self.type_,
                substitution_group: self.substitution_group,
                min_occurs: self.min_occurs,
                max_occurs: self.max_occurs,
                default: self.default,
                fixed: self.fixed,
                nillable: self.nillable,
                abstract_: self.abstract_,
                final_: self.final_,
                block: self.block,
                form: self.form,
                target_namespace: self.target_namespace,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct AttributeTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        type_: Option<String>,
        use_: super::AttributeUseType,
        default: Option<String>,
        fixed: Option<String>,
        form: Option<super::FormChoiceType>,
        target_namespace: Option<String>,
        inheritable: Option<bool>,
        annotation: Option<super::Annotation>,
        simple_type: Option<super::SimpleBaseType>,
        state: Box<AttributeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AttributeTypeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        SimpleType(
            Option<<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Done__,
    }
    impl AttributeTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut use_: Option<AttributeUseType> = None;
            let mut default: Option<String> = None;
            let mut fixed: Option<String> = None;
            let mut form: Option<FormChoiceType> = None;
            let mut target_namespace: Option<String> = None;
            let mut inheritable: Option<bool> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"ref")) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"type")) {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"use")) {
                    reader.read_attrib(&mut use_, b"use", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"default")
                ) {
                    reader.read_attrib(&mut default, b"default", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"fixed")) {
                    reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"form")) {
                    reader.read_attrib(&mut form, b"form", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"targetNamespace")
                ) {
                    reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"inheritable")
                ) {
                    reader.read_attrib(&mut inheritable, b"inheritable", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                type_: type_,
                use_: use_.unwrap_or_else(super::AttributeType::default_use_),
                default: default,
                fixed: fixed,
                form: form,
                target_namespace: target_namespace,
                inheritable: inheritable,
                annotation: None,
                simple_type: None,
                state: Box::new(AttributeTypeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::AttributeType>
        for AttributeTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AttributeType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AttributeType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, AttributeTypeDeserializerState::Done__),
                    event,
                ) {
                    (AttributeTypeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    AttributeTypeDeserializerState::Annotation(deserializer);
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
                                AttributeTypeDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = AttributeTypeDeserializerState::Annotation(None);
                        event
                    }
                    (AttributeTypeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = AttributeTypeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = AttributeTypeDeserializerState::SimpleType(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            AttributeTypeDeserializerState::Annotation(None),
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
                            *self.state = AttributeTypeDeserializerState::SimpleType(None);
                            allow_any_fallback
                                .get_or_insert(AttributeTypeDeserializerState::Annotation(None));
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
                            *self.state = AttributeTypeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (AttributeTypeDeserializerState::SimpleType(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.simple_type.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"simpleType",
                                )))?;
                            }
                            self.simple_type = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    AttributeTypeDeserializerState::SimpleType(deserializer);
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
                                AttributeTypeDeserializerState::SimpleType(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.simple_type.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"simpleType",
                                )))?;
                            }
                            self.simple_type = Some(data);
                        }
                        *self.state = AttributeTypeDeserializerState::SimpleType(None);
                        event
                    }
                    (AttributeTypeDeserializerState::SimpleType(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"simpleType")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.simple_type.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"simpleType",
                                    )))?;
                                }
                                self.simple_type = Some(data);
                            }
                            *self.state = AttributeTypeDeserializerState::SimpleType(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = AttributeTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            AttributeTypeDeserializerState::SimpleType(None),
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
                            *self.state = AttributeTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(AttributeTypeDeserializerState::SimpleType(None));
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
                            *self.state = AttributeTypeDeserializerState::SimpleType(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (AttributeTypeDeserializerState::Done__, event) => {
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
        ) -> Result<super::AttributeType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::AttributeType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                type_: self.type_,
                use_: self.use_,
                default: self.default,
                fixed: self.fixed,
                form: self.form,
                target_namespace: self.target_namespace,
                inheritable: self.inheritable,
                annotation: self.annotation,
                simple_type: self.simple_type,
            })
        }
    }
    #[derive(Debug)]
    pub struct NotationDeserializer {
        id: Option<String>,
        name: String,
        public: Option<String>,
        system: Option<String>,
        annotation: Option<super::Annotation>,
        state: Box<NotationDeserializerState>,
    }
    #[derive(Debug)]
    enum NotationDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl NotationDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut public: Option<String> = None;
            let mut system: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"public")
                ) {
                    reader.read_attrib(&mut public, b"public", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"system")
                ) {
                    reader.read_attrib(&mut system, b"system", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name.ok_or(ErrorKind::MissingAttribute("name".into()))?,
                public: public,
                system: system,
                annotation: None,
                state: Box::new(NotationDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Notation> for NotationDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Notation, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Notation, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, NotationDeserializerState::Done__),
                    event,
                ) {
                    (NotationDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = NotationDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(NotationDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = NotationDeserializerState::Annotation(None);
                        event
                    }
                    (NotationDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = NotationDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = NotationDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            NotationDeserializerState::Annotation(None),
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
                            *self.state = NotationDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(NotationDeserializerState::Annotation(None));
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
                            *self.state = NotationDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (NotationDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Notation, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Notation {
                id: self.id,
                name: self.name,
                public: self.public,
                system: self.system,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AppinfoDeserializer {
        source: Option<String>,
    }
    impl AppinfoDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut source: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"source")
                ) {
                    reader.read_attrib(&mut source, b"source", &attrib.value)?;
                }
            }
            Ok(Self { source: source })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Appinfo> for AppinfoDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Appinfo, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Appinfo, Self>
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
        fn finish<R>(self, _reader: &R) -> Result<super::Appinfo, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Appinfo {
                source: self.source,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentationDeserializer {
        source: Option<String>,
        lang: Option<String>,
    }
    impl DocumentationDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            const NS_XML: &[u8] = b"http://www.w3.org/XML/1998/namespace";
            let mut source: Option<String> = None;
            let mut lang: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"source")
                ) {
                    reader.read_attrib(&mut source, b"source", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XML), Some(b"lang")) {
                    reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
                }
            }
            Ok(Self {
                source: source,
                lang: lang,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Documentation>
        for DocumentationDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Documentation, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Documentation, Self>
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
        ) -> Result<super::Documentation, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Documentation {
                source: self.source,
                lang: self.lang,
            })
        }
    }
    #[derive(Debug)]
    pub struct WildcardTypeDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        annotation: Option<super::Annotation>,
        state: Box<WildcardTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WildcardTypeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl WildcardTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut namespace: Option<NamespaceListType> = None;
            let mut not_namespace: Option<NotNamespaceType> = None;
            let mut process_contents: Option<ProcessContentsType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"notNamespace")
                ) {
                    reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"processContents")
                ) {
                    reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::WildcardType::default_process_contents),
                annotation: None,
                state: Box::new(WildcardTypeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::WildcardType>
        for WildcardTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::WildcardType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::WildcardType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, WildcardTypeDeserializerState::Done__),
                    event,
                ) {
                    (WildcardTypeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    WildcardTypeDeserializerState::Annotation(deserializer);
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
                                WildcardTypeDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = WildcardTypeDeserializerState::Annotation(None);
                        event
                    }
                    (WildcardTypeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = WildcardTypeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = WildcardTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            WildcardTypeDeserializerState::Annotation(None),
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
                            *self.state = WildcardTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(WildcardTypeDeserializerState::Annotation(None));
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
                            *self.state = WildcardTypeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (WildcardTypeDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::WildcardType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::WildcardType {
                id: self.id,
                namespace: self.namespace,
                not_namespace: self.not_namespace,
                process_contents: self.process_contents,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct RestrictionDeserializer {
        id: Option<String>,
        base: Option<String>,
        content: Vec<super::RestrictionContent>,
        state: Box<RestrictionDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictionDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Facet(<Facet as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl RestrictionDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"base")) {
                    reader.read_attrib(&mut base, b"base", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                base: base,
                content: Vec::new(),
                state: Box::new(RestrictionDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Restriction> for RestrictionDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Restriction, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Restriction, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, RestrictionDeserializerState::Next__),
                &event,
            ) {
                (RestrictionDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(RestrictionContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        {
                            let mut allow_any_element = false;
                            let event = {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                                if let Some(data) = data {
                                    self.content.push(RestrictionContent::Facet(data));
                                }
                                if let Some(deserializer) = deserializer {
                                    *self.state = RestrictionDeserializerState::Facet(deserializer);
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
                    }
                }
                (RestrictionDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (RestrictionDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (RestrictionDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionDeserializerState::Facet(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionContent::Facet(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionDeserializerState::Facet(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::Restriction, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Restriction {
                id: self.id,
                base: self.base,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ListDeserializer {
        id: Option<String>,
        item_type: Option<String>,
        annotation: Option<super::Annotation>,
        simple_type: Option<super::SimpleBaseType>,
        state: Box<ListDeserializerState>,
    }
    #[derive(Debug)]
    enum ListDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        SimpleType(
            Option<<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Done__,
    }
    impl ListDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut item_type: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"itemType")
                ) {
                    reader.read_attrib(&mut item_type, b"itemType", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                item_type: item_type,
                annotation: None,
                simple_type: None,
                state: Box::new(ListDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::List> for ListDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::List, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::List, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ListDeserializerState::Done__),
                    event,
                ) {
                    (ListDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ListDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(ListDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = ListDeserializerState::Annotation(None);
                        event
                    }
                    (ListDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = ListDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ListDeserializerState::SimpleType(None);
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(ListDeserializerState::Annotation(None));
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
                            *self.state = ListDeserializerState::SimpleType(None);
                            allow_any_fallback
                                .get_or_insert(ListDeserializerState::Annotation(None));
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
                            *self.state = ListDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ListDeserializerState::SimpleType(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.simple_type.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"simpleType",
                                )))?;
                            }
                            self.simple_type = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ListDeserializerState::SimpleType(deserializer);
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
                                .get_or_insert(ListDeserializerState::SimpleType(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.simple_type.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"simpleType",
                                )))?;
                            }
                            self.simple_type = Some(data);
                        }
                        *self.state = ListDeserializerState::SimpleType(None);
                        event
                    }
                    (ListDeserializerState::SimpleType(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"simpleType")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.simple_type.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"simpleType",
                                    )))?;
                                }
                                self.simple_type = Some(data);
                            }
                            *self.state = ListDeserializerState::SimpleType(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ListDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(ListDeserializerState::SimpleType(None));
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
                            *self.state = ListDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(ListDeserializerState::SimpleType(None));
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
                            *self.state = ListDeserializerState::SimpleType(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ListDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::List, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::List {
                id: self.id,
                item_type: self.item_type,
                annotation: self.annotation,
                simple_type: self.simple_type,
            })
        }
    }
    #[derive(Debug)]
    pub struct UnionDeserializer {
        id: Option<String>,
        member_types: Option<super::UnionMemberTypesType>,
        annotation: Option<super::Annotation>,
        simple_type: Vec<super::SimpleBaseType>,
        state: Box<UnionDeserializerState>,
    }
    #[derive(Debug)]
    enum UnionDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        SimpleType(
            Option<<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Done__,
    }
    impl UnionDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut member_types: Option<UnionMemberTypesType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"memberTypes")
                ) {
                    reader.read_attrib(&mut member_types, b"memberTypes", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                member_types: member_types,
                annotation: None,
                simple_type: Vec::new(),
                state: Box::new(UnionDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Union> for UnionDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Union, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Union, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, UnionDeserializerState::Done__),
                    event,
                ) {
                    (UnionDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = UnionDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(UnionDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = UnionDeserializerState::Annotation(None);
                        event
                    }
                    (UnionDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = UnionDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = UnionDeserializerState::SimpleType(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            UnionDeserializerState::Annotation(None),
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
                            *self.state = UnionDeserializerState::SimpleType(None);
                            allow_any_fallback
                                .get_or_insert(UnionDeserializerState::Annotation(None));
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
                            *self.state = UnionDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (UnionDeserializerState::SimpleType(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.simple_type.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = UnionDeserializerState::SimpleType(deserializer);
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
                                .get_or_insert(UnionDeserializerState::SimpleType(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.simple_type.push(data);
                        }
                        *self.state = UnionDeserializerState::SimpleType(None);
                        event
                    }
                    (UnionDeserializerState::SimpleType(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"simpleType")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                self.simple_type.push(data);
                            }
                            *self.state = UnionDeserializerState::SimpleType(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = UnionDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            UnionDeserializerState::SimpleType(None),
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
                            *self.state = UnionDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(UnionDeserializerState::SimpleType(None));
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
                            *self.state = UnionDeserializerState::SimpleType(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (UnionDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Union, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Union {
                id: self.id,
                member_types: self.member_types,
                annotation: self.annotation,
                simple_type: self.simple_type,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleContentDeserializer {
        id: Option<String>,
        content: Vec<super::SimpleContentContent>,
        state: Box<SimpleContentDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleContentDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Restriction(<RestrictionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Extension(<ExtensionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl SimpleContentDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state: Box::new(SimpleContentDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SimpleContent>
        for SimpleContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SimpleContent, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SimpleContent, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, SimpleContentDeserializerState::Next__),
                &event,
            ) {
                (SimpleContentDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SimpleContentContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SimpleContentDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"restriction")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <RestrictionType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(SimpleContentContent::Restriction(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SimpleContentDeserializerState::Restriction(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"extension")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ExtensionType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(SimpleContentContent::Extension(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = SimpleContentDeserializerState::Extension(deserializer);
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
                (SimpleContentDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (SimpleContentDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (SimpleContentDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleContentContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleContentDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SimpleContentDeserializerState::Restriction(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleContentContent::Restriction(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleContentDeserializerState::Restriction(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (SimpleContentDeserializerState::Extension(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(SimpleContentContent::Extension(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = SimpleContentDeserializerState::Extension(deserializer);
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
        ) -> Result<super::SimpleContent, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::SimpleContent {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ComplexContentDeserializer {
        id: Option<String>,
        mixed: Option<bool>,
        content: Vec<super::ComplexContentContent>,
        state: Box<ComplexContentDeserializerState>,
    }
    #[derive(Debug)]
    enum ComplexContentDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Restriction(<RestrictionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Extension(<ExtensionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl ComplexContentDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut mixed: Option<bool> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"mixed")) {
                    reader.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                mixed: mixed,
                content: Vec::new(),
                state: Box::new(ComplexContentDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ComplexContent>
        for ComplexContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ComplexContent, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ComplexContent, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, ComplexContentDeserializerState::Next__),
                &event,
            ) {
                (ComplexContentDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexContentContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexContentDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"restriction")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <RestrictionType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(ComplexContentContent::Restriction(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ComplexContentDeserializerState::Restriction(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"extension")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ExtensionType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ComplexContentContent::Extension(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ComplexContentDeserializerState::Extension(deserializer);
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
                (ComplexContentDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (ComplexContentDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (ComplexContentDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexContentContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexContentDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexContentDeserializerState::Restriction(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexContentContent::Restriction(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexContentDeserializerState::Restriction(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ComplexContentDeserializerState::Extension(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ComplexContentContent::Extension(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ComplexContentDeserializerState::Extension(deserializer);
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
        ) -> Result<super::ComplexContent, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ComplexContent {
                id: self.id,
                mixed: self.mixed,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct OpenContentDeserializer {
        id: Option<String>,
        mode: super::OpenContentModeType,
        annotation: Option<super::Annotation>,
        any: Option<super::WildcardType>,
        state: Box<OpenContentDeserializerState>,
    }
    #[derive(Debug)]
    enum OpenContentDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Any(Option<<WildcardType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl OpenContentDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut mode: Option<OpenContentModeType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"mode")) {
                    reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                mode: mode.unwrap_or_else(super::OpenContent::default_mode),
                annotation: None,
                any: None,
                state: Box::new(OpenContentDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::OpenContent> for OpenContentDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::OpenContent, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::OpenContent, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, OpenContentDeserializerState::Done__),
                    event,
                ) {
                    (OpenContentDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    OpenContentDeserializerState::Annotation(deserializer);
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
                                OpenContentDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = OpenContentDeserializerState::Annotation(None);
                        event
                    }
                    (OpenContentDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = OpenContentDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = OpenContentDeserializerState::Any(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            OpenContentDeserializerState::Annotation(None),
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
                            *self.state = OpenContentDeserializerState::Any(None);
                            allow_any_fallback
                                .get_or_insert(OpenContentDeserializerState::Annotation(None));
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
                            *self.state = OpenContentDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (OpenContentDeserializerState::Any(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.any.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
                            }
                            self.any = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = OpenContentDeserializerState::Any(deserializer);
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
                                .get_or_insert(OpenContentDeserializerState::Any(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.any.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
                            }
                            self.any = Some(data);
                        }
                        *self.state = OpenContentDeserializerState::Any(None);
                        event
                    }
                    (OpenContentDeserializerState::Any(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"any")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <WildcardType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.any.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"any",
                                    )))?;
                                }
                                self.any = Some(data);
                            }
                            *self.state = OpenContentDeserializerState::Any(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = OpenContentDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(OpenContentDeserializerState::Any(None));
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
                            *self.state = OpenContentDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(OpenContentDeserializerState::Any(None));
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
                            *self.state = OpenContentDeserializerState::Any(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (OpenContentDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::OpenContent, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::OpenContent {
                id: self.id,
                mode: self.mode,
                annotation: self.annotation,
                any: self.any,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyAttributeDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        not_q_name: Option<super::QnameListAType>,
        annotation: Option<super::Annotation>,
        state: Box<AnyAttributeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyAttributeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl AnyAttributeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut namespace: Option<NamespaceListType> = None;
            let mut not_namespace: Option<NotNamespaceType> = None;
            let mut process_contents: Option<ProcessContentsType> = None;
            let mut not_q_name: Option<QnameListAType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"notNamespace")
                ) {
                    reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"processContents")
                ) {
                    reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"notQName")
                ) {
                    reader.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::AnyAttribute::default_process_contents),
                not_q_name: not_q_name,
                annotation: None,
                state: Box::new(AnyAttributeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::AnyAttribute>
        for AnyAttributeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AnyAttribute, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AnyAttribute, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, AnyAttributeDeserializerState::Done__),
                    event,
                ) {
                    (AnyAttributeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    AnyAttributeDeserializerState::Annotation(deserializer);
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
                                AnyAttributeDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = AnyAttributeDeserializerState::Annotation(None);
                        event
                    }
                    (AnyAttributeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = AnyAttributeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = AnyAttributeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            AnyAttributeDeserializerState::Annotation(None),
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
                            *self.state = AnyAttributeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(AnyAttributeDeserializerState::Annotation(None));
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
                            *self.state = AnyAttributeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (AnyAttributeDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::AnyAttribute, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::AnyAttribute {
                id: self.id,
                namespace: self.namespace,
                not_namespace: self.not_namespace,
                process_contents: self.process_contents,
                not_q_name: self.not_q_name,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AssertionTypeDeserializer {
        id: Option<String>,
        test: Option<String>,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        annotation: Option<super::Annotation>,
        state: Box<AssertionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AssertionTypeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl AssertionTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut xpath_default_namespace: Option<XpathDefaultNamespaceType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"test")) {
                    reader.read_attrib(&mut test, b"test", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    reader.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                }
            }
            Ok(Self {
                id: id,
                test: test,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state: Box::new(AssertionTypeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::AssertionType>
        for AssertionTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AssertionType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AssertionType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, AssertionTypeDeserializerState::Done__),
                    event,
                ) {
                    (AssertionTypeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    AssertionTypeDeserializerState::Annotation(deserializer);
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
                                AssertionTypeDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = AssertionTypeDeserializerState::Annotation(None);
                        event
                    }
                    (AssertionTypeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = AssertionTypeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = AssertionTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            AssertionTypeDeserializerState::Annotation(None),
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
                            *self.state = AssertionTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(AssertionTypeDeserializerState::Annotation(None));
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
                            *self.state = AssertionTypeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (AssertionTypeDeserializerState::Done__, event) => {
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
        ) -> Result<super::AssertionType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::AssertionType {
                id: self.id,
                test: self.test,
                xpath_default_namespace: self.xpath_default_namespace,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        not_q_name: Option<super::QnameListType>,
        min_occurs: usize,
        max_occurs: super::AllNNIType,
        annotation: Option<super::Annotation>,
        state: Box<AnyDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl AnyDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut namespace: Option<NamespaceListType> = None;
            let mut not_namespace: Option<NotNamespaceType> = None;
            let mut process_contents: Option<ProcessContentsType> = None;
            let mut not_q_name: Option<QnameListType> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<AllNNIType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"notNamespace")
                ) {
                    reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"processContents")
                ) {
                    reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"notQName")
                ) {
                    reader.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"minOccurs")
                ) {
                    reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"maxOccurs")
                ) {
                    reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::Any::default_process_contents),
                not_q_name: not_q_name,
                min_occurs: min_occurs.unwrap_or_else(super::Any::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::Any::default_max_occurs),
                annotation: None,
                state: Box::new(AnyDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Any> for AnyDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Any, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Any, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, AnyDeserializerState::Done__),
                    event,
                ) {
                    (AnyDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = AnyDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(AnyDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = AnyDeserializerState::Annotation(None);
                        event
                    }
                    (AnyDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = AnyDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = AnyDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(AnyDeserializerState::Annotation(None));
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
                            *self.state = AnyDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(AnyDeserializerState::Annotation(None));
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
                            *self.state = AnyDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (AnyDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Any, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Any {
                id: self.id,
                namespace: self.namespace,
                not_namespace: self.not_namespace,
                process_contents: self.process_contents,
                not_q_name: self.not_q_name,
                min_occurs: self.min_occurs,
                max_occurs: self.max_occurs,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AltTypeDeserializer {
        id: Option<String>,
        test: Option<String>,
        type_: Option<String>,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        content: Vec<super::AltTypeContent>,
        state: Box<AltTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AltTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ComplexType(<ComplexBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl AltTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut xpath_default_namespace: Option<XpathDefaultNamespaceType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"test")) {
                    reader.read_attrib(&mut test, b"test", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"type")) {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    reader.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                }
            }
            Ok(Self {
                id: id,
                test: test,
                type_: type_,
                xpath_default_namespace: xpath_default_namespace,
                content: Vec::new(),
                state: Box::new(AltTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::AltType> for AltTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AltType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::AltType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, AltTypeDeserializerState::Next__),
                &event,
            ) {
                (AltTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(AltTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = AltTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(AltTypeContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = AltTypeDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"complexType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <ComplexBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(AltTypeContent::ComplexType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = AltTypeDeserializerState::ComplexType(deserializer);
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
                (AltTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (AltTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (AltTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(AltTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AltTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (AltTypeDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(AltTypeContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AltTypeDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (AltTypeDeserializerState::ComplexType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(AltTypeContent::ComplexType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = AltTypeDeserializerState::ComplexType(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::AltType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::AltType {
                id: self.id,
                test: self.test,
                type_: self.type_,
                xpath_default_namespace: self.xpath_default_namespace,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeybaseTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        annotation: Option<super::Annotation>,
        selector: Option<super::Field>,
        field: Vec<super::Field>,
        state: Box<KeybaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeybaseTypeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Selector(Option<<Field as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Field(Option<<Field as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl KeybaseTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"ref")) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                annotation: None,
                selector: None,
                field: Vec::new(),
                state: Box::new(KeybaseTypeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::KeybaseType> for KeybaseTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::KeybaseType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::KeybaseType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, KeybaseTypeDeserializerState::Done__),
                    event,
                ) {
                    (KeybaseTypeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    KeybaseTypeDeserializerState::Annotation(deserializer);
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
                                KeybaseTypeDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = KeybaseTypeDeserializerState::Annotation(None);
                        event
                    }
                    (KeybaseTypeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = KeybaseTypeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = KeybaseTypeDeserializerState::Selector(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            KeybaseTypeDeserializerState::Annotation(None),
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
                            *self.state = KeybaseTypeDeserializerState::Selector(None);
                            allow_any_fallback
                                .get_or_insert(KeybaseTypeDeserializerState::Annotation(None));
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
                            *self.state = KeybaseTypeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (KeybaseTypeDeserializerState::Selector(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.selector.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"selector",
                                )))?;
                            }
                            self.selector = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = KeybaseTypeDeserializerState::Selector(deserializer);
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
                                KeybaseTypeDeserializerState::Selector(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.selector.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"selector",
                                )))?;
                            }
                            self.selector = Some(data);
                        }
                        *self.state = KeybaseTypeDeserializerState::Selector(None);
                        event
                    }
                    (KeybaseTypeDeserializerState::Selector(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"selector")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Field as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.selector.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"selector",
                                    )))?;
                                }
                                self.selector = Some(data);
                            }
                            *self.state = KeybaseTypeDeserializerState::Selector(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = KeybaseTypeDeserializerState::Field(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            KeybaseTypeDeserializerState::Selector(None),
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
                            *self.state = KeybaseTypeDeserializerState::Field(None);
                            allow_any_fallback
                                .get_or_insert(KeybaseTypeDeserializerState::Selector(None));
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
                            *self.state = KeybaseTypeDeserializerState::Selector(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (KeybaseTypeDeserializerState::Field(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.field.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = KeybaseTypeDeserializerState::Field(deserializer);
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
                                .get_or_insert(KeybaseTypeDeserializerState::Field(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.field.push(data);
                        }
                        *self.state = KeybaseTypeDeserializerState::Field(None);
                        event
                    }
                    (KeybaseTypeDeserializerState::Field(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"field")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Field as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                self.field.push(data);
                            }
                            *self.state = KeybaseTypeDeserializerState::Field(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = KeybaseTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            KeybaseTypeDeserializerState::Field(None),
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
                            *self.state = KeybaseTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(KeybaseTypeDeserializerState::Field(None));
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
                            *self.state = KeybaseTypeDeserializerState::Field(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (KeybaseTypeDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::KeybaseType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::KeybaseType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                annotation: self.annotation,
                selector: self
                    .selector
                    .ok_or_else(|| ErrorKind::MissingElement("selector".into()))?,
                field: self.field,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyrefDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        refer: Option<String>,
        annotation: Option<super::Annotation>,
        selector: Option<super::Field>,
        field: Vec<super::Field>,
        state: Box<KeyrefDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyrefDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Selector(Option<<Field as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Field(Option<<Field as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl KeyrefDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut refer: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"name")) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"ref")) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"refer")) {
                    reader.read_attrib(&mut refer, b"refer", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                refer: refer,
                annotation: None,
                selector: None,
                field: Vec::new(),
                state: Box::new(KeyrefDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Keyref> for KeyrefDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Keyref, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Keyref, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, KeyrefDeserializerState::Done__),
                    event,
                ) {
                    (KeyrefDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = KeyrefDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(KeyrefDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = KeyrefDeserializerState::Annotation(None);
                        event
                    }
                    (KeyrefDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = KeyrefDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = KeyrefDeserializerState::Selector(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            KeyrefDeserializerState::Annotation(None),
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
                            *self.state = KeyrefDeserializerState::Selector(None);
                            allow_any_fallback
                                .get_or_insert(KeyrefDeserializerState::Annotation(None));
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
                            *self.state = KeyrefDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (KeyrefDeserializerState::Selector(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.selector.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"selector",
                                )))?;
                            }
                            self.selector = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = KeyrefDeserializerState::Selector(deserializer);
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
                                .get_or_insert(KeyrefDeserializerState::Selector(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.selector.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"selector",
                                )))?;
                            }
                            self.selector = Some(data);
                        }
                        *self.state = KeyrefDeserializerState::Selector(None);
                        event
                    }
                    (KeyrefDeserializerState::Selector(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"selector")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Field as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.selector.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"selector",
                                    )))?;
                                }
                                self.selector = Some(data);
                            }
                            *self.state = KeyrefDeserializerState::Selector(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = KeyrefDeserializerState::Field(None);
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(KeyrefDeserializerState::Selector(None));
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
                            *self.state = KeyrefDeserializerState::Field(None);
                            allow_any_fallback
                                .get_or_insert(KeyrefDeserializerState::Selector(None));
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
                            *self.state = KeyrefDeserializerState::Selector(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (KeyrefDeserializerState::Field(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.field.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = KeyrefDeserializerState::Field(deserializer);
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
                                .get_or_insert(KeyrefDeserializerState::Field(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.field.push(data);
                        }
                        *self.state = KeyrefDeserializerState::Field(None);
                        event
                    }
                    (KeyrefDeserializerState::Field(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"field")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Field as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                self.field.push(data);
                            }
                            *self.state = KeyrefDeserializerState::Field(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = KeyrefDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(KeyrefDeserializerState::Field(None));
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
                            *self.state = KeyrefDeserializerState::Done__;
                            allow_any_fallback.get_or_insert(KeyrefDeserializerState::Field(None));
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
                            *self.state = KeyrefDeserializerState::Field(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (KeyrefDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Keyref, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Keyref {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                refer: self.refer,
                annotation: self.annotation,
                selector: self
                    .selector
                    .ok_or_else(|| ErrorKind::MissingElement("selector".into()))?,
                field: self.field,
            })
        }
    }
    #[derive(Debug)]
    pub struct FacetDeserializer {
        content: Option<super::Facet>,
        state: Box<FacetDeserializerState>,
    }
    #[derive(Debug)]
    enum FacetDeserializerState {
        Next__,
        MinExclusive(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        MinInclusive(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        MaxExclusive(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        MaxInclusive(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        TotalDigits(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        FractionDigits(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Length(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        MinLength(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        MaxLength(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Enumeration(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        WhiteSpace(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Pattern(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Assertion(<AssertionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        ExplicitTimezone(<FacetType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FacetDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Facet> for FacetDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Facet, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            let deserializer = Self {
                content: None,
                state: Box::new(FacetDeserializerState::Next__),
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Facet, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, FacetDeserializerState::Next__),
                &event,
            ) {
                (FacetDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"minExclusive")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"minExclusive",
                                )))?;
                            }
                            self.content = Some(Facet::MinExclusive(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::MinExclusive(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"minInclusive")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"minInclusive",
                                )))?;
                            }
                            self.content = Some(Facet::MinInclusive(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::MinInclusive(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"maxExclusive")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"maxExclusive",
                                )))?;
                            }
                            self.content = Some(Facet::MaxExclusive(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::MaxExclusive(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"maxInclusive")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"maxInclusive",
                                )))?;
                            }
                            self.content = Some(Facet::MaxInclusive(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::MaxInclusive(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"totalDigits")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"totalDigits",
                                )))?;
                            }
                            self.content = Some(Facet::TotalDigits(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::TotalDigits(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"fractionDigits")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"fractionDigits",
                                )))?;
                            }
                            self.content = Some(Facet::FractionDigits(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::FractionDigits(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"length"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"length",
                                )))?;
                            }
                            self.content = Some(Facet::Length(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::Length(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"minLength")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"minLength",
                                )))?;
                            }
                            self.content = Some(Facet::MinLength(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::MinLength(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"maxLength")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"maxLength",
                                )))?;
                            }
                            self.content = Some(Facet::MaxLength(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::MaxLength(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"enumeration")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"enumeration",
                                )))?;
                            }
                            self.content = Some(Facet::Enumeration(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::Enumeration(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"whiteSpace")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"whiteSpace",
                                )))?;
                            }
                            self.content = Some(Facet::WhiteSpace(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::WhiteSpace(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"pattern"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"pattern",
                                )))?;
                            }
                            self.content = Some(Facet::Pattern(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::Pattern(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"assertion")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"assertion",
                                )))?;
                            }
                            self.content = Some(Facet::Assertion(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::Assertion(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"explicitTimezone")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"explicitTimezone",
                                )))?;
                            }
                            self.content = Some(Facet::ExplicitTimezone(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FacetDeserializerState::ExplicitTimezone(deserializer);
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
                (FacetDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }
                (FacetDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FacetDeserializerState::MinExclusive(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"minExclusive",
                            )))?;
                        }
                        self.content = Some(Facet::MinExclusive(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::MinExclusive(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::MinInclusive(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"minInclusive",
                            )))?;
                        }
                        self.content = Some(Facet::MinInclusive(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::MinInclusive(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::MaxExclusive(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"maxExclusive",
                            )))?;
                        }
                        self.content = Some(Facet::MaxExclusive(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::MaxExclusive(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::MaxInclusive(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"maxInclusive",
                            )))?;
                        }
                        self.content = Some(Facet::MaxInclusive(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::MaxInclusive(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::TotalDigits(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"totalDigits",
                            )))?;
                        }
                        self.content = Some(Facet::TotalDigits(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::TotalDigits(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::FractionDigits(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"fractionDigits",
                            )))?;
                        }
                        self.content = Some(Facet::FractionDigits(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::FractionDigits(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::Length(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"length",
                            )))?;
                        }
                        self.content = Some(Facet::Length(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::Length(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::MinLength(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"minLength",
                            )))?;
                        }
                        self.content = Some(Facet::MinLength(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::MinLength(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::MaxLength(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"maxLength",
                            )))?;
                        }
                        self.content = Some(Facet::MaxLength(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::MaxLength(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::Enumeration(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"enumeration",
                            )))?;
                        }
                        self.content = Some(Facet::Enumeration(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::Enumeration(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::WhiteSpace(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"whiteSpace",
                            )))?;
                        }
                        self.content = Some(Facet::WhiteSpace(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::WhiteSpace(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::Pattern(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"pattern",
                            )))?;
                        }
                        self.content = Some(Facet::Pattern(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::Pattern(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::Assertion(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"assertion",
                            )))?;
                        }
                        self.content = Some(Facet::Assertion(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::Assertion(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FacetDeserializerState::ExplicitTimezone(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"explicitTimezone",
                            )))?;
                        }
                        self.content = Some(Facet::ExplicitTimezone(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FacetDeserializerState::ExplicitTimezone(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::Facet, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(self
                .content
                .ok_or(xsd_parser::quick_xml::ErrorKind::MissingContent)?)
        }
    }
    #[derive(Debug)]
    pub struct RestrictionTypeDeserializer {
        id: Option<String>,
        base: String,
        content: Vec<super::RestrictionTypeContent>,
        state: Box<RestrictionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictionTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        OpenContent(<OpenContent as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        All(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Choice(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Sequence(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        SimpleType(<SimpleBaseType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Facet(<Facet as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Attribute(<AttributeType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        AnyAttribute(<AnyAttribute as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Assert(<AssertionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl RestrictionTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"base")) {
                    reader.read_attrib(&mut base, b"base", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                base: base.ok_or(ErrorKind::MissingAttribute("base".into()))?,
                content: Vec::new(),
                state: Box::new(RestrictionTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::RestrictionType>
        for RestrictionTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::RestrictionType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::RestrictionType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, RestrictionTypeDeserializerState::Next__),
                &event,
            ) {
                (RestrictionTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                RestrictionTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"openContent")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <OpenContent as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::OpenContent(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                RestrictionTypeDeserializerState::OpenContent(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionTypeDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"all")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::All(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionTypeDeserializerState::All(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"choice"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::Choice(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionTypeDeserializerState::Choice(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"sequence")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::Sequence(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionTypeDeserializerState::Sequence(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"simpleType")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SimpleBaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::SimpleType(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                RestrictionTypeDeserializerState::SimpleType(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::Attribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionTypeDeserializerState::Attribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content
                                .push(RestrictionTypeContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                RestrictionTypeDeserializerState::AttributeGroup(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"anyAttribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content
                                .push(RestrictionTypeContent::AnyAttribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                RestrictionTypeDeserializerState::AnyAttribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"assert"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(RestrictionTypeContent::Assert(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = RestrictionTypeDeserializerState::Assert(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        {
                            let mut allow_any_element = false;
                            let event = {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                                if let Some(data) = data {
                                    self.content.push(RestrictionTypeContent::Facet(data));
                                }
                                if let Some(deserializer) = deserializer {
                                    *self.state =
                                        RestrictionTypeDeserializerState::Facet(deserializer);
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
                    }
                }
                (RestrictionTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (RestrictionTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (RestrictionTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::OpenContent(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::OpenContent(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::OpenContent(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::All(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::All(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::All(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::Choice(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Choice(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Choice(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::Sequence(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Sequence(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Sequence(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::SimpleType(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::SimpleType(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::SimpleType(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::Facet(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Facet(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Facet(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::Attribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Attribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Attribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(RestrictionTypeContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            RestrictionTypeDeserializerState::AttributeGroup(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::AnyAttribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(RestrictionTypeContent::AnyAttribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::AnyAttribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (RestrictionTypeDeserializerState::Assert(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(RestrictionTypeContent::Assert(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = RestrictionTypeDeserializerState::Assert(deserializer);
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
        ) -> Result<super::RestrictionType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::RestrictionType {
                id: self.id,
                base: self.base,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExtensionTypeDeserializer {
        id: Option<String>,
        base: String,
        content: Vec<super::ExtensionTypeContent>,
        state: Box<ExtensionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExtensionTypeDeserializerState {
        Next__,
        Annotation(<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        OpenContent(<OpenContent as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Group(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        All(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Choice(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Sequence(<GroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Attribute(<AttributeType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        AttributeGroup(
            <AttributeGroupType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        AnyAttribute(<AnyAttribute as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Assert(<AssertionType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl ExtensionTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"base")) {
                    reader.read_attrib(&mut base, b"base", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                base: base.ok_or(ErrorKind::MissingAttribute("base".into()))?,
                content: Vec::new(),
                state: Box::new(ExtensionTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ExtensionType>
        for ExtensionTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ExtensionType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ExtensionType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            match (
                core::mem::replace(&mut *self.state, ExtensionTypeDeserializerState::Next__),
                &event,
            ) {
                (ExtensionTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"annotation")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::Annotation(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::Annotation(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"openContent")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <OpenContent as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::OpenContent(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::OpenContent(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"group")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::Group(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::Group(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"all")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::All(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::All(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"choice"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::Choice(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::Choice(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"sequence")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::Sequence(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::Sequence(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::Attribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::Attribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"attributeGroup")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content
                                .push(ExtensionTypeContent::AttributeGroup(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ExtensionTypeDeserializerState::AttributeGroup(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_XS),
                        Some(b"anyAttribute")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::AnyAttribute(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                ExtensionTypeDeserializerState::AnyAttribute(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_XS), Some(b"assert"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(ExtensionTypeContent::Assert(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ExtensionTypeDeserializerState::Assert(deserializer);
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
                (ExtensionTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (ExtensionTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (ExtensionTypeDeserializerState::Annotation(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::Annotation(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::Annotation(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::OpenContent(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::OpenContent(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::OpenContent(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::Group(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::Group(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::Group(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::All(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::All(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::All(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::Choice(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::Choice(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::Choice(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::Sequence(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::Sequence(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::Sequence(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::Attribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::Attribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::Attribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::AttributeGroup(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content
                            .push(ExtensionTypeContent::AttributeGroup(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::AttributeGroup(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::AnyAttribute(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::AnyAttribute(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::AnyAttribute(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (ExtensionTypeDeserializerState::Assert(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(ExtensionTypeContent::Assert(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ExtensionTypeDeserializerState::Assert(deserializer);
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
        ) -> Result<super::ExtensionType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ExtensionType {
                id: self.id,
                base: self.base,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct FieldDeserializer {
        id: Option<String>,
        xpath: String,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        annotation: Option<super::Annotation>,
        state: Box<FieldDeserializerState>,
    }
    #[derive(Debug)]
    enum FieldDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl FieldDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut xpath: Option<String> = None;
            let mut xpath_default_namespace: Option<XpathDefaultNamespaceType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"xpath")) {
                    reader.read_attrib(&mut xpath, b"xpath", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    reader.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                }
            }
            Ok(Self {
                id: id,
                xpath: xpath.ok_or(ErrorKind::MissingAttribute("xpath".into()))?,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state: Box::new(FieldDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Field> for FieldDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Field, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Field, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, FieldDeserializerState::Done__),
                    event,
                ) {
                    (FieldDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FieldDeserializerState::Annotation(deserializer);
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
                                .get_or_insert(FieldDeserializerState::Annotation(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = FieldDeserializerState::Annotation(None);
                        event
                    }
                    (FieldDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = FieldDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FieldDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FieldDeserializerState::Annotation(None),
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
                            *self.state = FieldDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(FieldDeserializerState::Annotation(None));
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
                            *self.state = FieldDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FieldDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::Field, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Field {
                id: self.id,
                xpath: self.xpath,
                xpath_default_namespace: self.xpath_default_namespace,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct FacetTypeDeserializer {
        id: Option<String>,
        value: String,
        fixed: bool,
        annotation: Option<super::Annotation>,
        state: Box<FacetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FacetTypeDeserializerState {
        Annotation(Option<<Annotation as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl FacetTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut id: Option<String> = None;
            let mut value: Option<String> = None;
            let mut fixed: Option<bool> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"id")) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"value")) {
                    reader.read_attrib(&mut value, b"value", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_XS), Some(b"fixed")) {
                    reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                value: value.ok_or(ErrorKind::MissingAttribute("value".into()))?,
                fixed: fixed.unwrap_or_else(super::FacetType::default_fixed),
                annotation: None,
                state: Box::new(FacetTypeDeserializerState::Annotation(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FacetType> for FacetTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FacetType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FacetType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_XS: &[u8] = b"http://www.w3.org/2001/XMLSchema";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, FacetTypeDeserializerState::Done__),
                    event,
                ) {
                    (FacetTypeDeserializerState::Annotation(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FacetTypeDeserializerState::Annotation(deserializer);
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
                                FacetTypeDeserializerState::Annotation(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.annotation.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"annotation",
                                )))?;
                            }
                            self.annotation = Some(data);
                        }
                        *self.state = FacetTypeDeserializerState::Annotation(None);
                        event
                    }
                    (FacetTypeDeserializerState::Annotation(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_XS),
                                Some(b"annotation")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Annotation as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.annotation.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"annotation",
                                    )))?;
                                }
                                self.annotation = Some(data);
                            }
                            *self.state = FacetTypeDeserializerState::Annotation(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FacetTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FacetTypeDeserializerState::Annotation(None),
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
                            *self.state = FacetTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(FacetTypeDeserializerState::Annotation(None));
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
                            *self.state = FacetTypeDeserializerState::Annotation(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FacetTypeDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::FacetType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FacetType {
                id: self.id,
                value: self.value,
                fixed: self.fixed,
                annotation: self.annotation,
            })
        }
    }
}
