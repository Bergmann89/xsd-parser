pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
use xsd_parser::{
    quick_xml::{
        deserialize_new::{DeserializeBytes, DeserializeReader, WithDeserializer},
        Error, ErrorKind, RawByteStr,
    },
    schema::Namespace,
};
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
impl WithDeserializer for SchemaElementType {
    type Deserializer = quick_xml_deserialize::SchemaElementTypeDeserializer;
}
impl WithDeserializer for SchemaElementTypeContent {
    type Deserializer = quick_xml_deserialize::SchemaElementTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub enum FullDerivationSetType {
    All,
    TypeDerivationControlList(TypeDerivationControlList),
}
impl DeserializeBytes for FullDerivationSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl DeserializeBytes for TypeDerivationControlList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl DeserializeBytes for BlockSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl DeserializeBytes for BlockSetItemList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl DeserializeBytes for FormChoiceType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"qualified" => Ok(Self::Qualified),
            b"unqualified" => Ok(Self::Unqualified),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
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
impl DeserializeBytes for XpathDefaultNamespaceType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
pub struct IncludeElementType {
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for IncludeElementType {
    type Deserializer = quick_xml_deserialize::IncludeElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ImportElementType {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for ImportElementType {
    type Deserializer = quick_xml_deserialize::ImportElementTypeDeserializer;
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
impl WithDeserializer for RedefineElementType {
    type Deserializer = quick_xml_deserialize::RedefineElementTypeDeserializer;
}
impl WithDeserializer for RedefineElementTypeContent {
    type Deserializer = quick_xml_deserialize::RedefineElementTypeContentDeserializer;
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
impl WithDeserializer for OverrideElementType {
    type Deserializer = quick_xml_deserialize::OverrideElementTypeDeserializer;
}
impl WithDeserializer for OverrideElementTypeContent {
    type Deserializer = quick_xml_deserialize::OverrideElementTypeContentDeserializer;
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
impl WithDeserializer for AnnotationElementType {
    type Deserializer = quick_xml_deserialize::AnnotationElementTypeDeserializer;
}
impl WithDeserializer for AnnotationElementTypeContent {
    type Deserializer = quick_xml_deserialize::AnnotationElementTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct DefaultOpenContentElementType {
    pub id: Option<String>,
    pub applies_to_empty: bool,
    pub mode: DefaultOpenContentModeType,
    pub annotation: Option<AnnotationElementType>,
    pub any: WildcardType,
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
impl WithDeserializer for DefaultOpenContentElementType {
    type Deserializer = quick_xml_deserialize::DefaultOpenContentElementTypeDeserializer;
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
    Annotation(AnnotationElementType),
    Restriction(RestrictionElementType),
    List(ListElementType),
    Union(UnionElementType),
}
impl WithDeserializer for SimpleBaseType {
    type Deserializer = quick_xml_deserialize::SimpleBaseTypeDeserializer;
}
impl WithDeserializer for SimpleBaseTypeContent {
    type Deserializer = quick_xml_deserialize::SimpleBaseTypeContentDeserializer;
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
impl WithDeserializer for ComplexBaseType {
    type Deserializer = quick_xml_deserialize::ComplexBaseTypeDeserializer;
}
impl WithDeserializer for ComplexBaseTypeContent {
    type Deserializer = quick_xml_deserialize::ComplexBaseTypeContentDeserializer;
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
impl WithDeserializer for GroupType {
    type Deserializer = quick_xml_deserialize::GroupTypeDeserializer;
}
impl WithDeserializer for GroupTypeContent {
    type Deserializer = quick_xml_deserialize::GroupTypeContentDeserializer;
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
impl WithDeserializer for AttributeGroupType {
    type Deserializer = quick_xml_deserialize::AttributeGroupTypeDeserializer;
}
impl WithDeserializer for AttributeGroupTypeContent {
    type Deserializer = quick_xml_deserialize::AttributeGroupTypeContentDeserializer;
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
impl WithDeserializer for ElementType {
    type Deserializer = quick_xml_deserialize::ElementTypeDeserializer;
}
impl WithDeserializer for ElementTypeContent {
    type Deserializer = quick_xml_deserialize::ElementTypeContentDeserializer;
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
impl AttributeType {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}
impl WithDeserializer for AttributeType {
    type Deserializer = quick_xml_deserialize::AttributeTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct NotationElementType {
    pub id: Option<String>,
    pub name: String,
    pub public: Option<String>,
    pub system: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for NotationElementType {
    type Deserializer = quick_xml_deserialize::NotationElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum TypeDerivationControlType {
    Extension,
    Restriction,
    List,
    Union,
}
impl DeserializeBytes for TypeDerivationControlType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            b"list" => Ok(Self::List),
            b"union" => Ok(Self::Union),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BlockSetItemType {
    Extension,
    Restriction,
    Substitution,
}
impl DeserializeBytes for BlockSetItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            b"substitution" => Ok(Self::Substitution),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AppinfoElementType {
    pub source: Option<String>,
}
impl WithDeserializer for AppinfoElementType {
    type Deserializer = quick_xml_deserialize::AppinfoElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DocumentationElementType {
    pub source: Option<String>,
    pub lang: Option<String>,
}
impl WithDeserializer for DocumentationElementType {
    type Deserializer = quick_xml_deserialize::DocumentationElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum DefaultOpenContentModeType {
    Interleave,
    Suffix,
}
impl DeserializeBytes for DefaultOpenContentModeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"interleave" => Ok(Self::Interleave),
            b"suffix" => Ok(Self::Suffix),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WildcardType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: ProcessContentsType,
    pub annotation: Option<AnnotationElementType>,
}
impl WildcardType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
impl WithDeserializer for WildcardType {
    type Deserializer = quick_xml_deserialize::WildcardTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum SimpleDerivationSetType {
    All,
    SimpleDerivationSetItemList(SimpleDerivationSetItemList),
}
impl DeserializeBytes for SimpleDerivationSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl WithDeserializer for RestrictionElementType {
    type Deserializer = quick_xml_deserialize::RestrictionElementTypeDeserializer;
}
impl WithDeserializer for RestrictionElementTypeContent {
    type Deserializer = quick_xml_deserialize::RestrictionElementTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct ListElementType {
    pub id: Option<String>,
    pub item_type: Option<String>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Option<SimpleBaseType>,
}
impl WithDeserializer for ListElementType {
    type Deserializer = quick_xml_deserialize::ListElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct UnionElementType {
    pub id: Option<String>,
    pub member_types: Option<Entitiestype>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Vec<SimpleBaseType>,
}
impl WithDeserializer for UnionElementType {
    type Deserializer = quick_xml_deserialize::UnionElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum DerivationSetType {
    All,
    ReducedDerivationControlList(ReducedDerivationControlList),
}
impl DeserializeBytes for DerivationSetType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
pub struct SimpleContentElementType {
    pub id: Option<String>,
    pub content: Vec<SimpleContentElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum SimpleContentElementTypeContent {
    Annotation(AnnotationElementType),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl WithDeserializer for SimpleContentElementType {
    type Deserializer = quick_xml_deserialize::SimpleContentElementTypeDeserializer;
}
impl WithDeserializer for SimpleContentElementTypeContent {
    type Deserializer = quick_xml_deserialize::SimpleContentElementTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct ComplexContentElementType {
    pub id: Option<String>,
    pub mixed: Option<bool>,
    pub content: Vec<ComplexContentElementTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ComplexContentElementTypeContent {
    Annotation(AnnotationElementType),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl WithDeserializer for ComplexContentElementType {
    type Deserializer = quick_xml_deserialize::ComplexContentElementTypeDeserializer;
}
impl WithDeserializer for ComplexContentElementTypeContent {
    type Deserializer = quick_xml_deserialize::ComplexContentElementTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct OpenContentElementType {
    pub id: Option<String>,
    pub mode: OpenContentModeType,
    pub annotation: Option<AnnotationElementType>,
    pub any: Option<WildcardType>,
}
impl OpenContentElementType {
    #[must_use]
    pub fn default_mode() -> OpenContentModeType {
        OpenContentModeType::Interleave
    }
}
impl WithDeserializer for OpenContentElementType {
    type Deserializer = quick_xml_deserialize::OpenContentElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct AnyAttributeElementType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListAType>,
    pub annotation: Option<AnnotationElementType>,
}
impl AnyAttributeElementType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
impl WithDeserializer for AnyAttributeElementType {
    type Deserializer = quick_xml_deserialize::AnyAttributeElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct AssertionType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for AssertionType {
    type Deserializer = quick_xml_deserialize::AssertionTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum AllNNIType {
    Usize(usize),
    Unbounded,
}
impl DeserializeBytes for AllNNIType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"unbounded" => Ok(Self::Unbounded),
            x => Ok(Self::Usize(usize::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnyElementType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListType>,
    pub min_occurs: usize,
    pub max_occurs: AllNNIType,
    pub annotation: Option<AnnotationElementType>,
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
impl WithDeserializer for AnyElementType {
    type Deserializer = quick_xml_deserialize::AnyElementTypeDeserializer;
}
#[derive(Debug, Clone, Default)]
pub struct Entitiestype(pub Vec<String>);
impl DeserializeBytes for Entitiestype {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
pub struct AltType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub type_: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub content: Vec<AltTypeContent>,
}
#[derive(Debug, Clone)]
pub enum AltTypeContent {
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
}
impl WithDeserializer for AltType {
    type Deserializer = quick_xml_deserialize::AltTypeDeserializer;
}
impl WithDeserializer for AltTypeContent {
    type Deserializer = quick_xml_deserialize::AltTypeContentDeserializer;
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
impl WithDeserializer for KeybaseType {
    type Deserializer = quick_xml_deserialize::KeybaseTypeDeserializer;
}
impl WithDeserializer for KeybaseTypeContent {
    type Deserializer = quick_xml_deserialize::KeybaseTypeContentDeserializer;
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
impl WithDeserializer for KeyrefElementType {
    type Deserializer = quick_xml_deserialize::KeyrefElementTypeDeserializer;
}
impl WithDeserializer for KeyrefElementTypeContent {
    type Deserializer = quick_xml_deserialize::KeyrefElementTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub enum AttributeUseType {
    Prohibited,
    Optional,
    Required,
}
impl DeserializeBytes for AttributeUseType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"prohibited" => Ok(Self::Prohibited),
            b"optional" => Ok(Self::Optional),
            b"required" => Ok(Self::Required),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceListType {
    Any,
    Other,
    BasicNamespaceList(BasicNamespaceListType),
}
impl DeserializeBytes for NamespaceListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
impl DeserializeBytes for BasicNamespaceListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl DeserializeBytes for ProcessContentsType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"skip" => Ok(Self::Skip),
            b"lax" => Ok(Self::Lax),
            b"strict" => Ok(Self::Strict),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
impl DeserializeBytes for SimpleDerivationSetItemList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl WithDeserializer for Facet {
    type Deserializer = quick_xml_deserialize::FacetDeserializer;
}
#[derive(Debug, Clone, Default)]
pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
impl DeserializeBytes for ReducedDerivationControlList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| ReducedDerivationControlType::deserialize_bytes(reader, bytes))
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
impl WithDeserializer for RestrictionType {
    type Deserializer = quick_xml_deserialize::RestrictionTypeDeserializer;
}
impl WithDeserializer for RestrictionTypeContent {
    type Deserializer = quick_xml_deserialize::RestrictionTypeContentDeserializer;
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
impl WithDeserializer for ExtensionType {
    type Deserializer = quick_xml_deserialize::ExtensionTypeDeserializer;
}
impl WithDeserializer for ExtensionTypeContent {
    type Deserializer = quick_xml_deserialize::ExtensionTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub enum OpenContentModeType {
    None,
    Interleave,
    Suffix,
}
impl DeserializeBytes for OpenContentModeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"none" => Ok(Self::None),
            b"interleave" => Ok(Self::Interleave),
            b"suffix" => Ok(Self::Suffix),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct QnameListAType(pub Vec<QnameListAItemType>);
impl DeserializeBytes for QnameListAType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
pub struct QnameListType(pub Vec<QnameListItemType>);
impl DeserializeBytes for QnameListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
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
impl WithDeserializer for FieldElementType {
    type Deserializer = quick_xml_deserialize::FieldElementTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum BasicNamespaceListItemType {
    String(String),
    TargetNamespace,
    Local,
}
impl DeserializeBytes for BasicNamespaceListItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"##targetNamespace" => Ok(Self::TargetNamespace),
            b"##local" => Ok(Self::Local),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleDerivationSetItemType {
    List,
    Union,
    Restriction,
    Extension,
}
impl DeserializeBytes for SimpleDerivationSetItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"list" => Ok(Self::List),
            b"union" => Ok(Self::Union),
            b"restriction" => Ok(Self::Restriction),
            b"extension" => Ok(Self::Extension),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
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
impl FacetType {
    #[must_use]
    pub fn default_fixed() -> bool {
        false
    }
}
impl WithDeserializer for FacetType {
    type Deserializer = quick_xml_deserialize::FacetTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum ReducedDerivationControlType {
    Extension,
    Restriction,
}
impl DeserializeBytes for ReducedDerivationControlType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QnameListAItemType {
    String(String),
    Defined,
}
impl DeserializeBytes for QnameListAItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QnameListItemType {
    String(String),
    Defined,
    DefinedSibling,
}
impl DeserializeBytes for QnameListItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            b"##definedSibling" => Ok(Self::DefinedSibling),
            x => Ok(Self::String(String::deserialize_bytes(reader, x)?)),
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        deserialize_new::{
            DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, WithDeserializer,
        },
        filter_xmlns_attributes, BytesStart, Error, ErrorKind, Event, RawByteStr,
    };
    #[derive(Debug)]
    pub struct SchemaElementTypeDeserializer {
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
        content: Vec<super::SchemaElementTypeContent>,
        state: Box<SchemaElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SchemaElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SchemaElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SchemaElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut target_namespace: Option<String> = None;
            let mut version: Option<String> = None;
            let mut final_default: Option<super::FullDerivationSetType> = None;
            let mut block_default: Option<super::BlockSetType> = None;
            let mut attribute_form_default: Option<super::FormChoiceType> = None;
            let mut element_form_default: Option<super::FormChoiceType> = None;
            let mut default_attributes: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            let mut id: Option<String> = None;
            let mut lang: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"targetNamespace")
                ) {
                    reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"version")
                ) {
                    reader.read_attrib(&mut version, b"version", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"finalDefault")
                ) {
                    reader.read_attrib(&mut final_default, b"finalDefault", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"blockDefault")
                ) {
                    reader.read_attrib(&mut block_default, b"blockDefault", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"attributeFormDefault")
                ) {
                    reader.read_attrib(
                        &mut attribute_form_default,
                        b"attributeFormDefault",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"elementFormDefault")
                ) {
                    reader.read_attrib(
                        &mut element_form_default,
                        b"elementFormDefault",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"defaultAttributes")
                ) {
                    reader.read_attrib(
                        &mut default_attributes,
                        b"defaultAttributes",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    reader.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XML),
                    Some(b"lang")
                ) {
                    reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
                }
            }
            Ok(Self {
                target_namespace: target_namespace,
                version: version,
                final_default: final_default
                    .unwrap_or_else(super::SchemaElementType::default_final_default),
                block_default: block_default
                    .unwrap_or_else(super::SchemaElementType::default_block_default),
                attribute_form_default: attribute_form_default
                    .unwrap_or_else(super::SchemaElementType::default_attribute_form_default),
                element_form_default: element_form_default
                    .unwrap_or_else(super::SchemaElementType::default_element_form_default),
                default_attributes: default_attributes,
                xpath_default_namespace: xpath_default_namespace
                    .unwrap_or_else(super::SchemaElementType::default_xpath_default_namespace),
                id: id,
                lang: lang,
                content: Vec::new(),
                state: Box::new(SchemaElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SchemaElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SchemaElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SchemaElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SchemaElementTypeContent>,
            fallback: &mut Option<SchemaElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(SchemaElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SchemaElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SchemaElementTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = SchemaElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SchemaElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SchemaElementType> for SchemaElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementType>
        where
            R: DeserializeReader,
        {
            use SchemaElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: SchemaElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SchemaElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SchemaElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SchemaElementType {
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
    pub enum SchemaElementTypeContentDeserializer {
        Init__,
        Include(
            Option<super::IncludeElementType>,
            Option<<super::IncludeElementType as WithDeserializer>::Deserializer>,
        ),
        Import(
            Option<super::ImportElementType>,
            Option<<super::ImportElementType as WithDeserializer>::Deserializer>,
        ),
        Redefine(
            Option<super::RedefineElementType>,
            Option<<super::RedefineElementType as WithDeserializer>::Deserializer>,
        ),
        Override(
            Option<super::OverrideElementType>,
            Option<<super::OverrideElementType as WithDeserializer>::Deserializer>,
        ),
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        DefaultOpenContent(
            Option<super::DefaultOpenContentElementType>,
            Option<<super::DefaultOpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        Element(
            Option<super::ElementType>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        Notation(
            Option<super::NotationElementType>,
            Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SchemaElementTypeContent),
        Unknown__,
    }
    impl SchemaElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<SchemaElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"include")
            ) {
                let output = <super::IncludeElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"import")
            ) {
                let output = <super::ImportElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_import(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"redefine")
            ) {
                let output = <super::RedefineElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_redefine(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"override")
            ) {
                let output = <super::OverrideElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_override_(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"defaultOpenContent")
            ) {
                let output =
                    <super::DefaultOpenContentElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_default_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"element")
            ) {
                let output =
                    <super::ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"notation")
            ) {
                let output = <super::NotationElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_notation(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_include(
            values: &mut Option<super::IncludeElementType>,
            value: super::IncludeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"include",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_import(
            values: &mut Option<super::ImportElementType>,
            value: super::ImportElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"import",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_redefine(
            values: &mut Option<super::RedefineElementType>,
            value: super::RedefineElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"redefine",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_override_(
            values: &mut Option<super::OverrideElementType>,
            value: super::OverrideElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"override",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_default_open_content(
            values: &mut Option<super::DefaultOpenContentElementType>,
            value: super::DefaultOpenContentElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"defaultOpenContent",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_complex_type(
            values: &mut Option<super::ComplexBaseType>,
            value: super::ComplexBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"complexType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_element(
            values: &mut Option<super::ElementType>,
            value: super::ElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"element",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute(
            values: &mut Option<super::AttributeType>,
            value: super::AttributeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_notation(
            values: &mut Option<super::NotationElementType>,
            value: super::NotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"notation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_include<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::IncludeElementType>,
            output: DeserializerOutput<'de, super::IncludeElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Include(_, Some(deserializer))) => {
                        Self::Include(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Include(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_include(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_include(&mut values, data)?;
                    let data = Self::Include(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Include(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_import<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ImportElementType>,
            output: DeserializerOutput<'de, super::ImportElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Import(_, Some(deserializer))) => {
                        Self::Import(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Import(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_import(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_import(&mut values, data)?;
                    let data = Self::Import(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Import(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_redefine<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RedefineElementType>,
            output: DeserializerOutput<'de, super::RedefineElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Redefine(_, Some(deserializer))) => {
                        Self::Redefine(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Redefine(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_redefine(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_redefine(&mut values, data)?;
                    let data = Self::Redefine(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Redefine(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_override_<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OverrideElementType>,
            output: DeserializerOutput<'de, super::OverrideElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Override(_, Some(deserializer))) => {
                        Self::Override(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Override(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_override_(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_override_(&mut values, data)?;
                    let data = Self::Override(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Override(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_default_open_content<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DefaultOpenContentElementType>,
            output: DeserializerOutput<'de, super::DefaultOpenContentElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::DefaultOpenContent(_, Some(deserializer))) => {
                        Self::DefaultOpenContent(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::DefaultOpenContent(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_default_open_content(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_default_open_content(&mut values, data)?;
                    let data = Self::DefaultOpenContent(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::DefaultOpenContent(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_complex_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ComplexBaseType>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ComplexType(_, Some(deserializer))) => {
                        Self::ComplexType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ComplexType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_complex_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::ComplexType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ComplexType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_element<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ElementType>,
            output: DeserializerOutput<'de, super::ElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element(_, Some(deserializer))) => {
                        Self::Element(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element(&mut values, data)?;
                    let data = Self::Element(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Element(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeType>,
            output: DeserializerOutput<'de, super::AttributeType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Attribute(_, Some(deserializer))) => {
                        Self::Attribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Attribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::Attribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Attribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_notation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::NotationElementType>,
            output: DeserializerOutput<'de, super::NotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Notation(_, Some(deserializer))) => {
                        Self::Notation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Notation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_notation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_notation(&mut values, data)?;
                    let data = Self::Notation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Notation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SchemaElementTypeContent>
        for SchemaElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Include(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_include(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Import(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_import(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Redefine(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_redefine(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Override(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_override_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::DefaultOpenContent(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_default_open_content(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Notation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_notation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Include(values, None), event) => {
                        let output =
                            <super::IncludeElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_include(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Import(values, None), event) => {
                        let output =
                            <super::ImportElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_import(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Redefine(values, None), event) => {
                        let output =
                            <super::RedefineElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_redefine(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Override(values, None), event) => {
                        let output =
                            <super::OverrideElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_override_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::DefaultOpenContent(values, None), event) => {
                        let output = < super :: DefaultOpenContentElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_default_open_content(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, None), event) => {
                        let output =
                            <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element(values, None), event) => {
                        let output = <super::ElementType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_element(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, None), event) => {
                        let output =
                            <super::AttributeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Notation(values, None), event) => {
                        let output =
                            <super::NotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_notation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::SchemaElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Include(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_include(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Include(
                        values.ok_or_else(|| ErrorKind::MissingElement("include".into()))?,
                    ))
                }
                Self::Import(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_import(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Import(values.ok_or_else(
                        || ErrorKind::MissingElement("import".into()),
                    )?))
                }
                Self::Redefine(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_redefine(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Redefine(
                        values.ok_or_else(|| ErrorKind::MissingElement("redefine".into()))?,
                    ))
                }
                Self::Override(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_override_(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Override(
                        values.ok_or_else(|| ErrorKind::MissingElement("override".into()))?,
                    ))
                }
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::DefaultOpenContent(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_default_open_content(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::DefaultOpenContent(
                        values.ok_or_else(|| {
                            ErrorKind::MissingElement("defaultOpenContent".into())
                        })?,
                    ))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::SimpleType(
                        values.ok_or_else(|| ErrorKind::MissingElement("simpleType".into()))?,
                    ))
                }
                Self::ComplexType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::ComplexType(
                        values.ok_or_else(|| ErrorKind::MissingElement("complexType".into()))?,
                    ))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Group(values.ok_or_else(
                        || ErrorKind::MissingElement("group".into()),
                    )?))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::Element(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Element(
                        values.ok_or_else(|| ErrorKind::MissingElement("element".into()))?,
                    ))
                }
                Self::Attribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Attribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                    ))
                }
                Self::Notation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_notation(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Notation(
                        values.ok_or_else(|| ErrorKind::MissingElement("notation".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct IncludeElementTypeDeserializer {
        id: Option<String>,
        schema_location: String,
        annotation: Option<super::AnnotationElementType>,
        state: Box<IncludeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IncludeElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl IncludeElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut schema_location: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                schema_location: schema_location.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("schemaLocation".into()))
                })?,
                annotation: None,
                state: Box::new(IncludeElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IncludeElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use IncludeElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<IncludeElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(IncludeElementTypeDeserializerState::Annotation(None));
                *self.state = IncludeElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = IncludeElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                IncludeElementTypeDeserializerState::Annotation(Some(deserializer)),
                            );
                            *self.state = IncludeElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                IncludeElementTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::IncludeElementType> for IncludeElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IncludeElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IncludeElementType>
        where
            R: DeserializeReader,
        {
            use IncludeElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = IncludeElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::IncludeElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                IncludeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::IncludeElementType {
                id: self.id,
                schema_location: self.schema_location,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct ImportElementTypeDeserializer {
        id: Option<String>,
        namespace: Option<String>,
        schema_location: Option<String>,
        annotation: Option<super::AnnotationElementType>,
        state: Box<ImportElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ImportElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ImportElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut namespace: Option<String> = None;
            let mut schema_location: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(ImportElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ImportElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ImportElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<ImportElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ImportElementTypeDeserializerState::Annotation(None));
                *self.state = ImportElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = ImportElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ImportElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = ImportElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ImportElementTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ImportElementType> for ImportElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ImportElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ImportElementType>
        where
            R: DeserializeReader,
        {
            use ImportElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ImportElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ImportElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ImportElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ImportElementType {
                id: self.id,
                namespace: self.namespace,
                schema_location: self.schema_location,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct RedefineElementTypeDeserializer {
        schema_location: String,
        id: Option<String>,
        content: Vec<super::RedefineElementTypeContent>,
        state: Box<RedefineElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RedefineElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RedefineElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RedefineElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut schema_location: Option<String> = None;
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                schema_location: schema_location.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("schemaLocation".into()))
                })?,
                id: id,
                content: Vec::new(),
                state: Box::new(RedefineElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RedefineElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RedefineElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RedefineElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RedefineElementTypeContent>,
            fallback: &mut Option<RedefineElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(RedefineElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = RedefineElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RedefineElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = RedefineElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RedefineElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RedefineElementType> for RedefineElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementType>
        where
            R: DeserializeReader,
        {
            use RedefineElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: RedefineElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RedefineElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RedefineElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RedefineElementType {
                schema_location: self.schema_location,
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum RedefineElementTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RedefineElementTypeContent),
        Unknown__,
    }
    impl RedefineElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RedefineElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_complex_type(
            values: &mut Option<super::ComplexBaseType>,
            value: super::ComplexBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"complexType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_complex_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ComplexBaseType>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ComplexType(_, Some(deserializer))) => {
                        Self::ComplexType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ComplexType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_complex_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::ComplexType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ComplexType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RedefineElementTypeContent>
        for RedefineElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, None), event) => {
                        let output =
                            <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::RedefineElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::SimpleType(
                        values.ok_or_else(|| ErrorKind::MissingElement("simpleType".into()))?,
                    ))
                }
                Self::ComplexType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::ComplexType(
                        values.ok_or_else(|| ErrorKind::MissingElement("complexType".into()))?,
                    ))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::Group(
                        values.ok_or_else(|| ErrorKind::MissingElement("group".into()))?,
                    ))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct OverrideElementTypeDeserializer {
        schema_location: String,
        id: Option<String>,
        content: Vec<super::OverrideElementTypeContent>,
        state: Box<OverrideElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OverrideElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::OverrideElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl OverrideElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut schema_location: Option<String> = None;
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                schema_location: schema_location.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("schemaLocation".into()))
                })?,
                id: id,
                content: Vec::new(),
                state: Box::new(OverrideElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: OverrideElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let OverrideElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::OverrideElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::OverrideElementTypeContent>,
            fallback: &mut Option<OverrideElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(OverrideElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = OverrideElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                OverrideElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = OverrideElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                OverrideElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OverrideElementType> for OverrideElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementType>
        where
            R: DeserializeReader,
        {
            use OverrideElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: OverrideElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::OverrideElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                OverrideElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::OverrideElementType {
                schema_location: self.schema_location,
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum OverrideElementTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        Element(
            Option<super::ElementType>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        Notation(
            Option<super::NotationElementType>,
            Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::OverrideElementTypeContent),
        Unknown__,
    }
    impl OverrideElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<OverrideElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"element")
            ) {
                let output =
                    <super::ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"notation")
            ) {
                let output = <super::NotationElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_notation(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_complex_type(
            values: &mut Option<super::ComplexBaseType>,
            value: super::ComplexBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"complexType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_element(
            values: &mut Option<super::ElementType>,
            value: super::ElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"element",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute(
            values: &mut Option<super::AttributeType>,
            value: super::AttributeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_notation(
            values: &mut Option<super::NotationElementType>,
            value: super::NotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"notation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_complex_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ComplexBaseType>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ComplexType(_, Some(deserializer))) => {
                        Self::ComplexType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ComplexType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_complex_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::ComplexType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ComplexType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_element<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ElementType>,
            output: DeserializerOutput<'de, super::ElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element(_, Some(deserializer))) => {
                        Self::Element(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element(&mut values, data)?;
                    let data = Self::Element(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Element(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeType>,
            output: DeserializerOutput<'de, super::AttributeType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Attribute(_, Some(deserializer))) => {
                        Self::Attribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Attribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::Attribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Attribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_notation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::NotationElementType>,
            output: DeserializerOutput<'de, super::NotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Notation(_, Some(deserializer))) => {
                        Self::Notation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Notation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_notation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_notation(&mut values, data)?;
                    let data = Self::Notation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Notation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OverrideElementTypeContent>
        for OverrideElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Notation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_notation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, None), event) => {
                        let output =
                            <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element(values, None), event) => {
                        let output = <super::ElementType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_element(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, None), event) => {
                        let output =
                            <super::AttributeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Notation(values, None), event) => {
                        let output =
                            <super::NotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_notation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::OverrideElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::SimpleType(
                        values.ok_or_else(|| ErrorKind::MissingElement("simpleType".into()))?,
                    ))
                }
                Self::ComplexType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::ComplexType(
                        values.ok_or_else(|| ErrorKind::MissingElement("complexType".into()))?,
                    ))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Group(
                        values.ok_or_else(|| ErrorKind::MissingElement("group".into()))?,
                    ))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::Element(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Element(
                        values.ok_or_else(|| ErrorKind::MissingElement("element".into()))?,
                    ))
                }
                Self::Attribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Attribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                    ))
                }
                Self::Notation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_notation(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Notation(
                        values.ok_or_else(|| ErrorKind::MissingElement("notation".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct AnnotationElementTypeDeserializer {
        id: Option<String>,
        content: Vec<super::AnnotationElementTypeContent>,
        state: Box<AnnotationElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnnotationElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::AnnotationElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AnnotationElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state: Box::new(AnnotationElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AnnotationElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let AnnotationElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::AnnotationElementTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementTypeContent>,
            fallback: &mut Option<AnnotationElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(AnnotationElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = AnnotationElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                AnnotationElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = AnnotationElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AnnotationElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AnnotationElementType> for AnnotationElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementType>
        where
            R: DeserializeReader,
        {
            use AnnotationElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: AnnotationElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AnnotationElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AnnotationElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AnnotationElementType {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum AnnotationElementTypeContentDeserializer {
        Init__,
        Appinfo(
            Option<super::AppinfoElementType>,
            Option<<super::AppinfoElementType as WithDeserializer>::Deserializer>,
        ),
        Documentation(
            Option<super::DocumentationElementType>,
            Option<<super::DocumentationElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AnnotationElementTypeContent),
        Unknown__,
    }
    impl AnnotationElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<AnnotationElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"appinfo")
            ) {
                let output = <super::AppinfoElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_appinfo(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"documentation")
            ) {
                let output =
                    <super::DocumentationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_documentation(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_appinfo(
            values: &mut Option<super::AppinfoElementType>,
            value: super::AppinfoElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"appinfo",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_documentation(
            values: &mut Option<super::DocumentationElementType>,
            value: super::DocumentationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"documentation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_appinfo<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AppinfoElementType>,
            output: DeserializerOutput<'de, super::AppinfoElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Appinfo(_, Some(deserializer))) => {
                        Self::Appinfo(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Appinfo(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_appinfo(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_appinfo(&mut values, data)?;
                    let data = Self::Appinfo(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Appinfo(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_documentation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DocumentationElementType>,
            output: DeserializerOutput<'de, super::DocumentationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Documentation(_, Some(deserializer))) => {
                        Self::Documentation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Documentation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_documentation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_documentation(&mut values, data)?;
                    let data = Self::Documentation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Documentation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AnnotationElementTypeContent>
        for AnnotationElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Appinfo(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_appinfo(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Documentation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_documentation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Appinfo(values, None), event) => {
                        let output =
                            <super::AppinfoElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_appinfo(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Documentation(values, None), event) => {
                        let output = < super :: DocumentationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_documentation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::AnnotationElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Appinfo(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_appinfo(&mut values, value)?;
                    }
                    Ok(super::AnnotationElementTypeContent::Appinfo(
                        values.ok_or_else(|| ErrorKind::MissingElement("appinfo".into()))?,
                    ))
                }
                Self::Documentation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_documentation(&mut values, value)?;
                    }
                    Ok(super::AnnotationElementTypeContent::Documentation(
                        values.ok_or_else(|| ErrorKind::MissingElement("documentation".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct DefaultOpenContentElementTypeDeserializer {
        id: Option<String>,
        applies_to_empty: bool,
        mode: super::DefaultOpenContentModeType,
        annotation: Option<super::AnnotationElementType>,
        any: Option<super::WildcardType>,
        state: Box<DefaultOpenContentElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DefaultOpenContentElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Any(Option<<super::WildcardType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DefaultOpenContentElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut applies_to_empty: Option<bool> = None;
            let mut mode: Option<super::DefaultOpenContentModeType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"appliesToEmpty")
                ) {
                    reader.read_attrib(&mut applies_to_empty, b"appliesToEmpty", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mode")
                ) {
                    reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                applies_to_empty: applies_to_empty
                    .unwrap_or_else(super::DefaultOpenContentElementType::default_applies_to_empty),
                mode: mode.unwrap_or_else(super::DefaultOpenContentElementType::default_mode),
                annotation: None,
                any: None,
                state: Box::new(DefaultOpenContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DefaultOpenContentElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DefaultOpenContentElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_any(&mut self, value: super::WildcardType) -> Result<(), Error> {
            if self.any.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
            }
            self.any = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<DefaultOpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(DefaultOpenContentElementTypeDeserializerState::Annotation(
                    None,
                ));
                *self.state = DefaultOpenContentElementTypeDeserializerState::Any(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = DefaultOpenContentElementTypeDeserializerState::Any(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DefaultOpenContentElementTypeDeserializerState::Annotation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DefaultOpenContentElementTypeDeserializerState::Any(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DefaultOpenContentElementTypeDeserializerState::Annotation(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_any<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WildcardType>,
            fallback: &mut Option<DefaultOpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.any.is_some() {
                    fallback
                        .get_or_insert(DefaultOpenContentElementTypeDeserializerState::Any(None));
                    *self.state = DefaultOpenContentElementTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DefaultOpenContentElementTypeDeserializerState::Any(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state = DefaultOpenContentElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DefaultOpenContentElementTypeDeserializerState::Any(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DefaultOpenContentElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DefaultOpenContentElementTypeDeserializerState::Any(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DefaultOpenContentElementType>
        for DefaultOpenContentElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DefaultOpenContentElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DefaultOpenContentElementType>
        where
            R: DeserializeReader,
        {
            use DefaultOpenContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            DefaultOpenContentElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Any(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"any") {
                            let output =
                                <super::WildcardType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_any(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Any(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DefaultOpenContentElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DefaultOpenContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DefaultOpenContentElementType {
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
        Init__,
        Next__,
        Content__(<super::SimpleBaseTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SimpleBaseTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut final_: Option<super::SimpleDerivationSetType> = None;
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"final")
                ) {
                    reader.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                final_: final_,
                name: name,
                content: Vec::new(),
                state: Box::new(SimpleBaseTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SimpleBaseTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SimpleBaseTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SimpleBaseTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SimpleBaseTypeContent>,
            fallback: &mut Option<SimpleBaseTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(SimpleBaseTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SimpleBaseTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SimpleBaseTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = SimpleBaseTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SimpleBaseTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SimpleBaseType> for SimpleBaseTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SimpleBaseType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseType>
        where
            R: DeserializeReader,
        {
            use SimpleBaseTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::SimpleBaseTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SimpleBaseType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SimpleBaseTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SimpleBaseType {
                id: self.id,
                final_: self.final_,
                name: self.name,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum SimpleBaseTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::RestrictionElementType>,
            Option<<super::RestrictionElementType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListElementType>,
            Option<<super::ListElementType as WithDeserializer>::Deserializer>,
        ),
        Union(
            Option<super::UnionElementType>,
            Option<<super::UnionElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SimpleBaseTypeContent),
        Unknown__,
    }
    impl SimpleBaseTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<SimpleBaseTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"restriction")
            ) {
                let output =
                    <super::RestrictionElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_restriction(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"list")
            ) {
                let output = <super::ListElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_list(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"union")
            ) {
                let output = <super::UnionElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_union_(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_restriction(
            values: &mut Option<super::RestrictionElementType>,
            value: super::RestrictionElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"restriction",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListElementType>,
            value: super::ListElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_union_(
            values: &mut Option<super::UnionElementType>,
            value: super::UnionElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"union",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_restriction<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RestrictionElementType>,
            output: DeserializerOutput<'de, super::RestrictionElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Restriction(_, Some(deserializer))) => {
                        Self::Restriction(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Restriction(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_restriction(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restriction(&mut values, data)?;
                    let data = Self::Restriction(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Restriction(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListElementType>,
            output: DeserializerOutput<'de, super::ListElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::List(_, Some(deserializer))) => {
                        Self::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::List(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_union_<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnionElementType>,
            output: DeserializerOutput<'de, super::UnionElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Union(_, Some(deserializer))) => {
                        Self::Union(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Union(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_union_(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_union_(&mut values, data)?;
                    let data = Self::Union(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Union(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SimpleBaseTypeContent> for SimpleBaseTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Restriction(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_restriction(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Union(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_union_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Restriction(values, None), event) => {
                        let output = < super :: RestrictionElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_restriction(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::List(values, None), event) => {
                        let output =
                            <super::ListElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Union(values, None), event) => {
                        let output =
                            <super::UnionElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_union_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::SimpleBaseTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::Restriction(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_restriction(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Restriction(
                        values.ok_or_else(|| ErrorKind::MissingElement("restriction".into()))?,
                    ))
                }
                Self::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                Self::Union(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_union_(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Union(values.ok_or_else(
                        || ErrorKind::MissingElement("union".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
        Init__,
        Next__,
        Content__(<super::ComplexBaseTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ComplexBaseTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut mixed: Option<bool> = None;
            let mut abstract_: Option<bool> = None;
            let mut final_: Option<super::DerivationSetType> = None;
            let mut block: Option<super::DerivationSetType> = None;
            let mut default_attributes_apply: Option<bool> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mixed")
                ) {
                    reader.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"abstract")
                ) {
                    reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"final")
                ) {
                    reader.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"block")
                ) {
                    reader.read_attrib(&mut block, b"block", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(ComplexBaseTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ComplexBaseTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ComplexBaseTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ComplexBaseTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ComplexBaseTypeContent>,
            fallback: &mut Option<ComplexBaseTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(ComplexBaseTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ComplexBaseTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ComplexBaseTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = ComplexBaseTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ComplexBaseTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ComplexBaseType> for ComplexBaseTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ComplexBaseType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseType>
        where
            R: DeserializeReader,
        {
            use ComplexBaseTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: ComplexBaseTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ComplexBaseType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ComplexBaseTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
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
    pub enum ComplexBaseTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleContent(
            Option<super::SimpleContentElementType>,
            Option<<super::SimpleContentElementType as WithDeserializer>::Deserializer>,
        ),
        ComplexContent(
            Option<super::ComplexContentElementType>,
            Option<<super::ComplexContentElementType as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContentElementType>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Assert(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ComplexBaseTypeContent),
        Unknown__,
    }
    impl ComplexBaseTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ComplexBaseTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleContent")
            ) {
                let output =
                    <super::SimpleContentElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_simple_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"complexContent")
            ) {
                let output =
                    <super::ComplexContentElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_complex_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"openContent")
            ) {
                let output =
                    <super::OpenContentElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttributeElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"assert")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_content(
            values: &mut Option<super::SimpleContentElementType>,
            value: super::SimpleContentElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleContent",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_complex_content(
            values: &mut Option<super::ComplexContentElementType>,
            value: super::ComplexContentElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"complexContent",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_open_content(
            values: &mut Option<super::OpenContentElementType>,
            value: super::OpenContentElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"openContent",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_all(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_choice(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"choice",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sequence(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sequence",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute(
            values: &mut Option<super::AttributeType>,
            value: super::AttributeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_any_attribute(
            values: &mut Option<super::AnyAttributeElementType>,
            value: super::AnyAttributeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"anyAttribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_assert(
            values: &mut Option<super::AssertionType>,
            value: super::AssertionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"assert",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_content<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleContentElementType>,
            output: DeserializerOutput<'de, super::SimpleContentElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleContent(_, Some(deserializer))) => {
                        Self::SimpleContent(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleContent(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_content(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_content(&mut values, data)?;
                    let data = Self::SimpleContent(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleContent(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_complex_content<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ComplexContentElementType>,
            output: DeserializerOutput<'de, super::ComplexContentElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ComplexContent(_, Some(deserializer))) => {
                        Self::ComplexContent(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ComplexContent(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_complex_content(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_content(&mut values, data)?;
                    let data = Self::ComplexContent(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ComplexContent(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_open_content<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpenContentElementType>,
            output: DeserializerOutput<'de, super::OpenContentElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::OpenContent(_, Some(deserializer))) => {
                        Self::OpenContent(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::OpenContent(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_open_content(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_open_content(&mut values, data)?;
                    let data = Self::OpenContent(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::OpenContent(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_all<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::All(_, Some(deserializer))) => Self::All(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::All(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_all(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::All(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::All(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_choice<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Choice(_, Some(deserializer))) => {
                        Self::Choice(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Choice(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_choice(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::Choice(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Choice(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sequence<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Sequence(_, Some(deserializer))) => {
                        Self::Sequence(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Sequence(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sequence(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::Sequence(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Sequence(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeType>,
            output: DeserializerOutput<'de, super::AttributeType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Attribute(_, Some(deserializer))) => {
                        Self::Attribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Attribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::Attribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Attribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_any_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyAttributeElementType>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AnyAttribute(_, Some(deserializer))) => {
                        Self::AnyAttribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AnyAttribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_any_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::AnyAttribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AnyAttribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_assert<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AssertionType>,
            output: DeserializerOutput<'de, super::AssertionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Assert(_, Some(deserializer))) => {
                        Self::Assert(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Assert(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_assert(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assert(&mut values, data)?;
                    let data = Self::Assert(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Assert(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ComplexBaseTypeContent> for ComplexBaseTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleContent(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexContent(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_complex_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::OpenContent(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_open_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assert(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_assert(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleContent(values, None), event) => {
                        let output = < super :: SimpleContentElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_simple_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexContent(values, None), event) => {
                        let output = < super :: ComplexContentElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_complex_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::OpenContent(values, None), event) => {
                        let output = < super :: OpenContentElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_open_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, None), event) => {
                        let output =
                            <super::AttributeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, None), event) => {
                        let output = < super :: AnyAttributeElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assert(values, None), event) => {
                        let output =
                            <super::AssertionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_assert(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::ComplexBaseTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::SimpleContent(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::SimpleContent(
                        values.ok_or_else(|| ErrorKind::MissingElement("simpleContent".into()))?,
                    ))
                }
                Self::ComplexContent(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_complex_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::ComplexContent(
                        values.ok_or_else(|| ErrorKind::MissingElement("complexContent".into()))?,
                    ))
                }
                Self::OpenContent(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_open_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::OpenContent(
                        values.ok_or_else(|| ErrorKind::MissingElement("openContent".into()))?,
                    ))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Group(values.ok_or_else(
                        || ErrorKind::MissingElement("group".into()),
                    )?))
                }
                Self::All(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::All(
                        values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                    ))
                }
                Self::Choice(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Choice(values.ok_or_else(
                        || ErrorKind::MissingElement("choice".into()),
                    )?))
                }
                Self::Sequence(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Sequence(values.ok_or_else(
                        || ErrorKind::MissingElement("sequence".into()),
                    )?))
                }
                Self::Attribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Attribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                    ))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::AnyAttribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::AnyAttribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                    ))
                }
                Self::Assert(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_assert(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Assert(values.ok_or_else(
                        || ErrorKind::MissingElement("assert".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
        Init__,
        Next__,
        Content__(<super::GroupTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl GroupTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<super::AllNNIType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"minOccurs")
                ) {
                    reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(GroupTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: GroupTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let GroupTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::GroupTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::GroupTypeContent>,
            fallback: &mut Option<GroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(GroupTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = GroupTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(GroupTypeDeserializerState::Content__(deserializer));
                            *self.state = GroupTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = GroupTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::GroupType> for GroupTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::GroupType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupType>
        where
            R: DeserializeReader,
        {
            use GroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::GroupTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::GroupType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, GroupTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
    pub enum GroupTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Element(
            Option<super::ElementType>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Any(
            Option<super::AnyElementType>,
            Option<<super::AnyElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::GroupTypeContent),
        Unknown__,
    }
    impl GroupTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<GroupTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"element")
            ) {
                let output =
                    <super::ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"any")
            ) {
                let output =
                    <super::AnyElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_any(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_element(
            values: &mut Option<super::ElementType>,
            value: super::ElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"element",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_all(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_choice(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"choice",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sequence(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sequence",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_any(
            values: &mut Option<super::AnyElementType>,
            value: super::AnyElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_element<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ElementType>,
            output: DeserializerOutput<'de, super::ElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element(_, Some(deserializer))) => {
                        Self::Element(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element(&mut values, data)?;
                    let data = Self::Element(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Element(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_all<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::All(_, Some(deserializer))) => Self::All(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::All(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_all(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::All(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::All(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_choice<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Choice(_, Some(deserializer))) => {
                        Self::Choice(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Choice(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_choice(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::Choice(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Choice(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sequence<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Sequence(_, Some(deserializer))) => {
                        Self::Sequence(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Sequence(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sequence(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::Sequence(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Sequence(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_any<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyElementType>,
            output: DeserializerOutput<'de, super::AnyElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Any(_, Some(deserializer))) => Self::Any(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Any(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_any(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any(&mut values, data)?;
                    let data = Self::Any(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Any(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::GroupTypeContent> for GroupTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Any(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element(values, None), event) => {
                        let output = <super::ElementType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_element(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Any(values, None), event) => {
                        let output =
                            <super::AnyElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_any(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::GroupTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Annotation(values.ok_or_else(
                        || ErrorKind::MissingElement("annotation".into()),
                    )?))
                }
                Self::Element(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Element(values.ok_or_else(
                        || ErrorKind::MissingElement("element".into()),
                    )?))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Group(values.ok_or_else(|| {
                        ErrorKind::MissingElement("group".into())
                    })?))
                }
                Self::All(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::All(
                        values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                    ))
                }
                Self::Choice(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Choice(values.ok_or_else(
                        || ErrorKind::MissingElement("choice".into()),
                    )?))
                }
                Self::Sequence(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Sequence(values.ok_or_else(
                        || ErrorKind::MissingElement("sequence".into()),
                    )?))
                }
                Self::Any(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_any(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Any(
                        values.ok_or_else(|| ErrorKind::MissingElement("any".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
        Init__,
        Next__,
        Content__(<super::AttributeGroupTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AttributeGroupTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                content: Vec::new(),
                state: Box::new(AttributeGroupTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AttributeGroupTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let AttributeGroupTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::AttributeGroupTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AttributeGroupTypeContent>,
            fallback: &mut Option<AttributeGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(AttributeGroupTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = AttributeGroupTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AttributeGroupTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = AttributeGroupTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AttributeGroupTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AttributeGroupType> for AttributeGroupTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupType>
        where
            R: DeserializeReader,
        {
            use AttributeGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: AttributeGroupTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AttributeGroupType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AttributeGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AttributeGroupType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum AttributeGroupTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AttributeGroupTypeContent),
        Unknown__,
    }
    impl AttributeGroupTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<AttributeGroupTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttributeElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute(
            values: &mut Option<super::AttributeType>,
            value: super::AttributeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_any_attribute(
            values: &mut Option<super::AnyAttributeElementType>,
            value: super::AnyAttributeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"anyAttribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeType>,
            output: DeserializerOutput<'de, super::AttributeType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Attribute(_, Some(deserializer))) => {
                        Self::Attribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Attribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::Attribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Attribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_any_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyAttributeElementType>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AnyAttribute(_, Some(deserializer))) => {
                        Self::AnyAttribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AnyAttribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_any_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::AnyAttribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AnyAttribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AttributeGroupTypeContent>
        for AttributeGroupTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, None), event) => {
                        let output =
                            <super::AttributeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, None), event) => {
                        let output = < super :: AnyAttributeElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::AttributeGroupTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::Attribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::Attribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                    ))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::AnyAttribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::AnyAttribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct ElementTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        type_: Option<String>,
        substitution_group: Option<super::Entitiestype>,
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
        Init__,
        Next__,
        Content__(<super::ElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut substitution_group: Option<super::Entitiestype> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<super::AllNNIType> = None;
            let mut default: Option<String> = None;
            let mut fixed: Option<String> = None;
            let mut nillable: Option<bool> = None;
            let mut abstract_: Option<bool> = None;
            let mut final_: Option<super::DerivationSetType> = None;
            let mut block: Option<super::BlockSetType> = None;
            let mut form: Option<super::FormChoiceType> = None;
            let mut target_namespace: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"type")
                ) {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"substitutionGroup")
                ) {
                    reader.read_attrib(
                        &mut substitution_group,
                        b"substitutionGroup",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"minOccurs")
                ) {
                    reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"maxOccurs")
                ) {
                    reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"default")
                ) {
                    reader.read_attrib(&mut default, b"default", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"fixed")
                ) {
                    reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"nillable")
                ) {
                    reader.read_attrib(&mut nillable, b"nillable", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"abstract")
                ) {
                    reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"final")
                ) {
                    reader.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"block")
                ) {
                    reader.read_attrib(&mut block, b"block", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"form")
                ) {
                    reader.read_attrib(&mut form, b"form", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(ElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ElementTypeContent>,
            fallback: &mut Option<ElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(ElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ElementTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = ElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ElementType> for ElementTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementType>
        where
            R: DeserializeReader,
        {
            use ElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::ElementTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ElementTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
    pub enum ElementTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Alternative(
            Option<super::AltType>,
            Option<<super::AltType as WithDeserializer>::Deserializer>,
        ),
        Unique(
            Option<super::KeybaseType>,
            Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
        ),
        Key(
            Option<super::KeybaseType>,
            Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
        ),
        Keyref(
            Option<super::KeyrefElementType>,
            Option<<super::KeyrefElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ElementTypeContent),
        Unknown__,
    }
    impl ElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"alternative")
            ) {
                let output =
                    <super::AltType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_alternative(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"unique")
            ) {
                let output =
                    <super::KeybaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_unique(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"key")
            ) {
                let output =
                    <super::KeybaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_key(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"keyref")
            ) {
                let output = <super::KeyrefElementType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_keyref(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_complex_type(
            values: &mut Option<super::ComplexBaseType>,
            value: super::ComplexBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"complexType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_alternative(
            values: &mut Option<super::AltType>,
            value: super::AltType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"alternative",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unique(
            values: &mut Option<super::KeybaseType>,
            value: super::KeybaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"unique",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_key(
            values: &mut Option<super::KeybaseType>,
            value: super::KeybaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"key")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_keyref(
            values: &mut Option<super::KeyrefElementType>,
            value: super::KeyrefElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"keyref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_complex_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ComplexBaseType>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ComplexType(_, Some(deserializer))) => {
                        Self::ComplexType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ComplexType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_complex_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::ComplexType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ComplexType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_alternative<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AltType>,
            output: DeserializerOutput<'de, super::AltType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Alternative(_, Some(deserializer))) => {
                        Self::Alternative(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Alternative(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_alternative(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_alternative(&mut values, data)?;
                    let data = Self::Alternative(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Alternative(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unique<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::KeybaseType>,
            output: DeserializerOutput<'de, super::KeybaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Unique(_, Some(deserializer))) => {
                        Self::Unique(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Unique(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unique(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unique(&mut values, data)?;
                    let data = Self::Unique(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Unique(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_key<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::KeybaseType>,
            output: DeserializerOutput<'de, super::KeybaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Key(_, Some(deserializer))) => Self::Key(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Key(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_key(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key(&mut values, data)?;
                    let data = Self::Key(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Key(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_keyref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::KeyrefElementType>,
            output: DeserializerOutput<'de, super::KeyrefElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Keyref(_, Some(deserializer))) => {
                        Self::Keyref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Keyref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_keyref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_keyref(&mut values, data)?;
                    let data = Self::Keyref(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Keyref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ElementTypeContent> for ElementTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Alternative(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_alternative(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Unique(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unique(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Key(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_key(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Keyref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_keyref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, None), event) => {
                        let output =
                            <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Alternative(values, None), event) => {
                        let output = <super::AltType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_alternative(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Unique(values, None), event) => {
                        let output = <super::KeybaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unique(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Key(values, None), event) => {
                        let output = <super::KeybaseType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_key(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Keyref(values, None), event) => {
                        let output =
                            <super::KeyrefElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_keyref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::ElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Annotation(values.ok_or_else(
                        || ErrorKind::MissingElement("annotation".into()),
                    )?))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::SimpleType(values.ok_or_else(
                        || ErrorKind::MissingElement("simpleType".into()),
                    )?))
                }
                Self::ComplexType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::ComplexType(values.ok_or_else(
                        || ErrorKind::MissingElement("complexType".into()),
                    )?))
                }
                Self::Alternative(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_alternative(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Alternative(values.ok_or_else(
                        || ErrorKind::MissingElement("alternative".into()),
                    )?))
                }
                Self::Unique(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unique(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Unique(values.ok_or_else(
                        || ErrorKind::MissingElement("unique".into()),
                    )?))
                }
                Self::Key(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_key(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Key(
                        values.ok_or_else(|| ErrorKind::MissingElement("key".into()))?,
                    ))
                }
                Self::Keyref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_keyref(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Keyref(values.ok_or_else(
                        || ErrorKind::MissingElement("keyref".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
        annotation: Option<super::AnnotationElementType>,
        simple_type: Option<super::SimpleBaseType>,
        state: Box<AttributeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AttributeTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AttributeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut use_: Option<super::AttributeUseType> = None;
            let mut default: Option<String> = None;
            let mut fixed: Option<String> = None;
            let mut form: Option<super::FormChoiceType> = None;
            let mut target_namespace: Option<String> = None;
            let mut inheritable: Option<bool> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"type")
                ) {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"use")
                ) {
                    reader.read_attrib(&mut use_, b"use", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"default")
                ) {
                    reader.read_attrib(&mut default, b"default", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"fixed")
                ) {
                    reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"form")
                ) {
                    reader.read_attrib(&mut form, b"form", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"targetNamespace")
                ) {
                    reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(AttributeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AttributeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AttributeTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::SimpleType(Some(deserializer)) => {
                    self.store_simple_type(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_simple_type(&mut self, value: super::SimpleBaseType) -> Result<(), Error> {
            if self.simple_type.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            self.simple_type = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AttributeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(AttributeTypeDeserializerState::Annotation(None));
                *self.state = AttributeTypeDeserializerState::SimpleType(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = AttributeTypeDeserializerState::SimpleType(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AttributeTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = AttributeTypeDeserializerState::SimpleType(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AttributeTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<AttributeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(AttributeTypeDeserializerState::SimpleType(None));
                *self.state = AttributeTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_simple_type(data)?;
                    *self.state = AttributeTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AttributeTypeDeserializerState::SimpleType(
                                Some(deserializer),
                            ));
                            *self.state = AttributeTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AttributeTypeDeserializerState::SimpleType(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AttributeType> for AttributeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AttributeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeType>
        where
            R: DeserializeReader,
        {
            use AttributeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SimpleType(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = AttributeTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SimpleType(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"simpleType") {
                            let output =
                                <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_simple_type(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::SimpleType(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AttributeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AttributeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
    pub struct NotationElementTypeDeserializer {
        id: Option<String>,
        name: String,
        public: Option<String>,
        system: Option<String>,
        annotation: Option<super::AnnotationElementType>,
        state: Box<NotationElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NotationElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NotationElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut public: Option<String> = None;
            let mut system: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"public")
                ) {
                    reader.read_attrib(&mut public, b"public", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"system")
                ) {
                    reader.read_attrib(&mut system, b"system", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                public: public,
                system: system,
                annotation: None,
                state: Box::new(NotationElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: NotationElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use NotationElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<NotationElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(NotationElementTypeDeserializerState::Annotation(None));
                *self.state = NotationElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = NotationElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                NotationElementTypeDeserializerState::Annotation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = NotationElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = NotationElementTypeDeserializerState::Annotation(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::NotationElementType> for NotationElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NotationElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NotationElementType>
        where
            R: DeserializeReader,
        {
            use NotationElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = NotationElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::NotationElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                NotationElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::NotationElementType {
                id: self.id,
                name: self.name,
                public: self.public,
                system: self.system,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AppinfoElementTypeDeserializer {
        source: Option<String>,
        state: Box<AppinfoElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AppinfoElementTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl AppinfoElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut source: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"source")
                ) {
                    reader.read_attrib(&mut source, b"source", &attrib.value)?;
                }
            }
            Ok(Self {
                source: source,
                state: Box::new(AppinfoElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AppinfoElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AppinfoElementType> for AppinfoElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AppinfoElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AppinfoElementType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: true,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AppinfoElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AppinfoElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AppinfoElementType {
                source: self.source,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentationElementTypeDeserializer {
        source: Option<String>,
        lang: Option<String>,
        state: Box<DocumentationElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentationElementTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl DocumentationElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut source: Option<String> = None;
            let mut lang: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"source")
                ) {
                    reader.read_attrib(&mut source, b"source", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XML),
                    Some(b"lang")
                ) {
                    reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
                }
            }
            Ok(Self {
                source: source,
                lang: lang,
                state: Box::new(DocumentationElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentationElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::DocumentationElementType>
        for DocumentationElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentationElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentationElementType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: true,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentationElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DocumentationElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DocumentationElementType {
                source: self.source,
                lang: self.lang,
            })
        }
    }
    #[derive(Debug)]
    pub struct WildcardTypeDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::BasicNamespaceListType>,
        process_contents: super::ProcessContentsType,
        annotation: Option<super::AnnotationElementType>,
        state: Box<WildcardTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WildcardTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WildcardTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::BasicNamespaceListType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notNamespace")
                ) {
                    reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(WildcardTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: WildcardTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use WildcardTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<WildcardTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(WildcardTypeDeserializerState::Annotation(None));
                *self.state = WildcardTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = WildcardTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(WildcardTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = WildcardTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                WildcardTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::WildcardType> for WildcardTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WildcardType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WildcardType>
        where
            R: DeserializeReader,
        {
            use WildcardTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = WildcardTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::WildcardType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, WildcardTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
    pub struct RestrictionElementTypeDeserializer {
        id: Option<String>,
        base: Option<String>,
        content: Vec<super::RestrictionElementTypeContent>,
        state: Box<RestrictionElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictionElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RestrictionElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RestrictionElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"base")
                ) {
                    reader.read_attrib(&mut base, b"base", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                base: base,
                content: Vec::new(),
                state: Box::new(RestrictionElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RestrictionElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RestrictionElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::RestrictionElementTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RestrictionElementTypeContent>,
            fallback: &mut Option<RestrictionElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(RestrictionElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = RestrictionElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RestrictionElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = RestrictionElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RestrictionElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionElementType> for RestrictionElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementType>
        where
            R: DeserializeReader,
        {
            use RestrictionElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: RestrictionElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RestrictionElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RestrictionElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RestrictionElementType {
                id: self.id,
                base: self.base,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum RestrictionElementTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        Facet(
            Option<super::Facet>,
            Option<<super::Facet as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RestrictionElementTypeContent),
        Unknown__,
    }
    impl RestrictionElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RestrictionElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, true));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            let event = {
                let output = <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                match self.handle_facet(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, .. } => event,
                    output => {
                        return Ok(output);
                    }
                }
            };
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_facet(
            values: &mut Option<super::Facet>,
            value: super::Facet,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"facet",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_facet<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::Facet>,
            output: DeserializerOutput<'de, super::Facet>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Facet(_, Some(deserializer))) => {
                        Self::Facet(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Facet(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_facet(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_facet(&mut values, data)?;
                    let data = Self::Facet(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Facet(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionElementTypeContent>
        for RestrictionElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Facet(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_facet(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Facet(values, None), event) => {
                        let output =
                            <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_facet(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::RestrictionElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::SimpleType(
                        values.ok_or_else(|| ErrorKind::MissingElement("simpleType".into()))?,
                    ))
                }
                Self::Facet(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_facet(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::Facet(
                        values.ok_or_else(|| ErrorKind::MissingElement("facet".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct ListElementTypeDeserializer {
        id: Option<String>,
        item_type: Option<String>,
        annotation: Option<super::AnnotationElementType>,
        simple_type: Option<super::SimpleBaseType>,
        state: Box<ListElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ListElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut item_type: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(ListElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ListElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ListElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::SimpleType(Some(deserializer)) => {
                    self.store_simple_type(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_simple_type(&mut self, value: super::SimpleBaseType) -> Result<(), Error> {
            if self.simple_type.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            self.simple_type = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<ListElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ListElementTypeDeserializerState::Annotation(None));
                *self.state = ListElementTypeDeserializerState::SimpleType(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = ListElementTypeDeserializerState::SimpleType(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ListElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = ListElementTypeDeserializerState::SimpleType(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ListElementTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<ListElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ListElementTypeDeserializerState::SimpleType(None));
                *self.state = ListElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_simple_type(data)?;
                    *self.state = ListElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ListElementTypeDeserializerState::SimpleType(
                                Some(deserializer),
                            ));
                            *self.state = ListElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ListElementTypeDeserializerState::SimpleType(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ListElementType> for ListElementTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ListElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListElementType>
        where
            R: DeserializeReader,
        {
            use ListElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SimpleType(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ListElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SimpleType(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"simpleType") {
                            let output =
                                <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_simple_type(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::SimpleType(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ListElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ListElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ListElementType {
                id: self.id,
                item_type: self.item_type,
                annotation: self.annotation,
                simple_type: self.simple_type,
            })
        }
    }
    #[derive(Debug)]
    pub struct UnionElementTypeDeserializer {
        id: Option<String>,
        member_types: Option<super::Entitiestype>,
        annotation: Option<super::AnnotationElementType>,
        simple_type: Vec<super::SimpleBaseType>,
        state: Box<UnionElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UnionElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UnionElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut member_types: Option<super::Entitiestype> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(UnionElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: UnionElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use UnionElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::SimpleType(Some(deserializer)) => {
                    self.store_simple_type(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_simple_type(&mut self, value: super::SimpleBaseType) -> Result<(), Error> {
            self.simple_type.push(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<UnionElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(UnionElementTypeDeserializerState::Annotation(None));
                *self.state = UnionElementTypeDeserializerState::SimpleType(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = UnionElementTypeDeserializerState::SimpleType(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(UnionElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = UnionElementTypeDeserializerState::SimpleType(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                UnionElementTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<UnionElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(UnionElementTypeDeserializerState::SimpleType(None));
                *self.state = UnionElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_simple_type(data)?;
                    *self.state = UnionElementTypeDeserializerState::SimpleType(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(UnionElementTypeDeserializerState::SimpleType(
                                Some(deserializer),
                            ));
                            *self.state = UnionElementTypeDeserializerState::SimpleType(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                UnionElementTypeDeserializerState::SimpleType(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UnionElementType> for UnionElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnionElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnionElementType>
        where
            R: DeserializeReader,
        {
            use UnionElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SimpleType(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = UnionElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SimpleType(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"simpleType") {
                            let output =
                                <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_simple_type(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::SimpleType(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::UnionElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                UnionElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::UnionElementType {
                id: self.id,
                member_types: self.member_types,
                annotation: self.annotation,
                simple_type: self.simple_type,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleContentElementTypeDeserializer {
        id: Option<String>,
        content: Vec<super::SimpleContentElementTypeContent>,
        state: Box<SimpleContentElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleContentElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SimpleContentElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SimpleContentElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state: Box::new(SimpleContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SimpleContentElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SimpleContentElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::SimpleContentElementTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SimpleContentElementTypeContent>,
            fallback: &mut Option<SimpleContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(SimpleContentElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SimpleContentElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SimpleContentElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = SimpleContentElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SimpleContentElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SimpleContentElementType>
        for SimpleContentElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementType>
        where
            R: DeserializeReader,
        {
            use SimpleContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: SimpleContentElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SimpleContentElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SimpleContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SimpleContentElementType {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum SimpleContentElementTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::RestrictionType>,
            Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
        ),
        Extension(
            Option<super::ExtensionType>,
            Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SimpleContentElementTypeContent),
        Unknown__,
    }
    impl SimpleContentElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<SimpleContentElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"restriction")
            ) {
                let output = <super::RestrictionType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_restriction(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"extension")
            ) {
                let output =
                    <super::ExtensionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_extension(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_restriction(
            values: &mut Option<super::RestrictionType>,
            value: super::RestrictionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"restriction",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_extension(
            values: &mut Option<super::ExtensionType>,
            value: super::ExtensionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"extension",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_restriction<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RestrictionType>,
            output: DeserializerOutput<'de, super::RestrictionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Restriction(_, Some(deserializer))) => {
                        Self::Restriction(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Restriction(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_restriction(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restriction(&mut values, data)?;
                    let data = Self::Restriction(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Restriction(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_extension<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ExtensionType>,
            output: DeserializerOutput<'de, super::ExtensionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Extension(_, Some(deserializer))) => {
                        Self::Extension(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Extension(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_extension(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_extension(&mut values, data)?;
                    let data = Self::Extension(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Extension(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SimpleContentElementTypeContent>
        for SimpleContentElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Restriction(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_restriction(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Extension(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_extension(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Restriction(values, None), event) => {
                        let output =
                            <super::RestrictionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_restriction(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Extension(values, None), event) => {
                        let output =
                            <super::ExtensionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_extension(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::SimpleContentElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SimpleContentElementTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::Restriction(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_restriction(&mut values, value)?;
                    }
                    Ok(super::SimpleContentElementTypeContent::Restriction(
                        values.ok_or_else(|| ErrorKind::MissingElement("restriction".into()))?,
                    ))
                }
                Self::Extension(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_extension(&mut values, value)?;
                    }
                    Ok(super::SimpleContentElementTypeContent::Extension(
                        values.ok_or_else(|| ErrorKind::MissingElement("extension".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct ComplexContentElementTypeDeserializer {
        id: Option<String>,
        mixed: Option<bool>,
        content: Vec<super::ComplexContentElementTypeContent>,
        state: Box<ComplexContentElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ComplexContentElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ComplexContentElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ComplexContentElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut mixed: Option<bool> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mixed")
                ) {
                    reader.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                mixed: mixed,
                content: Vec::new(),
                state: Box::new(ComplexContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ComplexContentElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ComplexContentElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::ComplexContentElementTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ComplexContentElementTypeContent>,
            fallback: &mut Option<ComplexContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(ComplexContentElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ComplexContentElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ComplexContentElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = ComplexContentElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ComplexContentElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ComplexContentElementType>
        for ComplexContentElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementType>
        where
            R: DeserializeReader,
        {
            use ComplexContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: ComplexContentElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ComplexContentElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ComplexContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ComplexContentElementType {
                id: self.id,
                mixed: self.mixed,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum ComplexContentElementTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::RestrictionType>,
            Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
        ),
        Extension(
            Option<super::ExtensionType>,
            Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ComplexContentElementTypeContent),
        Unknown__,
    }
    impl ComplexContentElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ComplexContentElementTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"restriction")
            ) {
                let output = <super::RestrictionType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_restriction(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"extension")
            ) {
                let output =
                    <super::ExtensionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_extension(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_restriction(
            values: &mut Option<super::RestrictionType>,
            value: super::RestrictionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"restriction",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_extension(
            values: &mut Option<super::ExtensionType>,
            value: super::ExtensionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"extension",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_restriction<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RestrictionType>,
            output: DeserializerOutput<'de, super::RestrictionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Restriction(_, Some(deserializer))) => {
                        Self::Restriction(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Restriction(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_restriction(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restriction(&mut values, data)?;
                    let data = Self::Restriction(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Restriction(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_extension<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ExtensionType>,
            output: DeserializerOutput<'de, super::ExtensionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Extension(_, Some(deserializer))) => {
                        Self::Extension(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Extension(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_extension(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_extension(&mut values, data)?;
                    let data = Self::Extension(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Extension(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ComplexContentElementTypeContent>
        for ComplexContentElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Restriction(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_restriction(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Extension(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_extension(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Restriction(values, None), event) => {
                        let output =
                            <super::RestrictionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_restriction(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Extension(values, None), event) => {
                        let output =
                            <super::ExtensionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_extension(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::ComplexContentElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ComplexContentElementTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::Restriction(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_restriction(&mut values, value)?;
                    }
                    Ok(super::ComplexContentElementTypeContent::Restriction(
                        values.ok_or_else(|| ErrorKind::MissingElement("restriction".into()))?,
                    ))
                }
                Self::Extension(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_extension(&mut values, value)?;
                    }
                    Ok(super::ComplexContentElementTypeContent::Extension(
                        values.ok_or_else(|| ErrorKind::MissingElement("extension".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct OpenContentElementTypeDeserializer {
        id: Option<String>,
        mode: super::OpenContentModeType,
        annotation: Option<super::AnnotationElementType>,
        any: Option<super::WildcardType>,
        state: Box<OpenContentElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OpenContentElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Any(Option<<super::WildcardType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl OpenContentElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut mode: Option<super::OpenContentModeType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mode")
                ) {
                    reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                mode: mode.unwrap_or_else(super::OpenContentElementType::default_mode),
                annotation: None,
                any: None,
                state: Box::new(OpenContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: OpenContentElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use OpenContentElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_any(&mut self, value: super::WildcardType) -> Result<(), Error> {
            if self.any.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
            }
            self.any = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<OpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(OpenContentElementTypeDeserializerState::Annotation(None));
                *self.state = OpenContentElementTypeDeserializerState::Any(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = OpenContentElementTypeDeserializerState::Any(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                OpenContentElementTypeDeserializerState::Annotation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = OpenContentElementTypeDeserializerState::Any(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = OpenContentElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_any<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::WildcardType>,
            fallback: &mut Option<OpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(OpenContentElementTypeDeserializerState::Any(None));
                *self.state = OpenContentElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state = OpenContentElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(OpenContentElementTypeDeserializerState::Any(
                                Some(deserializer),
                            ));
                            *self.state = OpenContentElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                OpenContentElementTypeDeserializerState::Any(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OpenContentElementType> for OpenContentElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpenContentElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpenContentElementType>
        where
            R: DeserializeReader,
        {
            use OpenContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = OpenContentElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Any(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"any") {
                            let output =
                                <super::WildcardType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_any(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Any(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::OpenContentElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                OpenContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::OpenContentElementType {
                id: self.id,
                mode: self.mode,
                annotation: self.annotation,
                any: self.any,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyAttributeElementTypeDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::BasicNamespaceListType>,
        process_contents: super::ProcessContentsType,
        not_q_name: Option<super::QnameListAType>,
        annotation: Option<super::AnnotationElementType>,
        state: Box<AnyAttributeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyAttributeElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyAttributeElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::BasicNamespaceListType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            let mut not_q_name: Option<super::QnameListAType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notNamespace")
                ) {
                    reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"processContents")
                ) {
                    reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                    .unwrap_or_else(super::AnyAttributeElementType::default_process_contents),
                not_q_name: not_q_name,
                annotation: None,
                state: Box::new(AnyAttributeElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AnyAttributeElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AnyAttributeElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AnyAttributeElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(AnyAttributeElementTypeDeserializerState::Annotation(None));
                *self.state = AnyAttributeElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = AnyAttributeElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                AnyAttributeElementTypeDeserializerState::Annotation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = AnyAttributeElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = AnyAttributeElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AnyAttributeElementType>
        for AnyAttributeElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyAttributeElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyAttributeElementType>
        where
            R: DeserializeReader,
        {
            use AnyAttributeElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = AnyAttributeElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AnyAttributeElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AnyAttributeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AnyAttributeElementType {
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
        annotation: Option<super::AnnotationElementType>,
        state: Box<AssertionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AssertionTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AssertionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"test")
                ) {
                    reader.read_attrib(&mut test, b"test", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(AssertionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AssertionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AssertionTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AssertionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(AssertionTypeDeserializerState::Annotation(None));
                *self.state = AssertionTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = AssertionTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AssertionTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = AssertionTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AssertionTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AssertionType> for AssertionTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AssertionType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AssertionType>
        where
            R: DeserializeReader,
        {
            use AssertionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = AssertionTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AssertionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AssertionTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::AssertionType {
                id: self.id,
                test: self.test,
                xpath_default_namespace: self.xpath_default_namespace,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyElementTypeDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::BasicNamespaceListType>,
        process_contents: super::ProcessContentsType,
        not_q_name: Option<super::QnameListType>,
        min_occurs: usize,
        max_occurs: super::AllNNIType,
        annotation: Option<super::AnnotationElementType>,
        state: Box<AnyElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::BasicNamespaceListType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            let mut not_q_name: Option<super::QnameListType> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<super::AllNNIType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notNamespace")
                ) {
                    reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"processContents")
                ) {
                    reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notQName")
                ) {
                    reader.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"minOccurs")
                ) {
                    reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                    .unwrap_or_else(super::AnyElementType::default_process_contents),
                not_q_name: not_q_name,
                min_occurs: min_occurs.unwrap_or_else(super::AnyElementType::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::AnyElementType::default_max_occurs),
                annotation: None,
                state: Box::new(AnyElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AnyElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AnyElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AnyElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(AnyElementTypeDeserializerState::Annotation(None));
                *self.state = AnyElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = AnyElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AnyElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = AnyElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                AnyElementTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AnyElementType> for AnyElementTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnyElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyElementType>
        where
            R: DeserializeReader,
        {
            use AnyElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = AnyElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AnyElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AnyElementTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::AnyElementType {
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
        Init__,
        Next__,
        Content__(<super::AltTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AltTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"test")
                ) {
                    reader.read_attrib(&mut test, b"test", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"type")
                ) {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                state: Box::new(AltTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AltTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let AltTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::AltTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AltTypeContent>,
            fallback: &mut Option<AltTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(AltTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = AltTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(AltTypeDeserializerState::Content__(deserializer));
                            *self.state = AltTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = AltTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AltType> for AltTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AltType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltType>
        where
            R: DeserializeReader,
        {
            use AltTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::AltTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AltType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AltTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
    pub enum AltTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AltTypeContent),
        Unknown__,
    }
    impl AltTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<AltTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_complex_type(
            values: &mut Option<super::ComplexBaseType>,
            value: super::ComplexBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"complexType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_complex_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ComplexBaseType>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ComplexType(_, Some(deserializer))) => {
                        Self::ComplexType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ComplexType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_complex_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::ComplexType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ComplexType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AltTypeContent> for AltTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AltTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ComplexType(values, None), event) => {
                        let output =
                            <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_complex_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::AltTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::Annotation(values.ok_or_else(
                        || ErrorKind::MissingElement("annotation".into()),
                    )?))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::SimpleType(values.ok_or_else(
                        || ErrorKind::MissingElement("simpleType".into()),
                    )?))
                }
                Self::ComplexType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::ComplexType(values.ok_or_else(
                        || ErrorKind::MissingElement("complexType".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct KeybaseTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        content: Option<super::KeybaseTypeContent>,
        state: Box<KeybaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeybaseTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeybaseTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeybaseTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                content: None,
                state: Box::new(KeybaseTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: KeybaseTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let KeybaseTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::KeybaseTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::KeybaseTypeContent>,
            fallback: &mut Option<KeybaseTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(KeybaseTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = KeybaseTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeybaseTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = KeybaseTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeybaseTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeybaseType> for KeybaseTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::KeybaseType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseType>
        where
            R: DeserializeReader,
        {
            use KeybaseTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::KeybaseTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::KeybaseType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, KeybaseTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::KeybaseType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeybaseTypeContentDeserializer {
        annotation: Option<super::AnnotationElementType>,
        selector: Option<super::FieldElementType>,
        field: Vec<super::FieldElementType>,
        state: Box<KeybaseTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum KeybaseTypeContentDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Selector(Option<<super::FieldElementType as WithDeserializer>::Deserializer>),
        Field(Option<<super::FieldElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl KeybaseTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: KeybaseTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use KeybaseTypeContentDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::Selector(Some(deserializer)) => {
                    self.store_selector(deserializer.finish(reader)?)?
                }
                S::Field(Some(deserializer)) => self.store_field(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_selector(&mut self, value: super::FieldElementType) -> Result<(), Error> {
            if self.selector.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"selector",
                )))?;
            }
            self.selector = Some(value);
            Ok(())
        }
        fn store_field(&mut self, value: super::FieldElementType) -> Result<(), Error> {
            self.field.push(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<KeybaseTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(KeybaseTypeContentDeserializerState::Annotation(None));
                *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                KeybaseTypeContentDeserializerState::Annotation(Some(deserializer)),
                            );
                            *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                KeybaseTypeContentDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_selector<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeybaseTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.selector.is_some() {
                    fallback.get_or_insert(KeybaseTypeContentDeserializerState::Selector(None));
                    *self.state = KeybaseTypeContentDeserializerState::Field(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_selector(data)?;
                    *self.state = KeybaseTypeContentDeserializerState::Field(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeybaseTypeContentDeserializerState::Selector(
                                Some(deserializer),
                            ));
                            *self.state = KeybaseTypeContentDeserializerState::Field(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                KeybaseTypeContentDeserializerState::Selector(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeybaseTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.field.len() < 1usize {
                    *self.state = KeybaseTypeContentDeserializerState::Field(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(KeybaseTypeContentDeserializerState::Field(None));
                    *self.state = KeybaseTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_field(data)?;
                    *self.state = KeybaseTypeContentDeserializerState::Field(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeybaseTypeContentDeserializerState::Field(
                                Some(deserializer),
                            ));
                            if self.field.len().saturating_add(1) < 1usize {
                                *self.state = KeybaseTypeContentDeserializerState::Field(None);
                            } else {
                                *self.state = KeybaseTypeContentDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                KeybaseTypeContentDeserializerState::Field(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeybaseTypeContent> for KeybaseTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                annotation: None,
                selector: None,
                field: Vec::new(),
                state: Box::new(KeybaseTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, KeybaseTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseTypeContent>
        where
            R: DeserializeReader,
        {
            use KeybaseTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Selector(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_selector(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Field(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, event @ Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = KeybaseTypeContentDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Selector(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Selector(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"selector") {
                            let output =
                                <super::FieldElementType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_selector(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Field(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Selector(None));
                            event
                        }
                    }
                    (S::Field(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"field") {
                            let output =
                                <super::FieldElementType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_field(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Field(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::KeybaseTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                KeybaseTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::KeybaseTypeContent {
                annotation: self.annotation,
                selector: self
                    .selector
                    .ok_or_else(|| ErrorKind::MissingElement("selector".into()))?,
                field: self.field,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyrefElementTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        refer: Option<String>,
        content: Option<super::KeyrefElementTypeContent>,
        state: Box<KeyrefElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyrefElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyrefElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyrefElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut refer: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"refer")
                ) {
                    reader.read_attrib(&mut refer, b"refer", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                name: name,
                ref_: ref_,
                refer: refer,
                content: None,
                state: Box::new(KeyrefElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: KeyrefElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let KeyrefElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::KeyrefElementTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::KeyrefElementTypeContent>,
            fallback: &mut Option<KeyrefElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(KeyrefElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = KeyrefElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeyrefElementTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = KeyrefElementTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                KeyrefElementTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeyrefElementType> for KeyrefElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementType>
        where
            R: DeserializeReader,
        {
            use KeyrefElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: KeyrefElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::KeyrefElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                KeyrefElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::KeyrefElementType {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                refer: self.refer,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyrefElementTypeContentDeserializer {
        annotation: Option<super::AnnotationElementType>,
        selector: Option<super::FieldElementType>,
        field: Vec<super::FieldElementType>,
        state: Box<KeyrefElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyrefElementTypeContentDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Selector(Option<<super::FieldElementType as WithDeserializer>::Deserializer>),
        Field(Option<<super::FieldElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl KeyrefElementTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: KeyrefElementTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use KeyrefElementTypeContentDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                S::Selector(Some(deserializer)) => {
                    self.store_selector(deserializer.finish(reader)?)?
                }
                S::Field(Some(deserializer)) => self.store_field(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_selector(&mut self, value: super::FieldElementType) -> Result<(), Error> {
            if self.selector.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"selector",
                )))?;
            }
            self.selector = Some(value);
            Ok(())
        }
        fn store_field(&mut self, value: super::FieldElementType) -> Result<(), Error> {
            self.field.push(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<KeyrefElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(KeyrefElementTypeContentDeserializerState::Annotation(None));
                *self.state = KeyrefElementTypeContentDeserializerState::Selector(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = KeyrefElementTypeContentDeserializerState::Selector(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                KeyrefElementTypeContentDeserializerState::Annotation(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = KeyrefElementTypeContentDeserializerState::Selector(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeyrefElementTypeContentDeserializerState::Annotation(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_selector<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeyrefElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.selector.is_some() {
                    fallback
                        .get_or_insert(KeyrefElementTypeContentDeserializerState::Selector(None));
                    *self.state = KeyrefElementTypeContentDeserializerState::Field(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = KeyrefElementTypeContentDeserializerState::Selector(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_selector(data)?;
                    *self.state = KeyrefElementTypeContentDeserializerState::Field(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                KeyrefElementTypeContentDeserializerState::Selector(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = KeyrefElementTypeContentDeserializerState::Field(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeyrefElementTypeContentDeserializerState::Selector(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeyrefElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.field.len() < 1usize {
                    *self.state = KeyrefElementTypeContentDeserializerState::Field(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(KeyrefElementTypeContentDeserializerState::Field(None));
                    *self.state = KeyrefElementTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_field(data)?;
                    *self.state = KeyrefElementTypeContentDeserializerState::Field(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                KeyrefElementTypeContentDeserializerState::Field(Some(
                                    deserializer,
                                )),
                            );
                            if self.field.len().saturating_add(1) < 1usize {
                                *self.state =
                                    KeyrefElementTypeContentDeserializerState::Field(None);
                            } else {
                                *self.state = KeyrefElementTypeContentDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeyrefElementTypeContentDeserializerState::Field(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeyrefElementTypeContent>
        for KeyrefElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                annotation: None,
                selector: None,
                field: Vec::new(),
                state: Box::new(KeyrefElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, KeyrefElementTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementTypeContent>
        where
            R: DeserializeReader,
        {
            use KeyrefElementTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Selector(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_selector(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Field(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, event @ Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = KeyrefElementTypeContentDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Selector(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Selector(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"selector") {
                            let output =
                                <super::FieldElementType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_selector(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Field(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Selector(None));
                            event
                        }
                    }
                    (S::Field(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"field") {
                            let output =
                                <super::FieldElementType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_field(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Field(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::KeyrefElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                KeyrefElementTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::KeyrefElementTypeContent {
                annotation: self.annotation,
                selector: self
                    .selector
                    .ok_or_else(|| ErrorKind::MissingElement("selector".into()))?,
                field: self.field,
            })
        }
    }
    #[derive(Debug)]
    pub enum FacetDeserializer {
        Init__,
        MinExclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MinInclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MaxExclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MaxInclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        TotalDigits(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        FractionDigits(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Length(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MinLength(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MaxLength(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Enumeration(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        WhiteSpace(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Pattern(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Assertion(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        ExplicitTimezone(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::Facet),
        Unknown__,
    }
    impl FacetDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<FacetDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"minExclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_min_exclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"minInclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_min_inclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"maxExclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_max_exclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"maxInclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_max_inclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"totalDigits")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_total_digits(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"fractionDigits")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_fraction_digits(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"length")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_length(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"minLength")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_min_length(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"maxLength")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_max_length(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"enumeration")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_enumeration(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"whiteSpace")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_white_space(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"pattern")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_pattern(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"assertion")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assertion(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"explicitTimezone")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_explicit_timezone(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_min_exclusive(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"minExclusive",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_min_inclusive(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"minInclusive",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_max_exclusive(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"maxExclusive",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_max_inclusive(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"maxInclusive",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_total_digits(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"totalDigits",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fraction_digits(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fractionDigits",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_length(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"length",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_min_length(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"minLength",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_max_length(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"maxLength",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumeration(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumeration",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_white_space(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"whiteSpace",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_pattern(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"pattern",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_assertion(
            values: &mut Option<super::AssertionType>,
            value: super::AssertionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"assertion",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_explicit_timezone(
            values: &mut Option<super::FacetType>,
            value: super::FacetType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"explicitTimezone",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_min_exclusive<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MinExclusive(_, Some(deserializer))) => {
                        Self::MinExclusive(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MinExclusive(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_min_exclusive(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_min_exclusive(&mut values, data)?;
                    let data = Self::MinExclusive(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MinExclusive(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_min_inclusive<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MinInclusive(_, Some(deserializer))) => {
                        Self::MinInclusive(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MinInclusive(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_min_inclusive(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_min_inclusive(&mut values, data)?;
                    let data = Self::MinInclusive(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MinInclusive(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_max_exclusive<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MaxExclusive(_, Some(deserializer))) => {
                        Self::MaxExclusive(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MaxExclusive(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_max_exclusive(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_max_exclusive(&mut values, data)?;
                    let data = Self::MaxExclusive(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MaxExclusive(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_max_inclusive<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MaxInclusive(_, Some(deserializer))) => {
                        Self::MaxInclusive(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MaxInclusive(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_max_inclusive(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_max_inclusive(&mut values, data)?;
                    let data = Self::MaxInclusive(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MaxInclusive(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_total_digits<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::TotalDigits(_, Some(deserializer))) => {
                        Self::TotalDigits(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::TotalDigits(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_total_digits(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_total_digits(&mut values, data)?;
                    let data = Self::TotalDigits(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::TotalDigits(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fraction_digits<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::FractionDigits(_, Some(deserializer))) => {
                        Self::FractionDigits(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::FractionDigits(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fraction_digits(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fraction_digits(&mut values, data)?;
                    let data = Self::FractionDigits(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::FractionDigits(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_length<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Length(_, Some(deserializer))) => {
                        Self::Length(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Length(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_length(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_length(&mut values, data)?;
                    let data = Self::Length(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Length(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_min_length<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MinLength(_, Some(deserializer))) => {
                        Self::MinLength(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MinLength(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_min_length(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_min_length(&mut values, data)?;
                    let data = Self::MinLength(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MinLength(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_max_length<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MaxLength(_, Some(deserializer))) => {
                        Self::MaxLength(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MaxLength(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_max_length(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_max_length(&mut values, data)?;
                    let data = Self::MaxLength(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MaxLength(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumeration<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Enumeration(_, Some(deserializer))) => {
                        Self::Enumeration(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Enumeration(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumeration(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumeration(&mut values, data)?;
                    let data = Self::Enumeration(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Enumeration(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_white_space<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::WhiteSpace(_, Some(deserializer))) => {
                        Self::WhiteSpace(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::WhiteSpace(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_white_space(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_white_space(&mut values, data)?;
                    let data = Self::WhiteSpace(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::WhiteSpace(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_pattern<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Pattern(_, Some(deserializer))) => {
                        Self::Pattern(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Pattern(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pattern(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pattern(&mut values, data)?;
                    let data = Self::Pattern(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Pattern(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_assertion<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AssertionType>,
            output: DeserializerOutput<'de, super::AssertionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Assertion(_, Some(deserializer))) => {
                        Self::Assertion(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Assertion(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_assertion(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assertion(&mut values, data)?;
                    let data = Self::Assertion(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Assertion(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_explicit_timezone<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FacetType>,
            output: DeserializerOutput<'de, super::FacetType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::ExplicitTimezone(_, Some(deserializer))) => {
                        Self::ExplicitTimezone(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::ExplicitTimezone(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_explicit_timezone(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_explicit_timezone(&mut values, data)?;
                    let data = Self::ExplicitTimezone(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::ExplicitTimezone(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::Facet> for FacetDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Facet>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Facet>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::MinExclusive(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_min_exclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MinInclusive(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_min_inclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MaxExclusive(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_max_exclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MaxInclusive(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_max_inclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::TotalDigits(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_total_digits(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::FractionDigits(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fraction_digits(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Length(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_length(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MinLength(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_min_length(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MaxLength(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_max_length(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Enumeration(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumeration(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::WhiteSpace(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_white_space(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Pattern(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pattern(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assertion(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_assertion(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ExplicitTimezone(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_explicit_timezone(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MinExclusive(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_min_exclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MinInclusive(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_min_inclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MaxExclusive(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_max_exclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MaxInclusive(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_max_inclusive(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::TotalDigits(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_total_digits(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::FractionDigits(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fraction_digits(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Length(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_length(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MinLength(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_min_length(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MaxLength(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_max_length(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Enumeration(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumeration(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::WhiteSpace(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_white_space(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Pattern(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pattern(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assertion(values, None), event) => {
                        let output =
                            <super::AssertionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_assertion(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::ExplicitTimezone(values, None), event) => {
                        let output = <super::FacetType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_explicit_timezone(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::Facet, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::MinExclusive(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_min_exclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MinExclusive(values.ok_or_else(|| {
                        ErrorKind::MissingElement("minExclusive".into())
                    })?))
                }
                Self::MinInclusive(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_min_inclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MinInclusive(values.ok_or_else(|| {
                        ErrorKind::MissingElement("minInclusive".into())
                    })?))
                }
                Self::MaxExclusive(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_max_exclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxExclusive(values.ok_or_else(|| {
                        ErrorKind::MissingElement("maxExclusive".into())
                    })?))
                }
                Self::MaxInclusive(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_max_inclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxInclusive(values.ok_or_else(|| {
                        ErrorKind::MissingElement("maxInclusive".into())
                    })?))
                }
                Self::TotalDigits(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_total_digits(&mut values, value)?;
                    }
                    Ok(super::Facet::TotalDigits(values.ok_or_else(|| {
                        ErrorKind::MissingElement("totalDigits".into())
                    })?))
                }
                Self::FractionDigits(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fraction_digits(&mut values, value)?;
                    }
                    Ok(super::Facet::FractionDigits(values.ok_or_else(|| {
                        ErrorKind::MissingElement("fractionDigits".into())
                    })?))
                }
                Self::Length(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_length(&mut values, value)?;
                    }
                    Ok(super::Facet::Length(values.ok_or_else(|| {
                        ErrorKind::MissingElement("length".into())
                    })?))
                }
                Self::MinLength(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_min_length(&mut values, value)?;
                    }
                    Ok(super::Facet::MinLength(values.ok_or_else(|| {
                        ErrorKind::MissingElement("minLength".into())
                    })?))
                }
                Self::MaxLength(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_max_length(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxLength(values.ok_or_else(|| {
                        ErrorKind::MissingElement("maxLength".into())
                    })?))
                }
                Self::Enumeration(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumeration(&mut values, value)?;
                    }
                    Ok(super::Facet::Enumeration(values.ok_or_else(|| {
                        ErrorKind::MissingElement("enumeration".into())
                    })?))
                }
                Self::WhiteSpace(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_white_space(&mut values, value)?;
                    }
                    Ok(super::Facet::WhiteSpace(values.ok_or_else(|| {
                        ErrorKind::MissingElement("whiteSpace".into())
                    })?))
                }
                Self::Pattern(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pattern(&mut values, value)?;
                    }
                    Ok(super::Facet::Pattern(values.ok_or_else(|| {
                        ErrorKind::MissingElement("pattern".into())
                    })?))
                }
                Self::Assertion(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_assertion(&mut values, value)?;
                    }
                    Ok(super::Facet::Assertion(values.ok_or_else(|| {
                        ErrorKind::MissingElement("assertion".into())
                    })?))
                }
                Self::ExplicitTimezone(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_explicit_timezone(&mut values, value)?;
                    }
                    Ok(super::Facet::ExplicitTimezone(values.ok_or_else(|| {
                        ErrorKind::MissingElement("explicitTimezone".into())
                    })?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
        Init__,
        Next__,
        Content__(<super::RestrictionTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RestrictionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"base")
                ) {
                    reader.read_attrib(&mut base, b"base", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                base: base
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("base".into())))?,
                content: Vec::new(),
                state: Box::new(RestrictionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RestrictionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RestrictionTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RestrictionTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RestrictionTypeContent>,
            fallback: &mut Option<RestrictionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(RestrictionTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = RestrictionTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RestrictionTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = RestrictionTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = RestrictionTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionType> for RestrictionTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RestrictionType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionType>
        where
            R: DeserializeReader,
        {
            use RestrictionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: RestrictionTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RestrictionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RestrictionTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RestrictionType {
                id: self.id,
                base: self.base,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum RestrictionTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContentElementType>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        Facet(
            Option<super::Facet>,
            Option<<super::Facet as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Assert(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RestrictionTypeContent),
        Unknown__,
    }
    impl RestrictionTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RestrictionTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, true));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"openContent")
            ) {
                let output =
                    <super::OpenContentElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttributeElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"assert")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
            let event = {
                let output = <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                match self.handle_facet(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, .. } => event,
                    output => {
                        return Ok(output);
                    }
                }
            };
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_open_content(
            values: &mut Option<super::OpenContentElementType>,
            value: super::OpenContentElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"openContent",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_all(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_choice(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"choice",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sequence(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sequence",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_simple_type(
            values: &mut Option<super::SimpleBaseType>,
            value: super::SimpleBaseType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"simpleType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_facet(
            values: &mut Option<super::Facet>,
            value: super::Facet,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"facet",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute(
            values: &mut Option<super::AttributeType>,
            value: super::AttributeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_any_attribute(
            values: &mut Option<super::AnyAttributeElementType>,
            value: super::AnyAttributeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"anyAttribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_assert(
            values: &mut Option<super::AssertionType>,
            value: super::AssertionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"assert",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_open_content<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpenContentElementType>,
            output: DeserializerOutput<'de, super::OpenContentElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::OpenContent(_, Some(deserializer))) => {
                        Self::OpenContent(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::OpenContent(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_open_content(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_open_content(&mut values, data)?;
                    let data = Self::OpenContent(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::OpenContent(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_all<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::All(_, Some(deserializer))) => Self::All(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::All(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_all(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::All(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::All(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_choice<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Choice(_, Some(deserializer))) => {
                        Self::Choice(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Choice(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_choice(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::Choice(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Choice(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sequence<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Sequence(_, Some(deserializer))) => {
                        Self::Sequence(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Sequence(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sequence(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::Sequence(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Sequence(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_simple_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SimpleBaseType>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::SimpleType(_, Some(deserializer))) => {
                        Self::SimpleType(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::SimpleType(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_simple_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::SimpleType(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::SimpleType(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_facet<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::Facet>,
            output: DeserializerOutput<'de, super::Facet>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Facet(_, Some(deserializer))) => {
                        Self::Facet(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Facet(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_facet(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_facet(&mut values, data)?;
                    let data = Self::Facet(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Facet(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeType>,
            output: DeserializerOutput<'de, super::AttributeType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Attribute(_, Some(deserializer))) => {
                        Self::Attribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Attribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::Attribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Attribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_any_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyAttributeElementType>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AnyAttribute(_, Some(deserializer))) => {
                        Self::AnyAttribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AnyAttribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_any_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::AnyAttribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AnyAttribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_assert<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AssertionType>,
            output: DeserializerOutput<'de, super::AssertionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Assert(_, Some(deserializer))) => {
                        Self::Assert(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Assert(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_assert(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assert(&mut values, data)?;
                    let data = Self::Assert(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Assert(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionTypeContent> for RestrictionTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::OpenContent(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_open_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Facet(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_facet(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assert(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_assert(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::OpenContent(values, None), event) => {
                        let output = < super :: OpenContentElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_open_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::SimpleType(values, None), event) => {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Facet(values, None), event) => {
                        let output =
                            <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_facet(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, None), event) => {
                        let output =
                            <super::AttributeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, None), event) => {
                        let output = < super :: AnyAttributeElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assert(values, None), event) => {
                        let output =
                            <super::AssertionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_assert(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::RestrictionTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Annotation(
                        values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                    ))
                }
                Self::OpenContent(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_open_content(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::OpenContent(
                        values.ok_or_else(|| ErrorKind::MissingElement("openContent".into()))?,
                    ))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Group(values.ok_or_else(
                        || ErrorKind::MissingElement("group".into()),
                    )?))
                }
                Self::All(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::All(
                        values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                    ))
                }
                Self::Choice(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Choice(values.ok_or_else(
                        || ErrorKind::MissingElement("choice".into()),
                    )?))
                }
                Self::Sequence(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Sequence(values.ok_or_else(
                        || ErrorKind::MissingElement("sequence".into()),
                    )?))
                }
                Self::SimpleType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::SimpleType(
                        values.ok_or_else(|| ErrorKind::MissingElement("simpleType".into()))?,
                    ))
                }
                Self::Facet(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_facet(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Facet(values.ok_or_else(
                        || ErrorKind::MissingElement("facet".into()),
                    )?))
                }
                Self::Attribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Attribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                    ))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::AnyAttribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::AnyAttribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                    ))
                }
                Self::Assert(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_assert(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Assert(values.ok_or_else(
                        || ErrorKind::MissingElement("assert".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
        Init__,
        Next__,
        Content__(<super::ExtensionTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ExtensionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"base")
                ) {
                    reader.read_attrib(&mut base, b"base", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                base: base
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("base".into())))?,
                content: Vec::new(),
                state: Box::new(ExtensionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ExtensionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ExtensionTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ExtensionTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ExtensionTypeContent>,
            fallback: &mut Option<ExtensionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(ExtensionTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ExtensionTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ExtensionTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = ExtensionTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ExtensionTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExtensionType> for ExtensionTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ExtensionType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionType>
        where
            R: DeserializeReader,
        {
            use ExtensionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::ExtensionTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ExtensionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ExtensionTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ExtensionType {
                id: self.id,
                base: self.base,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub enum ExtensionTypeContentDeserializer {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContentElementType>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Assert(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ExtensionTypeContent),
        Unknown__,
    }
    impl ExtensionTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ExtensionTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"openContent")
            ) {
                let output =
                    <super::OpenContentElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttributeElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_XS),
                Some(b"assert")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_annotation(
            values: &mut Option<super::AnnotationElementType>,
            value: super::AnnotationElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_open_content(
            values: &mut Option<super::OpenContentElementType>,
            value: super::OpenContentElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"openContent",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_group(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"group",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_all(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_choice(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"choice",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sequence(
            values: &mut Option<super::GroupType>,
            value: super::GroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sequence",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute(
            values: &mut Option<super::AttributeType>,
            value: super::AttributeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_attribute_group(
            values: &mut Option<super::AttributeGroupType>,
            value: super::AttributeGroupType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"attributeGroup",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_any_attribute(
            values: &mut Option<super::AnyAttributeElementType>,
            value: super::AnyAttributeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"anyAttribute",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_assert(
            values: &mut Option<super::AssertionType>,
            value: super::AssertionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"assert",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnnotationElementType>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Annotation(_, Some(deserializer))) => {
                        Self::Annotation(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Annotation(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_annotation(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::Annotation(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Annotation(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_open_content<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpenContentElementType>,
            output: DeserializerOutput<'de, super::OpenContentElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::OpenContent(_, Some(deserializer))) => {
                        Self::OpenContent(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::OpenContent(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_open_content(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_open_content(&mut values, data)?;
                    let data = Self::OpenContent(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::OpenContent(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Group(_, Some(deserializer))) => {
                        Self::Group(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Group(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::Group(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Group(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_all<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::All(_, Some(deserializer))) => Self::All(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::All(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_all(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::All(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::All(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_choice<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Choice(_, Some(deserializer))) => {
                        Self::Choice(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Choice(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_choice(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::Choice(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Choice(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sequence<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::GroupType>,
            output: DeserializerOutput<'de, super::GroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Sequence(_, Some(deserializer))) => {
                        Self::Sequence(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Sequence(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sequence(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::Sequence(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Sequence(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeType>,
            output: DeserializerOutput<'de, super::AttributeType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Attribute(_, Some(deserializer))) => {
                        Self::Attribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Attribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::Attribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Attribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_attribute_group<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AttributeGroupType>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AttributeGroup(_, Some(deserializer))) => {
                        Self::AttributeGroup(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AttributeGroup(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_attribute_group(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::AttributeGroup(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AttributeGroup(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_any_attribute<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyAttributeElementType>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::AnyAttribute(_, Some(deserializer))) => {
                        Self::AnyAttribute(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::AnyAttribute(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_any_attribute(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::AnyAttribute(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::AnyAttribute(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_assert<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AssertionType>,
            output: DeserializerOutput<'de, super::AssertionType>,
            fallback: &mut Option<Self>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Assert(_, Some(deserializer))) => {
                        Self::Assert(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Assert(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_assert(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assert(&mut values, data)?;
                    let data = Self::Assert(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Assert(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExtensionTypeContent> for ExtensionTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Annotation(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::OpenContent(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_open_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assert(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_assert(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Annotation(values, None), event) => {
                        let output =
                            <super::AnnotationElementType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_annotation(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::OpenContent(values, None), event) => {
                        let output = < super :: OpenContentElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_open_content(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Group(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::All(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_all(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Choice(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_choice(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Sequence(values, None), event) => {
                        let output = <super::GroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sequence(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Attribute(values, None), event) => {
                        let output =
                            <super::AttributeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AttributeGroup(values, None), event) => {
                        let output =
                            <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::AnyAttribute(values, None), event) => {
                        let output = < super :: AnyAttributeElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Assert(values, None), event) => {
                        let output =
                            <super::AssertionType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_assert(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::ExtensionTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Annotation(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Annotation(values.ok_or_else(
                        || ErrorKind::MissingElement("annotation".into()),
                    )?))
                }
                Self::OpenContent(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_open_content(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::OpenContent(
                        values.ok_or_else(|| ErrorKind::MissingElement("openContent".into()))?,
                    ))
                }
                Self::Group(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Group(values.ok_or_else(
                        || ErrorKind::MissingElement("group".into()),
                    )?))
                }
                Self::All(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::All(
                        values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                    ))
                }
                Self::Choice(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Choice(values.ok_or_else(
                        || ErrorKind::MissingElement("choice".into()),
                    )?))
                }
                Self::Sequence(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Sequence(values.ok_or_else(
                        || ErrorKind::MissingElement("sequence".into()),
                    )?))
                }
                Self::Attribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Attribute(values.ok_or_else(
                        || ErrorKind::MissingElement("attribute".into()),
                    )?))
                }
                Self::AttributeGroup(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::AttributeGroup(
                        values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                    ))
                }
                Self::AnyAttribute(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::AnyAttribute(
                        values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                    ))
                }
                Self::Assert(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_assert(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Assert(values.ok_or_else(
                        || ErrorKind::MissingElement("assert".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct FieldElementTypeDeserializer {
        id: Option<String>,
        xpath: String,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        annotation: Option<super::AnnotationElementType>,
        state: Box<FieldElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FieldElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FieldElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut xpath: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpath")
                ) {
                    reader.read_attrib(&mut xpath, b"xpath", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
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
                xpath: xpath
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("xpath".into())))?,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state: Box::new(FieldElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FieldElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FieldElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<FieldElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(FieldElementTypeDeserializerState::Annotation(None));
                *self.state = FieldElementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = FieldElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FieldElementTypeDeserializerState::Annotation(
                                Some(deserializer),
                            ));
                            *self.state = FieldElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FieldElementTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FieldElementType> for FieldElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldElementType>
        where
            R: DeserializeReader,
        {
            use FieldElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FieldElementTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FieldElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                FieldElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::FieldElementType {
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
        annotation: Option<super::AnnotationElementType>,
        state: Box<FacetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FacetTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FacetTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut value: Option<String> = None;
            let mut fixed: Option<bool> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"value")
                ) {
                    reader.read_attrib(&mut value, b"value", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"fixed")
                ) {
                    reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id,
                value: value
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("value".into())))?,
                fixed: fixed.unwrap_or_else(super::FacetType::default_fixed),
                annotation: None,
                state: Box::new(FacetTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FacetTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FacetTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::AnnotationElementType) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn handle_annotation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<FacetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(FacetTypeDeserializerState::Annotation(None));
                *self.state = FacetTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state = FacetTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FacetTypeDeserializerState::Annotation(Some(
                                deserializer,
                            )));
                            *self.state = FacetTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FacetTypeDeserializerState::Annotation(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FacetType> for FacetTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FacetType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FacetType>
        where
            R: DeserializeReader,
        {
            use FacetTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FacetTypeDeserializerState::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_XS), b"annotation") {
                            let output = < super :: AnnotationElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_annotation(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Annotation(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FacetType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FacetTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FacetType {
                id: self.id,
                value: self.value,
                fixed: self.fixed,
                annotation: self.annotation,
            })
        }
    }
}
