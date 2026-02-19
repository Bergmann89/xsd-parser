use core::ops::Deref;
use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, ErrorKind, RawByteStr, ValidateError,
        WithDeserializer,
    },
    xml::{AnyAttributes, AnyElement, Mixed, Text},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub type Schema = SchemaElementType;
#[derive(Debug)]
pub struct SchemaElementType {
    pub any_attribute: AnyAttributes,
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
#[derive(Debug)]
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
///A utility type, not for public use
///#all or (possibly empty) subset of {extension, restriction, list, union}
#[derive(Debug)]
pub enum FullDerivationSetType {
    All,
    TypeDerivationControlList(TypeDerivationControlList),
}
impl DeserializeBytes for FullDerivationSetType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::TypeDerivationControlList(
                TypeDerivationControlList::deserialize_bytes(helper, x)?,
            )),
        }
    }
}
#[derive(Debug, Default)]
pub struct TypeDerivationControlList(pub Vec<TypeDerivationControlType>);
impl DeserializeBytes for TypeDerivationControlList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
///A utility type, not for public use
///#all or (possibly empty) subset of {substitution, extension,
///restriction}
#[derive(Debug)]
pub enum BlockSetType {
    All,
    BlockSetItemList(BlockSetItemList),
}
impl DeserializeBytes for BlockSetType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::BlockSetItemList(BlockSetItemList::deserialize_bytes(
                helper, x,
            )?)),
        }
    }
}
#[derive(Debug, Default)]
pub struct BlockSetItemList(pub Vec<BlockSetItemType>);
impl DeserializeBytes for BlockSetItemList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
///A utility type, not for public use
#[derive(Debug)]
pub enum FormChoiceType {
    Qualified,
    Unqualified,
}
impl DeserializeBytes for FormChoiceType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"qualified" => Ok(Self::Qualified),
            b"unqualified" => Ok(Self::Unqualified),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug)]
pub enum XpathDefaultNamespaceType {
    String(String),
    DefaultNamespace,
    TargetNamespace,
    Local,
}
impl DeserializeBytes for XpathDefaultNamespaceType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##defaultNamespace" => Ok(Self::DefaultNamespace),
            b"##targetNamespace" => Ok(Self::TargetNamespace),
            b"##local" => Ok(Self::Local),
            x => Ok(Self::String(String::deserialize_bytes(helper, x)?)),
        }
    }
}
#[derive(Debug)]
pub struct IncludeElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for IncludeElementType {
    type Deserializer = quick_xml_deserialize::IncludeElementTypeDeserializer;
}
#[derive(Debug)]
pub struct ImportElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for ImportElementType {
    type Deserializer = quick_xml_deserialize::ImportElementTypeDeserializer;
}
#[derive(Debug)]
pub struct RedefineElementType {
    pub any_attribute: AnyAttributes,
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<RedefineElementTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct OverrideElementType {
    pub any_attribute: AnyAttributes,
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<OverrideElementTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct AnnotationElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub content: Vec<AnnotationElementTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct DefaultOpenContentElementType {
    pub any_attribute: AnyAttributes,
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
#[derive(Debug)]
pub struct SimpleBaseType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub final_: Option<SimpleDerivationSetType>,
    ///Can be restricted to required or forbidden
    pub name: Option<String>,
    pub content: Vec<SimpleBaseTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ComplexBaseType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    ///Will be restricted to required or prohibited
    pub name: Option<String>,
    ///Not allowed if simpleContent child is chosen.
    ///May be overridden by setting on complexContent child.
    pub mixed: Option<bool>,
    pub abstract_: bool,
    pub final_: Option<DerivationSetType>,
    pub block: Option<DerivationSetType>,
    pub default_attributes_apply: bool,
    pub content: Vec<ComplexBaseTypeContent>,
}
#[derive(Debug)]
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
///group type for explicit groups, named top-level groups and
///group references
///for element, group and attributeGroup,
///which both define and reference
///for all particles
#[derive(Debug)]
pub struct GroupType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub min_occurs: usize,
    pub max_occurs: AllNniType,
    pub content: Vec<GroupTypeContent>,
}
///group type for explicit groups, named top-level groups and
///group references
///for element, group and attributeGroup,
///which both define and reference
///for all particles
#[derive(Debug)]
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
    pub fn default_max_occurs() -> AllNniType {
        AllNniType::Usize(1usize)
    }
}
impl WithDeserializer for GroupType {
    type Deserializer = quick_xml_deserialize::GroupTypeDeserializer;
}
impl WithDeserializer for GroupTypeContent {
    type Deserializer = quick_xml_deserialize::GroupTypeContentDeserializer;
}
///for element, group and attributeGroup,
///which both define and reference
#[derive(Debug)]
pub struct AttributeGroupType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub content: Vec<AttributeGroupTypeContent>,
}
///for element, group and attributeGroup,
///which both define and reference
#[derive(Debug)]
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
///The element element can be used either
///at the top level to define an element-type binding globally,
///or within a content model to either reference a globally-defined
///element or type or declare an element-type binding locally.
///The ref form is not allowed at the top level.
///for element, group and attributeGroup,
///which both define and reference
///for all particles
#[derive(Debug)]
pub struct ElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub type_: Option<String>,
    pub substitution_group: Option<EntitiesType>,
    pub min_occurs: usize,
    pub max_occurs: AllNniType,
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
///The element element can be used either
///at the top level to define an element-type binding globally,
///or within a content model to either reference a globally-defined
///element or type or declare an element-type binding locally.
///The ref form is not allowed at the top level.
///for element, group and attributeGroup,
///which both define and reference
///for all particles
#[derive(Debug)]
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
    pub fn default_max_occurs() -> AllNniType {
        AllNniType::Usize(1usize)
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
///for element, group and attributeGroup,
///which both define and reference
#[derive(Debug)]
pub struct AttributeType {
    pub any_attribute: AnyAttributes,
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
#[derive(Debug)]
pub struct NotationElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub name: String,
    pub public: Option<String>,
    pub system: Option<String>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for NotationElementType {
    type Deserializer = quick_xml_deserialize::NotationElementTypeDeserializer;
}
///A utility type, not for public use
#[derive(Debug)]
pub enum TypeDerivationControlType {
    Extension,
    Restriction,
    List,
    Union,
}
impl DeserializeBytes for TypeDerivationControlType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            b"list" => Ok(Self::List),
            b"union" => Ok(Self::Union),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug)]
pub enum BlockSetItemType {
    Extension,
    Restriction,
    Substitution,
}
impl DeserializeBytes for BlockSetItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            b"substitution" => Ok(Self::Substitution),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug)]
pub struct AppinfoElementType {
    pub source: Option<String>,
    pub any_attribute: AnyAttributes,
    pub text_before: Option<Text>,
    pub content: Vec<AppinfoElementTypeContent>,
}
#[derive(Debug)]
pub struct AppinfoElementTypeContent {
    pub any: Mixed<AnyElement>,
}
impl WithDeserializer for AppinfoElementType {
    type Deserializer = quick_xml_deserialize::AppinfoElementTypeDeserializer;
}
impl WithDeserializer for AppinfoElementTypeContent {
    type Deserializer = quick_xml_deserialize::AppinfoElementTypeContentDeserializer;
}
#[derive(Debug)]
pub struct DocumentationElementType {
    pub source: Option<String>,
    pub lang: Option<String>,
    pub any_attribute: AnyAttributes,
    pub text_before: Option<Text>,
    pub content: Vec<DocumentationElementTypeContent>,
}
#[derive(Debug)]
pub struct DocumentationElementTypeContent {
    pub any: Mixed<AnyElement>,
}
impl WithDeserializer for DocumentationElementType {
    type Deserializer = quick_xml_deserialize::DocumentationElementTypeDeserializer;
}
impl WithDeserializer for DocumentationElementTypeContent {
    type Deserializer = quick_xml_deserialize::DocumentationElementTypeContentDeserializer;
}
#[derive(Debug)]
pub enum DefaultOpenContentModeType {
    Interleave,
    Suffix,
}
impl DeserializeBytes for DefaultOpenContentModeType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"interleave" => Ok(Self::Interleave),
            b"suffix" => Ok(Self::Suffix),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug)]
pub struct WildcardType {
    pub any_attribute: AnyAttributes,
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
impl WithDeserializer for WildcardType {
    type Deserializer = quick_xml_deserialize::WildcardTypeDeserializer;
}
///#all or (possibly empty) subset of {restriction, extension, union, list}
///A utility type, not for public use
#[derive(Debug)]
pub enum SimpleDerivationSetType {
    All,
    SimpleDerivationSetItemList(SimpleDerivationSetItemList),
}
impl DeserializeBytes for SimpleDerivationSetType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::SimpleDerivationSetItemList(
                SimpleDerivationSetItemList::deserialize_bytes(helper, x)?,
            )),
        }
    }
}
///base attribute and simpleType child are mutually
///exclusive, but one or other is required
#[derive(Debug)]
pub struct RestrictionElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub base: Option<String>,
    pub content: Vec<RestrictionElementTypeContent>,
}
///base attribute and simpleType child are mutually
///exclusive, but one or other is required
#[derive(Debug)]
pub enum RestrictionElementTypeContent {
    Annotation(AnnotationElementType),
    SimpleType(SimpleBaseType),
    Facet(Facet),
    Any(AnyElement),
}
impl WithDeserializer for RestrictionElementType {
    type Deserializer = quick_xml_deserialize::RestrictionElementTypeDeserializer;
}
impl WithDeserializer for RestrictionElementTypeContent {
    type Deserializer = quick_xml_deserialize::RestrictionElementTypeContentDeserializer;
}
///itemType attribute and simpleType child are mutually
///exclusive, but one or other is required
#[derive(Debug)]
pub struct ListElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub item_type: Option<String>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Option<Box<SimpleBaseType>>,
}
impl WithDeserializer for ListElementType {
    type Deserializer = quick_xml_deserialize::ListElementTypeDeserializer;
}
///memberTypes attribute must be non-empty or there must be
///at least one simpleType child
#[derive(Debug)]
pub struct UnionElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub member_types: Option<EntitiesType>,
    pub annotation: Option<AnnotationElementType>,
    pub simple_type: Vec<SimpleBaseType>,
}
impl WithDeserializer for UnionElementType {
    type Deserializer = quick_xml_deserialize::UnionElementTypeDeserializer;
}
///A utility type, not for public use
///#all or (possibly empty) subset of {extension, restriction}
#[derive(Debug)]
pub enum DerivationSetType {
    All,
    ReducedDerivationControlList(ReducedDerivationControlList),
}
impl DeserializeBytes for DerivationSetType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"#all" => Ok(Self::All),
            x => Ok(Self::ReducedDerivationControlList(
                ReducedDerivationControlList::deserialize_bytes(helper, x)?,
            )),
        }
    }
}
#[derive(Debug)]
pub struct SimpleContentElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub content: Vec<SimpleContentElementTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ComplexContentElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    ///Overrides any setting on complexType parent.
    pub mixed: Option<bool>,
    pub content: Vec<ComplexContentElementTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct OpenContentElementType {
    pub any_attribute: AnyAttributes,
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
#[derive(Debug)]
pub struct AnyAttributeElementType {
    pub any_attribute: AnyAttributes,
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
impl WithDeserializer for AnyAttributeElementType {
    type Deserializer = quick_xml_deserialize::AnyAttributeElementTypeDeserializer;
}
#[derive(Debug)]
pub struct AssertionType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for AssertionType {
    type Deserializer = quick_xml_deserialize::AssertionTypeDeserializer;
}
///for maxOccurs
#[derive(Debug)]
pub enum AllNniType {
    Usize(usize),
    Unbounded,
}
impl DeserializeBytes for AllNniType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"unbounded" => Ok(Self::Unbounded),
            x => Ok(Self::Usize(usize::deserialize_bytes(helper, x)?)),
        }
    }
}
///for all particles
#[derive(Debug)]
pub struct AnyElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListType>,
    pub min_occurs: usize,
    pub max_occurs: AllNniType,
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
    pub fn default_max_occurs() -> AllNniType {
        AllNniType::Usize(1usize)
    }
}
impl WithDeserializer for AnyElementType {
    type Deserializer = quick_xml_deserialize::AnyElementTypeDeserializer;
}
#[derive(Debug, Default)]
pub struct EntitiesType(pub Vec<String>);
impl DeserializeBytes for EntitiesType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
///This type is used for 'alternative' elements.
#[derive(Debug)]
pub struct AltType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub test: Option<String>,
    pub type_: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub content: Vec<AltTypeContent>,
}
///This type is used for 'alternative' elements.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct KeybaseType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub content: Option<KeybaseTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct KeyrefElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<String>,
    pub refer: Option<String>,
    pub content: Option<KeyrefElementTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub enum AttributeUseType {
    Prohibited,
    Optional,
    Required,
}
impl DeserializeBytes for AttributeUseType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"prohibited" => Ok(Self::Prohibited),
            b"optional" => Ok(Self::Optional),
            b"required" => Ok(Self::Required),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
