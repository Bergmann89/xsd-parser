pub mod quick_xml_deserialize;
use crate::{
    models::schema::{MaxOccurs, QName},
    quick_xml::{
        DeserializeBytes, DeserializeReader, Error, ErrorKind, RawByteStr, WithDeserializer,
    },
    xml::AnyElement,
};
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Include {
    pub id: Option<String>,
    pub schema_location: String,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Include {
    type Deserializer = Box<quick_xml_deserialize::IncludeDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Import {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Import {
    type Deserializer = Box<quick_xml_deserialize::ImportDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Redefine {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<RedefineContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Override {
    pub schema_location: String,
    pub id: Option<String>,
    pub content: Vec<OverrideContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Annotation {
    pub id: Option<String>,
    pub content: Vec<AnnotationContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleBaseType {
    pub id: Option<String>,
    pub final_: Option<SimpleDerivationSetType>,
    pub name: Option<String>,
    pub content: Vec<SimpleBaseTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub min_occurs: usize,
    pub max_occurs: MaxOccurs,
    pub content: Vec<GroupTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeGroupType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub content: Vec<AttributeGroupTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WildcardType {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Restriction {
    pub id: Option<String>,
    pub base: Option<QName>,
    pub content: Vec<RestrictionContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct List {
    pub id: Option<String>,
    pub item_type: Option<QName>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<SimpleBaseType>,
}
impl WithDeserializer for List {
    type Deserializer = Box<quick_xml_deserialize::ListDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Union {
    pub id: Option<String>,
    pub member_types: Option<QNameList>,
    pub annotation: Option<Annotation>,
    pub simple_type: Vec<SimpleBaseType>,
}
impl WithDeserializer for Union {
    type Deserializer = Box<quick_xml_deserialize::UnionDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleContent {
    pub id: Option<String>,
    pub content: Vec<SimpleContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComplexContent {
    pub id: Option<String>,
    pub mixed: Option<bool>,
    pub content: Vec<ComplexContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyAttribute {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssertionType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for AssertionType {
    type Deserializer = Box<quick_xml_deserialize::AssertionTypeDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Any {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QNameList(pub Vec<QName>);
impl DeserializeBytes for QNameList {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| QName::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AltType {
    pub id: Option<String>,
    pub test: Option<String>,
    pub type_: Option<QName>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub content: Vec<AltTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeybaseType {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub content: Option<KeybaseTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Keyref {
    pub id: Option<String>,
    pub name: Option<String>,
    pub ref_: Option<QName>,
    pub refer: Option<QName>,
    pub content: Option<KeyrefContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RestrictionType {
    pub id: Option<String>,
    pub base: QName,
    pub content: Vec<RestrictionTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtensionType {
    pub id: Option<String>,
    pub base: QName,
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub id: Option<String>,
    pub xpath: String,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}
impl WithDeserializer for Field {
    type Deserializer = Box<quick_xml_deserialize::FieldDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QnameListAItemType {
    QName(QName),
    Defined,
}
impl DeserializeBytes for QnameListAItemType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"##defined" => Ok(Self::Defined),
            x => Ok(Self::QName(QName::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QnameListItemType {
    QName(QName),
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
            x => Ok(Self::QName(QName::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct EntitiesType(pub Vec<String>);
impl DeserializeBytes for EntitiesType {
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
