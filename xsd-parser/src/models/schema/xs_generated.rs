use crate::models::schema::{MaxOccurs, QName};
use core::ops::Deref;
use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, ErrorKind, RawByteStr, ValidateError,
        WithDeserializer,
    },
    xml::AnyElement,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    pub target_namespace: Option<String>,
    pub version: Option<String>,
    pub final_default: FullDerivationSetType,
    pub block_default: BlockSetType,
    pub attribute_form_default: FormChoiceType,
    pub element_form_default: FormChoiceType,
    pub default_attributes: Option<QName>,
    pub xpath_default_namespace: XpathDefaultNamespaceType,
    pub id: Option<String>,
    pub lang: Option<String>,
    pub content: Vec<SchemaContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
impl WithDeserializer for Schema {
    type Deserializer = Box<quick_xml_deserialize::SchemaDeserializer>;
}
impl WithDeserializer for SchemaContent {
    type Deserializer = Box<quick_xml_deserialize::SchemaContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TypeDerivationControlList(pub Vec<TypeDerivationControlType>);
impl DeserializeBytes for TypeDerivationControlList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BlockSetItemList(pub Vec<BlockSetItemType>);
impl DeserializeBytes for BlockSetItemList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Include {
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Include {
    type Deserializer = Box<quick_xml_deserialize::IncludeDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Import {
    type Deserializer = Box<quick_xml_deserialize::ImportDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redefine {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<RedefineContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RedefineContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}
impl WithDeserializer for Redefine {
    type Deserializer = Box<quick_xml_deserialize::RedefineDeserializer>;
}
impl WithDeserializer for RedefineContent {
    type Deserializer = Box<quick_xml_deserialize::RedefineContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Override {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<OverrideContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
impl WithDeserializer for Override {
    type Deserializer = Box<quick_xml_deserialize::OverrideDeserializer>;
}
impl WithDeserializer for OverrideContent {
    type Deserializer = Box<quick_xml_deserialize::OverrideContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    pub id: Option<String>,
    pub content: Vec<AnnotationContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnnotationContent {
    Appinfo(AnyElement),
    Documentation(AnyElement),
}
impl WithDeserializer for Annotation {
    type Deserializer = Box<quick_xml_deserialize::AnnotationDeserializer>;
}
impl WithDeserializer for AnnotationContent {
    type Deserializer = Box<quick_xml_deserialize::AnnotationContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultOpenContent {
    pub id: Option<String>,
    pub applies_to_empty: bool,
    pub mode: DefaultOpenContentModeType,
    pub annotation: Option<Annotation>,
    pub any: WildcardType,
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
impl WithDeserializer for DefaultOpenContent {
    type Deserializer = Box<quick_xml_deserialize::DefaultOpenContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBaseType {
    pub id: Option<String>,
    pub final_: Option<SimpleDerivationSetType>,
    pub name: Option<String>,
    pub content: Vec<SimpleBaseTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimpleBaseTypeContent {
    Annotation(Annotation),
    Restriction(Restriction),
    List(List),
    Union(Union),
}
impl WithDeserializer for SimpleBaseType {
    type Deserializer = Box<quick_xml_deserialize::SimpleBaseTypeDeserializer>;
}
impl WithDeserializer for SimpleBaseTypeContent {
    type Deserializer = Box<quick_xml_deserialize::SimpleBaseTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    type Deserializer = Box<quick_xml_deserialize::ComplexBaseTypeDeserializer>;
}
impl WithDeserializer for ComplexBaseTypeContent {
    type Deserializer = Box<quick_xml_deserialize::ComplexBaseTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub min_occurs: usize,
    pub max_occurs: MaxOccurs,
    pub content: Vec<GroupTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupTypeContent {
    Annotation(Annotation),
    Element(ElementType),
    Group(GroupType),
    All(GroupType),
    Choice(GroupType),
    Sequence(GroupType),
    Any(Any),
}
impl GroupType {
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> MaxOccurs {
        MaxOccurs::Bounded(1usize)
    }
}
impl WithDeserializer for GroupType {
    type Deserializer = Box<quick_xml_deserialize::GroupTypeDeserializer>;
}
impl WithDeserializer for GroupTypeContent {
    type Deserializer = Box<quick_xml_deserialize::GroupTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributeGroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub content: Vec<AttributeGroupTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeGroupTypeContent {
    Annotation(Annotation),
    Attribute(AttributeType),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttribute),
}
impl WithDeserializer for AttributeGroupType {
    type Deserializer = Box<quick_xml_deserialize::AttributeGroupTypeDeserializer>;
}
impl WithDeserializer for AttributeGroupTypeContent {
    type Deserializer = Box<quick_xml_deserialize::AttributeGroupTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub type_: Option<QName>,
    pub substitution_group: Option<QNameList>,
    pub min_occurs: usize,
    pub max_occurs: MaxOccurs,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElementTypeContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
    Alternative(AltType),
    Unique(KeybaseType),
    Key(KeybaseType),
    Keyref(Keyref),
}
impl ElementType {
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> MaxOccurs {
        MaxOccurs::Bounded(1usize)
    }
    #[must_use]
    pub fn default_abstract_() -> bool {
        false
    }
}
impl WithDeserializer for ElementType {
    type Deserializer = Box<quick_xml_deserialize::ElementTypeDeserializer>;
}
impl WithDeserializer for ElementTypeContent {
    type Deserializer = Box<quick_xml_deserialize::ElementTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributeType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub type_: Option<QName>,
    pub use_: AttributeUseType,
    pub default: Option<String>,
    pub fixed: Option<String>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<String>,
    pub inheritable: Option<bool>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<SimpleBaseType>,
}
impl AttributeType {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}
impl WithDeserializer for AttributeType {
    type Deserializer = Box<quick_xml_deserialize::AttributeTypeDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notation {
    pub id: Option<String>,
    pub name: String,
    pub public: Option<String>,
    pub system: Option<String>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Notation {
    type Deserializer = Box<quick_xml_deserialize::NotationDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WildcardType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub annotation: Option<Annotation>,
}
impl WildcardType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
impl WithDeserializer for WildcardType {
    type Deserializer = Box<quick_xml_deserialize::WildcardTypeDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Restriction {
    pub id: Option<String>,
    pub base: Option<QName>,
    pub content: Vec<RestrictionContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RestrictionContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    Facet(Facet),
}
impl WithDeserializer for Restriction {
    type Deserializer = Box<quick_xml_deserialize::RestrictionDeserializer>;
}
impl WithDeserializer for RestrictionContent {
    type Deserializer = Box<quick_xml_deserialize::RestrictionContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct List {
    pub id: Option<String>,
    pub item_type: Option<QName>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<Box<SimpleBaseType>>,
}
impl WithDeserializer for List {
    type Deserializer = Box<quick_xml_deserialize::ListDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Union {
    pub id: Option<String>,
    pub member_types: Option<QNameList>,
    pub annotation: Option<Annotation>,
    pub simple_type: Vec<SimpleBaseType>,
}
impl WithDeserializer for Union {
    type Deserializer = Box<quick_xml_deserialize::UnionDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleContent {
    pub id: Option<String>,
    pub content: Vec<SimpleContentContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimpleContentContent {
    Annotation(Annotation),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl WithDeserializer for SimpleContent {
    type Deserializer = Box<quick_xml_deserialize::SimpleContentDeserializer>;
}
impl WithDeserializer for SimpleContentContent {
    type Deserializer = Box<quick_xml_deserialize::SimpleContentContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplexContent {
    pub id: Option<String>,
    pub mixed: Option<bool>,
    pub content: Vec<ComplexContentContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComplexContentContent {
    Annotation(Annotation),
    Restriction(RestrictionType),
    Extension(ExtensionType),
}
impl WithDeserializer for ComplexContent {
    type Deserializer = Box<quick_xml_deserialize::ComplexContentDeserializer>;
}
impl WithDeserializer for ComplexContentContent {
    type Deserializer = Box<quick_xml_deserialize::ComplexContentContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenContent {
    pub id: Option<String>,
    pub mode: OpenContentModeType,
    pub annotation: Option<Annotation>,
    pub any: Option<WildcardType>,
}
impl OpenContent {
    #[must_use]
    pub fn default_mode() -> OpenContentModeType {
        OpenContentModeType::Interleave
    }
}
impl WithDeserializer for OpenContent {
    type Deserializer = Box<quick_xml_deserialize::OpenContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnyAttribute {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListAType>,
    pub annotation: Option<Annotation>,
}
impl AnyAttribute {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
impl WithDeserializer for AnyAttribute {
    type Deserializer = Box<quick_xml_deserialize::AnyAttributeDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssertionType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for AssertionType {
    type Deserializer = Box<quick_xml_deserialize::AssertionTypeDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Any {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NotNamespaceType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<QnameListType>,
    pub min_occurs: usize,
    pub max_occurs: MaxOccurs,
    pub annotation: Option<Annotation>,
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
    pub fn default_max_occurs() -> MaxOccurs {
        MaxOccurs::Bounded(1usize)
    }
}
impl WithDeserializer for Any {
    type Deserializer = Box<quick_xml_deserialize::AnyDeserializer>;
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QNameList(pub Vec<QName>);
impl DeserializeBytes for QNameList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AltType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub type_: Option<QName>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub content: Vec<AltTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AltTypeContent {
    Annotation(Annotation),
    SimpleType(SimpleBaseType),
    ComplexType(ComplexBaseType),
}
impl WithDeserializer for AltType {
    type Deserializer = Box<quick_xml_deserialize::AltTypeDeserializer>;
}
impl WithDeserializer for AltTypeContent {
    type Deserializer = Box<quick_xml_deserialize::AltTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeybaseType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub content: Option<KeybaseTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeybaseTypeContent {
    pub annotation: Option<Annotation>,
    pub selector: Field,
    pub field: Vec<Field>,
}
impl WithDeserializer for KeybaseType {
    type Deserializer = Box<quick_xml_deserialize::KeybaseTypeDeserializer>;
}
impl WithDeserializer for KeybaseTypeContent {
    type Deserializer = Box<quick_xml_deserialize::KeybaseTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keyref {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub refer: Option<QName>,
    pub content: Option<KeyrefContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyrefContent {
    pub annotation: Option<Annotation>,
    pub selector: Field,
    pub field: Vec<Field>,
}
impl WithDeserializer for Keyref {
    type Deserializer = Box<quick_xml_deserialize::KeyrefDeserializer>;
}
impl WithDeserializer for KeyrefContent {
    type Deserializer = Box<quick_xml_deserialize::KeyrefContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
        let inner = bytes
            .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
            .map(|bytes| BasicNamespaceListItemType::deserialize_bytes(helper, bytes))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
impl DeserializeBytes for SimpleDerivationSetItemList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
    type Deserializer = Box<quick_xml_deserialize::FacetDeserializer>;
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
impl DeserializeBytes for ReducedDerivationControlList {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestrictionType {
    pub id: Option<String>,
    pub base: QName,
    pub content: Vec<RestrictionTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
impl WithDeserializer for RestrictionType {
    type Deserializer = Box<quick_xml_deserialize::RestrictionTypeDeserializer>;
}
impl WithDeserializer for RestrictionTypeContent {
    type Deserializer = Box<quick_xml_deserialize::RestrictionTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionType {
    pub id: Option<String>,
    pub base: QName,
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
impl WithDeserializer for ExtensionType {
    type Deserializer = Box<quick_xml_deserialize::ExtensionTypeDeserializer>;
}
impl WithDeserializer for ExtensionTypeContent {
    type Deserializer = Box<quick_xml_deserialize::ExtensionTypeContentDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QnameListAType(pub Vec<QnameListAItemType>);
impl DeserializeBytes for QnameListAType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QnameListType(pub Vec<QnameListItemType>);
impl DeserializeBytes for QnameListType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub id: Option<String>,
    pub xpath: String,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Field {
    type Deserializer = Box<quick_xml_deserialize::FieldDeserializer>;
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
impl DeserializeBytes for BasicNamespaceListType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FacetType {
    pub id: Option<String>,
    pub value: String,
    pub fixed: bool,
    pub annotation: Option<Annotation>,
}
impl FacetType {
    #[must_use]
    pub fn default_fixed() -> bool {
        false
    }
}
impl WithDeserializer for FacetType {
    type Deserializer = Box<quick_xml_deserialize::FacetTypeDeserializer>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QnameListAItemType {
    QName(QName),
    Defined,
}
impl DeserializeBytes for QnameListAItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            x => Ok(Self::QName(QName::deserialize_bytes(helper, x)?)),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QnameListItemType {
    QName(QName),
    Defined,
    DefinedSibling,
}
impl DeserializeBytes for QnameListItemType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            b"##definedSibling" => Ok(Self::DefinedSibling),
            x => Ok(Self::QName(QName::deserialize_bytes(helper, x)?)),
        }
    }
}
pub mod quick_xml_deserialize {
    use crate::models::schema::{MaxOccurs, QName};
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        },
        xml::AnyElement,
    };
    #[derive(Debug)]
    pub struct SchemaDeserializer {
        target_namespace: Option<String>,
        version: Option<String>,
        final_default: super::FullDerivationSetType,
        block_default: super::BlockSetType,
        attribute_form_default: super::FormChoiceType,
        element_form_default: super::FormChoiceType,
        default_attributes: Option<QName>,
        xpath_default_namespace: super::XpathDefaultNamespaceType,
        id: Option<String>,
        lang: Option<String>,
        content: Vec<super::SchemaContent>,
        state__: Box<SchemaDeserializerState>,
    }
    #[derive(Debug)]
    enum SchemaDeserializerState {
        Init__,
        Next__,
        Content__(<super::SchemaContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SchemaDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut target_namespace: Option<String> = None;
            let mut version: Option<String> = None;
            let mut final_default: Option<super::FullDerivationSetType> = None;
            let mut block_default: Option<super::BlockSetType> = None;
            let mut attribute_form_default: Option<super::FormChoiceType> = None;
            let mut element_form_default: Option<super::FormChoiceType> = None;
            let mut default_attributes: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
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
                state__: Box::new(SchemaDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SchemaDeserializerState,
        ) -> Result<(), Error> {
            if let SchemaDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SchemaContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SchemaContent>,
            fallback: &mut Option<SchemaDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Schema> for Box<SchemaDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Schema> {
            helper.init_deserializer_from_start_event(event, SchemaDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Schema> {
            use SchemaDeserializerState as S;
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
                            <super::SchemaContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Schema, Error> {
            let state = replace(&mut *self.state__, SchemaDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SchemaContentDeserializer {
        state__: Box<SchemaContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SchemaContentDeserializerState {
        Init__,
        Include(
            Option<super::Include>,
            Option<<super::Include as WithDeserializer>::Deserializer>,
            Option<<super::Include as WithDeserializer>::Deserializer>,
        ),
        Import(
            Option<super::Import>,
            Option<<super::Import as WithDeserializer>::Deserializer>,
            Option<<super::Import as WithDeserializer>::Deserializer>,
        ),
        Redefine(
            Option<super::Redefine>,
            Option<<super::Redefine as WithDeserializer>::Deserializer>,
            Option<<super::Redefine as WithDeserializer>::Deserializer>,
        ),
        Override(
            Option<super::Override>,
            Option<<super::Override as WithDeserializer>::Deserializer>,
            Option<<super::Override as WithDeserializer>::Deserializer>,
        ),
        Annotation(
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
        ),
        DefaultOpenContent(
            Option<super::DefaultOpenContent>,
            Option<<super::DefaultOpenContent as WithDeserializer>::Deserializer>,
            Option<<super::DefaultOpenContent as WithDeserializer>::Deserializer>,
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
            Option<super::Notation>,
            Option<<super::Notation as WithDeserializer>::Deserializer>,
            Option<<super::Notation as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SchemaContent),
        Unknown__,
    }
    impl SchemaContentDeserializer {
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
                    let output = <super::Include as WithDeserializer>::init(helper, event)?;
                    return self.handle_include(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"import")
                ) {
                    let output = <super::Import as WithDeserializer>::init(helper, event)?;
                    return self.handle_import(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"redefine")
                ) {
                    let output = <super::Redefine as WithDeserializer>::init(helper, event)?;
                    return self.handle_redefine(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"override")
                ) {
                    let output = <super::Override as WithDeserializer>::init(helper, event)?;
                    return self.handle_override_(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"defaultOpenContent")
                ) {
                    let output =
                        <super::DefaultOpenContent as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::Notation as WithDeserializer>::init(helper, event)?;
                    return self.handle_notation(helper, Default::default(), None, output);
                }
            }
            *self.state__ = SchemaContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SchemaContentDeserializerState,
        ) -> Result<super::SchemaContent, Error> {
            use SchemaContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Include(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_include(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Include(
                        helper.finish_element("include", values)?,
                    ))
                }
                S::Import(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_import(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Import(
                        helper.finish_element("import", values)?,
                    ))
                }
                S::Redefine(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_redefine(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Redefine(
                        helper.finish_element("redefine", values)?,
                    ))
                }
                S::Override(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_override_(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Override(
                        helper.finish_element("override", values)?,
                    ))
                }
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::DefaultOpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_default_open_content(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::DefaultOpenContent(
                        helper.finish_element("defaultOpenContent", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::Element(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_element(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Element(
                        helper.finish_element("element", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_attribute(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::Notation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SchemaContentDeserializer::store_notation(&mut values, value)?;
                    }
                    Ok(super::SchemaContent::Notation(
                        helper.finish_element("notation", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_include(
            values: &mut Option<super::Include>,
            value: super::Include,
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
            values: &mut Option<super::Import>,
            value: super::Import,
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
            values: &mut Option<super::Redefine>,
            value: super::Redefine,
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
            values: &mut Option<super::Override>,
            value: super::Override,
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::DefaultOpenContent>,
            value: super::DefaultOpenContent,
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
            values: &mut Option<super::Notation>,
            value: super::Notation,
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
            mut values: Option<super::Include>,
            fallback: Option<<super::Include as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Include>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_include(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_include(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Include(values, None, None),
                    )?;
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
            mut values: Option<super::Import>,
            fallback: Option<<super::Import as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Import>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_import(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_import(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Import(values, None, None),
                    )?;
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
            mut values: Option<super::Redefine>,
            fallback: Option<<super::Redefine as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Redefine>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_redefine(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_redefine(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Redefine(values, None, None),
                    )?;
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
            mut values: Option<super::Override>,
            fallback: Option<<super::Override as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Override>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_override_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_override_(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Override(values, None, None),
                    )?;
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_annotation(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            mut values: Option<super::DefaultOpenContent>,
            fallback: Option<<super::DefaultOpenContent as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DefaultOpenContent>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_default_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_default_open_content(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::DefaultOpenContent(values, None, None),
                    )?;
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
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_complex_type(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::ComplexType(values, None, None),
                    )?;
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
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_group(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_attribute_group(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_element(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_element(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Element(values, None, None),
                    )?;
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
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_attribute(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Attribute(values, None, None),
                    )?;
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
            mut values: Option<super::Notation>,
            fallback: Option<<super::Notation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Notation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SchemaContentDeserializerState as S;
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
                SchemaContentDeserializer::store_notation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SchemaContentDeserializer::store_notation(&mut values, data)?;
                    let data = SchemaContentDeserializer::finish_state(
                        helper,
                        S::Notation(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::SchemaContent> for Box<SchemaContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SchemaContent> {
            let deserializer = Box::new(SchemaContentDeserializer {
                state__: Box::new(SchemaContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, SchemaContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::SchemaContent> {
            use SchemaContentDeserializerState as S;
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
                            artifact: DeserializerArtifact::Data(
                                SchemaContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::SchemaContent, Error> {
            SchemaContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct IncludeDeserializer {
        id: Option<String>,
        schema_location: String,
        annotation: Option<super::Annotation>,
        state__: Box<IncludeDeserializerState>,
    }
    #[derive(Debug)]
    enum IncludeDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl IncludeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                schema_location: schema_location
                    .ok_or_else(|| ErrorKind::MissingAttribute("schemaLocation".into()))?,
                annotation: None,
                state__: Box::new(IncludeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: IncludeDeserializerState,
        ) -> Result<(), Error> {
            use IncludeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<IncludeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use IncludeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Include> for Box<IncludeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Include> {
            helper.init_deserializer_from_start_event(event, IncludeDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Include> {
            use IncludeDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Include, Error> {
            let state = replace(&mut *self.state__, IncludeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        state__: Box<ImportDeserializerState>,
    }
    #[derive(Debug)]
    enum ImportDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ImportDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                namespace: namespace,
                schema_location: schema_location,
                annotation: None,
                state__: Box::new(ImportDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ImportDeserializerState,
        ) -> Result<(), Error> {
            use ImportDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<ImportDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ImportDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Import> for Box<ImportDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Import> {
            helper.init_deserializer_from_start_event(event, ImportDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Import> {
            use ImportDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Import, Error> {
            let state = replace(&mut *self.state__, ImportDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        state__: Box<RedefineDeserializerState>,
    }
    #[derive(Debug)]
    enum RedefineDeserializerState {
        Init__,
        Next__,
        Content__(<super::RedefineContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RedefineDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                schema_location: schema_location
                    .ok_or_else(|| ErrorKind::MissingAttribute("schemaLocation".into()))?,
                id: id,
                content: Vec::new(),
                state__: Box::new(RedefineDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RedefineDeserializerState,
        ) -> Result<(), Error> {
            if let RedefineDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RedefineContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RedefineContent>,
            fallback: &mut Option<RedefineDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Redefine> for Box<RedefineDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Redefine> {
            helper.init_deserializer_from_start_event(event, RedefineDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Redefine> {
            use RedefineDeserializerState as S;
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
                            <super::RedefineContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Redefine, Error> {
            let state = replace(&mut *self.state__, RedefineDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::Redefine {
                schema_location: self.schema_location,
                id: self.id,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RedefineContentDeserializer {
        state__: Box<RedefineContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RedefineContentDeserializerState {
        Init__,
        Annotation(
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
        Done__(super::RedefineContent),
        Unknown__,
    }
    impl RedefineContentDeserializer {
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
            *self.state__ = RedefineContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RedefineContentDeserializerState,
        ) -> Result<super::RedefineContent, Error> {
            use RedefineContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RedefineContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RedefineContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RedefineContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RedefineContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RedefineContentDeserializer::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::RedefineContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RedefineContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::RedefineContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RedefineContentDeserializer::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::RedefineContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_annotation(
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RedefineContentDeserializerState as S;
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
                RedefineContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RedefineContentDeserializer::store_annotation(&mut values, data)?;
                    let data = RedefineContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            use RedefineContentDeserializerState as S;
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
                RedefineContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RedefineContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = RedefineContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
            use RedefineContentDeserializerState as S;
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
                RedefineContentDeserializer::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RedefineContentDeserializer::store_complex_type(&mut values, data)?;
                    let data = RedefineContentDeserializer::finish_state(
                        helper,
                        S::ComplexType(values, None, None),
                    )?;
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
            use RedefineContentDeserializerState as S;
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
                RedefineContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RedefineContentDeserializer::store_group(&mut values, data)?;
                    let data = RedefineContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
            use RedefineContentDeserializerState as S;
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
                RedefineContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RedefineContentDeserializer::store_attribute_group(&mut values, data)?;
                    let data = RedefineContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::RedefineContent> for Box<RedefineContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RedefineContent> {
            let deserializer = Box::new(RedefineContentDeserializer {
                state__: Box::new(RedefineContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RedefineContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::RedefineContent> {
            use RedefineContentDeserializerState as S;
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
                            artifact: DeserializerArtifact::Data(
                                RedefineContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::RedefineContent, Error> {
            RedefineContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct OverrideDeserializer {
        schema_location: String,
        id: Option<String>,
        content: Vec<super::OverrideContent>,
        state__: Box<OverrideDeserializerState>,
    }
    #[derive(Debug)]
    enum OverrideDeserializerState {
        Init__,
        Next__,
        Content__(<super::OverrideContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl OverrideDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                schema_location: schema_location
                    .ok_or_else(|| ErrorKind::MissingAttribute("schemaLocation".into()))?,
                id: id,
                content: Vec::new(),
                state__: Box::new(OverrideDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OverrideDeserializerState,
        ) -> Result<(), Error> {
            if let OverrideDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::OverrideContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::OverrideContent>,
            fallback: &mut Option<OverrideDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Override> for Box<OverrideDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Override> {
            helper.init_deserializer_from_start_event(event, OverrideDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Override> {
            use OverrideDeserializerState as S;
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
                            <super::OverrideContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Override, Error> {
            let state = replace(&mut *self.state__, OverrideDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::Override {
                schema_location: self.schema_location,
                id: self.id,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct OverrideContentDeserializer {
        state__: Box<OverrideContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum OverrideContentDeserializerState {
        Init__,
        Annotation(
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
            Option<super::Notation>,
            Option<<super::Notation as WithDeserializer>::Deserializer>,
            Option<<super::Notation as WithDeserializer>::Deserializer>,
        ),
        Done__(super::OverrideContent),
        Unknown__,
    }
    impl OverrideContentDeserializer {
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::Notation as WithDeserializer>::init(helper, event)?;
                    return self.handle_notation(helper, Default::default(), None, output);
                }
            }
            *self.state__ = OverrideContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: OverrideContentDeserializerState,
        ) -> Result<super::OverrideContent, Error> {
            use OverrideContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_attribute_group(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::Element(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_element(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::Element(
                        helper.finish_element("element", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_attribute(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::Notation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        OverrideContentDeserializer::store_notation(&mut values, value)?;
                    }
                    Ok(super::OverrideContent::Notation(
                        helper.finish_element("notation", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_annotation(
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::Notation>,
            value: super::Notation,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_annotation(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_complex_type(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::ComplexType(values, None, None),
                    )?;
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
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_group(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_attribute_group(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_element(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_element(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::Element(values, None, None),
                    )?;
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
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_attribute(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::Attribute(values, None, None),
                    )?;
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
            mut values: Option<super::Notation>,
            fallback: Option<<super::Notation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Notation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OverrideContentDeserializerState as S;
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
                OverrideContentDeserializer::store_notation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    OverrideContentDeserializer::store_notation(&mut values, data)?;
                    let data = OverrideContentDeserializer::finish_state(
                        helper,
                        S::Notation(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::OverrideContent> for Box<OverrideContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OverrideContent> {
            let deserializer = Box::new(OverrideContentDeserializer {
                state__: Box::new(OverrideContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, OverrideContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::OverrideContent> {
            use OverrideContentDeserializerState as S;
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
                            artifact: DeserializerArtifact::Data(
                                OverrideContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::OverrideContent, Error> {
            OverrideContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AnnotationDeserializer {
        id: Option<String>,
        content: Vec<super::AnnotationContent>,
        state__: Box<AnnotationDeserializerState>,
    }
    #[derive(Debug)]
    enum AnnotationDeserializerState {
        Init__,
        Next__,
        Content__(<super::AnnotationContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AnnotationDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Box::new(Self {
                id: id,
                content: Vec::new(),
                state__: Box::new(AnnotationDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnnotationDeserializerState,
        ) -> Result<(), Error> {
            if let AnnotationDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::AnnotationContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnnotationContent>,
            fallback: &mut Option<AnnotationDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnnotationDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Annotation> for Box<AnnotationDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Annotation> {
            helper
                .init_deserializer_from_start_event(event, AnnotationDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Annotation> {
            use AnnotationDeserializerState as S;
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
                            <super::AnnotationContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Annotation, Error> {
            let state = replace(&mut *self.state__, AnnotationDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::Annotation {
                id: self.id,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnnotationContentDeserializer {
        state__: Box<AnnotationContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AnnotationContentDeserializerState {
        Init__,
        Appinfo(
            Option<AnyElement>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
        ),
        Documentation(
            Option<AnyElement>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AnnotationContent),
        Unknown__,
    }
    impl AnnotationContentDeserializer {
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
                    let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                    return self.handle_appinfo(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"documentation")
                ) {
                    let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                    return self.handle_documentation(helper, Default::default(), None, output);
                }
            }
            *self.state__ = AnnotationContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AnnotationContentDeserializerState,
        ) -> Result<super::AnnotationContent, Error> {
            use AnnotationContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Appinfo(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AnnotationContentDeserializer::store_appinfo(&mut values, value)?;
                    }
                    Ok(super::AnnotationContent::Appinfo(
                        helper.finish_element("appinfo", values)?,
                    ))
                }
                S::Documentation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AnnotationContentDeserializer::store_documentation(&mut values, value)?;
                    }
                    Ok(super::AnnotationContent::Documentation(
                        helper.finish_element("documentation", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_appinfo(values: &mut Option<AnyElement>, value: AnyElement) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"appinfo",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_documentation(
            values: &mut Option<AnyElement>,
            value: AnyElement,
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
            mut values: Option<AnyElement>,
            fallback: Option<<AnyElement as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, AnyElement>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnnotationContentDeserializerState as S;
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
                AnnotationContentDeserializer::store_appinfo(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AnnotationContentDeserializer::store_appinfo(&mut values, data)?;
                    let data = AnnotationContentDeserializer::finish_state(
                        helper,
                        S::Appinfo(values, None, None),
                    )?;
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
            mut values: Option<AnyElement>,
            fallback: Option<<AnyElement as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, AnyElement>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnnotationContentDeserializerState as S;
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
                AnnotationContentDeserializer::store_documentation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AnnotationContentDeserializer::store_documentation(&mut values, data)?;
                    let data = AnnotationContentDeserializer::finish_state(
                        helper,
                        S::Documentation(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::AnnotationContent> for Box<AnnotationContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnnotationContent> {
            let deserializer = Box::new(AnnotationContentDeserializer {
                state__: Box::new(AnnotationContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, AnnotationContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::AnnotationContent> {
            use AnnotationContentDeserializerState as S;
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
                            artifact: DeserializerArtifact::Data(
                                AnnotationContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AnnotationContent, Error> {
            AnnotationContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct DefaultOpenContentDeserializer {
        id: Option<String>,
        applies_to_empty: bool,
        mode: super::DefaultOpenContentModeType,
        annotation: Option<super::Annotation>,
        any: Option<super::WildcardType>,
        state__: Box<DefaultOpenContentDeserializerState>,
    }
    #[derive(Debug)]
    enum DefaultOpenContentDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Any(Option<<super::WildcardType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DefaultOpenContentDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                applies_to_empty: applies_to_empty
                    .unwrap_or_else(super::DefaultOpenContent::default_applies_to_empty),
                mode: mode.unwrap_or_else(super::DefaultOpenContent::default_mode),
                annotation: None,
                any: None,
                state__: Box::new(DefaultOpenContentDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DefaultOpenContentDeserializerState,
        ) -> Result<(), Error> {
            use DefaultOpenContentDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<DefaultOpenContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DefaultOpenContentDeserializerState as S;
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
            fallback: &mut Option<DefaultOpenContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DefaultOpenContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::DefaultOpenContent> for Box<DefaultOpenContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DefaultOpenContent> {
            helper.init_deserializer_from_start_event(
                event,
                DefaultOpenContentDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DefaultOpenContent> {
            use DefaultOpenContentDeserializerState as S;
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
                            false,
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
                            false,
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
        ) -> Result<super::DefaultOpenContent, Error> {
            let state = replace(
                &mut *self.state__,
                DefaultOpenContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DefaultOpenContent {
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
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                final_: final_,
                name: name,
                content: Vec::new(),
                state__: Box::new(SimpleBaseTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::SimpleBaseType> for Box<SimpleBaseTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseType> {
            helper.init_deserializer_from_start_event(
                event,
                SimpleBaseTypeDeserializer::from_bytes_start,
            )
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
        ),
        Restriction(
            Option<super::Restriction>,
            Option<<super::Restriction as WithDeserializer>::Deserializer>,
            Option<<super::Restriction as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::List>,
            Option<<super::List as WithDeserializer>::Deserializer>,
            Option<<super::List as WithDeserializer>::Deserializer>,
        ),
        Union(
            Option<super::Union>,
            Option<<super::Union as WithDeserializer>::Deserializer>,
            Option<<super::Union as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"restriction")
                ) {
                    let output = <super::Restriction as WithDeserializer>::init(helper, event)?;
                    return self.handle_restriction(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"list")
                ) {
                    let output = <super::List as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"union")
                ) {
                    let output = <super::Union as WithDeserializer>::init(helper, event)?;
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
                        SimpleBaseTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Restriction(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SimpleBaseTypeContentDeserializer::store_restriction(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::Restriction(
                        helper.finish_element("restriction", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SimpleBaseTypeContentDeserializer::store_list(&mut values, value)?;
                    }
                    Ok(super::SimpleBaseTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Union(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SimpleBaseTypeContentDeserializer::store_union_(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::Restriction>,
            value: super::Restriction,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"restriction",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(values: &mut Option<super::List>, value: super::List) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_union_(
            values: &mut Option<super::Union>,
            value: super::Union,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                SimpleBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = SimpleBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            mut values: Option<super::Restriction>,
            fallback: Option<<super::Restriction as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Restriction>,
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
                SimpleBaseTypeContentDeserializer::store_restriction(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleBaseTypeContentDeserializer::store_restriction(&mut values, data)?;
                    let data = SimpleBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Restriction(values, None, None),
                    )?;
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
            mut values: Option<super::List>,
            fallback: Option<<super::List as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::List>,
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
                SimpleBaseTypeContentDeserializer::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleBaseTypeContentDeserializer::store_list(&mut values, data)?;
                    let data = SimpleBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::List(values, None, None),
                    )?;
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
            mut values: Option<super::Union>,
            fallback: Option<<super::Union as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Union>,
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
                SimpleBaseTypeContentDeserializer::store_union_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleBaseTypeContentDeserializer::store_union_(&mut values, data)?;
                    let data = SimpleBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Union(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::SimpleBaseTypeContent>
        for Box<SimpleBaseTypeContentDeserializer>
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleBaseTypeContent> {
            let deserializer = Box::new(SimpleBaseTypeContentDeserializer {
                state__: Box::new(SimpleBaseTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                SimpleBaseTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
            SimpleBaseTypeContentDeserializer::finish_state(helper, *self.state__)
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
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
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
            }))
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
    impl<'de> Deserializer<'de, super::ComplexBaseType> for Box<ComplexBaseTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseType> {
            helper.init_deserializer_from_start_event(
                event,
                ComplexBaseTypeDeserializer::from_bytes_start,
            )
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
        ),
        SimpleContent(
            Option<super::SimpleContent>,
            Option<<super::SimpleContent as WithDeserializer>::Deserializer>,
            Option<<super::SimpleContent as WithDeserializer>::Deserializer>,
        ),
        ComplexContent(
            Option<super::ComplexContent>,
            Option<<super::ComplexContent as WithDeserializer>::Deserializer>,
            Option<<super::ComplexContent as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContent>,
            Option<<super::OpenContent as WithDeserializer>::Deserializer>,
            Option<<super::OpenContent as WithDeserializer>::Deserializer>,
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
            Option<super::AnyAttribute>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"simpleContent")
                ) {
                    let output = <super::SimpleContent as WithDeserializer>::init(helper, event)?;
                    return self.handle_simple_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"complexContent")
                ) {
                    let output = <super::ComplexContent as WithDeserializer>::init(helper, event)?;
                    return self.handle_complex_content(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"openContent")
                ) {
                    let output = <super::OpenContent as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::AnyAttribute as WithDeserializer>::init(helper, event)?;
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
                        ComplexBaseTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_simple_content(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::ComplexBaseTypeContent::SimpleContent(
                        helper.finish_element("simpleContent", values)?,
                    ))
                }
                S::ComplexContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_complex_content(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::ComplexBaseTypeContent::ComplexContent(
                        helper.finish_element("complexContent", values)?,
                    ))
                }
                S::OpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_open_content(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::OpenContent(
                        helper.finish_element("openContent", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_all(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_choice(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_sequence(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_attribute(&mut values, value)?;
                    }
                    Ok(super::ComplexBaseTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_attribute_group(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::ComplexBaseTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_any_attribute(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::ComplexBaseTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Assert(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexBaseTypeContentDeserializer::store_assert(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::SimpleContent>,
            value: super::SimpleContent,
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
            values: &mut Option<super::ComplexContent>,
            value: super::ComplexContent,
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
            values: &mut Option<super::OpenContent>,
            value: super::OpenContent,
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
            values: &mut Option<super::AnyAttribute>,
            value: super::AnyAttribute,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                ComplexBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            mut values: Option<super::SimpleContent>,
            fallback: Option<<super::SimpleContent as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SimpleContent>,
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
                ComplexBaseTypeContentDeserializer::store_simple_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_simple_content(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::SimpleContent(values, None, None),
                    )?;
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
            mut values: Option<super::ComplexContent>,
            fallback: Option<<super::ComplexContent as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ComplexContent>,
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
                ComplexBaseTypeContentDeserializer::store_complex_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_complex_content(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::ComplexContent(values, None, None),
                    )?;
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
            mut values: Option<super::OpenContent>,
            fallback: Option<<super::OpenContent as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpenContent>,
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
                ComplexBaseTypeContentDeserializer::store_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_open_content(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::OpenContent(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_group(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_all(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::All(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_choice(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Choice(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_sequence(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Sequence(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_attribute(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Attribute(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
            mut values: Option<super::AnyAttribute>,
            fallback: Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttribute>,
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
                ComplexBaseTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::AnyAttribute(values, None, None),
                    )?;
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
                ComplexBaseTypeContentDeserializer::store_assert(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexBaseTypeContentDeserializer::store_assert(&mut values, data)?;
                    let data = ComplexBaseTypeContentDeserializer::finish_state(
                        helper,
                        S::Assert(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::ComplexBaseTypeContent>
        for Box<ComplexBaseTypeContentDeserializer>
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexBaseTypeContent> {
            let deserializer = Box::new(ComplexBaseTypeContentDeserializer {
                state__: Box::new(ComplexBaseTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                ComplexBaseTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
            ComplexBaseTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct GroupTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<QName>,
        min_occurs: usize,
        max_occurs: MaxOccurs,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<QName> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<MaxOccurs> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                name: name,
                ref_: ref_,
                min_occurs: min_occurs.unwrap_or_else(super::GroupType::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::GroupType::default_max_occurs),
                content: Vec::new(),
                state__: Box::new(GroupTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::GroupType> for Box<GroupTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupType> {
            helper
                .init_deserializer_from_start_event(event, GroupTypeDeserializer::from_bytes_start)
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
            Option<super::Any>,
            Option<<super::Any as WithDeserializer>::Deserializer>,
            Option<<super::Any as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::Any as WithDeserializer>::init(helper, event)?;
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
                        GroupTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Element(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        GroupTypeContentDeserializer::store_element(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Element(
                        helper.finish_element("element", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        GroupTypeContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        GroupTypeContentDeserializer::store_all(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        GroupTypeContentDeserializer::store_choice(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        GroupTypeContentDeserializer::store_sequence(&mut values, value)?;
                    }
                    Ok(super::GroupTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::Any(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        GroupTypeContentDeserializer::store_any(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
        fn store_any(values: &mut Option<super::Any>, value: super::Any) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                GroupTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
                GroupTypeContentDeserializer::store_element(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_element(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Element(values, None, None),
                    )?;
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
                GroupTypeContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_group(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
                GroupTypeContentDeserializer::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_all(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::All(values, None, None),
                    )?;
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
                GroupTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_choice(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Choice(values, None, None),
                    )?;
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
                GroupTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_sequence(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Sequence(values, None, None),
                    )?;
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
            mut values: Option<super::Any>,
            fallback: Option<<super::Any as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Any>,
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
                GroupTypeContentDeserializer::store_any(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    GroupTypeContentDeserializer::store_any(&mut values, data)?;
                    let data = GroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Any(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::GroupTypeContent> for Box<GroupTypeContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupTypeContent> {
            let deserializer = Box::new(GroupTypeContentDeserializer {
                state__: Box::new(GroupTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                GroupTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
            GroupTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AttributeGroupTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<QName>,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                name: name,
                ref_: ref_,
                content: Vec::new(),
                state__: Box::new(AttributeGroupTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::AttributeGroupType> for Box<AttributeGroupTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupType> {
            helper.init_deserializer_from_start_event(
                event,
                AttributeGroupTypeDeserializer::from_bytes_start,
            )
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
            Option<super::AnyAttribute>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::AnyAttribute as WithDeserializer>::init(helper, event)?;
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
                        AttributeGroupTypeContentDeserializer::store_annotation(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::AttributeGroupTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AttributeGroupTypeContentDeserializer::store_attribute(&mut values, value)?;
                    }
                    Ok(super::AttributeGroupTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AttributeGroupTypeContentDeserializer::store_attribute_group(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::AttributeGroupTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AttributeGroupTypeContentDeserializer::store_any_attribute(
                            &mut values,
                            value,
                        )?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::AnyAttribute>,
            value: super::AnyAttribute,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                AttributeGroupTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AttributeGroupTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = AttributeGroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
                AttributeGroupTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AttributeGroupTypeContentDeserializer::store_attribute(&mut values, data)?;
                    let data = AttributeGroupTypeContentDeserializer::finish_state(
                        helper,
                        S::Attribute(values, None, None),
                    )?;
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
                AttributeGroupTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AttributeGroupTypeContentDeserializer::store_attribute_group(
                        &mut values,
                        data,
                    )?;
                    let data = AttributeGroupTypeContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
            mut values: Option<super::AnyAttribute>,
            fallback: Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttribute>,
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
                AttributeGroupTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AttributeGroupTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                    let data = AttributeGroupTypeContentDeserializer::finish_state(
                        helper,
                        S::AnyAttribute(values, None, None),
                    )?;
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
        for Box<AttributeGroupTypeContentDeserializer>
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeGroupTypeContent> {
            let deserializer = Box::new(AttributeGroupTypeContentDeserializer {
                state__: Box::new(AttributeGroupTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                AttributeGroupTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
            AttributeGroupTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ElementTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<QName>,
        type_: Option<QName>,
        substitution_group: Option<super::QNameList>,
        min_occurs: usize,
        max_occurs: MaxOccurs,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<QName> = None;
            let mut type_: Option<QName> = None;
            let mut substitution_group: Option<super::QNameList> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<MaxOccurs> = None;
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
                }
            }
            Ok(Box::new(Self {
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
            }))
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
    impl<'de> Deserializer<'de, super::ElementType> for Box<ElementTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementType> {
            helper.init_deserializer_from_start_event(
                event,
                ElementTypeDeserializer::from_bytes_start,
            )
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
            Option<super::Keyref>,
            Option<<super::Keyref as WithDeserializer>::Deserializer>,
            Option<<super::Keyref as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::Keyref as WithDeserializer>::init(helper, event)?;
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
                        ElementTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ElementTypeContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ElementTypeContentDeserializer::store_complex_type(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::ComplexType(
                        helper.finish_element("complexType", values)?,
                    ))
                }
                S::Alternative(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ElementTypeContentDeserializer::store_alternative(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Alternative(
                        helper.finish_element("alternative", values)?,
                    ))
                }
                S::Unique(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ElementTypeContentDeserializer::store_unique(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Unique(
                        helper.finish_element("unique", values)?,
                    ))
                }
                S::Key(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ElementTypeContentDeserializer::store_key(&mut values, value)?;
                    }
                    Ok(super::ElementTypeContent::Key(
                        helper.finish_element("key", values)?,
                    ))
                }
                S::Keyref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ElementTypeContentDeserializer::store_keyref(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::Keyref>,
            value: super::Keyref,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                ElementTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
                ElementTypeContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
                ElementTypeContentDeserializer::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_complex_type(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::ComplexType(values, None, None),
                    )?;
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
                ElementTypeContentDeserializer::store_alternative(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_alternative(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::Alternative(values, None, None),
                    )?;
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
                ElementTypeContentDeserializer::store_unique(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_unique(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::Unique(values, None, None),
                    )?;
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
                ElementTypeContentDeserializer::store_key(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_key(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::Key(values, None, None),
                    )?;
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
            mut values: Option<super::Keyref>,
            fallback: Option<<super::Keyref as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Keyref>,
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
                ElementTypeContentDeserializer::store_keyref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ElementTypeContentDeserializer::store_keyref(&mut values, data)?;
                    let data = ElementTypeContentDeserializer::finish_state(
                        helper,
                        S::Keyref(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::ElementTypeContent> for Box<ElementTypeContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementTypeContent> {
            let deserializer = Box::new(ElementTypeContentDeserializer {
                state__: Box::new(ElementTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                ElementTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
            ElementTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AttributeTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<QName>,
        type_: Option<QName>,
        use_: super::AttributeUseType,
        default: Option<String>,
        fixed: Option<String>,
        form: Option<super::FormChoiceType>,
        target_namespace: Option<String>,
        inheritable: Option<bool>,
        annotation: Option<super::Annotation>,
        simple_type: Option<super::SimpleBaseType>,
        state__: Box<AttributeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AttributeTypeDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AttributeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<QName> = None;
            let mut type_: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
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
            }))
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
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
    impl<'de> Deserializer<'de, super::AttributeType> for Box<AttributeTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeType> {
            helper.init_deserializer_from_start_event(
                event,
                AttributeTypeDeserializer::from_bytes_start,
            )
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
                            false,
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
        state__: Box<NotationDeserializerState>,
    }
    #[derive(Debug)]
    enum NotationDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NotationDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                public: public,
                system: system,
                annotation: None,
                state__: Box::new(NotationDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NotationDeserializerState,
        ) -> Result<(), Error> {
            use NotationDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<NotationDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NotationDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Notation> for Box<NotationDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Notation> {
            helper.init_deserializer_from_start_event(event, NotationDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Notation> {
            use NotationDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Notation, Error> {
            let state = replace(&mut *self.state__, NotationDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
    pub struct WildcardTypeDeserializer {
        id: Option<String>,
        namespace: Option<super::NamespaceListType>,
        not_namespace: Option<super::NotNamespaceType>,
        process_contents: super::ProcessContentsType,
        annotation: Option<super::Annotation>,
        state__: Box<WildcardTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WildcardTypeDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WildcardTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::WildcardType::default_process_contents),
                annotation: None,
                state__: Box::new(WildcardTypeDeserializerState::Init__),
            }))
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
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
    impl<'de> Deserializer<'de, super::WildcardType> for Box<WildcardTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WildcardType> {
            helper.init_deserializer_from_start_event(
                event,
                WildcardTypeDeserializer::from_bytes_start,
            )
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
                            false,
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
        base: Option<QName>,
        content: Vec<super::RestrictionContent>,
        state__: Box<RestrictionDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictionDeserializerState {
        Init__,
        Next__,
        Content__(<super::RestrictionContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RestrictionDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut base: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                base: base,
                content: Vec::new(),
                state__: Box::new(RestrictionDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RestrictionDeserializerState,
        ) -> Result<(), Error> {
            if let RestrictionDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RestrictionContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RestrictionContent>,
            fallback: &mut Option<RestrictionDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Restriction> for Box<RestrictionDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Restriction> {
            helper.init_deserializer_from_start_event(
                event,
                RestrictionDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Restriction> {
            use RestrictionDeserializerState as S;
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
                            <super::RestrictionContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Restriction, Error> {
            let state = replace(&mut *self.state__, RestrictionDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::Restriction {
                id: self.id,
                base: self.base,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RestrictionContentDeserializer {
        state__: Box<RestrictionContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RestrictionContentDeserializerState {
        Init__,
        Annotation(
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
        Done__(super::RestrictionContent),
        Unknown__,
    }
    impl RestrictionContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
                        ElementHandlerOutput::Continue { event, .. } => event,
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = RestrictionContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RestrictionContentDeserializerState,
        ) -> Result<super::RestrictionContent, Error> {
            use RestrictionContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RestrictionContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RestrictionContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::Facet(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionContentDeserializer::store_facet(&mut values, value)?;
                    }
                    Ok(super::RestrictionContent::Facet(
                        helper.finish_element("facet", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_annotation(
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RestrictionContentDeserializerState as S;
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
                RestrictionContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionContentDeserializer::store_annotation(&mut values, data)?;
                    let data = RestrictionContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            use RestrictionContentDeserializerState as S;
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
                RestrictionContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = RestrictionContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
            use RestrictionContentDeserializerState as S;
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
                RestrictionContentDeserializer::store_facet(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionContentDeserializer::store_facet(&mut values, data)?;
                    let data = RestrictionContentDeserializer::finish_state(
                        helper,
                        S::Facet(values, None, None),
                    )?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Facet(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RestrictionContent> for Box<RestrictionContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionContent> {
            let deserializer = Box::new(RestrictionContentDeserializer {
                state__: Box::new(RestrictionContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RestrictionContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::RestrictionContent> {
            use RestrictionContentDeserializerState as S;
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
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(
                                RestrictionContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
        ) -> Result<super::RestrictionContent, Error> {
            RestrictionContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ListDeserializer {
        id: Option<String>,
        item_type: Option<QName>,
        annotation: Option<super::Annotation>,
        simple_type: Option<super::SimpleBaseType>,
        state__: Box<ListDeserializerState>,
    }
    #[derive(Debug)]
    enum ListDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ListDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut item_type: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                item_type: item_type,
                annotation: None,
                simple_type: None,
                state__: Box::new(ListDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ListDeserializerState,
        ) -> Result<(), Error> {
            use ListDeserializerState as S;
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<ListDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListDeserializerState as S;
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
            fallback: &mut Option<ListDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::List> for Box<ListDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::List> {
            helper.init_deserializer_from_start_event(event, ListDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::List> {
            use ListDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::List, Error> {
            let state = replace(&mut *self.state__, ListDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::List {
                id: self.id,
                item_type: self.item_type,
                annotation: self.annotation,
                simple_type: self.simple_type.map(Box::new),
            })
        }
    }
    #[derive(Debug)]
    pub struct UnionDeserializer {
        id: Option<String>,
        member_types: Option<super::QNameList>,
        annotation: Option<super::Annotation>,
        simple_type: Vec<super::SimpleBaseType>,
        state__: Box<UnionDeserializerState>,
    }
    #[derive(Debug)]
    enum UnionDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UnionDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut member_types: Option<super::QNameList> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                member_types: member_types,
                annotation: None,
                simple_type: Vec::new(),
                state__: Box::new(UnionDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: UnionDeserializerState,
        ) -> Result<(), Error> {
            use UnionDeserializerState as S;
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<UnionDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnionDeserializerState as S;
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
            fallback: &mut Option<UnionDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnionDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Union> for Box<UnionDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Union> {
            helper.init_deserializer_from_start_event(event, UnionDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Union> {
            use UnionDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Union, Error> {
            let state = replace(&mut *self.state__, UnionDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        state__: Box<SimpleContentDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleContentDeserializerState {
        Init__,
        Next__,
        Content__(<super::SimpleContentContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SimpleContentDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_XS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Box::new(Self {
                id: id,
                content: Vec::new(),
                state__: Box::new(SimpleContentDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SimpleContentDeserializerState,
        ) -> Result<(), Error> {
            if let SimpleContentDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SimpleContentContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SimpleContentContent>,
            fallback: &mut Option<SimpleContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::SimpleContent> for Box<SimpleContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContent> {
            helper.init_deserializer_from_start_event(
                event,
                SimpleContentDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContent> {
            use SimpleContentDeserializerState as S;
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
                            <super::SimpleContentContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SimpleContent, Error> {
            let state = replace(
                &mut *self.state__,
                SimpleContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SimpleContent {
                id: self.id,
                content: helper.finish_vec(1usize, Some(2usize), self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SimpleContentContentDeserializer {
        state__: Box<SimpleContentContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SimpleContentContentDeserializerState {
        Init__,
        Annotation(
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
        Done__(super::SimpleContentContent),
        Unknown__,
    }
    impl SimpleContentContentDeserializer {
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
            *self.state__ = SimpleContentContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SimpleContentContentDeserializerState,
        ) -> Result<super::SimpleContentContent, Error> {
            use SimpleContentContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SimpleContentContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::SimpleContentContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Restriction(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SimpleContentContentDeserializer::store_restriction(&mut values, value)?;
                    }
                    Ok(super::SimpleContentContent::Restriction(
                        helper.finish_element("restriction", values)?,
                    ))
                }
                S::Extension(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        SimpleContentContentDeserializer::store_extension(&mut values, value)?;
                    }
                    Ok(super::SimpleContentContent::Extension(
                        helper.finish_element("extension", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_annotation(
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SimpleContentContentDeserializerState as S;
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
                SimpleContentContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleContentContentDeserializer::store_annotation(&mut values, data)?;
                    let data = SimpleContentContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            use SimpleContentContentDeserializerState as S;
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
                SimpleContentContentDeserializer::store_restriction(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleContentContentDeserializer::store_restriction(&mut values, data)?;
                    let data = SimpleContentContentDeserializer::finish_state(
                        helper,
                        S::Restriction(values, None, None),
                    )?;
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
            use SimpleContentContentDeserializerState as S;
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
                SimpleContentContentDeserializer::store_extension(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    SimpleContentContentDeserializer::store_extension(&mut values, data)?;
                    let data = SimpleContentContentDeserializer::finish_state(
                        helper,
                        S::Extension(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::SimpleContentContent> for Box<SimpleContentContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleContentContent> {
            let deserializer = Box::new(SimpleContentContentDeserializer {
                state__: Box::new(SimpleContentContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, SimpleContentContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::SimpleContentContent> {
            use SimpleContentContentDeserializerState as S;
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
                            artifact: DeserializerArtifact::Data(
                                SimpleContentContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
        ) -> Result<super::SimpleContentContent, Error> {
            SimpleContentContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ComplexContentDeserializer {
        id: Option<String>,
        mixed: Option<bool>,
        content: Vec<super::ComplexContentContent>,
        state__: Box<ComplexContentDeserializerState>,
    }
    #[derive(Debug)]
    enum ComplexContentDeserializerState {
        Init__,
        Next__,
        Content__(<super::ComplexContentContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ComplexContentDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                mixed: mixed,
                content: Vec::new(),
                state__: Box::new(ComplexContentDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ComplexContentDeserializerState,
        ) -> Result<(), Error> {
            if let ComplexContentDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ComplexContentContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ComplexContentContent>,
            fallback: &mut Option<ComplexContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ComplexContent> for Box<ComplexContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContent> {
            helper.init_deserializer_from_start_event(
                event,
                ComplexContentDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContent> {
            use ComplexContentDeserializerState as S;
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
                        let output = <super::ComplexContentContent as WithDeserializer>::init(
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
        ) -> Result<super::ComplexContent, Error> {
            let state = replace(
                &mut *self.state__,
                ComplexContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ComplexContent {
                id: self.id,
                mixed: self.mixed,
                content: helper.finish_vec(1usize, Some(2usize), self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ComplexContentContentDeserializer {
        state__: Box<ComplexContentContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ComplexContentContentDeserializerState {
        Init__,
        Annotation(
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
        Done__(super::ComplexContentContent),
        Unknown__,
    }
    impl ComplexContentContentDeserializer {
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
            *self.state__ = ComplexContentContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ComplexContentContentDeserializerState,
        ) -> Result<super::ComplexContentContent, Error> {
            use ComplexContentContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Annotation(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexContentContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ComplexContentContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::Restriction(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexContentContentDeserializer::store_restriction(&mut values, value)?;
                    }
                    Ok(super::ComplexContentContent::Restriction(
                        helper.finish_element("restriction", values)?,
                    ))
                }
                S::Extension(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ComplexContentContentDeserializer::store_extension(&mut values, value)?;
                    }
                    Ok(super::ComplexContentContent::Extension(
                        helper.finish_element("extension", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_annotation(
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ComplexContentContentDeserializerState as S;
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
                ComplexContentContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexContentContentDeserializer::store_annotation(&mut values, data)?;
                    let data = ComplexContentContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            use ComplexContentContentDeserializerState as S;
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
                ComplexContentContentDeserializer::store_restriction(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexContentContentDeserializer::store_restriction(&mut values, data)?;
                    let data = ComplexContentContentDeserializer::finish_state(
                        helper,
                        S::Restriction(values, None, None),
                    )?;
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
            use ComplexContentContentDeserializerState as S;
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
                ComplexContentContentDeserializer::store_extension(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ComplexContentContentDeserializer::store_extension(&mut values, data)?;
                    let data = ComplexContentContentDeserializer::finish_state(
                        helper,
                        S::Extension(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::ComplexContentContent>
        for Box<ComplexContentContentDeserializer>
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ComplexContentContent> {
            let deserializer = Box::new(ComplexContentContentDeserializer {
                state__: Box::new(ComplexContentContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ComplexContentContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ComplexContentContent> {
            use ComplexContentContentDeserializerState as S;
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
                            artifact: DeserializerArtifact::Data(
                                ComplexContentContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
        ) -> Result<super::ComplexContentContent, Error> {
            ComplexContentContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct OpenContentDeserializer {
        id: Option<String>,
        mode: super::OpenContentModeType,
        annotation: Option<super::Annotation>,
        any: Option<super::WildcardType>,
        state__: Box<OpenContentDeserializerState>,
    }
    #[derive(Debug)]
    enum OpenContentDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Any(Option<<super::WildcardType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl OpenContentDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                mode: mode.unwrap_or_else(super::OpenContent::default_mode),
                annotation: None,
                any: None,
                state__: Box::new(OpenContentDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OpenContentDeserializerState,
        ) -> Result<(), Error> {
            use OpenContentDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<OpenContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpenContentDeserializerState as S;
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
            fallback: &mut Option<OpenContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpenContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::OpenContent> for Box<OpenContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpenContent> {
            helper.init_deserializer_from_start_event(
                event,
                OpenContentDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpenContent> {
            use OpenContentDeserializerState as S;
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
                            false,
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::OpenContent, Error> {
            let state = replace(&mut *self.state__, OpenContentDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        state__: Box<AnyAttributeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyAttributeDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyAttributeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::AnyAttribute::default_process_contents),
                not_q_name: not_q_name,
                annotation: None,
                state__: Box::new(AnyAttributeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnyAttributeDeserializerState,
        ) -> Result<(), Error> {
            use AnyAttributeDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<AnyAttributeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnyAttributeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::AnyAttribute> for Box<AnyAttributeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyAttribute> {
            helper.init_deserializer_from_start_event(
                event,
                AnyAttributeDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyAttribute> {
            use AnyAttributeDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnyAttribute, Error> {
            let state = replace(&mut *self.state__, AnyAttributeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        state__: Box<AssertionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AssertionTypeDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AssertionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                test: test,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state__: Box::new(AssertionTypeDeserializerState::Init__),
            }))
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
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
    impl<'de> Deserializer<'de, super::AssertionType> for Box<AssertionTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AssertionType> {
            helper.init_deserializer_from_start_event(
                event,
                AssertionTypeDeserializer::from_bytes_start,
            )
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
                            false,
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
        max_occurs: MaxOccurs,
        annotation: Option<super::Annotation>,
        state__: Box<AnyDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut namespace: Option<super::NamespaceListType> = None;
            let mut not_namespace: Option<super::NotNamespaceType> = None;
            let mut process_contents: Option<super::ProcessContentsType> = None;
            let mut not_q_name: Option<super::QnameListType> = None;
            let mut min_occurs: Option<usize> = None;
            let mut max_occurs: Option<MaxOccurs> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                namespace: namespace,
                not_namespace: not_namespace,
                process_contents: process_contents
                    .unwrap_or_else(super::Any::default_process_contents),
                not_q_name: not_q_name,
                min_occurs: min_occurs.unwrap_or_else(super::Any::default_min_occurs),
                max_occurs: max_occurs.unwrap_or_else(super::Any::default_max_occurs),
                annotation: None,
                state__: Box::new(AnyDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnyDeserializerState,
        ) -> Result<(), Error> {
            use AnyDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<AnyDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnyDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Any> for Box<AnyDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Any> {
            helper.init_deserializer_from_start_event(event, AnyDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Any> {
            use AnyDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Any, Error> {
            let state = replace(&mut *self.state__, AnyDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        type_: Option<QName>,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut test: Option<String> = None;
            let mut type_: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                test: test,
                type_: type_,
                xpath_default_namespace: xpath_default_namespace,
                content: Vec::new(),
                state__: Box::new(AltTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::AltType> for Box<AltTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltType> {
            helper.init_deserializer_from_start_event(event, AltTypeDeserializer::from_bytes_start)
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
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
                        AltTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AltTypeContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::AltTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::ComplexType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        AltTypeContentDeserializer::store_complex_type(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                AltTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AltTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = AltTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
                AltTypeContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AltTypeContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = AltTypeContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
                AltTypeContentDeserializer::store_complex_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    AltTypeContentDeserializer::store_complex_type(&mut values, data)?;
                    let data = AltTypeContentDeserializer::finish_state(
                        helper,
                        S::ComplexType(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::AltTypeContent> for Box<AltTypeContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AltTypeContent> {
            let deserializer = Box::new(AltTypeContentDeserializer {
                state__: Box::new(AltTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                AltTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
            AltTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct KeybaseTypeDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<QName>,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                name: name,
                ref_: ref_,
                content: None,
                state__: Box::new(KeybaseTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::KeybaseType> for Box<KeybaseTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseType> {
            helper.init_deserializer_from_start_event(
                event,
                KeybaseTypeDeserializer::from_bytes_start,
            )
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
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeybaseTypeContentDeserializer {
        annotation: Option<super::Annotation>,
        selector: Option<super::Field>,
        field: Vec<super::Field>,
        state__: Box<KeybaseTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum KeybaseTypeContentDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Selector(Option<<super::Field as WithDeserializer>::Deserializer>),
        Field(Option<<super::Field as WithDeserializer>::Deserializer>),
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_selector(&mut self, value: super::Field) -> Result<(), Error> {
            if self.selector.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"selector",
                )))?;
            }
            self.selector = Some(value);
            Ok(())
        }
        fn store_field(&mut self, value: super::Field) -> Result<(), Error> {
            self.field.push(value);
            Ok(())
        }
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Annotation>,
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
            output: DeserializerOutput<'de, super::Field>,
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
            output: DeserializerOutput<'de, super::Field>,
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
    impl<'de> Deserializer<'de, super::KeybaseTypeContent> for Box<KeybaseTypeContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeybaseTypeContent> {
            let deserializer = Box::new(KeybaseTypeContentDeserializer {
                annotation: None,
                selector: None,
                field: Vec::new(),
                state__: Box::new(KeybaseTypeContentDeserializerState::Init__),
            });
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
                            false,
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
                            false,
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
                            false,
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
    pub struct KeyrefDeserializer {
        id: Option<String>,
        name: Option<String>,
        ref_: Option<QName>,
        refer: Option<QName>,
        content: Option<super::KeyrefContent>,
        state__: Box<KeyrefDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyrefDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyrefContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyrefDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut ref_: Option<QName> = None;
            let mut refer: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                name: name,
                ref_: ref_,
                refer: refer,
                content: None,
                state__: Box::new(KeyrefDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeyrefDeserializerState,
        ) -> Result<(), Error> {
            if let KeyrefDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::KeyrefContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::KeyrefContent>,
            fallback: &mut Option<KeyrefDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Keyref> for Box<KeyrefDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Keyref> {
            helper.init_deserializer_from_start_event(event, KeyrefDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Keyref> {
            use KeyrefDeserializerState as S;
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
                            <super::KeyrefContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Keyref, Error> {
            let state = replace(&mut *self.state__, KeyrefDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::Keyref {
                id: self.id,
                name: self.name,
                ref_: self.ref_,
                refer: self.refer,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyrefContentDeserializer {
        annotation: Option<super::Annotation>,
        selector: Option<super::Field>,
        field: Vec<super::Field>,
        state__: Box<KeyrefContentDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyrefContentDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Selector(Option<<super::Field as WithDeserializer>::Deserializer>),
        Field(Option<<super::Field as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl KeyrefContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeyrefContentDeserializerState,
        ) -> Result<(), Error> {
            use KeyrefContentDeserializerState as S;
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
            if self.annotation.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"annotation",
                )))?;
            }
            self.annotation = Some(value);
            Ok(())
        }
        fn store_selector(&mut self, value: super::Field) -> Result<(), Error> {
            if self.selector.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"selector",
                )))?;
            }
            self.selector = Some(value);
            Ok(())
        }
        fn store_field(&mut self, value: super::Field) -> Result<(), Error> {
            self.field.push(value);
            Ok(())
        }
        fn handle_annotation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<KeyrefContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefContentDeserializerState as S;
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
            output: DeserializerOutput<'de, super::Field>,
            fallback: &mut Option<KeyrefContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefContentDeserializerState as S;
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
            output: DeserializerOutput<'de, super::Field>,
            fallback: &mut Option<KeyrefContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyrefContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::KeyrefContent> for Box<KeyrefContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyrefContent> {
            let deserializer = Box::new(KeyrefContentDeserializer {
                annotation: None,
                selector: None,
                field: Vec::new(),
                state__: Box::new(KeyrefContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, KeyrefContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::KeyrefContent> {
            use KeyrefContentDeserializerState as S;
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
                            false,
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
                            false,
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::KeyrefContent, Error> {
            let state = replace(
                &mut *self.state__,
                KeyrefContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::KeyrefContent {
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
                        FacetDeserializer::store_min_exclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MinExclusive(
                        helper.finish_element("minExclusive", values)?,
                    ))
                }
                S::MinInclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_min_inclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MinInclusive(
                        helper.finish_element("minInclusive", values)?,
                    ))
                }
                S::MaxExclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_max_exclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxExclusive(
                        helper.finish_element("maxExclusive", values)?,
                    ))
                }
                S::MaxInclusive(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_max_inclusive(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxInclusive(
                        helper.finish_element("maxInclusive", values)?,
                    ))
                }
                S::TotalDigits(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_total_digits(&mut values, value)?;
                    }
                    Ok(super::Facet::TotalDigits(
                        helper.finish_element("totalDigits", values)?,
                    ))
                }
                S::FractionDigits(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_fraction_digits(&mut values, value)?;
                    }
                    Ok(super::Facet::FractionDigits(
                        helper.finish_element("fractionDigits", values)?,
                    ))
                }
                S::Length(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_length(&mut values, value)?;
                    }
                    Ok(super::Facet::Length(
                        helper.finish_element("length", values)?,
                    ))
                }
                S::MinLength(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_min_length(&mut values, value)?;
                    }
                    Ok(super::Facet::MinLength(
                        helper.finish_element("minLength", values)?,
                    ))
                }
                S::MaxLength(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_max_length(&mut values, value)?;
                    }
                    Ok(super::Facet::MaxLength(
                        helper.finish_element("maxLength", values)?,
                    ))
                }
                S::Enumeration(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_enumeration(&mut values, value)?;
                    }
                    Ok(super::Facet::Enumeration(
                        helper.finish_element("enumeration", values)?,
                    ))
                }
                S::WhiteSpace(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_white_space(&mut values, value)?;
                    }
                    Ok(super::Facet::WhiteSpace(
                        helper.finish_element("whiteSpace", values)?,
                    ))
                }
                S::Pattern(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_pattern(&mut values, value)?;
                    }
                    Ok(super::Facet::Pattern(
                        helper.finish_element("pattern", values)?,
                    ))
                }
                S::Assertion(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_assertion(&mut values, value)?;
                    }
                    Ok(super::Facet::Assertion(
                        helper.finish_element("assertion", values)?,
                    ))
                }
                S::ExplicitTimezone(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        FacetDeserializer::store_explicit_timezone(&mut values, value)?;
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
                FacetDeserializer::store_min_exclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_min_exclusive(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::MinExclusive(values, None, None),
                    )?;
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
                FacetDeserializer::store_min_inclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_min_inclusive(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::MinInclusive(values, None, None),
                    )?;
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
                FacetDeserializer::store_max_exclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_max_exclusive(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::MaxExclusive(values, None, None),
                    )?;
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
                FacetDeserializer::store_max_inclusive(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_max_inclusive(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::MaxInclusive(values, None, None),
                    )?;
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
                FacetDeserializer::store_total_digits(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_total_digits(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::TotalDigits(values, None, None),
                    )?;
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
                FacetDeserializer::store_fraction_digits(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_fraction_digits(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::FractionDigits(values, None, None),
                    )?;
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
                FacetDeserializer::store_length(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_length(&mut values, data)?;
                    let data =
                        FacetDeserializer::finish_state(helper, S::Length(values, None, None))?;
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
                FacetDeserializer::store_min_length(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_min_length(&mut values, data)?;
                    let data =
                        FacetDeserializer::finish_state(helper, S::MinLength(values, None, None))?;
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
                FacetDeserializer::store_max_length(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_max_length(&mut values, data)?;
                    let data =
                        FacetDeserializer::finish_state(helper, S::MaxLength(values, None, None))?;
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
                FacetDeserializer::store_enumeration(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_enumeration(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::Enumeration(values, None, None),
                    )?;
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
                FacetDeserializer::store_white_space(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_white_space(&mut values, data)?;
                    let data =
                        FacetDeserializer::finish_state(helper, S::WhiteSpace(values, None, None))?;
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
                FacetDeserializer::store_pattern(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_pattern(&mut values, data)?;
                    let data =
                        FacetDeserializer::finish_state(helper, S::Pattern(values, None, None))?;
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
                FacetDeserializer::store_assertion(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_assertion(&mut values, data)?;
                    let data =
                        FacetDeserializer::finish_state(helper, S::Assertion(values, None, None))?;
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
                FacetDeserializer::store_explicit_timezone(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    FacetDeserializer::store_explicit_timezone(&mut values, data)?;
                    let data = FacetDeserializer::finish_state(
                        helper,
                        S::ExplicitTimezone(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::Facet> for Box<FacetDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Facet> {
            let deserializer = Box::new(FacetDeserializer {
                state__: Box::new(FacetDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(FacetDeserializer::finish_state(
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
            FacetDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct RestrictionTypeDeserializer {
        id: Option<String>,
        base: QName,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut base: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                base: base.ok_or_else(|| ErrorKind::MissingAttribute("base".into()))?,
                content: Vec::new(),
                state__: Box::new(RestrictionTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::RestrictionType> for Box<RestrictionTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionType> {
            helper.init_deserializer_from_start_event(
                event,
                RestrictionTypeDeserializer::from_bytes_start,
            )
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContent>,
            Option<<super::OpenContent as WithDeserializer>::Deserializer>,
            Option<<super::OpenContent as WithDeserializer>::Deserializer>,
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
            Option<super::AnyAttribute>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
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
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"annotation")
                ) {
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"openContent")
                ) {
                    let output = <super::OpenContent as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::AnyAttribute as WithDeserializer>::init(helper, event)?;
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
                        ElementHandlerOutput::Continue { event, .. } => event,
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = RestrictionTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
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
                        RestrictionTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::OpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_open_content(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::OpenContent(
                        helper.finish_element("openContent", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_all(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_choice(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_sequence(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::SimpleType(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_simple_type(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::SimpleType(
                        helper.finish_element("simpleType", values)?,
                    ))
                }
                S::Facet(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_facet(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Facet(
                        helper.finish_element("facet", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_attribute(&mut values, value)?;
                    }
                    Ok(super::RestrictionTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_attribute_group(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::RestrictionTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_any_attribute(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::RestrictionTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Assert(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        RestrictionTypeContentDeserializer::store_assert(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::OpenContent>,
            value: super::OpenContent,
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
            values: &mut Option<super::AnyAttribute>,
            value: super::AnyAttribute,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                RestrictionTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            mut values: Option<super::OpenContent>,
            fallback: Option<<super::OpenContent as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpenContent>,
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
                RestrictionTypeContentDeserializer::store_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_open_content(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::OpenContent(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_group(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_all(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::All(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_choice(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Choice(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_sequence(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Sequence(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_simple_type(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_simple_type(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::SimpleType(values, None, None),
                    )?;
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
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                RestrictionTypeContentDeserializer::store_facet(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_facet(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Facet(values, None, None),
                    )?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Facet(values, None, Some(deserializer));
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
                RestrictionTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_attribute(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Attribute(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
            mut values: Option<super::AnyAttribute>,
            fallback: Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttribute>,
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
                RestrictionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::AnyAttribute(values, None, None),
                    )?;
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
                RestrictionTypeContentDeserializer::store_assert(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    RestrictionTypeContentDeserializer::store_assert(&mut values, data)?;
                    let data = RestrictionTypeContentDeserializer::finish_state(
                        helper,
                        S::Assert(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::RestrictionTypeContent>
        for Box<RestrictionTypeContentDeserializer>
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictionTypeContent> {
            let deserializer = Box::new(RestrictionTypeContentDeserializer {
                state__: Box::new(RestrictionTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                RestrictionTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                        S::Attribute(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_XS),
                            b"attribute",
                            false,
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
                            false,
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
                            false,
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
                            false,
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
            RestrictionTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ExtensionTypeDeserializer {
        id: Option<String>,
        base: QName,
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
        ) -> Result<Box<Self>, Error> {
            let mut id: Option<String> = None;
            let mut base: Option<QName> = None;
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                base: base.ok_or_else(|| ErrorKind::MissingAttribute("base".into()))?,
                content: Vec::new(),
                state__: Box::new(ExtensionTypeDeserializerState::Init__),
            }))
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
    impl<'de> Deserializer<'de, super::ExtensionType> for Box<ExtensionTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionType> {
            helper.init_deserializer_from_start_event(
                event,
                ExtensionTypeDeserializer::from_bytes_start,
            )
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
            Option<super::Annotation>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
            Option<<super::Annotation as WithDeserializer>::Deserializer>,
        ),
        OpenContent(
            Option<super::OpenContent>,
            Option<<super::OpenContent as WithDeserializer>::Deserializer>,
            Option<<super::OpenContent as WithDeserializer>::Deserializer>,
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
            Option<super::AnyAttribute>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
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
                    let output = <super::Annotation as WithDeserializer>::init(helper, event)?;
                    return self.handle_annotation(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_XS),
                    Some(b"openContent")
                ) {
                    let output = <super::OpenContent as WithDeserializer>::init(helper, event)?;
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
                    let output = <super::AnyAttribute as WithDeserializer>::init(helper, event)?;
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
                        ExtensionTypeContentDeserializer::store_annotation(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Annotation(
                        helper.finish_element("annotation", values)?,
                    ))
                }
                S::OpenContent(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_open_content(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::OpenContent(
                        helper.finish_element("openContent", values)?,
                    ))
                }
                S::Group(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_group(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Group(
                        helper.finish_element("group", values)?,
                    ))
                }
                S::All(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_all(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::All(
                        helper.finish_element("all", values)?,
                    ))
                }
                S::Choice(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_choice(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Choice(
                        helper.finish_element("choice", values)?,
                    ))
                }
                S::Sequence(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_sequence(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Sequence(
                        helper.finish_element("sequence", values)?,
                    ))
                }
                S::Attribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_attribute(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::Attribute(
                        helper.finish_element("attribute", values)?,
                    ))
                }
                S::AttributeGroup(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_attribute_group(
                            &mut values,
                            value,
                        )?;
                    }
                    Ok(super::ExtensionTypeContent::AttributeGroup(
                        helper.finish_element("attributeGroup", values)?,
                    ))
                }
                S::AnyAttribute(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_any_attribute(&mut values, value)?;
                    }
                    Ok(super::ExtensionTypeContent::AnyAttribute(
                        helper.finish_element("anyAttribute", values)?,
                    ))
                }
                S::Assert(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        ExtensionTypeContentDeserializer::store_assert(&mut values, value)?;
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
            values: &mut Option<super::Annotation>,
            value: super::Annotation,
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
            values: &mut Option<super::OpenContent>,
            value: super::OpenContent,
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
            values: &mut Option<super::AnyAttribute>,
            value: super::AnyAttribute,
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
            mut values: Option<super::Annotation>,
            fallback: Option<<super::Annotation as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Annotation>,
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
                ExtensionTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_annotation(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::Annotation(values, None, None),
                    )?;
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
            mut values: Option<super::OpenContent>,
            fallback: Option<<super::OpenContent as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpenContent>,
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
                ExtensionTypeContentDeserializer::store_open_content(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_open_content(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::OpenContent(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_group(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::Group(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_all(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_all(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::All(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_choice(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::Choice(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_sequence(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::Sequence(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_attribute(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::Attribute(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::AttributeGroup(values, None, None),
                    )?;
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
            mut values: Option<super::AnyAttribute>,
            fallback: Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyAttribute>,
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
                ExtensionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::AnyAttribute(values, None, None),
                    )?;
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
                ExtensionTypeContentDeserializer::store_assert(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    ExtensionTypeContentDeserializer::store_assert(&mut values, data)?;
                    let data = ExtensionTypeContentDeserializer::finish_state(
                        helper,
                        S::Assert(values, None, None),
                    )?;
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
    impl<'de> Deserializer<'de, super::ExtensionTypeContent> for Box<ExtensionTypeContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtensionTypeContent> {
            let deserializer = Box::new(ExtensionTypeContentDeserializer {
                state__: Box::new(ExtensionTypeContentDeserializerState::Init__),
            });
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
                            artifact: DeserializerArtifact::Data(
                                ExtensionTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
                            false,
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
            ExtensionTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct FieldDeserializer {
        id: Option<String>,
        xpath: String,
        xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
        annotation: Option<super::Annotation>,
        state__: Box<FieldDeserializerState>,
    }
    #[derive(Debug)]
    enum FieldDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FieldDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                xpath: xpath.ok_or_else(|| ErrorKind::MissingAttribute("xpath".into()))?,
                xpath_default_namespace: xpath_default_namespace,
                annotation: None,
                state__: Box::new(FieldDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FieldDeserializerState,
        ) -> Result<(), Error> {
            use FieldDeserializerState as S;
            match state {
                S::Annotation(Some(deserializer)) => {
                    self.store_annotation(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
            fallback: &mut Option<FieldDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FieldDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::Field> for Box<FieldDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Field> {
            helper.init_deserializer_from_start_event(event, FieldDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Field> {
            use FieldDeserializerState as S;
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
                            false,
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Field, Error> {
            let state = replace(&mut *self.state__, FieldDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
        state__: Box<FacetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FacetTypeDeserializerState {
        Init__,
        Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FacetTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
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
                }
            }
            Ok(Box::new(Self {
                id: id,
                value: value.ok_or_else(|| ErrorKind::MissingAttribute("value".into()))?,
                fixed: fixed.unwrap_or_else(super::FacetType::default_fixed),
                annotation: None,
                state__: Box::new(FacetTypeDeserializerState::Init__),
            }))
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
        fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
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
            output: DeserializerOutput<'de, super::Annotation>,
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
    impl<'de> Deserializer<'de, super::FacetType> for Box<FacetTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FacetType> {
            helper
                .init_deserializer_from_start_event(event, FacetTypeDeserializer::from_bytes_start)
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
                            false,
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
                id: self.id,
                value: self.value,
                fixed: self.fixed,
                annotation: self.annotation,
            })
        }
    }
}