///A utility type, not for public use
#[derive(Debug)]
pub enum NamespaceListType {
    Any,
    Other,
    BasicNamespaceList(BasicNamespaceListType),
}
impl DeserializeBytes for NamespaceListType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##any" => Ok(Self::Any),
            b"##other" => Ok(Self::Other),
            x => Ok(Self::BasicNamespaceList(
                BasicNamespaceListType::deserialize_bytes(helper, x)?,
            )),
        }
    }
}
#[derive(Debug)]
pub struct NotNamespaceType(pub Vec<BasicNamespaceListItemType>);
impl NotNamespaceType {
    pub fn new(inner: Vec<BasicNamespaceListItemType>) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> Vec<BasicNamespaceListItemType> {
        self.0
    }
    pub fn validate_value(value: &Vec<BasicNamespaceListItemType>) -> Result<(), ValidateError> {
        if value.is_empty() {
            return Err(ValidateError::MinLength(1usize));
        }
        Ok(())
    }
}
impl From<NotNamespaceType> for Vec<BasicNamespaceListItemType> {
    fn from(value: NotNamespaceType) -> Vec<BasicNamespaceListItemType> {
        value.0
    }
}
impl TryFrom<Vec<BasicNamespaceListItemType>> for NotNamespaceType {
    type Error = ValidateError;
    fn try_from(value: Vec<BasicNamespaceListItemType>) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for NotNamespaceType {
    type Target = Vec<BasicNamespaceListItemType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DeserializeBytes for NotNamespaceType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let inner = helper.deserialize_list(bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
#[derive(Debug)]
pub enum ProcessContentsType {
    Skip,
    Lax,
    Strict,
}
impl DeserializeBytes for ProcessContentsType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"skip" => Ok(Self::Skip),
            b"lax" => Ok(Self::Lax),
            b"strict" => Ok(Self::Strict),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug, Default)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
impl DeserializeBytes for SimpleDerivationSetItemList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
///An abstract element, representing facets in general.
///The facets defined by this spec are substitutable for
///this element, and implementation-defined facets should
///also name this as a substitution-group head.
#[derive(Debug)]
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
#[derive(Debug, Default)]
pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
impl DeserializeBytes for ReducedDerivationControlList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Debug)]
pub struct RestrictionType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub base: String,
    pub content: Vec<RestrictionTypeContent>,
}
#[derive(Debug)]
pub enum RestrictionTypeContent {
    Annotation(AnnotationElementType),
    OpenContent(OpenContentElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    SimpleType(SimpleBaseType),
    Facet(Facet),
    Any(AnyElement),
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
#[derive(Debug)]
pub struct ExtensionType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub base: String,
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub enum OpenContentModeType {
    None,
    Interleave,
    Suffix,
}
impl DeserializeBytes for OpenContentModeType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"none" => Ok(Self::None),
            b"interleave" => Ok(Self::Interleave),
            b"suffix" => Ok(Self::Suffix),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
///A utility type, not for public use
#[derive(Debug, Default)]
pub struct QnameListAType(pub Vec<QnameListAItemType>);
impl DeserializeBytes for QnameListAType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
///A utility type, not for public use
#[derive(Debug, Default)]
pub struct QnameListType(pub Vec<QnameListItemType>);
impl DeserializeBytes for QnameListType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Debug)]
pub struct FieldElementType {
    pub any_attribute: AnyAttributes,
    pub id: Option<String>,
    pub xpath: String,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<AnnotationElementType>,
}
impl WithDeserializer for FieldElementType {
    type Deserializer = quick_xml_deserialize::FieldElementTypeDeserializer;
}
///A utility type, not for public use
#[derive(Debug, Default)]
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
impl DeserializeBytes for BasicNamespaceListType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Debug)]
pub enum BasicNamespaceListItemType {
    String(String),
    TargetNamespace,
    Local,
}
impl DeserializeBytes for BasicNamespaceListItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##targetNamespace" => Ok(Self::TargetNamespace),
            b"##local" => Ok(Self::Local),
            x => Ok(Self::String(String::deserialize_bytes(helper, x)?)),
        }
    }
}
#[derive(Debug)]
pub enum SimpleDerivationSetItemType {
    List,
    Union,
    Restriction,
    Extension,
}
impl DeserializeBytes for SimpleDerivationSetItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"list" => Ok(Self::List),
            b"union" => Ok(Self::Union),
            b"restriction" => Ok(Self::Restriction),
            b"extension" => Ok(Self::Extension),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug)]
