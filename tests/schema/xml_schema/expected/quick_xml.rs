use xsd_parser::schema::Namespace;
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
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
#[derive(Debug, Clone)]
pub enum FullDerivationSetType {
    All,
    TypeDerivationControlList(TypeDerivationControlList),
}
#[derive(Debug, Clone, Default)]
pub struct TypeDerivationControlList(pub Vec<TypeDerivationControlType>);
#[derive(Debug, Clone)]
pub enum BlockSetType {
    All,
    BlockSetItemList(BlockSetItemList),
}
#[derive(Debug, Clone, Default)]
pub struct BlockSetItemList(pub Vec<BlockSetItemType>);
#[derive(Debug, Clone)]
pub enum FormChoiceType {
    Qualified,
    Unqualified,
}
#[derive(Debug, Clone)]
pub enum XpathDefaultNamespaceType {
    String(String),
    DefaultNamespace,
    TargetNamespace,
    Local,
}
#[derive(Debug, Clone)]
pub struct IncludeElementType {
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<AnnotationElementType>,
}
#[derive(Debug, Clone)]
pub struct ImportElementType {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<AnnotationElementType>,
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
#[derive(Debug, Clone)]
pub struct NotationElementType {
    pub id: Option<String>,
    pub name: String,
    pub public: Option<String>,
    pub system: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
#[derive(Debug, Clone)]
pub enum TypeDerivationControlType {
    Extension,
    Restriction,
    List,
    Union,
}
#[derive(Debug, Clone)]
pub enum BlockSetItemType {
    Extension,
    Restriction,
    Substitution,
}
#[derive(Debug, Clone)]
pub struct AppinfoElementType {
    pub source: Option<String>,
}
#[derive(Debug, Clone)]
pub struct DocumentationElementType {
    pub source: Option<String>,
    pub lang: Option<String>,
}
#[derive(Debug, Clone)]
pub enum DefaultOpenContentModeType {
    Interleave,
    Suffix,
}
#[derive(Debug, Clone)]
pub struct WildcardType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub annotation: Option<AnnotationElementType>,
}
impl WildcardType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
#[derive(Debug, Clone)]
pub enum SimpleDerivationSetType {
    All,
    SimpleDerivationSetItemList(SimpleDerivationSetItemList),
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
#[derive(Debug, Clone)]
pub struct ListElementType {
    pub id: Option<String>,
    pub item_type: Option<String>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Option<Box<SimpleBaseType>>,
}
#[derive(Debug, Clone)]
pub struct UnionElementType {
    pub id: Option<String>,
    pub member_types: Option<Entitiestype>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Vec<SimpleBaseType>,
}
#[derive(Debug, Clone)]
pub enum DerivationSetType {
    All,
    ReducedDerivationControlList(ReducedDerivationControlList),
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
#[derive(Debug, Clone)]
pub struct AnyAttributeElementType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
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
#[derive(Debug, Clone)]
pub struct AssertionType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
#[derive(Debug, Clone)]
pub enum AllNNIType {
    Usize(usize),
    Unbounded,
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
#[derive(Debug, Clone, Default)]
pub struct Entitiestype(pub Vec<String>);
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
#[derive(Debug, Clone)]
pub enum AttributeUseType {
    Prohibited,
    Optional,
    Required,
}
#[derive(Debug, Clone)]
pub enum NamespaceListType {
    Any,
    Other,
    BasicNamespaceList(BasicNamespaceListType),
}
#[derive(Debug, Clone, Default)]
pub struct NotNamespaceType(pub Vec<BasicNamespaceListItemType>);
#[derive(Debug, Clone)]
pub enum ProcessContentsType {
    Skip,
    Lax,
    Strict,
}
#[derive(Debug, Clone, Default)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
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
#[derive(Debug, Clone, Default)]
pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
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
#[derive(Debug, Clone)]
pub enum OpenContentModeType {
    None,
    Interleave,
    Suffix,
}
#[derive(Debug, Clone, Default)]
pub struct QnameListAType(pub Vec<QnameListAItemType>);
#[derive(Debug, Clone, Default)]
pub struct QnameListType(pub Vec<QnameListItemType>);
#[derive(Debug, Clone)]
pub struct FieldElementType {
    pub id: Option<String>,
    pub xpath: String,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
#[derive(Debug, Clone, Default)]
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
#[derive(Debug, Clone)]
pub enum BasicNamespaceListItemType {
    String(String),
    TargetNamespace,
    Local,
}
#[derive(Debug, Clone)]
pub enum SimpleDerivationSetItemType {
    List,
    Union,
    Restriction,
    Extension,
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
#[derive(Debug, Clone)]
pub enum ReducedDerivationControlType {
    Extension,
    Restriction,
}
#[derive(Debug, Clone)]
pub enum QnameListAItemType {
    String(String),
    Defined,
}
#[derive(Debug, Clone)]
pub enum QnameListItemType {
    String(String),
    Defined,
    DefinedSibling,
}
