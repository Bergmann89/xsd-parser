pub type Schema = SchemaElementType;
#[derive(Debug, Clone)]
pub struct SchemaElementType {
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
    pub content: Vec<SchemaElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum SchemaElementTypeContent {
    Include(IncludeElementType),
    Import(ImportElementType),
    Redefine(RedefineElementType),
    Override(OverrideElementType),
    Annotation(AnnotationElementType),
    DefaultOpenContent(DefaultOpenContentElementType),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
    Element(ElementType),
    Attribute(AttributeType),
    Notation(NotationElementType),
}
impl xsd_parser::WithNamespace for SchemaElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl SchemaElementType {
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
#[derive(Debug, Clone)]
pub struct IncludeElementType {
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for IncludeElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct ImportElementType {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for ImportElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct RedefineElementType {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<RedefineElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum RedefineElementTypeContent {
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}
impl xsd_parser::WithNamespace for RedefineElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct OverrideElementType {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<OverrideElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum OverrideElementTypeContent {
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
    Element(ElementType),
    Attribute(AttributeType),
    Notation(NotationElementType),
}
impl xsd_parser::WithNamespace for OverrideElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct AnnotationElementType {
    pub id: Option<String>,
    pub content: Vec<AnnotationElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum AnnotationElementTypeContent {
    Appinfo(AppinfoElementType),
    Documentation(DocumentationElementType),
}
impl xsd_parser::WithNamespace for AnnotationElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct DefaultOpenContentElementType {
    pub id: Option<String>,
    pub applies_to_empty: bool,
    pub mode: DefaultOpenContentModeType,
    pub annotation: Option<AnnotationElementType>,
    pub any: WildcardType,
}
impl xsd_parser::WithNamespace for DefaultOpenContentElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl DefaultOpenContentElementType {
    #[must_use]
    pub fn default_applies_to_empty() -> bool {
        false
    }
    #[must_use]
    pub fn default_mode() -> DefaultOpenContentModeType {
        DefaultOpenContentModeType::Interleave
    }
}
#[derive(Debug, Clone)]
pub struct SimpleBaseType {
    pub id: Option<String>,
    pub final_: Option<SimpleDerivationSetType>,
    pub name: Option<String>,
    pub content: Option<SimpleBaseTypeContent>,
}
#[derive(Debug, Clone)]
pub enum SimpleBaseTypeContent {
    Annotation(AnnotationElementType),
    Restriction(RestrictionElementType),
    List(Box<ListElementType>),
    Union(UnionElementType),
}
impl xsd_parser::WithNamespace for SimpleBaseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
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
    Annotation(AnnotationElementType),
    SimpleContent(SimpleContentElementType),
    ComplexContent(ComplexContentElementType),
    OpenContent(OpenContentElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttributeElementType),
    Assert(AssertionType),
}
impl xsd_parser::WithNamespace for ComplexBaseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    Annotation(AnnotationElementType),
    Element(ElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Any(AnyElementType),
}
impl xsd_parser::WithNamespace for GroupType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
#[derive(Debug, Clone)]
pub struct AttributeGroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub content: Vec<AttributeGroupTypeContent>,
}
#[derive(Debug, Clone)]
pub enum AttributeGroupTypeContent {
    Annotation(AnnotationElementType),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttributeElementType),
}
impl xsd_parser::WithNamespace for AttributeGroupType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct ElementType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub type_: Option<String>,
    pub substitution_group: Option<Entitiestype>,
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
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Alternative(AltType),
    Unique(KeybaseType),
    Key(KeybaseType),
    Keyref(KeyrefElementType),
}
impl xsd_parser::WithNamespace for ElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Option<SimpleBaseType>,
}
impl xsd_parser::WithNamespace for AttributeType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl AttributeType {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}
#[derive(Debug, Clone)]
pub struct NotationElementType {
    pub id: Option<String>,
    pub name: String,
    pub public: Option<String>,
    pub system: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for NotationElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
pub struct AppinfoElementType {
    pub source: Option<String>,
}
impl xsd_parser::WithNamespace for AppinfoElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct DocumentationElementType {
    pub source: Option<String>,
    pub lang: Option<String>,
}
impl xsd_parser::WithNamespace for DocumentationElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct WildcardType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for WildcardType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl WildcardType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
pub struct RestrictionElementType {
    pub id: Option<String>,
    pub base: Option<String>,
    pub content: Vec<RestrictionElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum RestrictionElementTypeContent {
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    Facet(Facet),
}
impl xsd_parser::WithNamespace for RestrictionElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct ListElementType {
    pub id: Option<String>,
    pub item_type: Option<String>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Option<Box<SimpleBaseType>>,
}
impl xsd_parser::WithNamespace for ListElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct UnionElementType {
    pub id: Option<String>,
    pub member_types: Option<Entitiestype>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Vec<SimpleBaseType>,
}
impl xsd_parser::WithNamespace for UnionElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
pub struct SimpleContentElementType {
    pub id: Option<String>,
    pub content: SimpleContentElementTypeContent,
}
#[derive(Debug, Clone)]
pub enum SimpleContentElementTypeContent {
    Annotation(AnnotationElementType),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl xsd_parser::WithNamespace for SimpleContentElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct ComplexContentElementType {
    pub id: Option<String>,
    pub mixed: Option<bool>,
    pub content: Option<ComplexContentElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ComplexContentElementTypeContent {
    Annotation(AnnotationElementType),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl xsd_parser::WithNamespace for ComplexContentElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct OpenContentElementType {
    pub id: Option<String>,
    pub mode: OpenContentModeType,
    pub annotation: Option<AnnotationElementType>,
    pub any: Option<WildcardType>,
}
impl xsd_parser::WithNamespace for OpenContentElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl OpenContentElementType {
    #[must_use]
    pub fn default_mode() -> OpenContentModeType {
        OpenContentModeType::Interleave
    }
}
#[derive(Debug, Clone)]
pub struct AnyAttributeElementType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListAType>,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for AnyAttributeElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl AnyAttributeElementType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
#[derive(Debug, Clone)]
pub struct AssertionType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for AssertionType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
pub struct AnyElementType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListType>,
    pub min_occurs: usize,
    pub max_occurs: AllNNIType,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for AnyElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl AnyElementType {
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
#[derive(Debug, Clone)]
pub enum AllNNIType {
    Usize(usize),
    Unbounded,
}
impl xsd_parser::WithNamespace for AllNNIType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    pub content: Option<AltTypeContent>,
}
#[derive(Debug, Clone)]
pub enum AltTypeContent {
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
}
impl xsd_parser::WithNamespace for AltType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct KeybaseType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub content: Option<KeybaseTypeContent>,
}
#[derive(Debug, Clone)]
pub struct KeybaseTypeContent {
    pub annotation: Option<AnnotationElementType>,
    pub selector: FieldElementType,
    pub field: Vec<FieldElementType>,
}
impl xsd_parser::WithNamespace for KeybaseType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct KeyrefElementType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub refer: Option<String>,
    pub content: Option<KeyrefElementTypeContent>,
}
#[derive(Debug, Clone)]
pub struct KeyrefElementTypeContent {
    pub annotation: Option<AnnotationElementType>,
    pub selector: FieldElementType,
    pub field: Vec<FieldElementType>,
}
impl xsd_parser::WithNamespace for KeyrefElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone, Default)]
pub struct Entitiestype(pub Vec<String>);
impl xsd_parser::WithNamespace for Entitiestype {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for Entitiestype {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| String::deserialize_bytes(reader, bytes))
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
#[derive(Debug, Clone)]
pub enum TypeDerivationControlType {
    Extension,
    Restriction,
    List,
    Union,
}
impl xsd_parser::WithNamespace for TypeDerivationControlType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for TypeDerivationControlType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
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
#[derive(Debug, Clone)]
pub enum BlockSetItemType {
    Extension,
    Restriction,
    Substitution,
}
impl xsd_parser::WithNamespace for BlockSetItemType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for BlockSetItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            b"substitution" => Ok(Self::Substitution),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone, Default)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
impl xsd_parser::WithNamespace for SimpleDerivationSetItemList {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    Annotation(AnnotationElementType),
    OpenContent(OpenContentElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    SimpleType(SimpleBaseType),
    Facet(Facet),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttributeElementType),
    Assert(AssertionType),
}
impl xsd_parser::WithNamespace for RestrictionType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone)]
pub struct ExtensionType {
    pub id: Option<String>,
    pub base: String,
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ExtensionTypeContent {
    Annotation(AnnotationElementType),
    OpenContent(OpenContentElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttributeElementType),
    Assert(AssertionType),
}
impl xsd_parser::WithNamespace for ExtensionType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
pub struct FieldElementType {
    pub id: Option<String>,
    pub xpath: String,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for FieldElementType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
#[derive(Debug, Clone, Default)]
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
impl xsd_parser::WithNamespace for BasicNamespaceListType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
    pub annotation: Option<AnnotationElementType>,
}
impl xsd_parser::WithNamespace for FacetType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl FacetType {
    #[must_use]
    pub fn default_fixed() -> bool {
        false
    }
}
#[derive(Debug, Clone)]
pub enum SimpleDerivationSetItemType {
    List,
    Union,
    Restriction,
    Extension,
}
impl xsd_parser::WithNamespace for SimpleDerivationSetItemType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for SimpleDerivationSetItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"list" => Ok(Self::List),
            b"union" => Ok(Self::Union),
            b"restriction" => Ok(Self::Restriction),
            b"extension" => Ok(Self::Extension),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum QnameListAItemType {
    String(String),
    Defined,
}
impl xsd_parser::WithNamespace for QnameListAItemType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
#[derive(Debug, Clone)]
pub enum ReducedDerivationControlType {
    Extension,
    Restriction,
}
impl xsd_parser::WithNamespace for ReducedDerivationControlType {
    fn prefix() -> Option<&'static str> {
        Some("xs")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for ReducedDerivationControlType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
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
    fn namespace() -> Option<&'static str> {
        Some("http://www.w3.org/2001/XMLSchema")
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
}