pub struct FacetType {
    pub any_attribute: AnyAttributes,
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
///A utility type, not for public use
#[derive(Debug)]
pub enum ReducedDerivationControlType {
    Extension,
    Restriction,
}
impl DeserializeBytes for ReducedDerivationControlType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"extension" => Ok(Self::Extension),
            b"restriction" => Ok(Self::Restriction),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
#[derive(Debug)]
pub enum QnameListAItemType {
    String(String),
    Defined,
}
impl DeserializeBytes for QnameListAItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            x => Ok(Self::String(String::deserialize_bytes(helper, x)?)),
        }
    }
}
#[derive(Debug)]
pub enum QnameListItemType {
    String(String),
    Defined,
    DefinedSibling,
}
impl DeserializeBytes for QnameListItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            b"##definedSibling" => Ok(Self::DefinedSibling),
            x => Ok(Self::String(String::deserialize_bytes(helper, x)?)),
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        },
        xml::{AnyAttributes, AnyElement, Mixed, Text},
    };
    #[derive(Debug)]
    pub struct SchemaElementTypeDeserializer {
        any_attribute: AnyAttributes,
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
        state__: Box<SchemaElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SchemaElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SchemaElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SchemaElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
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
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"targetNamespace")
                ) {
                    helper.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"version")
                ) {
                    helper.read_attrib(&mut version, b"version", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"finalDefault")
                ) {
                    helper.read_attrib(&mut final_default, b"finalDefault", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"blockDefault")
                ) {
                    helper.read_attrib(&mut block_default, b"blockDefault", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"attributeFormDefault")
                ) {
                    helper.read_attrib(
                        &mut attribute_form_default,
                        b"attributeFormDefault",
                        &attrib.value,
                    )?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"elementFormDefault")
                ) {
                    helper.read_attrib(
                        &mut element_form_default,
                        b"elementFormDefault",
                        &attrib.value,
                    )?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"defaultAttributes")
                ) {
                    helper.read_attrib(
                        &mut default_attributes,
                        b"defaultAttributes",
                        &attrib.value,
                    )?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    helper.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XML),
                    Some(b"lang")
                ) {
                    helper.read_attrib(&mut lang, b"lang", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
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
                state__: Box::new(SchemaElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SchemaElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SchemaElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SchemaElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SchemaElementTypeContent>,
            fallback: &mut Option<SchemaElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SchemaElementType> for SchemaElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementType> {
            use SchemaElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::SchemaElementTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SchemaElementType, Error> {
            let state = replace(
                &mut *self.state__,
                SchemaElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SchemaElementType {
                any_attribute: self.any_attribute,
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
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SchemaElementTypeContentDeserializer {
        state__: Box<SchemaElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SchemaElementTypeContentDeserializerState {
        Init__,
        Include(
            Option<super::IncludeElementType>,
            Option<<super::IncludeElementType as WithDeserializer>::Deserializer>,
            Option<<super::IncludeElementType as WithDeserializer>::Deserializer>,
        ),
        Import(
            Option<super::ImportElementType>,
            Option<<super::ImportElementType as WithDeserializer>::Deserializer>,
            Option<<super::ImportElementType as WithDeserializer>::Deserializer>,
        ),
        Redefine(
            Option<super::RedefineElementType>,
            Option<<super::RedefineElementType as WithDeserializer>::Deserializer>,
            Option<<super::RedefineElementType as WithDeserializer>::Deserializer>,
        ),
        Override(
            Option<super::OverrideElementType>,
            Option<<super::OverrideElementType as WithDeserializer>::Deserializer>,
            Option<<super::OverrideElementType as WithDeserializer>::Deserializer>,
        ),
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        DefaultOpenContent(
            Option<super::DefaultOpenContentElementType>,
            Option<<super::DefaultOpenContentElementType as WithDeserializer>::Deserializer>,
            Option<<super::DefaultOpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        Element(
            Option<super::ElementType>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        Notation(
            Option<super::NotationElementType>,
            Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SchemaElementTypeContent),
        Unknown__,
    }
    impl SchemaElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"include")
                ) {
                    let output =
                        <super::IncludeElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_include(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"import")
                ) {
                    let output =
                        <super::ImportElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_import(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"redefine")
                ) {
                    let output =
                        <super::RedefineElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_redefine(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"override")
                ) {
                    let output =
                        <super::OverrideElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_override_(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"defaultOpenContent")
                ) {
                    let output = <super::DefaultOpenContentElementType as WithDeserializer>::init(
                        helper, event,
                    )?;
                    return self.handle_default_open_content(
                        helper,
                        Default::default(),
                        None,
                        output,
                    );
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexType")
                ) {
                    let output = <super::ComplexBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_complex_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"element")
                ) {
                    let output = <super::ElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_element(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attribute")
                ) {
                    let output = <super::AttributeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"notation")
                ) {
                    let output =
                        <super::NotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_notation(helper, Default::default(), None, output);
                }
            }
            *self.state__ = SchemaElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SchemaElementTypeContentDeserializerState,
        ) -> Result<super::SchemaElementTypeContent, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Include(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_include(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Include(
                        helper.finish_element("include", values)?,
                    ))
                }
                S::Import(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_import(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Import(
                        helper.finish_element("import", values)?,
                    ))
                }
                S::Redefine(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_redefine(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Redefine(
                        helper.finish_element("redefine", values)?,
                    ))
                }
                S::Override(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_override_(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Override(
                        helper.finish_element("override", values)?,
                    ))
                }
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::DefaultOpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_default_open_content(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::DefaultOpenContent(
                        helper.finish_element("defaultOpenContent", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::Element(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_element(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Element(
                        helper.finish_element("element", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::Notation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_notation(&mut values, value)?;
                    }
                    Ok(super::SchemaElementTypeContent::Notation(
                        helper.finish_element("notation", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_include<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::IncludeElementType>,
            fallback: Option<<super::IncludeElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::IncludeElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_include(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_include(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Include(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Include(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_import<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ImportElementType>,
            fallback: Option<<super::ImportElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ImportElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_import(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_import(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Import(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Import(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_redefine<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RedefineElementType>,
            fallback: Option<<super::RedefineElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RedefineElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_redefine(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_redefine(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Redefine(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Redefine(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_override_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OverrideElementType>,
            fallback: Option<<super::OverrideElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OverrideElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_override_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_override_(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Override(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Override(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_default_open_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DefaultOpenContentElementType>,
            fallback: Option<
                <super::DefaultOpenContentElementType as WithDeserializer>::Deserializer,
            >,
            output: DeserializerOutput<'de, super::DefaultOpenContentElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_default_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_default_open_content(&mut values, data)?;
                    let data =
                        Self::finish_state(helper, S::DefaultOpenContent(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::DefaultOpenContent(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_complex_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ComplexBaseType>,
            fallback: Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ComplexType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ComplexType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_element<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ElementType>,
            fallback: Option<<super::ElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_element(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Element(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Element(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeType>,
            fallback: Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Attribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Attribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_notation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::NotationElementType>,
            fallback: Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::NotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_notation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_notation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Notation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Notation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SchemaElementTypeContent>
        for SchemaElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(SchemaElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        SchemaElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaElementTypeContent> {
            use SchemaElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Include(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_include(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Import(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_import(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Redefine(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_redefine(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Override(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_override_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::DefaultOpenContent(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_default_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ComplexType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Element(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_element(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Attribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Notation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_notation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Include(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"include",
                            true,
                        )?;
                        match self.handle_include(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Import(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"import",
                            true,
                        )?;
                        match self.handle_import(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Redefine(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"redefine",
                            true,
                        )?;
                        match self.handle_redefine(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Override(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"override",
                            true,
                        )?;
                        match self.handle_override_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::DefaultOpenContent(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"defaultOpenContent",
                            true,
                        )?;
                        match self.handle_default_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ComplexType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"complexType",
                            true,
                        )?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Element(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"element",
                            true,
                        )?;
                        match self.handle_element(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            true,
                        )?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Notation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"notation",
                            true,
                        )?;
                        match self.handle_notation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SchemaElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct IncludeElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        schema_location: String,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<IncludeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IncludeElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl IncludeElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut schema_location: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    helper.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                schema_location: schema_location
                    .ok_or_else(|| ErrorKind::MissingAttribute("schemaLocation".into()))?,
                annotation: None,
                state__: Box::new(IncludeElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: IncludeElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use IncludeElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<IncludeElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use IncludeElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IncludeElementType> for IncludeElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IncludeElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IncludeElementType> {
            use IncludeElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::IncludeElementType, Error> {
            let state = replace(
                &mut *self.state__,
                IncludeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::IncludeElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                schema_location: self.schema_location,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct ImportElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        namespace: Option<String>,
        schema_location: Option<String>,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<ImportElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ImportElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ImportElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut namespace: Option<String> = None;
            let mut schema_location: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    helper.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    helper.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                namespace: namespace,
                schema_location: schema_location,
                annotation: None,
                state__: Box::new(ImportElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ImportElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use ImportElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<ImportElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ImportElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ImportElementType> for ImportElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ImportElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ImportElementType> {
            use ImportElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ImportElementType, Error> {
            let state = replace(
                &mut *self.state__,
                ImportElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ImportElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                namespace: self.namespace,
                schema_location: self.schema_location,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct RedefineElementTypeDeserializer {
        any_attribute: AnyAttributes,
        schema_location: String,
        id: Option<String>,
        content: Vec<super::RedefineElementTypeContent>,
        state__: Box<RedefineElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RedefineElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RedefineElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RedefineElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut schema_location: Option<String> = None;
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    helper.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                schema_location: schema_location
                    .ok_or_else(|| ErrorKind::MissingAttribute("schemaLocation".into()))?,
                id: id,
                content: Vec::new(),
                state__: Box::new(RedefineElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RedefineElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RedefineElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RedefineElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RedefineElementTypeContent>,
            fallback: &mut Option<RedefineElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RedefineElementType> for RedefineElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementType> {
            use RedefineElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::RedefineElementTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RedefineElementType, Error> {
            let state = replace(
                &mut *self.state__,
                RedefineElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RedefineElementType {
                any_attribute: self.any_attribute,
                schema_location: self.schema_location,
                id: self.id,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RedefineElementTypeContentDeserializer {
        state__: Box<RedefineElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RedefineElementTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RedefineElementTypeContent),
        Unknown__,
    }
    impl RedefineElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexType")
                ) {
                    let output = <super::ComplexBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_complex_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
            }
            *self.state__ = RedefineElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RedefineElementTypeContentDeserializerState,
        ) -> Result<super::RedefineElementTypeContent, Error> {
            use RedefineElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::RedefineElementTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_complex_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ComplexBaseType>,
            fallback: Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ComplexType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ComplexType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RedefineElementTypeContent>
        for RedefineElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(RedefineElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        RedefineElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineElementTypeContent> {
            use RedefineElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ComplexType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ComplexType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"complexType",
                            true,
                        )?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RedefineElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct OverrideElementTypeDeserializer {
        any_attribute: AnyAttributes,
        schema_location: String,
        id: Option<String>,
        content: Vec<super::OverrideElementTypeContent>,
        state__: Box<OverrideElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OverrideElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::OverrideElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl OverrideElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut schema_location: Option<String> = None;
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"schemaLocation")
                ) {
                    helper.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                schema_location: schema_location
                    .ok_or_else(|| ErrorKind::MissingAttribute("schemaLocation".into()))?,
                id: id,
                content: Vec::new(),
                state__: Box::new(OverrideElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OverrideElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let OverrideElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::OverrideElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::OverrideElementTypeContent>,
            fallback: &mut Option<OverrideElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OverrideElementType> for OverrideElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementType> {
            use OverrideElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::OverrideElementTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::OverrideElementType, Error> {
            let state = replace(
                &mut *self.state__,
                OverrideElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::OverrideElementType {
                any_attribute: self.any_attribute,
                schema_location: self.schema_location,
                id: self.id,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct OverrideElementTypeContentDeserializer {
        state__: Box<OverrideElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum OverrideElementTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        Element(
            Option<super::ElementType>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        Notation(
            Option<super::NotationElementType>,
            Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::OverrideElementTypeContent),
        Unknown__,
    }
    impl OverrideElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexType")
                ) {
                    let output = <super::ComplexBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_complex_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"element")
                ) {
                    let output = <super::ElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_element(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attribute")
                ) {
                    let output = <super::AttributeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"notation")
                ) {
                    let output =
                        <super::NotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_notation(helper, Default::default(), None, output);
                }
            }
            *self.state__ = OverrideElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: OverrideElementTypeContentDeserializerState,
        ) -> Result<super::OverrideElementTypeContent, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::Element(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_element(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Element(
                        helper.finish_element("element", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::Notation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_notation(&mut values, value)?;
                    }
                    Ok(super::OverrideElementTypeContent::Notation(
                        helper.finish_element("notation", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_complex_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ComplexBaseType>,
            fallback: Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ComplexType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ComplexType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_element<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ElementType>,
            fallback: Option<<super::ElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_element(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Element(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Element(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeType>,
            fallback: Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Attribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Attribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_notation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::NotationElementType>,
            fallback: Option<<super::NotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::NotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_notation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_notation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Notation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Notation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OverrideElementTypeContent>
        for OverrideElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(OverrideElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        OverrideElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideElementTypeContent> {
            use OverrideElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ComplexType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Element(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_element(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Attribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Notation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_notation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ComplexType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"complexType",
                            true,
                        )?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Element(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"element",
                            true,
                        )?;
                        match self.handle_element(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            true,
                        )?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Notation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"notation",
                            true,
                        )?;
                        match self.handle_notation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::OverrideElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AnnotationElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        content: Vec<super::AnnotationElementTypeContent>,
        state__: Box<AnnotationElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnnotationElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::AnnotationElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AnnotationElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                content: Vec::new(),
                state__: Box::new(AnnotationElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnnotationElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let AnnotationElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
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
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementTypeContent>,
            fallback: &mut Option<AnnotationElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnnotationElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AnnotationElementType> for AnnotationElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementType> {
            use AnnotationElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::AnnotationElementTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AnnotationElementType, Error> {
            let state = replace(
                &mut *self.state__,
                AnnotationElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AnnotationElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnnotationElementTypeContentDeserializer {
        state__: Box<AnnotationElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AnnotationElementTypeContentDeserializerState {
        Init__,
        Appinfo(
            Option<super::AppinfoElementType>,
            Option<<super::AppinfoElementType as WithDeserializer>::Deserializer>,
            Option<<super::AppinfoElementType as WithDeserializer>::Deserializer>,
        ),
        Documentation(
            Option<super::DocumentationElementType>,
            Option<<super::DocumentationElementType as WithDeserializer>::Deserializer>,
            Option<<super::DocumentationElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AnnotationElementTypeContent),
        Unknown__,
    }
    impl AnnotationElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"appinfo")
                ) {
                    let output =
                        <super::AppinfoElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_appinfo(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"documentation")
                ) {
                    let output =
                        <super::DocumentationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_documentation(helper, Default::default(), None, output);
                }
            }
            *self.state__ = AnnotationElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AnnotationElementTypeContentDeserializerState,
        ) -> Result<super::AnnotationElementTypeContent, Error> {
            use AnnotationElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Appinfo(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_appinfo(&mut values, value)?;
                    }
                    Ok(super::AnnotationElementTypeContent::Appinfo(
                        helper.finish_element("appinfo", values)?,
                    ))
                }
                S::Documentation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_documentation(&mut values, value)?;
                    }
                    Ok(super::AnnotationElementTypeContent::Documentation(
                        helper.finish_element("documentation", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_appinfo<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AppinfoElementType>,
            fallback: Option<<super::AppinfoElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AppinfoElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnnotationElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_appinfo(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_appinfo(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Appinfo(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Appinfo(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_documentation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DocumentationElementType>,
            fallback: Option<<super::DocumentationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DocumentationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnnotationElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_documentation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_documentation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Documentation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Documentation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AnnotationElementTypeContent>
        for AnnotationElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(AnnotationElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        AnnotationElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationElementTypeContent> {
            use AnnotationElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Appinfo(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_appinfo(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Documentation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_documentation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Appinfo(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"appinfo",
                            true,
                        )?;
                        match self.handle_appinfo(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Documentation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"documentation",
                            true,
                        )?;
                        match self.handle_documentation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AnnotationElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct DefaultOpenContentElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        applies_to_empty: bool,
        mode: super::DefaultOpenContentModeType,
        annotation: Option<super::AnnotationElementType>,
        any: Option<super::WildcardType>,
        state__: Box<DefaultOpenContentElementTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut applies_to_empty: Option<bool> = None;
            let mut mode: Option<super::DefaultOpenContentModeType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"appliesToEmpty")
                ) {
                    helper.read_attrib(&mut applies_to_empty, b"appliesToEmpty", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mode")
                ) {
                    helper.read_attrib(&mut mode, b"mode", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                applies_to_empty: applies_to_empty
                    .unwrap_or_else(super::DefaultOpenContentElementType::default_applies_to_empty),
                mode: mode.unwrap_or_else(super::DefaultOpenContentElementType::default_mode),
                annotation: None,
                any: None,
                state__: Box::new(DefaultOpenContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DefaultOpenContentElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use DefaultOpenContentElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<DefaultOpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DefaultOpenContentElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Any(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::WildcardType>,
            fallback: &mut Option<DefaultOpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DefaultOpenContentElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Any(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DefaultOpenContentElementType>
        for DefaultOpenContentElementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DefaultOpenContentElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DefaultOpenContentElementType> {
            use DefaultOpenContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"any",
                            true,
                        )?;
                        match self.handle_any(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DefaultOpenContentElementType, Error> {
            let state = replace(
                &mut *self.state__,
                DefaultOpenContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DefaultOpenContentElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                applies_to_empty: self.applies_to_empty,
                mode: self.mode,
                annotation: self.annotation,
                any: helper.finish_element("any", self.any)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleBaseTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        final_: Option<super::SimpleDerivationSetType>,
        name: Option<String>,
        content: Vec<super::SimpleBaseTypeContent>,
        state__: Box<SimpleBaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleBaseTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SimpleBaseTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SimpleBaseTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut final_: Option<super::SimpleDerivationSetType> = None;
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"final")
                ) {
                    helper.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                final_: final_,
                name: name,
                content: Vec::new(),
                state__: Box::new(SimpleBaseTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SimpleBaseTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SimpleBaseTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SimpleBaseTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SimpleBaseTypeContent>,
            fallback: &mut Option<SimpleBaseTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleBaseTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if self.content.len() < 1usize {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Content__(deserializer);
                        Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                    }
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SimpleBaseType> for SimpleBaseTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseType> {
            use SimpleBaseTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::SimpleBaseTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SimpleBaseType, Error> {
            let state = replace(
                &mut *self.state__,
                SimpleBaseTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SimpleBaseType {
                any_attribute: self.any_attribute,
                id: self.id,
                final_: self.final_,
                name: self.name,
                content: helper.finish_vec(1usize, Some(2usize), self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleBaseTypeContentDeserializer {
        state__: Box<SimpleBaseTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SimpleBaseTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::RestrictionElementType>,
            Option<<super::RestrictionElementType as WithDeserializer>::Deserializer>,
            Option<<super::RestrictionElementType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListElementType>,
            Option<<super::ListElementType as WithDeserializer>::Deserializer>,
            Option<<super::ListElementType as WithDeserializer>::Deserializer>,
        ),
        Union(
            Option<super::UnionElementType>,
            Option<<super::UnionElementType as WithDeserializer>::Deserializer>,
            Option<<super::UnionElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SimpleBaseTypeContent),
        Unknown__,
    }
    impl SimpleBaseTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"restriction")
                ) {
                    let output =
                        <super::RestrictionElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_restriction(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"list")
                ) {
                    let output = <super::ListElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"union")
                ) {
                    let output =
                        <super::UnionElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_union_(helper, Default::default(), None, output);
                }
            }
            *self.state__ = SimpleBaseTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SimpleBaseTypeContentDeserializerState,
        ) -> Result<super::SimpleBaseTypeContent, Error> {
            use SimpleBaseTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Restriction(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_restriction(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Restriction(
                        helper.finish_element("restriction", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Union(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_union_(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Union(
                        helper.finish_element("union", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_restriction<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RestrictionElementType>,
            fallback: Option<<super::RestrictionElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RestrictionElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_restriction(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restriction(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Restriction(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Restriction(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListElementType>,
            fallback: Option<<super::ListElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_union_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnionElementType>,
            fallback: Option<<super::UnionElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnionElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_union_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_union_(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Union(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Union(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SimpleBaseTypeContent> for SimpleBaseTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseTypeContent> {
            let deserializer = Self {
                state__: Box::new(SimpleBaseTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, SimpleBaseTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseTypeContent> {
            use SimpleBaseTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Restriction(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_restriction(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Union(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_union_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Restriction(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"restriction",
                            true,
                        )?;
                        match self.handle_restriction(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"list",
                            true,
                        )?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Union(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"union",
                            true,
                        )?;
                        match self.handle_union_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SimpleBaseTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ComplexBaseTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: Option<String>,
        mixed: Option<bool>,
        abstract_: bool,
        final_: Option<super::DerivationSetType>,
        block: Option<super::DerivationSetType>,
        default_attributes_apply: bool,
        content: Vec<super::ComplexBaseTypeContent>,
        state__: Box<ComplexBaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ComplexBaseTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ComplexBaseTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ComplexBaseTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut mixed: Option<bool> = None;
            let mut abstract_: Option<bool> = None;
            let mut final_: Option<super::DerivationSetType> = None;
            let mut block: Option<super::DerivationSetType> = None;
            let mut default_attributes_apply: Option<bool> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mixed")
                ) {
                    helper.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"abstract")
                ) {
                    helper.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"final")
                ) {
                    helper.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"block")
                ) {
                    helper.read_attrib(&mut block, b"block", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"defaultAttributesApply")
                ) {
                    helper.read_attrib(
                        &mut default_attributes_apply,
                        b"defaultAttributesApply",
                        &attrib.value,
                    )?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                name: name,
                mixed: mixed,
                abstract_: abstract_.unwrap_or_else(super::ComplexBaseType::default_abstract_),
                final_: final_,
                block: block,
                default_attributes_apply: default_attributes_apply
                    .unwrap_or_else(super::ComplexBaseType::default_default_attributes_apply),
                content: Vec::new(),
                state__: Box::new(ComplexBaseTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ComplexBaseTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ComplexBaseTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ComplexBaseTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ComplexBaseTypeContent>,
            fallback: &mut Option<ComplexBaseTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ComplexBaseType> for ComplexBaseTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseType> {
            use ComplexBaseTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::ComplexBaseTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ComplexBaseType, Error> {
            let state = replace(
                &mut *self.state__,
                ComplexBaseTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ComplexBaseType {
                any_attribute: self.any_attribute,
                id: self.id,
                name: self.name,
                mixed: self.mixed,
                abstract_: self.abstract_,
                final_: self.final_,
                block: self.block,
                default_attributes_apply: self.default_attributes_apply,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ComplexBaseTypeContentDeserializer {
        state__: Box<ComplexBaseTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ComplexBaseTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleContent(
            Option<super::SimpleContentElementType>,
            Option<<super::SimpleContentElementType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleContentElementType as WithDeserializer>::Deserializer>,
        ),
        ComplexContent(
            Option<super::ComplexContentElementType>,
            Option<<super::ComplexContentElementType as WithDeserializer>::Deserializer>,
            Option<<super::ComplexContentElementType as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContentElementType>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Assert(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ComplexBaseTypeContent),
        Unknown__,
    }
    impl ComplexBaseTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleContent")
                ) {
                    let output =
                        <super::SimpleContentElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexContent")
                ) {
                    let output = <super::ComplexContentElementType as WithDeserializer>::init(
                        helper, event,
                    )?;
                    return self.handle_complex_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"openContent")
                ) {
                    let output =
                        <super::OpenContentElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_open_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"all")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_all(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"choice")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_choice(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"sequence")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sequence(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attribute")
                ) {
                    let output = <super::AttributeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"anyAttribute")
                ) {
                    let output =
                        <super::AnyAttributeElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_any_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"assert")
                ) {
                    let output = <super::AssertionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_assert(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ComplexBaseTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ComplexBaseTypeContentDeserializerState,
        ) -> Result<super::ComplexBaseTypeContent, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::SimpleContent(
                        helper.finish_element("simpleContent", values)?,
                    ))
                }
                S::ComplexContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_complex_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::ComplexContent(
                        helper.finish_element("complexContent", values)?,
                    ))
                }
                S::OpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_open_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::OpenContent(
                        helper.finish_element("openContent", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Assert(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_assert(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Assert(
                        helper.finish_element("assert", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleContentElementType>,
            fallback: Option<<super::SimpleContentElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleContentElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_content(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleContent(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleContent(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_complex_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ComplexContentElementType>,
            fallback: Option<<super::ComplexContentElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexContentElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_complex_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_content(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ComplexContent(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ComplexContent(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_open_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpenContentElementType>,
            fallback: Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpenContentElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_open_content(&mut values, data)?;
                    let data = Self::finish_state(helper, S::OpenContent(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::OpenContent(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_all<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::finish_state(helper, S::All(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::All(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_choice<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Choice(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Choice(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sequence<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sequence(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sequence(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeType>,
            fallback: Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Attribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Attribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyAttributeElementType>,
            fallback: Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AnyAttribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AnyAttribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_assert<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AssertionType>,
            fallback: Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AssertionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexBaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_assert(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assert(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Assert(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Assert(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ComplexBaseTypeContent> for ComplexBaseTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseTypeContent> {
            let deserializer = Self {
                state__: Box::new(ComplexBaseTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ComplexBaseTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseTypeContent> {
            use ComplexBaseTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleContent(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ComplexContent(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_complex_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OpenContent(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::All(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Choice(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sequence(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Attribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AnyAttribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Assert(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_assert(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleContent(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleContent",
                            true,
                        )?;
                        match self.handle_simple_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ComplexContent(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"complexContent",
                            true,
                        )?;
                        match self.handle_complex_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::OpenContent(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"openContent",
                            true,
                        )?;
                        match self.handle_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::All(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"all",
                            true,
                        )?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Choice(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"choice",
                            true,
                        )?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sequence(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"sequence",
                            true,
                        )?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            true,
                        )?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AnyAttribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"anyAttribute",
                            true,
                        )?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Assert(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"assert",
                            true,
                        )?;
                        match self.handle_assert(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ComplexBaseTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct GroupTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        min_occurs: usize,
        max_occurs: super::AllNniType,
        content: Vec<super::GroupTypeContent>,
        state__: Box<GroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GroupTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::GroupTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl GroupTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<super::AllNniType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"minOccurs")
                ) {
                    helper.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"maxOccurs")
                ) {
                    helper.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                name: name,
                ref_: ref_,
                min_occurs: min_occurs.unwrap_or_else(super::GroupType::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::GroupType::default_max_occurs),
                content: Vec::new(),
                state__: Box::new(GroupTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: GroupTypeDeserializerState,
        ) -> Result<(), Error> {
            if let GroupTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::GroupTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::GroupTypeContent>,
            fallback: &mut Option<GroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GroupType> for GroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupType> {
            use GroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::GroupTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::GroupType, Error> {
            let state = replace(&mut *self.state__, GroupTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::GroupType {
                any_attribute: self.any_attribute,
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                min_occurs: self.min_occurs,
                max_occurs: self.max_occurs,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct GroupTypeContentDeserializer {
        state__: Box<GroupTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum GroupTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Element(
            Option<super::ElementType>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
            Option<<super::ElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Any(
            Option<super::AnyElementType>,
            Option<<super::AnyElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnyElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::GroupTypeContent),
        Unknown__,
    }
    impl GroupTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"element")
                ) {
                    let output = <super::ElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_element(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"all")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_all(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"choice")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_choice(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"sequence")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sequence(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"any")
                ) {
                    let output = <super::AnyElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_any(helper, Default::default(), None, output);
                }
            }
            *self.state__ = GroupTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: GroupTypeContentDeserializerState,
        ) -> Result<super::GroupTypeContent, Error> {
            use GroupTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Element(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_element(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Element(
                        helper.finish_element("element", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::Any(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Any(
                        helper.finish_element("any", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_element<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ElementType>,
            fallback: Option<<super::ElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_element(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Element(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Element(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_all<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::finish_state(helper, S::All(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::All(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_choice<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Choice(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Choice(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sequence<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sequence(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sequence(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyElementType>,
            fallback: Option<<super::AnyElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Any(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Any(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GroupTypeContent> for GroupTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupTypeContent> {
            let deserializer = Self {
                state__: Box::new(GroupTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, GroupTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupTypeContent> {
            use GroupTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Element(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_element(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::All(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Choice(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sequence(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Any(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Element(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"element",
                            true,
                        )?;
                        match self.handle_element(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::All(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"all",
                            true,
                        )?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Choice(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"choice",
                            true,
                        )?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sequence(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"sequence",
                            true,
                        )?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Any(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"any",
                            true,
                        )?;
                        match self.handle_any(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::GroupTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AttributeGroupTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        content: Vec<super::AttributeGroupTypeContent>,
        state__: Box<AttributeGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AttributeGroupTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::AttributeGroupTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AttributeGroupTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                name: name,
                ref_: ref_,
                content: Vec::new(),
                state__: Box::new(AttributeGroupTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AttributeGroupTypeDeserializerState,
        ) -> Result<(), Error> {
            if let AttributeGroupTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::AttributeGroupTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AttributeGroupTypeContent>,
            fallback: &mut Option<AttributeGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AttributeGroupType> for AttributeGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupType> {
            use AttributeGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::AttributeGroupTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AttributeGroupType, Error> {
            let state = replace(
                &mut *self.state__,
                AttributeGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AttributeGroupType {
                any_attribute: self.any_attribute,
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AttributeGroupTypeContentDeserializer {
        state__: Box<AttributeGroupTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AttributeGroupTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AttributeGroupTypeContent),
        Unknown__,
    }
    impl AttributeGroupTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attribute")
                ) {
                    let output = <super::AttributeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"anyAttribute")
                ) {
                    let output =
                        <super::AnyAttributeElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_any_attribute(helper, Default::default(), None, output);
                }
            }
            *self.state__ = AttributeGroupTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AttributeGroupTypeContentDeserializerState,
        ) -> Result<super::AttributeGroupTypeContent, Error> {
            use AttributeGroupTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeGroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeType>,
            fallback: Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeGroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Attribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Attribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeGroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyAttributeElementType>,
            fallback: Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeGroupTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AnyAttribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AnyAttribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AttributeGroupTypeContent>
        for AttributeGroupTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupTypeContent> {
            let deserializer = Self {
                state__: Box::new(AttributeGroupTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        AttributeGroupTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupTypeContent> {
            use AttributeGroupTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Attribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AnyAttribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            true,
                        )?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AnyAttribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"anyAttribute",
                            true,
                        )?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AttributeGroupTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        type_: Option<String>,
        substitution_group: Option<super::EntitiesType>,
        min_occurs: usize,
        max_occurs: super::AllNniType,
        default: Option<String>,
        fixed: Option<String>,
        nillable: Option<bool>,
        abstract_: bool,
        final_: Option<super::DerivationSetType>,
        block: Option<super::BlockSetType>,
        form: Option<super::FormChoiceType>,
        target_namespace: Option<String>,
        content: Vec<super::ElementTypeContent>,
        state__: Box<ElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut substitution_group: Option<super::EntitiesType> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<super::AllNniType> = None;
            let mut default: Option<String> = None;
            let mut fixed: Option<String> = None;
            let mut nillable: Option<bool> = None;
            let mut abstract_: Option<bool> = None;
            let mut final_: Option<super::DerivationSetType> = None;
            let mut block: Option<super::BlockSetType> = None;
            let mut form: Option<super::FormChoiceType> = None;
            let mut target_namespace: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"substitutionGroup")
                ) {
                    helper.read_attrib(
                        &mut substitution_group,
                        b"substitutionGroup",
                        &attrib.value,
                    )?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"minOccurs")
                ) {
                    helper.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"maxOccurs")
                ) {
                    helper.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"default")
                ) {
                    helper.read_attrib(&mut default, b"default", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"fixed")
                ) {
                    helper.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"nillable")
                ) {
                    helper.read_attrib(&mut nillable, b"nillable", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"abstract")
                ) {
                    helper.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"final")
                ) {
                    helper.read_attrib(&mut final_, b"final", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"block")
                ) {
                    helper.read_attrib(&mut block, b"block", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"form")
                ) {
                    helper.read_attrib(&mut form, b"form", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"targetNamespace")
                ) {
                    helper.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
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
                state__: Box::new(ElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ElementTypeContent>,
            fallback: &mut Option<ElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ElementType> for ElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementType> {
            use ElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ElementTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ElementType, Error> {
            let state = replace(&mut *self.state__, ElementTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ElementType {
                any_attribute: self.any_attribute,
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
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ElementTypeContentDeserializer {
        state__: Box<ElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ElementTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Alternative(
            Option<super::AltType>,
            Option<<super::AltType as WithDeserializer>::Deserializer>,
            Option<<super::AltType as WithDeserializer>::Deserializer>,
        ),
        Unique(
            Option<super::KeybaseType>,
            Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
            Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
        ),
        Key(
            Option<super::KeybaseType>,
            Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
            Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
        ),
        Keyref(
            Option<super::KeyrefElementType>,
            Option<<super::KeyrefElementType as WithDeserializer>::Deserializer>,
            Option<<super::KeyrefElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ElementTypeContent),
        Unknown__,
    }
    impl ElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexType")
                ) {
                    let output = <super::ComplexBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_complex_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"alternative")
                ) {
                    let output = <super::AltType as WithDeserializer>::init(helper, event)?;
                    return self.handle_alternative(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"unique")
                ) {
                    let output = <super::KeybaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unique(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"key")
                ) {
                    let output = <super::KeybaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_key(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"keyref")
                ) {
                    let output =
                        <super::KeyrefElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_keyref(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ElementTypeContentDeserializerState,
        ) -> Result<super::ElementTypeContent, Error> {
            use ElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Alternative(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_alternative(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Alternative(
                        helper.finish_element("alternative", values)?,
                    ))
                }
                S::Unique(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unique(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Unique(
                        helper.finish_element("unique", values)?,
                    ))
                }
                S::Key(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_key(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Key(
                        helper.finish_element("key", values)?,
                    ))
                }
                S::Keyref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_keyref(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Keyref(
                        helper.finish_element("keyref", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_complex_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ComplexBaseType>,
            fallback: Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ComplexType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ComplexType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_alternative<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AltType>,
            fallback: Option<<super::AltType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AltType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_alternative(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_alternative(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Alternative(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Alternative(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unique<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::KeybaseType>,
            fallback: Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::KeybaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unique(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unique(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unique(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unique(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_key<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::KeybaseType>,
            fallback: Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::KeybaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_key(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Key(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Key(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_keyref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::KeyrefElementType>,
            fallback: Option<<super::KeyrefElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::KeyrefElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_keyref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_keyref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Keyref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Keyref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ElementTypeContent> for ElementTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(ElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ElementTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementTypeContent> {
            use ElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ComplexType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Alternative(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_alternative(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unique(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unique(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Key(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_key(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Keyref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_keyref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ComplexType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"complexType",
                            true,
                        )?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Alternative(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"alternative",
                            true,
                        )?;
                        match self.handle_alternative(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unique(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"unique",
                            true,
                        )?;
                        match self.handle_unique(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Key(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"key",
                            true,
                        )?;
                        match self.handle_key(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Keyref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"keyref",
                            true,
                        )?;
                        match self.handle_keyref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AttributeTypeDeserializer {
        any_attribute: AnyAttributes,
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
        state__: Box<AttributeTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
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
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"use")
                ) {
                    helper.read_attrib(&mut use_, b"use", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"default")
                ) {
                    helper.read_attrib(&mut default, b"default", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"fixed")
                ) {
                    helper.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"form")
                ) {
                    helper.read_attrib(&mut form, b"form", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"targetNamespace")
                ) {
                    helper.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"inheritable")
                ) {
                    helper.read_attrib(&mut inheritable, b"inheritable", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
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
                state__: Box::new(AttributeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AttributeTypeDeserializerState,
        ) -> Result<(), Error> {
            use AttributeTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::SimpleType(Some(deserializer)) => {
                    self.store_simple_type(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AttributeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::SimpleType(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<AttributeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AttributeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SimpleType(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_simple_type(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SimpleType(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AttributeType> for AttributeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeType> {
            use AttributeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AttributeType, Error> {
            let state = replace(
                &mut *self.state__,
                AttributeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AttributeType {
                any_attribute: self.any_attribute,
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
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: String,
        public: Option<String>,
        system: Option<String>,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<NotationElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NotationElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NotationElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut public: Option<String> = None;
            let mut system: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"public")
                ) {
                    helper.read_attrib(&mut public, b"public", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"system")
                ) {
                    helper.read_attrib(&mut system, b"system", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                public: public,
                system: system,
                annotation: None,
                state__: Box::new(NotationElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NotationElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use NotationElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<NotationElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NotationElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::NotationElementType> for NotationElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NotationElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NotationElementType> {
            use NotationElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::NotationElementType, Error> {
            let state = replace(
                &mut *self.state__,
                NotationElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NotationElementType {
                any_attribute: self.any_attribute,
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
        any_attribute: AnyAttributes,
        text_before: Option<Text>,
        content: Vec<super::AppinfoElementTypeContent>,
        state__: Box<AppinfoElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AppinfoElementTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        Content(Option<<super::AppinfoElementTypeContent as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AppinfoElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut source: Option<String> = None;
            let mut any_attribute = AnyAttributes::default();
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"source")
                ) {
                    helper.read_attrib(&mut source, b"source", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                source: source,
                any_attribute: any_attribute,
                text_before: None,
                content: Vec::new(),
                state__: Box::new(AppinfoElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AppinfoElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use AppinfoElementTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_content(&mut self, value: super::AppinfoElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<AppinfoElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AppinfoElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextBefore(None));
                *self.state__ = S::Content(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AppinfoElementTypeContent>,
            fallback: &mut Option<AppinfoElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AppinfoElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content(Some(deserializer)));
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AppinfoElementType> for AppinfoElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AppinfoElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AppinfoElementType> {
            use AppinfoElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::AppinfoElementTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AppinfoElementType, Error> {
            let state = replace(
                &mut *self.state__,
                AppinfoElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AppinfoElementType {
                source: self.source,
                any_attribute: self.any_attribute,
                text_before: self.text_before,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct AppinfoElementTypeContentDeserializer {
        any: Option<Mixed<AnyElement>>,
        state__: Box<AppinfoElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum AppinfoElementTypeContentDeserializerState {
        Init__,
        Any(Option<<Mixed<AnyElement> as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AppinfoElementTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AppinfoElementTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use AppinfoElementTypeContentDeserializerState as S;
            match state {
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_any(&mut self, value: Mixed<AnyElement>) -> Result<(), Error> {
            if self.any.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"any123",
                )))?;
            }
            self.any = Some(value);
            Ok(())
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<AnyElement>>,
            fallback: &mut Option<AppinfoElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AppinfoElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Any(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AppinfoElementTypeContent>
        for AppinfoElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AppinfoElementTypeContent> {
            let deserializer = Self {
                any: None,
                state__: Box::new(AppinfoElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        AppinfoElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AppinfoElementTypeContent> {
            use AppinfoElementTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Any(None);
                        event
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output =
                                <Mixed<AnyElement> as WithDeserializer>::init(helper, event)?;
                            match self.handle_any(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any(None));
                            *self.state__ = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        if let Some(state) = any_fallback.take() {
                            is_any_retry = true;
                            *self.state__ = state;
                            event
                        } else {
                            *self.state__ = S::Done__;
                            break (DeserializerEvent::Continue(event), allow_any_element);
                        }
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AppinfoElementTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                AppinfoElementTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AppinfoElementTypeContent {
                any: helper.finish_element("any123", self.any)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentationElementTypeDeserializer {
        source: Option<String>,
        lang: Option<String>,
        any_attribute: AnyAttributes,
        text_before: Option<Text>,
        content: Vec<super::DocumentationElementTypeContent>,
        state__: Box<DocumentationElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentationElementTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        Content(Option<<super::DocumentationElementTypeContent as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentationElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut source: Option<String> = None;
            let mut lang: Option<String> = None;
            let mut any_attribute = AnyAttributes::default();
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"source")
                ) {
                    helper.read_attrib(&mut source, b"source", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XML),
                    Some(b"lang")
                ) {
                    helper.read_attrib(&mut lang, b"lang", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                source: source,
                lang: lang,
                any_attribute: any_attribute,
                text_before: None,
                content: Vec::new(),
                state__: Box::new(DocumentationElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DocumentationElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use DocumentationElementTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::DocumentationElementTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<DocumentationElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocumentationElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextBefore(None));
                *self.state__ = S::Content(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DocumentationElementTypeContent>,
            fallback: &mut Option<DocumentationElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocumentationElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content(Some(deserializer)));
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocumentationElementType>
        for DocumentationElementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentationElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentationElementType> {
            use DocumentationElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::DocumentationElementTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DocumentationElementType, Error> {
            let state = replace(
                &mut *self.state__,
                DocumentationElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DocumentationElementType {
                source: self.source,
                lang: self.lang,
                any_attribute: self.any_attribute,
                text_before: self.text_before,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentationElementTypeContentDeserializer {
        any: Option<Mixed<AnyElement>>,
        state__: Box<DocumentationElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentationElementTypeContentDeserializerState {
        Init__,
        Any(Option<<Mixed<AnyElement> as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentationElementTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DocumentationElementTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use DocumentationElementTypeContentDeserializerState as S;
            match state {
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_any(&mut self, value: Mixed<AnyElement>) -> Result<(), Error> {
            if self.any.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"any125",
                )))?;
            }
            self.any = Some(value);
            Ok(())
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<AnyElement>>,
            fallback: &mut Option<DocumentationElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocumentationElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Any(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocumentationElementTypeContent>
        for DocumentationElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentationElementTypeContent> {
            let deserializer = Self {
                any: None,
                state__: Box::new(DocumentationElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        DocumentationElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentationElementTypeContent> {
            use DocumentationElementTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Any(None);
                        event
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output =
                                <Mixed<AnyElement> as WithDeserializer>::init(helper, event)?;
                            match self.handle_any(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any(None));
                            *self.state__ = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        if let Some(state) = any_fallback.take() {
                            is_any_retry = true;
                            *self.state__ = state;
                            event
                        } else {
                            *self.state__ = S::Done__;
                            break (DeserializerEvent::Continue(event), allow_any_element);
                        }
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DocumentationElementTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                DocumentationElementTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DocumentationElementTypeContent {
                any: helper.finish_element("any125", self.any)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct WildcardTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<WildcardTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WildcardTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WildcardTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::NotNamespaceType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    helper.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notNamespace")
                ) {
                    helper.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"processContents")
                ) {
                    helper.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::WildcardType::default_process_contents),
                annotation: None,
                state__: Box::new(WildcardTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: WildcardTypeDeserializerState,
        ) -> Result<(), Error> {
            use WildcardTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<WildcardTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WildcardTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WildcardType> for WildcardTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WildcardType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WildcardType> {
            use WildcardTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::WildcardType, Error> {
            let state = replace(&mut *self.state__, WildcardTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::WildcardType {
                any_attribute: self.any_attribute,
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
        any_attribute: AnyAttributes,
        id: Option<String>,
        base: Option<String>,
        content: Vec<super::RestrictionElementTypeContent>,
        state__: Box<RestrictionElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictionElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RestrictionElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RestrictionElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"base")
                ) {
                    helper.read_attrib(&mut base, b"base", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                base: base,
                content: Vec::new(),
                state__: Box::new(RestrictionElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RestrictionElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RestrictionElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
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
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RestrictionElementTypeContent>,
            fallback: &mut Option<RestrictionElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionElementType> for RestrictionElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementType> {
            use RestrictionElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::RestrictionElementTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RestrictionElementType, Error> {
            let state = replace(
                &mut *self.state__,
                RestrictionElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RestrictionElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                base: self.base,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RestrictionElementTypeContentDeserializer {
        state__: Box<RestrictionElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RestrictionElementTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        Facet(
            Option<super::Facet>,
            Option<<super::Facet as WithDeserializer>::Deserializer>,
            Option<<super::Facet as WithDeserializer>::Deserializer>,
        ),
        Any(
            Option<AnyElement>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RestrictionElementTypeContent),
        Unknown__,
    }
    impl RestrictionElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::Facet as WithDeserializer>::init(helper, event)?;
                    match self.handle_facet(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
                event = {
                    let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                    match self.handle_any(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, .. } => event,
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = RestrictionElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RestrictionElementTypeContentDeserializerState,
        ) -> Result<super::RestrictionElementTypeContent, Error> {
            use RestrictionElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::Facet(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_facet(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::Facet(
                        helper.finish_element("facet", values)?,
                    ))
                }
                S::Any(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any(&mut values, value)?;
                    }
                    Ok(super::RestrictionElementTypeContent::Any(
                        helper.finish_element("any135", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn store_any(values: &mut Option<AnyElement>, value: AnyElement) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"any135",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_facet<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::Facet>,
            fallback: Option<<super::Facet as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Facet>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_facet(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_facet(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Facet(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Facet(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<AnyElement>,
            fallback: Option<<AnyElement as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, AnyElement>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Any(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Any(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionElementTypeContent>
        for RestrictionElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(RestrictionElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        RestrictionElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionElementTypeContent> {
            use RestrictionElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Facet(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_facet(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Any(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Facet(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::Facet as WithDeserializer>::init(helper, event)?;
                        match self.handle_facet(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Any(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                        match self.handle_any(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RestrictionElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ListElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        item_type: Option<String>,
        annotation: Option<super::AnnotationElementType>,
        simple_type: Option<super::SimpleBaseType>,
        state__: Box<ListElementTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut item_type: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"itemType")
                ) {
                    helper.read_attrib(&mut item_type, b"itemType", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                item_type: item_type,
                annotation: None,
                simple_type: None,
                state__: Box::new(ListElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ListElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use ListElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::SimpleType(Some(deserializer)) => {
                    self.store_simple_type(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<ListElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::SimpleType(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<ListElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SimpleType(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_simple_type(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SimpleType(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ListElementType> for ListElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListElementType> {
            use ListElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ListElementType, Error> {
            let state = replace(
                &mut *self.state__,
                ListElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ListElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                item_type: self.item_type,
                annotation: self.annotation,
                simple_type: self.simple_type.map(Box::new),
            })
        }
    }
    #[derive(Debug)]
    pub struct UnionElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        member_types: Option<super::EntitiesType>,
        annotation: Option<super::AnnotationElementType>,
        simple_type: Vec<super::SimpleBaseType>,
        state__: Box<UnionElementTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut member_types: Option<super::EntitiesType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"memberTypes")
                ) {
                    helper.read_attrib(&mut member_types, b"memberTypes", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                member_types: member_types,
                annotation: None,
                simple_type: Vec::new(),
                state__: Box::new(UnionElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: UnionElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use UnionElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::SimpleType(Some(deserializer)) => {
                    self.store_simple_type(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<UnionElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnionElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::SimpleType(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
            fallback: &mut Option<UnionElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnionElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SimpleType(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_simple_type(data)?;
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SimpleType(Some(deserializer)));
                    *self.state__ = S::SimpleType(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::UnionElementType> for UnionElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnionElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnionElementType> {
            use UnionElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::UnionElementType, Error> {
            let state = replace(
                &mut *self.state__,
                UnionElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::UnionElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                member_types: self.member_types,
                annotation: self.annotation,
                simple_type: self.simple_type,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleContentElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        content: Vec<super::SimpleContentElementTypeContent>,
        state__: Box<SimpleContentElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleContentElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SimpleContentElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SimpleContentElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                content: Vec::new(),
                state__: Box::new(SimpleContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SimpleContentElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SimpleContentElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
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
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SimpleContentElementTypeContent>,
            fallback: &mut Option<SimpleContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleContentElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if self.content.len() < 1usize {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Content__(deserializer);
                        Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                    }
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SimpleContentElementType>
        for SimpleContentElementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementType> {
            use SimpleContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::SimpleContentElementTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SimpleContentElementType, Error> {
            let state = replace(
                &mut *self.state__,
                SimpleContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SimpleContentElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                content: helper.finish_vec(1usize, Some(2usize), self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleContentElementTypeContentDeserializer {
        state__: Box<SimpleContentElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SimpleContentElementTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::RestrictionType>,
            Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
            Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
        ),
        Extension(
            Option<super::ExtensionType>,
            Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
            Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SimpleContentElementTypeContent),
        Unknown__,
    }
    impl SimpleContentElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"restriction")
                ) {
                    let output = <super::RestrictionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_restriction(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"extension")
                ) {
                    let output = <super::ExtensionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_extension(helper, Default::default(), None, output);
                }
            }
            *self.state__ = SimpleContentElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SimpleContentElementTypeContentDeserializerState,
        ) -> Result<super::SimpleContentElementTypeContent, Error> {
            use SimpleContentElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SimpleContentElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Restriction(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_restriction(&mut values, value)?;
                    }
                    Ok(super::SimpleContentElementTypeContent::Restriction(
                        helper.finish_element("restriction", values)?,
                    ))
                }
                S::Extension(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_extension(&mut values, value)?;
                    }
                    Ok(super::SimpleContentElementTypeContent::Extension(
                        helper.finish_element("extension", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleContentElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_restriction<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RestrictionType>,
            fallback: Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RestrictionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleContentElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_restriction(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restriction(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Restriction(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Restriction(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_extension<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ExtensionType>,
            fallback: Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ExtensionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleContentElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_extension(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_extension(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Extension(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Extension(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SimpleContentElementTypeContent>
        for SimpleContentElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(SimpleContentElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        SimpleContentElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentElementTypeContent> {
            use SimpleContentElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Restriction(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_restriction(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Extension(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_extension(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Restriction(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"restriction",
                            true,
                        )?;
                        match self.handle_restriction(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Extension(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"extension",
                            true,
                        )?;
                        match self.handle_extension(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SimpleContentElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ComplexContentElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        mixed: Option<bool>,
        content: Vec<super::ComplexContentElementTypeContent>,
        state__: Box<ComplexContentElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ComplexContentElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ComplexContentElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ComplexContentElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut mixed: Option<bool> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mixed")
                ) {
                    helper.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                mixed: mixed,
                content: Vec::new(),
                state__: Box::new(ComplexContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ComplexContentElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ComplexContentElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
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
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ComplexContentElementTypeContent>,
            fallback: &mut Option<ComplexContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexContentElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if self.content.len() < 1usize {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Content__(deserializer);
                        Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                    }
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ComplexContentElementType>
        for ComplexContentElementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementType> {
            use ComplexContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ComplexContentElementTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ComplexContentElementType, Error> {
            let state = replace(
                &mut *self.state__,
                ComplexContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ComplexContentElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                mixed: self.mixed,
                content: helper.finish_vec(1usize, Some(2usize), self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ComplexContentElementTypeContentDeserializer {
        state__: Box<ComplexContentElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ComplexContentElementTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::RestrictionType>,
            Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
            Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
        ),
        Extension(
            Option<super::ExtensionType>,
            Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
            Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ComplexContentElementTypeContent),
        Unknown__,
    }
    impl ComplexContentElementTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"restriction")
                ) {
                    let output = <super::RestrictionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_restriction(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"extension")
                ) {
                    let output = <super::ExtensionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_extension(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ComplexContentElementTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ComplexContentElementTypeContentDeserializerState,
        ) -> Result<super::ComplexContentElementTypeContent, Error> {
            use ComplexContentElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ComplexContentElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Restriction(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_restriction(&mut values, value)?;
                    }
                    Ok(super::ComplexContentElementTypeContent::Restriction(
                        helper.finish_element("restriction", values)?,
                    ))
                }
                S::Extension(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_extension(&mut values, value)?;
                    }
                    Ok(super::ComplexContentElementTypeContent::Extension(
                        helper.finish_element("extension", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexContentElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_restriction<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RestrictionType>,
            fallback: Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RestrictionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexContentElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_restriction(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restriction(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Restriction(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Restriction(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_extension<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ExtensionType>,
            fallback: Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ExtensionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexContentElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_extension(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_extension(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Extension(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Extension(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ComplexContentElementTypeContent>
        for ComplexContentElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementTypeContent> {
            let deserializer = Self {
                state__: Box::new(ComplexContentElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        ComplexContentElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentElementTypeContent> {
            use ComplexContentElementTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Restriction(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_restriction(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Extension(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_extension(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Restriction(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"restriction",
                            true,
                        )?;
                        match self.handle_restriction(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Extension(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"extension",
                            true,
                        )?;
                        match self.handle_extension(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ComplexContentElementTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct OpenContentElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        mode: super::OpenContentModeType,
        annotation: Option<super::AnnotationElementType>,
        any: Option<super::WildcardType>,
        state__: Box<OpenContentElementTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut mode: Option<super::OpenContentModeType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"mode")
                ) {
                    helper.read_attrib(&mut mode, b"mode", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                mode: mode.unwrap_or_else(super::OpenContentElementType::default_mode),
                annotation: None,
                any: None,
                state__: Box::new(OpenContentElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OpenContentElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use OpenContentElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<OpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpenContentElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Any(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::WildcardType>,
            fallback: &mut Option<OpenContentElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpenContentElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Any(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OpenContentElementType> for OpenContentElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpenContentElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpenContentElementType> {
            use OpenContentElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"any",
                            true,
                        )?;
                        match self.handle_any(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::OpenContentElementType, Error> {
            let state = replace(
                &mut *self.state__,
                OpenContentElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::OpenContentElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                mode: self.mode,
                annotation: self.annotation,
                any: self.any,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyAttributeElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        not_q_name: Option<super::QnameListAType>,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<AnyAttributeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyAttributeElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyAttributeElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::NotNamespaceType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            let mut not_q_name: Option<super::QnameListAType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    helper.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notNamespace")
                ) {
                    helper.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"processContents")
                ) {
                    helper.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notQName")
                ) {
                    helper.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::AnyAttributeElementType::default_process_contents),
                not_q_name: not_q_name,
                annotation: None,
                state__: Box::new(AnyAttributeElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnyAttributeElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use AnyAttributeElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AnyAttributeElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnyAttributeElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AnyAttributeElementType>
        for AnyAttributeElementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyAttributeElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyAttributeElementType> {
            use AnyAttributeElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AnyAttributeElementType, Error> {
            let state = replace(
                &mut *self.state__,
                AnyAttributeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AnyAttributeElementType {
                any_attribute: self.any_attribute,
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
        any_attribute: AnyAttributes,
        id: Option<String>,
        test: Option<String>,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<AssertionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AssertionTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AssertionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"test")
                ) {
                    helper.read_attrib(&mut test, b"test", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    helper.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                test: test,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state__: Box::new(AssertionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AssertionTypeDeserializerState,
        ) -> Result<(), Error> {
            use AssertionTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AssertionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AssertionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AssertionType> for AssertionTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AssertionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AssertionType> {
            use AssertionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AssertionType, Error> {
            let state = replace(
                &mut *self.state__,
                AssertionTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AssertionType {
                any_attribute: self.any_attribute,
                id: self.id,
                test: self.test,
                xpath_default_namespace: self.xpath_default_namespace,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        not_q_name: Option<super::QnameListType>,
        min_occurs: usize,
        max_occurs: super::AllNniType,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<AnyElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::NotNamespaceType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            let mut not_q_name: Option<super::QnameListType> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<super::AllNniType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"namespace")
                ) {
                    helper.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notNamespace")
                ) {
                    helper.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"processContents")
                ) {
                    helper.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"notQName")
                ) {
                    helper.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"minOccurs")
                ) {
                    helper.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"maxOccurs")
                ) {
                    helper.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::AnyElementType::default_process_contents),
                not_q_name: not_q_name,
                min_occurs: min_occurs.unwrap_or_else(super::AnyElementType::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::AnyElementType::default_max_occurs),
                annotation: None,
                state__: Box::new(AnyElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnyElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use AnyElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<AnyElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnyElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AnyElementType> for AnyElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyElementType> {
            use AnyElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AnyElementType, Error> {
            let state = replace(
                &mut *self.state__,
                AnyElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AnyElementType {
                any_attribute: self.any_attribute,
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
        any_attribute: AnyAttributes,
        id: Option<String>,
        test: Option<String>,
        type_: Option<String>,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        content: Vec<super::AltTypeContent>,
        state__: Box<AltTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AltTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::AltTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AltTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"test")
                ) {
                    helper.read_attrib(&mut test, b"test", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    helper.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                test: test,
                type_: type_,
                xpath_default_namespace: xpath_default_namespace,
                content: Vec::new(),
                state__: Box::new(AltTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AltTypeDeserializerState,
        ) -> Result<(), Error> {
            if let AltTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::AltTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AltTypeContent>,
            fallback: &mut Option<AltTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AltTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if self.content.len() < 1usize {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Content__(deserializer);
                        Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                    }
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AltType> for AltTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltType> {
            use AltTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::AltTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AltType, Error> {
            let state = replace(&mut *self.state__, AltTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AltType {
                any_attribute: self.any_attribute,
                id: self.id,
                test: self.test,
                type_: self.type_,
                xpath_default_namespace: self.xpath_default_namespace,
                content: helper.finish_vec(0usize, Some(2usize), self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AltTypeContentDeserializer {
        state__: Box<AltTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AltTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        ComplexType(
            Option<super::ComplexBaseType>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AltTypeContent),
        Unknown__,
    }
    impl AltTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexType")
                ) {
                    let output = <super::ComplexBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_complex_type(helper, Default::default(), None, output);
                }
            }
            *self.state__ = AltTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AltTypeContentDeserializerState,
        ) -> Result<super::AltTypeContent, Error> {
            use AltTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AltTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AltTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_complex_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ComplexBaseType>,
            fallback: Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AltTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_complex_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ComplexType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ComplexType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AltTypeContent> for AltTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltTypeContent> {
            let deserializer = Self {
                state__: Box::new(AltTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, AltTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltTypeContent> {
            use AltTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ComplexType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ComplexType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"complexType",
                            true,
                        )?;
                        match self.handle_complex_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AltTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct KeybaseTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        content: Option<super::KeybaseTypeContent>,
        state__: Box<KeybaseTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeybaseTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeybaseTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeybaseTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                name: name,
                ref_: ref_,
                content: None,
                state__: Box::new(KeybaseTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeybaseTypeDeserializerState,
        ) -> Result<(), Error> {
            if let KeybaseTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
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
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::KeybaseTypeContent>,
            fallback: &mut Option<KeybaseTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeybaseTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::KeybaseType> for KeybaseTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseType> {
            use KeybaseTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::KeybaseTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::KeybaseType, Error> {
            let state = replace(&mut *self.state__, KeybaseTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::KeybaseType {
                any_attribute: self.any_attribute,
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
        state__: Box<KeybaseTypeContentDeserializerState>,
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
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeybaseTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use KeybaseTypeContentDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::Selector(Some(deserializer)) => {
                    self.store_selector(deserializer.finish(helper)?)?
                }
                S::Field(Some(deserializer)) => self.store_field(deserializer.finish(helper)?)?,
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<KeybaseTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeybaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Selector(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Selector(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Selector(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_selector<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeybaseTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeybaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Selector(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_selector(data)?;
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Selector(Some(deserializer)));
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeybaseTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeybaseTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.field.len() < 1usize {
                    fallback.get_or_insert(S::Field(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Field(None));
                    *self.state__ = S::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_field(data)?;
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Field(Some(deserializer)));
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::KeybaseTypeContent> for KeybaseTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseTypeContent> {
            let deserializer = Self {
                annotation: None,
                selector: None,
                field: Vec::new(),
                state__: Box::new(KeybaseTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, KeybaseTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseTypeContent> {
            use KeybaseTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_selector(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Selector(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"selector",
                            true,
                        )?;
                        match self.handle_selector(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Field(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"field",
                            true,
                        )?;
                        match self.handle_field(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::KeybaseTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                KeybaseTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::KeybaseTypeContent {
                annotation: self.annotation,
                selector: helper.finish_element("selector", self.selector)?,
                field: helper.finish_vec(1usize, None, self.field)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyrefElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        name: Option<String>,
        ref_: Option<String>,
        refer: Option<String>,
        content: Option<super::KeyrefElementTypeContent>,
        state__: Box<KeyrefElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyrefElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyrefElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyrefElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<String> = None;
            let mut refer: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"ref")
                ) {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"refer")
                ) {
                    helper.read_attrib(&mut refer, b"refer", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                name: name,
                ref_: ref_,
                refer: refer,
                content: None,
                state__: Box::new(KeyrefElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeyrefElementTypeDeserializerState,
        ) -> Result<(), Error> {
            if let KeyrefElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
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
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::KeyrefElementTypeContent>,
            fallback: &mut Option<KeyrefElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::KeyrefElementType> for KeyrefElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementType> {
            use KeyrefElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::KeyrefElementTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::KeyrefElementType, Error> {
            let state = replace(
                &mut *self.state__,
                KeyrefElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::KeyrefElementType {
                any_attribute: self.any_attribute,
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
        state__: Box<KeyrefElementTypeContentDeserializerState>,
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
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeyrefElementTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use KeyrefElementTypeContentDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::Selector(Some(deserializer)) => {
                    self.store_selector(deserializer.finish(helper)?)?
                }
                S::Field(Some(deserializer)) => self.store_field(deserializer.finish(helper)?)?,
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<KeyrefElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Selector(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Selector(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Selector(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_selector<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeyrefElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Selector(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_selector(data)?;
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Selector(Some(deserializer)));
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FieldElementType>,
            fallback: &mut Option<KeyrefElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefElementTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.field.len() < 1usize {
                    fallback.get_or_insert(S::Field(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Field(None));
                    *self.state__ = S::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_field(data)?;
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Field(Some(deserializer)));
                    *self.state__ = S::Field(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::KeyrefElementTypeContent>
        for KeyrefElementTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementTypeContent> {
            let deserializer = Self {
                annotation: None,
                selector: None,
                field: Vec::new(),
                state__: Box::new(KeyrefElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        KeyrefElementTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefElementTypeContent> {
            use KeyrefElementTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_selector(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Selector(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"selector",
                            true,
                        )?;
                        match self.handle_selector(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Field(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"field",
                            true,
                        )?;
                        match self.handle_field(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::KeyrefElementTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                KeyrefElementTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::KeyrefElementTypeContent {
                annotation: self.annotation,
                selector: helper.finish_element("selector", self.selector)?,
                field: helper.finish_vec(1usize, None, self.field)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FacetDeserializer {
        state__: Box<FacetDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FacetDeserializerState {
        Init__,
        MinExclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MinInclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MaxExclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MaxInclusive(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        TotalDigits(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        FractionDigits(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Length(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MinLength(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        MaxLength(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Enumeration(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        WhiteSpace(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Pattern(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Assertion(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        ExplicitTimezone(
            Option<super::FacetType>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
            Option<<super::FacetType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::Facet),
        Unknown__,
    }
    impl FacetDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"minExclusive")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_min_exclusive(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"minInclusive")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_min_inclusive(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"maxExclusive")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_max_exclusive(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"maxInclusive")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_max_inclusive(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"totalDigits")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_total_digits(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"fractionDigits")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fraction_digits(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"length")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_length(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"minLength")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_min_length(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"maxLength")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_max_length(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"enumeration")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumeration(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"whiteSpace")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_white_space(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"pattern")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pattern(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"assertion")
                ) {
                    let output = <super::AssertionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_assertion(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"explicitTimezone")
                ) {
                    let output = <super::FacetType as WithDeserializer>::init(helper, event)?;
                    return self.handle_explicit_timezone(helper, Default::default(), None, output);
                }
            }
            *self.state__ = FacetDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: FacetDeserializerState,
        ) -> Result<super::Facet, Error> {
            use FacetDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::MinExclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_min_exclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MinExclusive(
                        helper.finish_element("minExclusive", values)?,
                    ))
                }
                S::MinInclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_min_inclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MinInclusive(
                        helper.finish_element("minInclusive", values)?,
                    ))
                }
                S::MaxExclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_max_exclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxExclusive(
                        helper.finish_element("maxExclusive", values)?,
                    ))
                }
                S::MaxInclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_max_inclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxInclusive(
                        helper.finish_element("maxInclusive", values)?,
                    ))
                }
                S::TotalDigits(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_total_digits(&mut values, value)?;
                    }
                    Ok(super::Facet::TotalDigits(
                        helper.finish_element("totalDigits", values)?,
                    ))
                }
                S::FractionDigits(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fraction_digits(&mut values, value)?;
                    }
                    Ok(super::Facet::FractionDigits(
                        helper.finish_element("fractionDigits", values)?,
                    ))
                }
                S::Length(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_length(&mut values, value)?;
                    }
                    Ok(super::Facet::Length(
                        helper.finish_element("length", values)?,
                    ))
                }
                S::MinLength(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_min_length(&mut values, value)?;
                    }
                    Ok(super::Facet::MinLength(
                        helper.finish_element("minLength", values)?,
                    ))
                }
                S::MaxLength(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_max_length(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxLength(
                        helper.finish_element("maxLength", values)?,
                    ))
                }
                S::Enumeration(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumeration(&mut values, value)?;
                    }
                    Ok(super::Facet::Enumeration(
                        helper.finish_element("enumeration", values)?,
                    ))
                }
                S::WhiteSpace(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_white_space(&mut values, value)?;
                    }
                    Ok(super::Facet::WhiteSpace(
                        helper.finish_element("whiteSpace", values)?,
                    ))
                }
                S::Pattern(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pattern(&mut values, value)?;
                    }
                    Ok(super::Facet::Pattern(
                        helper.finish_element("pattern", values)?,
                    ))
                }
                S::Assertion(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_assertion(&mut values, value)?;
                    }
                    Ok(super::Facet::Assertion(
                        helper.finish_element("assertion", values)?,
                    ))
                }
                S::ExplicitTimezone(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_explicit_timezone(&mut values, value)?;
                    }
                    Ok(super::Facet::ExplicitTimezone(
                        helper.finish_element("explicitTimezone", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_min_exclusive<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_min_exclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_min_exclusive(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MinExclusive(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MinExclusive(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_min_inclusive<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_min_inclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_min_inclusive(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MinInclusive(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MinInclusive(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_max_exclusive<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_max_exclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_max_exclusive(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MaxExclusive(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MaxExclusive(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_max_inclusive<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_max_inclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_max_inclusive(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MaxInclusive(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MaxInclusive(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_total_digits<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_total_digits(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_total_digits(&mut values, data)?;
                    let data = Self::finish_state(helper, S::TotalDigits(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::TotalDigits(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fraction_digits<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fraction_digits(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fraction_digits(&mut values, data)?;
                    let data = Self::finish_state(helper, S::FractionDigits(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::FractionDigits(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_length<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_length(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_length(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Length(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Length(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_min_length<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_min_length(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_min_length(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MinLength(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MinLength(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_max_length<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_max_length(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_max_length(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MaxLength(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MaxLength(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumeration<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumeration(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumeration(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumeration(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumeration(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_white_space<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_white_space(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_white_space(&mut values, data)?;
                    let data = Self::finish_state(helper, S::WhiteSpace(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::WhiteSpace(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_pattern<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pattern(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pattern(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pattern(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pattern(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_assertion<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AssertionType>,
            fallback: Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AssertionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_assertion(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assertion(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Assertion(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Assertion(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_explicit_timezone<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FacetType>,
            fallback: Option<<super::FacetType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FacetType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_explicit_timezone(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_explicit_timezone(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ExplicitTimezone(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ExplicitTimezone(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::Facet> for FacetDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Facet> {
            let deserializer = Self {
                state__: Box::new(FacetDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, FacetDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Facet> {
            use FacetDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::MinExclusive(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_min_exclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MinInclusive(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_min_inclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MaxExclusive(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_max_exclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MaxInclusive(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_max_inclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TotalDigits(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_total_digits(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::FractionDigits(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fraction_digits(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Length(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_length(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MinLength(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_min_length(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MaxLength(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_max_length(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumeration(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumeration(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::WhiteSpace(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_white_space(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pattern(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pattern(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Assertion(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_assertion(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ExplicitTimezone(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_explicit_timezone(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::MinExclusive(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"minExclusive",
                            true,
                        )?;
                        match self.handle_min_exclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::MinInclusive(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"minInclusive",
                            true,
                        )?;
                        match self.handle_min_inclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::MaxExclusive(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"maxExclusive",
                            true,
                        )?;
                        match self.handle_max_exclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::MaxInclusive(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"maxInclusive",
                            true,
                        )?;
                        match self.handle_max_inclusive(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::TotalDigits(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"totalDigits",
                            true,
                        )?;
                        match self.handle_total_digits(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::FractionDigits(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"fractionDigits",
                            true,
                        )?;
                        match self.handle_fraction_digits(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Length(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"length",
                            true,
                        )?;
                        match self.handle_length(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::MinLength(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"minLength",
                            true,
                        )?;
                        match self.handle_min_length(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::MaxLength(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"maxLength",
                            true,
                        )?;
                        match self.handle_max_length(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumeration(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"enumeration",
                            true,
                        )?;
                        match self.handle_enumeration(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::WhiteSpace(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"whiteSpace",
                            true,
                        )?;
                        match self.handle_white_space(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Pattern(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"pattern",
                            true,
                        )?;
                        match self.handle_pattern(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Assertion(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"assertion",
                            true,
                        )?;
                        match self.handle_assertion(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ExplicitTimezone(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"explicitTimezone",
                            true,
                        )?;
                        match self.handle_explicit_timezone(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Facet, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct RestrictionTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        base: String,
        content: Vec<super::RestrictionTypeContent>,
        state__: Box<RestrictionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictionTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RestrictionTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RestrictionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"base")
                ) {
                    helper.read_attrib(&mut base, b"base", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                base: base.ok_or_else(|| ErrorKind::MissingAttribute("base".into()))?,
                content: Vec::new(),
                state__: Box::new(RestrictionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RestrictionTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RestrictionTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RestrictionTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RestrictionTypeContent>,
            fallback: &mut Option<RestrictionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionType> for RestrictionTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionType> {
            use RestrictionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::RestrictionTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RestrictionType, Error> {
            let state = replace(
                &mut *self.state__,
                RestrictionTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RestrictionType {
                any_attribute: self.any_attribute,
                id: self.id,
                base: self.base,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RestrictionTypeContentDeserializer {
        state__: Box<RestrictionTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RestrictionTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContentElementType>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        SimpleType(
            Option<super::SimpleBaseType>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
        ),
        Facet(
            Option<super::Facet>,
            Option<<super::Facet as WithDeserializer>::Deserializer>,
            Option<<super::Facet as WithDeserializer>::Deserializer>,
        ),
        Any(
            Option<AnyElement>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Assert(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RestrictionTypeContent),
        Unknown__,
    }
    impl RestrictionTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"openContent")
                ) {
                    let output =
                        <super::OpenContentElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_open_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"all")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_all(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"choice")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_choice(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"sequence")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sequence(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleType")
                ) {
                    let output = <super::SimpleBaseType as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_type(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attribute")
                ) {
                    let output = <super::AttributeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"anyAttribute")
                ) {
                    let output =
                        <super::AnyAttributeElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_any_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"assert")
                ) {
                    let output = <super::AssertionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_assert(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::Facet as WithDeserializer>::init(helper, event)?;
                    match self.handle_facet(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
                event = {
                    let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                    match self.handle_any(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, .. } => event,
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = RestrictionTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RestrictionTypeContentDeserializerState,
        ) -> Result<super::RestrictionTypeContent, Error> {
            use RestrictionTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::OpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_open_content(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::OpenContent(
                        helper.finish_element("openContent", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::Facet(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_facet(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Facet(
                        helper.finish_element("facet", values)?,
                    ))
                }
                S::Any(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Any(
                        helper.finish_element("any45", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Assert(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_assert(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Assert(
                        helper.finish_element("assert", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn store_any(values: &mut Option<AnyElement>, value: AnyElement) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"any45",
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_open_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpenContentElementType>,
            fallback: Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpenContentElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_open_content(&mut values, data)?;
                    let data = Self::finish_state(helper, S::OpenContent(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::OpenContent(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_all<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::finish_state(helper, S::All(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::All(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_choice<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Choice(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Choice(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sequence<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sequence(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sequence(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_simple_type<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SimpleBaseType>,
            fallback: Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleBaseType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_simple_type(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SimpleType(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SimpleType(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_facet<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::Facet>,
            fallback: Option<<super::Facet as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Facet>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_facet(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_facet(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Facet(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Facet(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<AnyElement>,
            fallback: Option<<AnyElement as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, AnyElement>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Any(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Any(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeType>,
            fallback: Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Attribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Attribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyAttributeElementType>,
            fallback: Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AnyAttribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AnyAttribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_assert<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AssertionType>,
            fallback: Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AssertionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_assert(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assert(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Assert(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Assert(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionTypeContent> for RestrictionTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionTypeContent> {
            let deserializer = Self {
                state__: Box::new(RestrictionTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RestrictionTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionTypeContent> {
            use RestrictionTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OpenContent(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::All(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Choice(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sequence(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SimpleType(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Facet(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_facet(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Any(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Attribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AnyAttribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Assert(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_assert(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::OpenContent(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"openContent",
                            true,
                        )?;
                        match self.handle_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::All(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"all",
                            true,
                        )?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Choice(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"choice",
                            true,
                        )?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sequence(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"sequence",
                            true,
                        )?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SimpleType(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"simpleType",
                            true,
                        )?;
                        match self.handle_simple_type(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Facet(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::Facet as WithDeserializer>::init(helper, event)?;
                        match self.handle_facet(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Any(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                        match self.handle_any(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            true,
                        )?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AnyAttribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"anyAttribute",
                            true,
                        )?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Assert(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"assert",
                            true,
                        )?;
                        match self.handle_assert(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RestrictionTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ExtensionTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        base: String,
        content: Vec<super::ExtensionTypeContent>,
        state__: Box<ExtensionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExtensionTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ExtensionTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ExtensionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut base: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"base")
                ) {
                    helper.read_attrib(&mut base, b"base", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                base: base.ok_or_else(|| ErrorKind::MissingAttribute("base".into()))?,
                content: Vec::new(),
                state__: Box::new(ExtensionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ExtensionTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ExtensionTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ExtensionTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ExtensionTypeContent>,
            fallback: &mut Option<ExtensionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ExtensionType> for ExtensionTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionType> {
            use ExtensionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ExtensionTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ExtensionType, Error> {
            let state = replace(
                &mut *self.state__,
                ExtensionTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ExtensionType {
                any_attribute: self.any_attribute,
                id: self.id,
                base: self.base,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExtensionTypeContentDeserializer {
        state__: Box<ExtensionTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ExtensionTypeContentDeserializerState {
        Init__,
        Annotation(
            Option<super::AnnotationElementType>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContentElementType>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
            Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
        ),
        Group(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        All(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Choice(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Sequence(
            Option<super::GroupType>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
            Option<<super::GroupType as WithDeserializer>::Deserializer>,
        ),
        Attribute(
            Option<super::AttributeType>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeType as WithDeserializer>::Deserializer>,
        ),
        AttributeGroup(
            Option<super::AttributeGroupType>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
        ),
        AnyAttribute(
            Option<super::AnyAttributeElementType>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
        ),
        Assert(
            Option<super::AssertionType>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            Option<<super::AssertionType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ExtensionTypeContent),
        Unknown__,
    }
    impl ExtensionTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output =
                        <super::AnnotationElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"openContent")
                ) {
                    let output =
                        <super::OpenContentElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_open_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"group")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"all")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_all(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"choice")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_choice(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"sequence")
                ) {
                    let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sequence(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attribute")
                ) {
                    let output = <super::AttributeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"attributeGroup")
                ) {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::init(helper, event)?;
                    return self.handle_attribute_group(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"anyAttribute")
                ) {
                    let output =
                        <super::AnyAttributeElementType as WithDeserializer>::init(helper, event)?;
                    return self.handle_any_attribute(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"assert")
                ) {
                    let output = <super::AssertionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_assert(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ExtensionTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ExtensionTypeContentDeserializerState,
        ) -> Result<super::ExtensionTypeContent, Error> {
            use ExtensionTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::OpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_open_content(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::OpenContent(
                        helper.finish_element("openContent", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_group(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_all(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_choice(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sequence(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Assert(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_assert(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Assert(
                        helper.finish_element("assert", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnnotationElementType>,
            fallback: Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_annotation(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Annotation(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Annotation(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_open_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpenContentElementType>,
            fallback: Option<<super::OpenContentElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpenContentElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_open_content(&mut values, data)?;
                    let data = Self::finish_state(helper, S::OpenContent(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::OpenContent(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Group(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Group(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_all<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_all(&mut values, data)?;
                    let data = Self::finish_state(helper, S::All(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::All(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_choice<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_choice(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Choice(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Choice(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sequence<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupType>,
            fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sequence(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sequence(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sequence(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeType>,
            fallback: Option<<super::AttributeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Attribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Attribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_attribute_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AttributeGroupType>,
            fallback: Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AttributeGroupType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_attribute_group(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AttributeGroup(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AttributeGroup(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_any_attribute<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyAttributeElementType>,
            fallback: Option<<super::AnyAttributeElementType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttributeElementType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any_attribute(&mut values, data)?;
                    let data = Self::finish_state(helper, S::AnyAttribute(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::AnyAttribute(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_assert<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AssertionType>,
            fallback: Option<<super::AssertionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AssertionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExtensionTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_assert(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_assert(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Assert(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Assert(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ExtensionTypeContent> for ExtensionTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionTypeContent> {
            let deserializer = Self {
                state__: Box::new(ExtensionTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ExtensionTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionTypeContent> {
            use ExtensionTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OpenContent(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Group(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::All(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Choice(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sequence(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Attribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AttributeGroup(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::AnyAttribute(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Assert(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_assert(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Annotation(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::OpenContent(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"openContent",
                            true,
                        )?;
                        match self.handle_open_content(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Group(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"group",
                            true,
                        )?;
                        match self.handle_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::All(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"all",
                            true,
                        )?;
                        match self.handle_all(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Choice(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"choice",
                            true,
                        )?;
                        match self.handle_choice(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sequence(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"sequence",
                            true,
                        )?;
                        match self.handle_sequence(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            true,
                        )?;
                        match self.handle_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AttributeGroup(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attributeGroup",
                            true,
                        )?;
                        match self.handle_attribute_group(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::AnyAttribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"anyAttribute",
                            true,
                        )?;
                        match self.handle_any_attribute(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Assert(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"assert",
                            true,
                        )?;
                        match self.handle_assert(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ExtensionTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct FieldElementTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        xpath: String,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<FieldElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FieldElementTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FieldElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut xpath: Option<String> = None;
            let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpath")
                ) {
                    helper.read_attrib(&mut xpath, b"xpath", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"xpathDefaultNamespace")
                ) {
                    helper.read_attrib(
                        &mut xpath_default_namespace,
                        b"xpathDefaultNamespace",
                        &attrib.value,
                    )?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                xpath: xpath.ok_or_else(|| ErrorKind::MissingAttribute("xpath".into()))?,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state__: Box::new(FieldElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FieldElementTypeDeserializerState,
        ) -> Result<(), Error> {
            use FieldElementTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<FieldElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FieldElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FieldElementType> for FieldElementTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldElementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldElementType> {
            use FieldElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::FieldElementType, Error> {
            let state = replace(
                &mut *self.state__,
                FieldElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::FieldElementType {
                any_attribute: self.any_attribute,
                id: self.id,
                xpath: self.xpath,
                xpath_default_namespace: self.xpath_default_namespace,
                annotation: self.annotation,
            })
        }
    }
    #[derive(Debug)]
    pub struct FacetTypeDeserializer {
        any_attribute: AnyAttributes,
        id: Option<String>,
        value: String,
        fixed: bool,
        annotation: Option<super::AnnotationElementType>,
        state__: Box<FacetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FacetTypeDeserializerState {
        Init__,
        Annotation(Option<<super::AnnotationElementType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FacetTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut any_attribute = AnyAttributes::default();
            let mut id: Option<String> = None;
            let mut value: Option<String> = None;
            let mut fixed: Option<bool> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"value")
                ) {
                    helper.read_attrib(&mut value, b"value", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"fixed")
                ) {
                    helper.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                any_attribute: any_attribute,
                id: id,
                value: value.ok_or_else(|| ErrorKind::MissingAttribute("value".into()))?,
                fixed: fixed.unwrap_or_else(super::FacetType::default_fixed),
                annotation: None,
                state__: Box::new(FacetTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FacetTypeDeserializerState,
        ) -> Result<(), Error> {
            use FacetTypeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationElementType>,
            fallback: &mut Option<FacetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FacetTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Annotation(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_annotation(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Annotation(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FacetType> for FacetTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FacetType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FacetType> {
            use FacetTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Annotation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Annotation(None);
                        event
                    }
                    (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"annotation",
                            true,
                        )?;
                        match self.handle_annotation(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FacetType, Error> {
            let state = replace(&mut *self.state__, FacetTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FacetType {
                any_attribute: self.any_attribute,
                id: self.id,
                value: self.value,
                fixed: self.fixed,
                annotation: self.annotation,
            })
        }
    }
}
