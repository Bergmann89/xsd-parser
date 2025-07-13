pub mod cdf {
    use xsd_parser::xml::{AnyAttributes, AnyElement, Mixed, Text};
    /// This is the root element of the XCCDF document; it
    ///must appear exactly once. It encloses the entire benchmark, and contains both
    ///descriptive information and structural information. Note that the order of
    ///xccdf:Group and xccdf:Rule child elements may matter for the
    ///appearance of a generated document. xccdf:Group and xccdf:Rule
    ///children may be freely intermingled, but they must appear after any
    ///xccdf:Value children. All the other children must appear in the order
    ///shown.
    pub type Benchmark = BenchmarkElementType;
    #[derive(Debug)]
    pub struct BenchmarkElementType {
        ///Unique xccdf:Benchmark
        ///identifier.
        pub id: String,
        ///An identifier used for referencing elements
        ///included in an XML signature.
        pub xml_id: Option<String>,
        ///True if xccdf:Benchmark has already
        ///undergone the resolution process.
        pub resolved: bool,
        ///Name of an xccdf:Benchmark authoring
        ///style or set of conventions or constraints to which this
        ///xccdf:Benchmark conforms (e.g., “SCAP 1.2”).
        pub style: Option<String>,
        ///URL of a supplementary stylesheet or schema
        ///extension that can be used to verify conformance to the named
        ///style.
        pub style_href: Option<String>,
        pub lang: Option<String>,
        pub content: Vec<BenchmarkElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum BenchmarkElementTypeContent {
        ///Status of the xccdf:Benchmark
        ///indicating its level of maturity or consensus. If more than one
        ///xccdf:status element appears, the element's @date attribute
        ///should be included.
        Status(StatusElementType),
        ///Holds additional status information using
        ///the Dublin Core format.
        DcStatus(DcStatusType),
        ///Title of the xccdf:Benchmark; an
        ///xccdf:Benchmark should have an
        ///xccdf:title.
        Title(TextType),
        ///Text that describes the
        ///xccdf:Benchmark; an xccdf:Benchmark should have an
        ///xccdf:description.
        Description(HtmlTextWithSubType),
        ///Legal notices (licensing information, terms
        ///of use, etc.), copyright statements, warnings, and other advisory
        ///notices about this xccdf:Benchmark and its
        ///use.
        Notice(NoticeType),
        ///Introductory matter for the beginning of
        ///the xccdf:Benchmark document; intended for use during Document
        ///Generation.
        FrontMatter(HtmlTextWithSubType),
        ///Concluding material for the end of the
        ///xccdf:Benchmark document; intended for use during Document
        ///Generation.
        RearMatter(HtmlTextWithSubType),
        ///Supporting references for the
        ///xccdf:Benchmark document.
        Reference(ReferenceType),
        ///Definitions for reusable text blocks, each
        ///with a unique identifier.
        PlainText(PlainTextType),
        ///A list of identifiers for complex platform
        ///definitions, written in CPE applicability language format. Authors may
        ///define complex platforms within this element, and then use their locally
        ///unique identifiers anywhere in the xccdf:Benchmark element in
        ///place of a CPE name.
        PlatformSpecification(super::cpe::PlatformSpecificationType),
        ///Applicable platforms for this
        ///xccdf:Benchmark. Authors should use the element to identify the
        ///systems or products to which the xccdf:Benchmark
        ///applies.
        Platform(Cpe2IdrefType),
        ///Version number of the
        ///xccdf:Benchmark.
        Version(VersionType),
        ///XML metadata for the
        ///xccdf:Benchmark. Metadata allows many additional pieces of
        ///information, including authorship, publisher, support, and other similar
        ///details, to be embedded in an
        ///xccdf:Benchmark.
        Metadata(MetadataType),
        ///URIs of suggested scoring models to be used
        ///when computing a score for this xccdf:Benchmark. A suggested
        ///list of scoring models and their URIs is provided in the XCCDF
        ///specification.
        Model(ModelElementType),
        ///xccdf:Profile elements that
        ///reference and customize sets of items in the
        ///xccdf:Benchmark.
        Profile(ProfileType),
        ///Parameter xccdf:Value elements that
        ///support xccdf:Rule elements and descriptions in the
        ///xccdf:Benchmark.
        Value(ValueType),
        ///xccdf:Group elements that
        ///comprise the xccdf:Benchmark; each may contain additional
        ///xccdf:Value, xccdf:Rule, and other
        ///xccdf:Group elements.
        Group(GroupType),
        ///xccdf:Rule elements that
        ///comprise the xccdf:Benchmark.
        Rule(RuleType),
        ///xccdf:Benchmark test result records
        ///(one per xccdf:Benchmark run).
        TestResult(TestResultType),
        ///A digital signature asserting authorship
        ///and allowing verification of the integrity of the
        ///xccdf:Benchmark.
        Signature(SignatureType),
    }
    #[derive(Debug)]
    pub struct StatusElementType {
        ///The date the parent element achieved
        ///the indicated status.
        pub date: Option<String>,
        pub content: StatusType,
    }
    ///Data type element for the xccdf:dc-status element, which
    ///holds status information about its parent element using the Dublin Core format,
    ///expressed as elements of the DCMI Simple DC Element specification.
    #[derive(Debug)]
    pub struct DcStatusType {
        pub any: Vec<AnyElement>,
    }
    ///Type for a simple text string with an @override
    ///attribute for controlling inheritance.
    #[derive(Debug)]
    pub struct TextType {
        pub lang: Option<String>,
        ///Used to manage inheritance.
        pub override_: bool,
        pub content: String,
    }
    /// The type for a string with optional XHTML elements,
    ///and an @xml:lang attribute.
    #[derive(Debug)]
    pub struct HtmlTextWithSubType {
        pub lang: Option<String>,
        ///Used to manage inheritance.
        pub override_: bool,
        pub content: Vec<HtmlTextWithSubTypeContent>,
    }
    /// The type for a string with optional XHTML elements,
    ///and an @xml:lang attribute.
    #[derive(Debug)]
    pub enum HtmlTextWithSubTypeContent {
        ///Specifies an xccdf:Value or
        ///xccdf:plain-text element to be used for text
        ///substitution
        Sub(SubType),
        Any(AnyElement),
    }
    ///Data type for an xccdf:notice element.
    ///xccdf:notice elements are used to include legal notices (licensing
    ///information, terms of use, etc.), copyright statements, warnings, and other advisory
    ///notices about this xccdf:Benchmark and its use. This information may be
    ///expressed using XHTML or may be a simply text expression. Each xccdf:notice
    ///element must have a unique identifier.
    #[derive(Debug)]
    pub struct NoticeType {
        ///The unique identifier for this
        ///xccdf:notice.
        pub id: Option<String>,
        pub base: Option<String>,
        pub lang: Option<String>,
        pub text_before: Option<Text>,
        pub any: Vec<AnyElement>,
    }
    /// This element provides supplementary descriptive text
    ///for a XCCDF elements. When used, it has either a simple string value or a value
    ///consisting of simple Dublin Core elements. If a bare string appears, then it is
    ///taken to be the string content for a Dublin Core title element. Multiple
    ///xccdf:reference elements may appear; a document generation processing tool
    ///may concatenate them, or put them into a reference list, and may choose to number
    ///them.
    #[derive(Debug)]
    pub struct ReferenceType {
        ///A URL pointing to the referenced
        ///resource.
        pub href: Option<String>,
        ///Used to manage inheritance
        ///processing.
        pub override_: Option<bool>,
        pub text_before: Option<Text>,
        pub any: Vec<AnyElement>,
    }
    ///The data type for an xccdf:plain-text element,
    ///which is a reusable text block for reference by the xccdf:sub element. This
    ///allows text to be defined once and then reused multiple times. Each
    ///xccdf:plain-text element mush have a unique id.
    #[derive(Debug)]
    pub struct PlainTextType {
        ///The unique identifier for this
        ///xccdf:plain-text element.
        pub id: String,
        pub content: String,
    }
    /// Data type for xccdf:platform elements that do
    ///not need @override attributes. (I.e., xccdf:platform elements that are in
    ///structures that cannot be extended, such as xccdf:TestResult and
    ///xccdf:Benchmark elements.) This is used to identify the applicable target
    ///platform for its respective parent elements.
    #[derive(Debug)]
    pub struct Cpe2IdrefType {
        ///Should be a CPE 2.3 Applicability Language
        ///identifier using the Formatted String binding or the value of a
        ///cpe:platform-specification element's @id attribute, the latter acting as
        ///a reference to some expression defined using the CPE schema in the
        ///xccdf:Benchmark element's cpe:platform-specification element.
        ///The @idref may be a CPE Applicability Language identifier using the URI binding,
        ///although this is less preferred.
        pub idref: String,
    }
    /// Type for most xccdf:version elements.
    #[derive(Debug)]
    pub struct VersionType {
        ///The time that this version of the
        ///associated element was completed.
        pub time: Option<String>,
        ///A URI indicating a location where updates
        ///to the associated element may be obtained.
        pub update: Option<String>,
        pub content: String,
    }
    /// Data type that supports inclusion of metadata about a
    ///document or element. This is particularly useful for facilitating the discovery and
    ///retrieval of XCCDF checklists from public repositories. When used, the contents of
    ///the xccdf:metadata element are expressed in XML. The xccdf:Benchmark
    ///element's metadata should contain information formatted using the Dublin Core
    ///Metadata Initiative (DCMI) Simple DC Element specification, as described in [DCES]
    ///and [DCXML]. Benchmark consumers should be prepared to process Dublin Core metadata
    ///in the xccdf:metadata element. Other metadata schemes, including ad-hoc
    ///elements, are also allowed, both in the xccdf:Benchmark and in other
    ///elements.
    #[derive(Debug)]
    pub struct MetadataType {
        pub any: Vec<AnyElement>,
    }
    #[derive(Debug)]
    pub struct ModelElementType {
        ///A URI designating a scoring
        ///model.
        pub system: String,
        ///Parameters provided as input to the
        ///designated scoring model.
        pub param: Vec<ParamType>,
    }
    /// Data type for the xccdf:Profile element, which
    ///holds a specific tailoring of the xccdf:Benchmark. The main part of an
    ///xccdf:Profile is the selectors: xccdf:select,
    ///xccdf:set-value, xccdf:set-complex-value, xccdf:refine-rule,
    ///and xccdf:refine-value. An xccdf:Profile may also be signed with an
    ///XML-Signature.
    #[derive(Debug)]
    pub struct ProfileType {
        ///Unique identifier for this
        ///xccdf:Profile.
        pub id: String,
        ///Whether or not products should prohibit changes to
        ///this xccdf:Profile.
        pub prohibit_changes: bool,
        ///If true, then this xccdf:Profile exists
        ///solely to be extended by other xccdf:Profile elements.
        pub abstract_: bool,
        ///Tag identifier to specify which
        ///xccdf:profile-note element from an xccdf:Rule should be
        ///associated with this xccdf:Profile.
        pub note_tag: Option<String>,
        ///The id of an xccdf:Profile on which to base
        ///this xccdf:Profile.
        pub extends: Option<String>,
        pub base: Option<String>,
        ///An identifier used for referencing elements
        ///included in an XML signature.
        pub xml_id: Option<String>,
        pub content: Vec<ProfileTypeContent>,
    }
    /// Data type for the xccdf:Profile element, which
    ///holds a specific tailoring of the xccdf:Benchmark. The main part of an
    ///xccdf:Profile is the selectors: xccdf:select,
    ///xccdf:set-value, xccdf:set-complex-value, xccdf:refine-rule,
    ///and xccdf:refine-value. An xccdf:Profile may also be signed with an
    ///XML-Signature.
    #[derive(Debug)]
    pub enum ProfileTypeContent {
        ///Status of the xccdf:Profile and date at
        ///which it attained that status. Authors may use this element to record the
        ///maturity or consensus level of an xccdf:Profile. If the
        ///xccdf:status is not given explicitly, then the xccdf:Profile
        ///is taken to have the same status as its parent
        ///xccdf:Benchmark.
        Status(StatusElementType),
        ///Holds additional status information using the
        ///Dublin Core format.
        DcStatus(DcStatusType),
        ///Version information about this
        ///xccdf:Profile.
        Version(VersionType),
        ///Title of the xccdf:Profile.
        Title(TextWithSubType),
        ///Text that describes the xccdf:Profile.
        Description(HtmlTextWithSubType),
        ///A reference where the user can learn more about
        ///the subject of this xccdf:Profile.
        Reference(ReferenceType),
        ///A target platform for this
        ///xccdf:Profile.
        Platform(OverrideableCpe2IdrefType),
        ///Select or deselect xccdf:Group and
        ///xccdf:Rule elements.
        Select(ProfileSelectType),
        ///Set the value of an xccdf:Value to
        ///a list.
        SetComplexValue(ProfileSetComplexValueType),
        ///Set the value of an xccdf:Value to
        ///a simple data value.
        SetValue(ProfileSetValueType),
        ///Customize the properties of an
        ///xccdf:Value.
        RefineValue(ProfileRefineValueType),
        ///Customize the properties of an
        ///xccdf:Rule or xccdf:Group.
        RefineRule(ProfileRefineRuleType),
        ///Metadata associated with this
        ///xccdf:Profile.
        Metadata(MetadataType),
        ///A digital signature asserting authorship and
        ///allowing verification of the integrity of the
        ///xccdf:Profile.
        Signature(SignatureType),
    }
    /// Data type for the xccdf:Value element, which
    ///is a named parameter that can be substituted into properties of other elements
    ///within the xccdf:Benchmark, including the interior of structured check
    ///specifications and fix scripts.
    #[derive(Debug)]
    pub struct ValueType {
        ///If true, then this item is abstract and exists only
        ///to be extended. The use of this attribute for xccdf:Group elements is
        ///deprecated and should be avoided.
        pub abstract_: bool,
        ///An identifier to be used as a means to identify
        ///(refer to) related items. It designates membership in a cluster of items, which
        ///are used for controlling items via xccdf:Profile elements. All the items
        ///with the same cluster identifier belong to the same cluster. A selector in an
        ///xccdf:Profile may refer to a cluster, thus making it easier for authors
        ///to create and maintain xccdf:Profile elements in a complex
        ///xccdf:Benchmark.
        pub cluster_id: Option<String>,
        ///The identifier of an item on which to base this
        ///item. If present, it must have a value equal to the @id attribute of another
        ///item. The use of this attribute for xccdf:Group elements is deprecated
        ///and should be avoided.
        pub extends: Option<String>,
        ///If this item should be excluded from any generated
        ///documents although it may still be used during assessments.
        pub hidden: bool,
        ///If benchmark producers should prohibit changes to
        ///this item during tailoring. An author should use this when they do not want to
        ///allow end users to change the item.
        pub prohibit_changes: bool,
        pub lang: Option<String>,
        pub base: Option<String>,
        ///An identifier used for referencing elements
        ///included in an XML signature
        pub xml_id: Option<String>,
        ///The unique identifier for this element.
        pub id: String,
        ///The data type of the xccdf:Value. A
        ///tool may choose any convenient form to store an xccdf:Value
        ///element’s xccdf:value element, but the @type attribute conveys
        ///how the xccdf:Value should be treated for user input validation
        ///purposes during tailoring processing. The @type attribute may also be
        ///used to give additional guidance to the user or to validate the user’s
        ///input. In the case of a list of values, the @type attribute, if present,
        ///applies to all elements of the list individually.
        pub type_: ValueTypeType,
        ///The operator to be used for comparing this
        ///xccdf:Value to some part of the test system’s configuration
        ///during xccdf:Rule checking.
        pub operator: ValueOperatorType,
        ///Whether tailoring for this
        ///xccdf:Value should be performed during xccdf:Benchmark
        ///application. The benchmark consumer may ignore the attribute if asking
        ///the user is not feasible or not supported.
        pub interactive: bool,
        ///A hint or recommendation to a benchmark
        ///consumer or producer about how the user might select or adjust the
        ///xccdf:Value.
        pub interface_hint: Option<InterfaceHintType>,
        pub content: Vec<ValueTypeContent>,
    }
    /// Data type for the xccdf:Value element, which
    ///is a named parameter that can be substituted into properties of other elements
    ///within the xccdf:Benchmark, including the interior of structured check
    ///specifications and fix scripts.
    #[derive(Debug)]
    pub enum ValueTypeContent {
        ///Status of the item and date at which it
        ///attained that status. xccdf:Benchmark authors may use this element
        ///to record the maturity or consensus level for elements in the
        ///xccdf:Benchmark. If an item does not have an explicit
        ///xccdf:status given, then its status is that of its
        ///parent.
        Status(StatusElementType),
        ///Holds additional status information using the
        ///Dublin Core format.
        DcStatus(DcStatusType),
        ///Version information about this item.
        Version(VersionType),
        ///Title of the item. Every item should have an
        ///xccdf:title, because this helps people understand the purpose of the
        ///item.
        Title(TextWithSubType),
        ///Text that describes the item.
        Description(HtmlTextWithSubType),
        ///A note or caveat about the item intended to
        ///convey important cautionary information for the xccdf:Benchmark user
        ///(e.g., “Complying with this rule will cause the system to reject all IP
        ///packets”). If multiple xccdf:warning elements appear, benchmark
        ///consumers should concatenate them for generating reports or documents.
        ///Benchmark consumers may present this information in a special manner in
        ///generated documents.
        Warning(WarningType),
        ///Interrogative text to present to the user
        ///during tailoring. It may also be included into a generated document. For
        ///xccdf:Rule and xccdf:Group elements, the
        ///xccdf:question text should be a simple binary (yes/no) question
        ///because it is supporting the selection aspect of tailoring. For
        ///xccdf:Value elements, the xccdf:question should solicit the
        ///user to provide a specific value. Tools may also display constraints on
        ///values and any defaults as specified by the other xccdf:Value
        ///properties.
        Question(TextType),
        ///References where the user can learn more about
        ///the subject of this item.
        Reference(ReferenceType),
        ///XML metadata associated with this item, such as
        ///sources, special information, or other details.
        Metadata(MetadataType),
        ///A simple (number, string, or
        ///boolean) value associated with this xccdf:Value. At any
        ///time an xccdf:Value has one active (simple or complex)
        ///value. If a selector value has been provided under
        ///xccdf:Profile selection or tailoring then the active
        ///xccdf:value/xccdf:complex-value is the one with
        ///a matching @selector. If there is no provided selector or if the
        ///provided selector does not match the @selector attribute of any
        ///xccdf:value or xccdf:complex-value, the active
        ///xccdf:value/xccdf:complex-value is the one with
        ///an empty or absent @selector or, failing that, the first
        ///xccdf:value or xccdf:complex-value in the XML.
        ///When an xccdf:Value is exported or used in text
        ///substitution, it is the currently active xccdf:value or
        ///xccdf:complex-value that is actually used. If there are
        ///multiple xccdf:value and/or xccdf:complex-value
        ///elements, only one may omit a @selector attribute and no two may
        ///have the same @selector value.
        Value(SelStringType),
        ///A complex (list) value associated
        ///with this xccdf:Value. See the description of the
        ///xccdf:value property for xccdf:Rule elements
        ///regarding activation of an xccdf:complex-value.
        ComplexValue(SelComplexValueType),
        ///The default value displayed to the
        ///user as a suggestion by benchmark producers during tailoring of
        ///this xccdf:Value element. (This is not the default value
        ///of an xccdf:Value; it is just the default display.) If
        ///there are multiple xccdf:default and/or
        ///xccdf:complex-default elements, only one may omit a
        ///@selector attribute and no two may have the same @selector
        ///value.
        Default(SelStringType),
        ///The default
        ///xccdf:complex-value displayed to the user as a
        ///suggestion by benchmark producers during tailoring of this
        ///xccdf:Value element. (This is not the default value of
        ///an xccdf:Value; it is just the default display.) If
        ///there are multiple xccdf:default and
        ///xccdf:complex-default elements, only one may omit a
        ///@selector attribute and no two may have the same @selector
        ///value.
        ComplexDefault(SelComplexValueType),
        ///A Perl Compatible Regular Expression
        ///that a benchmark producer may apply during tailoring to validate a
        ///user’s input for the xccdf:Value. It uses implicit
        ///anchoring. It applies only when the @type property is “string” or
        ///“number” or a list of strings and/or numbers.
        Match(SelStringType),
        ///Minimum legal value for this
        ///xccdf:Value. It is used to constrain value input during
        ///tailoring, when the @type property is “number”. Values supplied by
        ///the user for tailoring the xccdf:Benchmark must be equal to
        ///or greater than this number.
        LowerBound(SelNumType),
        ///Maximum legal value for this
        ///xccdf:Value. It is used to constrain value input during
        ///tailoring, when the @type is “number”. Values supplied by the user
        ///for tailoring the xccdf:Benchmark must be less than or equal
        ///to than this number.
        UpperBound(SelNumType),
        ///A list of legal or suggested choices
        ///(values) for an xccdf:Value element, to be used during
        ///tailoring and document generation.
        Choices(SelChoicesType),
        ///URI indicating where the tool may
        ///acquire values, value bounds, or value choices for this
        ///xccdf:Value element. XCCDF does not attach any meaning to
        ///the URI; it may be an arbitrary community or tool-specific value, or
        ///a pointer directly to a resource. If several instances of the
        ///xccdf:source property appear, then they represent
        ///alternative means or locations for obtaining the value in descending
        ///order of preference (i.e., most preferred first).
        Source(UriRefType),
        ///A digital signature asserting
        ///authorship and allowing verification of the integrity of the
        ///xccdf:Value.
        Signature(SignatureType),
    }
    /// Data type for the xccdf:Group element. A
    ///xccdf:Group element contains descriptive information about a portion of an
    ///xccdf:Benchmark, as well as xccdf:Rule, xccdf:Value, and/or
    ///other xccdf:Group elements
    #[derive(Debug)]
    pub struct GroupType {
        ///If true, then this item is abstract and exists only
        ///to be extended. The use of this attribute for xccdf:Group elements is
        ///deprecated and should be avoided.
        pub abstract_: bool,
        ///An identifier to be used as a means to identify
        ///(refer to) related items. It designates membership in a cluster of items, which
        ///are used for controlling items via xccdf:Profile elements. All the items
        ///with the same cluster identifier belong to the same cluster. A selector in an
        ///xccdf:Profile may refer to a cluster, thus making it easier for authors
        ///to create and maintain xccdf:Profile elements in a complex
        ///xccdf:Benchmark.
        pub cluster_id: Option<String>,
        ///The identifier of an item on which to base this
        ///item. If present, it must have a value equal to the @id attribute of another
        ///item. The use of this attribute for xccdf:Group elements is deprecated
        ///and should be avoided.
        pub extends: Option<String>,
        ///If this item should be excluded from any generated
        ///documents although it may still be used during assessments.
        pub hidden: bool,
        ///If benchmark producers should prohibit changes to
        ///this item during tailoring. An author should use this when they do not want to
        ///allow end users to change the item.
        pub prohibit_changes: bool,
        pub lang: Option<String>,
        pub base: Option<String>,
        ///An identifier used for referencing elements
        ///included in an XML signature
        pub xml_id: Option<String>,
        ///If true, this
        ///xccdf:Group/xccdf:Rule is selected to be processed as
        ///part of the xccdf:Benchmark when it is applied to a target
        ///system. An unselected xccdf:Group does not get processed, and
        ///its contents are not processed either (i.e., all descendants of an
        ///unselected xccdf:Group are implicitly unselected). An unselected
        ///xccdf:Rule is not checked and does not contribute to scoring.
        pub selected: bool,
        ///The relative scoring weight of this
        ///xccdf:Group/xccdf:Rule, for computing a score, expressed
        ///as a non-negative real number. It denotes the importance of an
        ///xccdf:Group/xccdf:Rule. Under some scoring models,
        ///scoring is computed independently for each collection of sibling
        ///xccdf:Group and xccdf:Rule elements, then normalized as
        ///part of the overall scoring process.
        pub weight: f64,
        ///Unique element identifier; used by other
        ///elements to refer to this element.
        pub id: String,
        pub content: Vec<GroupTypeContent>,
    }
    /// Data type for the xccdf:Group element. A
    ///xccdf:Group element contains descriptive information about a portion of an
    ///xccdf:Benchmark, as well as xccdf:Rule, xccdf:Value, and/or
    ///other xccdf:Group elements
    #[derive(Debug)]
    pub enum GroupTypeContent {
        ///Status of the item and date at which it
        ///attained that status. xccdf:Benchmark authors may use this element
        ///to record the maturity or consensus level for elements in the
        ///xccdf:Benchmark. If an item does not have an explicit
        ///xccdf:status given, then its status is that of its
        ///parent.
        Status(StatusElementType),
        ///Holds additional status information using the
        ///Dublin Core format.
        DcStatus(DcStatusType),
        ///Version information about this item.
        Version(VersionType),
        ///Title of the item. Every item should have an
        ///xccdf:title, because this helps people understand the purpose of the
        ///item.
        Title(TextWithSubType),
        ///Text that describes the item.
        Description(HtmlTextWithSubType),
        ///A note or caveat about the item intended to
        ///convey important cautionary information for the xccdf:Benchmark user
        ///(e.g., “Complying with this rule will cause the system to reject all IP
        ///packets”). If multiple xccdf:warning elements appear, benchmark
        ///consumers should concatenate them for generating reports or documents.
        ///Benchmark consumers may present this information in a special manner in
        ///generated documents.
        Warning(WarningType),
        ///Interrogative text to present to the user
        ///during tailoring. It may also be included into a generated document. For
        ///xccdf:Rule and xccdf:Group elements, the
        ///xccdf:question text should be a simple binary (yes/no) question
        ///because it is supporting the selection aspect of tailoring. For
        ///xccdf:Value elements, the xccdf:question should solicit the
        ///user to provide a specific value. Tools may also display constraints on
        ///values and any defaults as specified by the other xccdf:Value
        ///properties.
        Question(TextType),
        ///References where the user can learn more about
        ///the subject of this item.
        Reference(ReferenceType),
        ///XML metadata associated with this item, such as
        ///sources, special information, or other details.
        Metadata(MetadataType),
        ///Descriptive text giving rationale or
        ///motivations for abiding by this
        ///xccdf:Group/xccdf:Rule (i.e., why it is important to
        ///the security of the target platform).
        Rationale(HtmlTextWithSubType),
        ///Platforms to which this
        ///xccdf:Group/xccdf:Rule applies.
        Platform(OverrideableCpe2IdrefType),
        ///The identifiers of other
        ///xccdf:Group or xccdf:Rule elements that must be
        ///selected for this xccdf:Group/xccdf:Rule to be
        ///evaluated and scored properly. Each xccdf:requires element
        ///specifies a list of one or more required items by their identifiers.
        ///If at least one of the specified xccdf:Group or
        ///xccdf:Rule elements is selected, the requirement is met.
        Requires(IdrefListType),
        ///The identifier of another
        ///xccdf:Group or xccdf:Rule that must be unselected
        ///for this xccdf:Group/xccdf:Rule to be evaluated and
        ///scored properly. Each xccdf:conflicts element specifies a
        ///single conflicting item using its idref attribute. If the specified
        ///xccdf:Group or xccdf:Rule element is not selected,
        ///the requirement is met.
        Conflicts(IdrefType),
        ///xccdf:Value elements that
        ///belong to this xccdf:Group.
        Value(ValueType),
        ///Sub-xccdf:Groups under this
        ///xccdf:Group.
        Group(GroupType),
        ///xccdf:Rule elements that
        ///belong to this xccdf:Group.
        Rule(RuleType),
        ///A digital signature asserting
        ///authorship and allowing verification of the integrity of the
        ///xccdf:Group.
        Signature(SignatureType),
    }
    /// Data type for the xccdf:Rule element that
    ///represents a specific xccdf:Benchmark test.
    #[derive(Debug)]
    pub struct RuleType {
        ///If true, then this item is abstract and exists only
        ///to be extended. The use of this attribute for xccdf:Group elements is
        ///deprecated and should be avoided.
        pub abstract_: bool,
        ///An identifier to be used as a means to identify
        ///(refer to) related items. It designates membership in a cluster of items, which
        ///are used for controlling items via xccdf:Profile elements. All the items
        ///with the same cluster identifier belong to the same cluster. A selector in an
        ///xccdf:Profile may refer to a cluster, thus making it easier for authors
        ///to create and maintain xccdf:Profile elements in a complex
        ///xccdf:Benchmark.
        pub cluster_id: Option<String>,
        ///The identifier of an item on which to base this
        ///item. If present, it must have a value equal to the @id attribute of another
        ///item. The use of this attribute for xccdf:Group elements is deprecated
        ///and should be avoided.
        pub extends: Option<String>,
        ///If this item should be excluded from any generated
        ///documents although it may still be used during assessments.
        pub hidden: bool,
        ///If benchmark producers should prohibit changes to
        ///this item during tailoring. An author should use this when they do not want to
        ///allow end users to change the item.
        pub prohibit_changes: bool,
        pub lang: Option<String>,
        pub base: Option<String>,
        ///An identifier used for referencing elements
        ///included in an XML signature
        pub xml_id: Option<String>,
        ///If true, this
        ///xccdf:Group/xccdf:Rule is selected to be processed as
        ///part of the xccdf:Benchmark when it is applied to a target
        ///system. An unselected xccdf:Group does not get processed, and
        ///its contents are not processed either (i.e., all descendants of an
        ///unselected xccdf:Group are implicitly unselected). An unselected
        ///xccdf:Rule is not checked and does not contribute to scoring.
        pub selected: bool,
        ///The relative scoring weight of this
        ///xccdf:Group/xccdf:Rule, for computing a score, expressed
        ///as a non-negative real number. It denotes the importance of an
        ///xccdf:Group/xccdf:Rule. Under some scoring models,
        ///scoring is computed independently for each collection of sibling
        ///xccdf:Group and xccdf:Rule elements, then normalized as
        ///part of the overall scoring process.
        pub weight: f64,
        ///Unique element identifier used by other
        ///elements to refer to this element.
        pub id: String,
        ///The xccdf:Rule element’s role in
        ///scoring and reporting.
        pub role: RoleEnumType,
        ///Severity level code to be used for metrics
        ///and tracking.
        pub severity: SeverityEnumType,
        ///Applicable in cases where there are
        ///multiple instances of a target. For example, an xccdf:Rule may
        ///provide a recommendation about the configuration of application user
        ///accounts, but an application may have many user accounts. Each account
        ///would be considered an instance of the broader assessment target of user
        ///accounts. If the @multiple attribute is set to true, each instance of
        ///the target to which the xccdf:Rule can apply should be tested
        ///separately and the results should be recorded separately. If @multiple
        ///is set to false, the test results of such instances should be combined.
        ///If the checking system does not combine these results automatically, the
        ///results of each instance should be ANDed together to produce a single
        ///result. If the benchmark consumer cannot perform multiple instantiation,
        ///or if multiple instantiation of the xccdf:Rule is not applicable
        ///for the target system, then the benchmark consumer may ignore this
        ///attribute.
        pub multiple: bool,
        pub content: Vec<RuleTypeContent>,
    }
    /// Data type for the xccdf:Rule element that
    ///represents a specific xccdf:Benchmark test.
    #[derive(Debug)]
    pub enum RuleTypeContent {
        ///Status of the item and date at which it
        ///attained that status. xccdf:Benchmark authors may use this element
        ///to record the maturity or consensus level for elements in the
        ///xccdf:Benchmark. If an item does not have an explicit
        ///xccdf:status given, then its status is that of its
        ///parent.
        Status(StatusElementType),
        ///Holds additional status information using the
        ///Dublin Core format.
        DcStatus(DcStatusType),
        ///Version information about this item.
        Version(VersionType),
        ///Title of the item. Every item should have an
        ///xccdf:title, because this helps people understand the purpose of the
        ///item.
        Title(TextWithSubType),
        ///Text that describes the item.
        Description(HtmlTextWithSubType),
        ///A note or caveat about the item intended to
        ///convey important cautionary information for the xccdf:Benchmark user
        ///(e.g., “Complying with this rule will cause the system to reject all IP
        ///packets”). If multiple xccdf:warning elements appear, benchmark
        ///consumers should concatenate them for generating reports or documents.
        ///Benchmark consumers may present this information in a special manner in
        ///generated documents.
        Warning(WarningType),
        ///Interrogative text to present to the user
        ///during tailoring. It may also be included into a generated document. For
        ///xccdf:Rule and xccdf:Group elements, the
        ///xccdf:question text should be a simple binary (yes/no) question
        ///because it is supporting the selection aspect of tailoring. For
        ///xccdf:Value elements, the xccdf:question should solicit the
        ///user to provide a specific value. Tools may also display constraints on
        ///values and any defaults as specified by the other xccdf:Value
        ///properties.
        Question(TextType),
        ///References where the user can learn more about
        ///the subject of this item.
        Reference(ReferenceType),
        ///XML metadata associated with this item, such as
        ///sources, special information, or other details.
        Metadata(MetadataType),
        ///Descriptive text giving rationale or
        ///motivations for abiding by this
        ///xccdf:Group/xccdf:Rule (i.e., why it is important to
        ///the security of the target platform).
        Rationale(HtmlTextWithSubType),
        ///Platforms to which this
        ///xccdf:Group/xccdf:Rule applies.
        Platform(OverrideableCpe2IdrefType),
        ///The identifiers of other
        ///xccdf:Group or xccdf:Rule elements that must be
        ///selected for this xccdf:Group/xccdf:Rule to be
        ///evaluated and scored properly. Each xccdf:requires element
        ///specifies a list of one or more required items by their identifiers.
        ///If at least one of the specified xccdf:Group or
        ///xccdf:Rule elements is selected, the requirement is met.
        Requires(IdrefListType),
        ///The identifier of another
        ///xccdf:Group or xccdf:Rule that must be unselected
        ///for this xccdf:Group/xccdf:Rule to be evaluated and
        ///scored properly. Each xccdf:conflicts element specifies a
        ///single conflicting item using its idref attribute. If the specified
        ///xccdf:Group or xccdf:Rule element is not selected,
        ///the requirement is met.
        Conflicts(IdrefType),
        ///A globally meaningful identifier for
        ///this xccdf:Rule. This may be the name or identifier of a
        ///security configuration issue or vulnerability that the
        ///xccdf:Rule assesses.
        Ident(IdentType),
        ///The potential impact of failure to
        ///conform to the xccdf:Rule, expressed as a CVSS 2.0 base
        ///vector.
        ImpactMetric(String),
        ///Text that describes special aspects of
        ///the xccdf:Rule related to one or more xccdf:Profile
        ///elements. This allows an author to document things within
        ///xccdf:Rule elements that are specific to a given
        ///xccdf:Profile, and then select the appropriate text based on
        ///the selected xccdf:Profile and display it to the
        ///reader.
        ProfileNote(ProfileNoteType),
        ///Data that describes how to bring a
        ///target system into compliance with this
        ///xccdf:Rule.
        Fixtext(FixTextType),
        ///A command string, script, or other
        ///system modification statement that, if executed on the target
        ///system, can bring it into full, or at least better, compliance with
        ///this xccdf:Rule.
        Fix(FixType),
        ///The definition of, or a reference
        ///to, the target system check needed to test compliance with this
        ///xccdf:Rule. Sibling xccdf:check elements must
        ///have different values for the combination of their @selector and
        ///@system attributes, and must have different values for their @id
        ///attribute (if any).
        Check(CheckType),
        ///A boolean expression composed of
        ///operators (and, or, not) and individual
        ///checks.
        ComplexCheck(ComplexCheckType),
        ///A digital signature asserting
        ///authorship and allowing verification of the integrity of the
        ///xccdf:Rule.
        Signature(SignatureType),
    }
    /// Data type for the xccdf:TestResult element,
    ///which holds the results of one application of the xccdf:Benchmark. The
    ///xccdf:TestResult element normally appears as the child of the
    ///xccdf:Benchmark element, although it may also appear as the top-level
    ///element of an XCCDF results document. XCCDF is not intended to be a database format
    ///for detailed results; the xccdf:TestResult element offers a way to store the
    ///results of individual tests in modest detail, with the ability to reference
    ///lower-level testing data. Although several of the child elements of this type
    ///technically support the @override attribute, the xccdf:TestResult element
    ///cannot be extended. Therefore, @override has no meaning within an
    ///xccdf:TestResult element and its children, and should not be used for
    ///them.
    #[derive(Debug)]
    pub struct TestResultType {
        ///Unique identifier for this
        ///element.
        pub id: String,
        ///Time when testing began.
        pub start_time: Option<String>,
        ///Time when testing was completed and the results
        ///recorded.
        pub end_time: String,
        ///Name of the benchmark consumer program that
        ///generated this xccdf:TestResult element; should be either a CPE name or
        ///a CPE applicability language expression.
        pub test_system: Option<String>,
        ///The version number string copied from the
        ///xccdf:Benchmark used to direct this assessment.
        pub version: Option<String>,
        ///An identifier used for referencing elements
        ///included in an XML signature.
        pub xml_id: Option<String>,
        pub content: Vec<TestResultTypeContent>,
    }
    /// Data type for the xccdf:TestResult element,
    ///which holds the results of one application of the xccdf:Benchmark. The
    ///xccdf:TestResult element normally appears as the child of the
    ///xccdf:Benchmark element, although it may also appear as the top-level
    ///element of an XCCDF results document. XCCDF is not intended to be a database format
    ///for detailed results; the xccdf:TestResult element offers a way to store the
    ///results of individual tests in modest detail, with the ability to reference
    ///lower-level testing data. Although several of the child elements of this type
    ///technically support the @override attribute, the xccdf:TestResult element
    ///cannot be extended. Therefore, @override has no meaning within an
    ///xccdf:TestResult element and its children, and should not be used for
    ///them.
    #[derive(Debug)]
    pub enum TestResultTypeContent {
        ///Reference to the xccdf:Benchmark for
        ///which the xccdf:TestResult records results. This property is
        ///required if this xccdf:TestResult element is the top-level element
        ///and optional otherwise.
        Benchmark(BenchmarkReferenceType),
        ///The tailoring file element contains attributes
        ///used to identify an xccdf:Tailoring element used to guide the
        ///assessment reported on in this xccdf:TestResult. The tailoring
        ///element is required in an xccdf:TestResult if and only if an
        ///xccdf:Tailoring element guided the assessment recorded in the
        ///xccdf:TestResult or if the xccdf:Tailoring element records
        ///manual tailoring actions applied to this assessment.
        TailoringFile(TailoringReferenceType),
        ///Title of the test.
        Title(TextType),
        ///A remark about the test, possibly supplied by
        ///the person administering the xccdf:Benchmark
        ///assessment
        Remark(TextType),
        ///The name of the organization or other entity
        ///responsible for applying this xccdf:Benchmark and generating this
        ///result. When multiple xccdf:organization elements are used to
        ///indicate multiple organization names in a hierarchical organization, the
        ///highest-level organization should appear first.
        Organization(String),
        ///Information about the system identity or user
        ///employed during application of the xccdf:Benchmark. If used,
        ///specifies the name of the authenticated identity.
        Identity(IdentityType),
        ///The xccdf:profile element holds the
        ///value of the @id attribute value of the xccdf:Profile selected to be
        ///used in the assessment reported on by this xccdf:TestResult. This
        ///xccdf:Profile might be from the xccdf:Benchmark or from an
        ///xccdf:Tailoring file, if used. This element should appear if and
        ///only if an xccdf:Profile was selected to guide the
        ///assessment.
        Profile(IdrefType),
        ///Name or description of the target system whose
        ///test results are recorded in the xccdf:TestResult element (the
        ///system to which an xccdf:Benchmark test was applied). Each
        ///appearance of the element supplies a name by which the target host or device
        ///was identified at the time the test was run. The name may be any string, but
        ///applications should include the fully qualified DNS name whenever possible.
        Target(String),
        ///Network address of the target system to which
        ///an xccdf:Benchmark test was applied. Typical forms for the address
        ///include IP version 4 (IPv4), IP version 6 (IPv6), and Ethernet media access
        ///control (MAC).
        TargetAddress(String),
        ///A list of named facts about the target system
        ///or platform.
        TargetFacts(TargetFactsType),
        ///References to external structures with
        ///identifying information about the target of this
        ///assessment.
        TargetIdRef(TargetIdRefType),
        Any(AnyElement),
        ///A platform on the target system. There should
        ///be one instance of this property for every platform that the target system
        ///was found to meet.
        Platform(Cpe2IdrefType),
        ///Specific setting for a single
        ///xccdf:Value element used during the test.
        SetValue(ProfileSetValueType),
        ///Specific setting for a single
        ///xccdf:Value element used during the test when the given value is
        ///set to a complex type, such as a list.
        SetComplexValue(ProfileSetComplexValueType),
        ///The result of a single instance of an
        ///xccdf:Rule application against the target. The
        ///xccdf:TestResult must include at least one xccdf:rule-result
        ///record for each xccdf:Rule that was selected in the resolved
        ///xccdf:Benchmark.
        RuleResult(RuleResultType),
        ///An overall score for this
        ///xccdf:Benchmark test.
        Score(ScoreType),
        ///XML metadata associated with this
        ///xccdf:TestResult.
        Metadata(MetadataType),
        ///A digital signature asserting authorship and
        ///allowing verification of the integrity of the xccdf:TestResult.
        Signature(SignatureType),
    }
    /// The type of an XMLDSig:signature element,
    ///which holds an enveloped digital signature asserting authorship and allowing
    ///verification of the integrity of associated data (e.g., its parent element, other
    ///documents, portions of other documents).
    #[derive(Debug)]
    pub struct SignatureType {
        pub any: AnyElement,
    }
    /// The statusType represents the possible levels of
    ///maturity or consensus level for its parent element as recorded by an
    ///xccdf:status element.
    #[derive(Debug)]
    pub enum StatusType {
        ///Released as final
        Accepted,
        ///No longer needed
        Deprecated,
        ///Released in draft state
        Draft,
        ///Under initial development
        Incomplete,
        ///Revised and in the process of being
        ///finalized
        Interim,
    }
    ///The type used for xccdf:sub elements. The
    ///xccdf:sub element identifies replacement content that should appear in place
    ///of the xccdf:sub element during text substitution. The subType consists of a
    ///regular idrefType with an additional @use attribute to dictate the behavior of the
    ///xccdf:sub element under substitution. When the @idref is to an
    ///xccdf:Value, the @use attribute indicates whether the xccdf:Value
    ///element's title or value should replace the xccdf:sub element. The @use
    ///attribute is ignored when the @idref is to an xccdf:plain-text element; the
    ///body of the xccdf:plain-text element is always used to replace the
    ///xccdf:sub element.
    #[derive(Debug)]
    pub struct SubType {
        ///The id value of another XCCDF
        ///element
        pub idref: String,
        ///Dictates the nature of the content inserted
        ///under text substitution processing.
        pub use_: SubUseEnumType,
    }
    /// Type for a parameter used in the xccdf:model
    ///element, which records scoring model information. The contents of this type
    ///represent a name-value pair, where the name is recorded in the @name attribute and
    ///the value appears in the element body. xccdf:param elements with equal
    ///values for the @name attribute may not appear as children of the same
    ///xccdf:model element.
    #[derive(Debug)]
    pub struct ParamType {
        ///The name associated with the contained
        ///value.
        pub name: String,
        pub content: String,
    }
    /// Type for a string with embedded xccdf:Value
    ///substitutions and an @override attribute to help manage inheritance.
    #[derive(Debug)]
    pub struct TextWithSubType {
        pub lang: Option<String>,
        ///Used to manage inheritance.
        pub override_: bool,
        pub text_before: Option<Text>,
        ///Specifies an xccdf:Value or
        ///xccdf:plain-text element to be used for text substitution.
        pub sub: Vec<Mixed<SubType>>,
    }
    ///Data type for xccdf:platform elements that need
    ///@override attributes. (I.e., xccdf:platform elements that are in structures
    ///that can be extended, such as Items and xccdf:Profile elements.) This is
    ///used to identify the applicable target platform for its respective parent elements.
    #[derive(Debug)]
    pub struct OverrideableCpe2IdrefType {
        ///Should be a CPE 2.3 Applicability Language
        ///identifier using the Formatted String binding or the value of a
        ///cpe:platform-specification element's @id attribute, the latter acting as
        ///a reference to some expression defined using the CPE schema in the
        ///xccdf:Benchmark element's cpe:platform-specification element.
        ///The @idref may be a CPE Applicability Language identifier using the URI binding,
        ///although this is less preferred.
        pub idref: String,
        ///Used to manage inheritance.
        pub override_: bool,
    }
    /// Type for the xccdf:select element in an
    ///xccdf:Profile. This element designates an xccdf:Rule,
    ///xccdf:Group, or cluster of xccdf:Rule and xccdf:Group
    ///elements and overrides the @selected attribute on the designated items, providing a
    ///means for including or excluding xccdf:Rule elements from an
    ///assessment.
    #[derive(Debug)]
    pub struct ProfileSelectType {
        ///The @id value of an xccdf:Rule or
        ///xccdf:Group, or the @cluster-id value of one or more xccdf:Rule
        ///or xccdf:Group elements.
        pub idref: String,
        ///The new value for the indicated item's @selected
        ///property.
        pub selected: bool,
        ///Explanatory material or other
        ///prose.
        pub remark: Vec<TextType>,
    }
    /// Type for the xccdf:set-complex-value element
    ///in an xccdf:Profile. This element supports the direct specification of
    ///complex value types such as lists. Zero or more xccdf:item elements may
    ///appear as children of this element; if no child elements are present, this element
    ///represents an empty list. This overrides the xccdf:value and
    ///xccdf:complex-value element(s) of an xccdf:Value
    ///element.
    #[derive(Debug)]
    pub struct ProfileSetComplexValueType {
        ///The @id value of an xccdf:Value or
        ///the @cluster-id value of one or more xccdf:Value elements
        pub idref: String,
        ///A single item in the list of values.
        pub item: Vec<String>,
    }
    /// Type for the xccdf:set-value element in an
    ///xccdf:Profile. This element upports the direct specification of simple value
    ///types such as numbers, strings, and boolean values. This overrides the
    ///xccdf:value and xccdf:complex-value element(s) of an
    ///xccdf:Value element.
    #[derive(Debug)]
    pub struct ProfileSetValueType {
        ///The @id value of an xccdf:Value or
        ///the @cluster-id value of one or more xccdf:Value elements
        pub idref: String,
        pub content: String,
    }
    /// Type for the xccdf:refine-value element in an
    ///xccdf:Profile. This element designates the xccdf:Value constraints
    ///to be applied during tailoring for an xccdf:Value element or the
    ///xccdf:Value members of a cluster.
    #[derive(Debug)]
    pub struct ProfileRefineValueType {
        ///The @id value of an xccdf:Value or the
        ///@cluster-id value of one or more xccdf:Value elements
        pub idref: String,
        ///Holds a selector value corresponding to the value
        ///of a @selector property in an xccdf:Value element's child properties.
        ///Properties with a matching @selector are considered active and all other
        ///properties are inactive. This may mean that, after selector application, some
        ///classes of xccdf:Value properties will be completely inactive because
        ///none of those properties had a matching @selector. The only exception is the
        ///xccdf:value and xccdf:complex-value properties of an
        ///xccdf:Value element - if there is no xccdf:value or
        ///xccdf:complex-value property with a matching @selector value then the
        ///xccdf:value/xccdf:complex-value property with an empty or absent
        ///@selector attribute becomes active. If there is no such xccdf:value or
        ///xccdf:complex-value, then the first xccdf:value or
        ///xccdf:complex-value listed in the XML becomes active. This reflects the
        ///fact that all xccdf:Value elements require an active value property at
        ///all times.
        pub selector: Option<String>,
        ///The new value for the identified
        ///xccdf:Value element's @operator property.
        pub operator: Option<ValueOperatorType>,
        ///Explanatory material or other
        ///prose.
        pub remark: Vec<TextType>,
    }
    /// Type for the xccdf:refine-rule element in an
    ///xccdf:Profile. A xccdf:refine-rule element allows the author to
    ///select xccdf:check statements and override the @weight, @severity, and @role
    ///of an xccdf:Rule, xccdf:Group, or cluster of xccdf:Rule and
    ///xccdf:Group elements. Despite the name, this selector does apply for
    ///xccdf:Group elements and for clusters that include xccdf:Group
    ///elements, but it only affects their @weight attribute.
    #[derive(Debug)]
    pub struct ProfileRefineRuleType {
        ///The @id value of an xccdf:Rule or
        ///xccdf:Group, or the @cluster-id value of one or more xccdf:Rule
        ///or xccdf:Group elements.
        pub idref: String,
        ///The new value for the identified element's @weight
        ///property.
        pub weight: Option<f64>,
        ///Holds a selector value corresponding to the value
        ///of a @selector property in an xccdf:Rule element's xccdf:check
        ///element. If the selector specified does not match any of the @selector
        ///attributes specified on any of the xccdf:check children of an
        ///xccdf:Rule, then the xccdf:check child element without a
        ///@selector attribute is used. If there is no child without a @selector attribute,
        ///then that Rule would have no effective xccdf:check
        ///element.
        pub selector: Option<String>,
        ///The new value for the identified xccdf:Rule
        ///element's @severity property.
        pub severity: Option<SeverityEnumType>,
        ///The new value for the identified xccdf:Rule
        ///element's @role property.
        pub role: Option<RoleEnumType>,
        ///Explanatory material or other
        ///prose.
        pub remark: Vec<TextType>,
    }
    ///Allowed data types for xccdf:Value elements,
    ///string, numeric, and boolean. A tool may choose any convenient form to store an
    ///xccdf:Value element’s xccdf:value element, but the @type conveys how
    ///the value should be treated for user input validation purposes during tailoring
    ///processing. The @type may also be used to give additional guidance to the user or to
    ///validate the user’s input. For example, if an xccdf:value element’s @type
    ///attribute is “number”, then a tool might choose to reject user tailoring input that
    ///is not composed of digits. In the case of a list of values, the @type applies to all
    ///elements of the list individually. Note that checking systems may have their own
    ///understanding of data types that may not be identical to the typing indicated in
    ///XCCDF
    #[derive(Debug)]
    pub enum ValueTypeType {
        ///A numeric value. This may be decimal or
        ///integer.
        Number,
        ///Any character data
        String,
        ///True/false
        Boolean,
    }
    /// This type enumerates allowed values of the @operator
    ///property of xccdf:Value elements. The specific interpretation of these
    ///operators depends on the checking system used.
    #[derive(Debug)]
    pub enum ValueOperatorType {
        Equals,
        NotEqual,
        GreaterThan,
        LessThan,
        GreaterThanOrEqual,
        LessThanOrEqual,
        PatternMatch,
    }
    /// Allowed interface hint values. xccdf:Value
    ///elements may contain a hint or recommendation to a benchmark consumer or producer
    ///about how the user might select or adjust the xccdf:Value. This type
    ///enumerates the possible values of this hint.
    #[derive(Debug)]
    pub enum InterfaceHintType {
        ///Multiple choice
        Choice,
        ///Multiple lines of text
        Textline,
        ///Single line of text
        Text,
        ///Date
        Date,
        ///Date and time
        Datetime,
    }
    /// Data type for the xccdf:warning element under
    ///the xccdf:Rule element. This element holds a note or caveat about the item
    ///intended to convey important cautionary information for the xccdf:Benchmark
    ///user.
    #[derive(Debug)]
    pub struct WarningType {
        pub lang: Option<String>,
        ///Used to manage inheritance.
        pub override_: bool,
        ///A hint as to the nature of the
        ///warning.
        pub category: WarningCategoryEnumType,
        pub content: Vec<WarningTypeContent>,
    }
    /// Data type for the xccdf:warning element under
    ///the xccdf:Rule element. This element holds a note or caveat about the item
    ///intended to convey important cautionary information for the xccdf:Benchmark
    ///user.
    #[derive(Debug)]
    pub enum WarningTypeContent {
        ///Specifies an xccdf:Value or
        ///xccdf:plain-text element to be used for text
        ///substitution
        Sub(SubType),
        Any(AnyElement),
    }
    /// This type is for an element that has string content
    ///and a @selector attribute for use in tailoring.
    #[derive(Debug)]
    pub struct SelStringType {
        ///This may be referenced from
        ///xccdf:Profile selection elements or used during manual tailoring
        ///to refine the application of this property. If no selectors are
        ///specified for a given property by xccdf:Profile elements or
        ///manual tailoring, properties with empty or non-existent @selector
        ///attributes are activated. If a selector is applied that does not match
        ///the @selector attribute of any of a given type of property, then no
        ///property of that type is considered activated. The only exception is the
        ///xccdf:value and xccdf:complex-value properties of an
        ///xccdf:Value element - if there is no xccdf:value or
        ///xccdf:complex-value property with a matching @selector value
        ///then the xccdf:value/xccdf:complex-value property with
        ///an empty or absent @selector attribute becomes active. If there is no
        ///such xccdf:value or xccdf:complex-value, then the first
        ///xccdf:value or xccdf:complex-value listed in the XML
        ///becomes active. This reflects the fact that all xccdf:Value
        ///elements require an active value property at all
        ///times.
        pub selector: String,
        pub content: String,
    }
    /// Data type that supports values that are lists of
    ///simple types with an associated @selector attribute used in tailoring.
    #[derive(Debug)]
    pub struct SelComplexValueType {
        ///This may be referenced from
        ///xccdf:Profile selection elements or used during manual tailoring
        ///to refine the application of this property. If no selectors are
        ///specified for a given item by xccdf:Profile elements or manual
        ///tailoring, properties with empty or non-existent @selector attributes
        ///are activated. If a selector is applied that does not match the
        ///@selector attribute of any of a given type of property, then no
        ///xccdf:choices element is considered activated. The only
        ///exception is the xccdf:value and xccdf:complex-value
        ///properties of an xccdf:Value element - if there is no
        ///xccdf:value or xccdf:complex-value property with a
        ///matching @selector value then the
        ///xccdf:value/xccdf:complex-value property with an empty
        ///or absent @selector attribute becomes active. If there is no such
        ///xccdf:value or xccdf:complex-value, then the first
        ///xccdf:value or xccdf:complex-value listed becomes
        ///active. This reflects the fact that all xccdf:Value elements
        ///require an active value property at all times.
        pub selector: String,
        ///A single item in the list of values.
        pub item: Vec<String>,
    }
    /// This type is for an element that has numeric content
    ///and a @selector attribute for use during tailoring.
    #[derive(Debug)]
    pub struct SelNumType {
        ///This may be referenced from
        ///xccdf:Profile selection elements or used during manual tailoring
        ///to refine the application of this property. If no selectors are
        ///specified for a given property by xccdf:Profile elements or
        ///manual tailoring, properties with empty or non-existent @selector
        ///attributes are activated. If a selector is applied that does not match
        ///the @selector attribute of any of a given type of property, then no
        ///property of that type considered activated.
        pub selector: String,
        pub content: f64,
    }
    /// The type of the xccdf:choice element, which
    ///specifies a list of legal or suggested choices for an xccdf:Value object.
    #[derive(Debug)]
    pub struct SelChoicesType {
        ///True if the listed choices are the only permissible
        ///settings for the given xccdf:Value. False if choices not specified in
        ///this xccdf:choices element are acceptable settings for this
        ///xccdf:Value.
        pub must_match: Option<bool>,
        ///This may be referenced from xccdf:Profile
        ///selection elements or used during manual tailoring to refine the application of
        ///the xccdf:Rule. If no selectors are specified for a given
        ///xccdf:Value by xccdf:Profile elements or manual tailoring, an
        ///xccdf:choice element with an empty or non-existent @selector attribute
        ///is activated. If a selector is applied that does not match the @selector
        ///attribute of any xccdf:choices element, then no xccdf:choices
        ///element is considered activated.
        pub selector: String,
        pub content: Vec<SelChoicesTypeContent>,
    }
    /// The type of the xccdf:choice element, which
    ///specifies a list of legal or suggested choices for an xccdf:Value object.
    #[derive(Debug)]
    pub enum SelChoicesTypeContent {
        ///A single choice holding a simple type. (I.e.,
        ///number, string, or boolean.)
        Choice(String),
        ///A single choice holding a list of simple
        ///types.
        ComplexChoice(ComplexValueType),
    }
    /// Data type for elements that have no content and a
    ///single @uri attribute.
    #[derive(Debug)]
    pub struct UriRefType {
        ///A URI.
        pub uri: String,
    }
    /// Data type for elements contain list of references to
    ///other XCCDF elements
    #[derive(Debug)]
    pub struct IdrefListType {
        ///A space-separated list of id values from other
        ///XCCDF elements
        pub idref: super::xs::EntitiesType,
    }
    /// Data type for elements that contain a reference to
    ///another XCCDF element
    #[derive(Debug)]
    pub struct IdrefType {
        ///The id value of another XCCDF
        ///element
        pub idref: String,
    }
    ///Allowed checking and scoring roles for an
    ///xccdf:Rule.
    #[derive(Debug)]
    pub enum RoleEnumType {
        ///If the xccdf:Rule is selected, then
        ///check it and let the result contribute to the score and appear in reports
        ///(default).
        Full,
        ///If the xccdf:Rule is selected, then
        ///check it and include it in the test report, but give the result a status of
        ///informational and do not use the result in score computations.
        Unscored,
        ///Do not check the xccdf:Rule; just force
        ///the result status to notchecked.
        Unchecked,
    }
    ///Allowed severity values for the @severity attribute of
    ///an xccdf:Rule. The value of this attribute provides an indication of the
    ///importance of the xccdf:Rule element's recommendation. This information is
    ///informative only and does not affect scoring.
    #[derive(Debug)]
    pub enum SeverityEnumType {
        ///Severity not defined (default).
        Unknown,
        ///xccdf:Rule is informational and failure
        ///does not represent a problem.
        Info,
        ///Not a serious problem.
        Low,
        ///Fairly serious problem.
        Medium,
        ///A grave or critical problem.
        High,
    }
    /// Data type for the xccdf:ident element, a
    ///globally meaningful identifier for an xccdf:Rule. The body of
    ///xccdf:ident element is the name or identifier of a security configuration
    ///issue or vulnerability that the xccdf:Rule addresses. It has an associated
    ///URI that denotes the organization or naming scheme that assigned the name. By
    ///setting an xccdf:ident element on an xccdf:Rule, the
    ///xccdf:Benchmark author effectively declares that the xccdf:Rule
    ///instantiates, implements, or remediates the issue for which the name was assigned.
    #[derive(Debug)]
    pub struct IdentType {
        ///Denotes the organization or naming scheme
        ///that assigned the identifier.
        pub system: String,
        pub any_attribute: AnyAttributes,
        pub content: String,
    }
    /// Type for an xccdf:profile-note within an
    ///xccdf:Rule. This element contains text that describes special aspects of an
    ///xccdf:Rule relative to one or more xccdf:Profile elements. This
    ///allows an author to document things within xccdf:Rule elements that are
    ///specific to a given xccdf:Profile. This information might then be displayed
    ///to a reader based on the selection of a particular xccdf:Profile. The body
    ///text may include XHTML mark-up as well as xccdf:sub elements.
    #[derive(Debug)]
    pub struct ProfileNoteType {
        pub lang: Option<String>,
        ///The identifier of this note.
        pub tag: String,
        pub content: Vec<ProfileNoteTypeContent>,
    }
    /// Type for an xccdf:profile-note within an
    ///xccdf:Rule. This element contains text that describes special aspects of an
    ///xccdf:Rule relative to one or more xccdf:Profile elements. This
    ///allows an author to document things within xccdf:Rule elements that are
    ///specific to a given xccdf:Profile. This information might then be displayed
    ///to a reader based on the selection of a particular xccdf:Profile. The body
    ///text may include XHTML mark-up as well as xccdf:sub elements.
    #[derive(Debug)]
    pub enum ProfileNoteTypeContent {
        ///Specifies an xccdf:Value or
        ///xccdf:plain-text element to be used for text
        ///substitution
        Sub(SubType),
        Any(AnyElement),
    }
    /// Data type for the xccdf:fixtext element, which
    ///contains data that describes how to bring a target system into compliance with an
    ///xccdf:Rule. Each xccdf:fixtext element may be associated with one or
    ///more xccdf:fix elements through the @fixref attribute. The body holds
    ///explanatory text about the fix procedures.
    #[derive(Debug)]
    pub struct FixTextType {
        pub lang: Option<String>,
        ///Used to manage inheritance.
        pub override_: bool,
        ///A reference to the @id of an
        ///xccdf:fix element.
        pub fixref: Option<String>,
        ///True if a reboot is known to be required
        ///and false otherwise.
        pub reboot: bool,
        ///The method or approach for making the
        ///described fix.
        pub strategy: FixStrategyEnumType,
        ///An estimate of the potential for disruption
        ///or operational degradation that the application of this fix will impose
        ///on the target.
        pub disruption: RatingEnumType,
        ///The estimated complexity or difficulty of
        ///applying the fix to the target.
        pub complexity: RatingEnumType,
        pub content: Vec<FixTextTypeContent>,
    }
    /// Data type for the xccdf:fixtext element, which
    ///contains data that describes how to bring a target system into compliance with an
    ///xccdf:Rule. Each xccdf:fixtext element may be associated with one or
    ///more xccdf:fix elements through the @fixref attribute. The body holds
    ///explanatory text about the fix procedures.
    #[derive(Debug)]
    pub enum FixTextTypeContent {
        ///Specifies an xccdf:Value or
        ///xccdf:plain-text element to be used for text
        ///substitution
        Sub(SubType),
        Any(AnyElement),
    }
    /// Data type for the xccdf:fix element. The body
    ///of this element contains a command string, script, or other system modification
    ///statement that, if executed on the target system, can bring it into full, or at
    ///least better, compliance with this xccdf:Rule.
    #[derive(Debug)]
    pub struct FixType {
        ///A local identifier for the element. It is optional
        ///for the @id to be unique; multiple xccdf:fix elements may have the same
        ///@id but different values for their other attributes. It is used primarily to
        ///allow xccdf:fixtext elements to be associated with one or more
        ///xccdf:fix elements
        pub id: Option<String>,
        ///True if a reboot is known to be required and false
        ///otherwise.
        pub reboot: bool,
        ///The method or approach for making the described
        ///fix.
        pub strategy: FixStrategyEnumType,
        ///An estimate of the potential for disruption or
        ///operational degradation that the application of this fix will impose on the
        ///target.
        pub disruption: RatingEnumType,
        ///The estimated complexity or difficulty of applying
        ///the fix to the target.
        pub complexity: RatingEnumType,
        ///A URI that identifies the scheme, language, engine,
        ///or process for which the fix contents are written. Table 17 in the XCCDF
        ///specification defines several general-purpose URNs that may be used for this,
        ///and tool vendors and system providers may define and use target-specific
        ///URNs.
        pub system: Option<String>,
        ///In case different fix scripts or procedures are
        ///required for different target platform types (e.g., different patches for
        ///Windows Vista and Windows 7), this attribute allows a CPE name or CPE
        ///applicability language expression to be associated with an xccdf:fix
        ///element. This should appear on an xccdf:fix when the content applies to
        ///only one platform out of several to which the xccdf:Rule could apply.
        pub platform: Option<String>,
        pub content: Vec<FixTypeContent>,
    }
    /// Data type for the xccdf:fix element. The body
    ///of this element contains a command string, script, or other system modification
    ///statement that, if executed on the target system, can bring it into full, or at
    ///least better, compliance with this xccdf:Rule.
    #[derive(Debug)]
    pub enum FixTypeContent {
        ///Specifies an xccdf:Value or
        ///xccdf:plain-text element to be used for text substitution
        Sub(SubType),
        ///Designates a spot where the name of the
        ///instance should be substituted into the fix template to generate the final
        ///fix data. If the @context attribute is omitted, the value of the @context
        ///defaults to “undefined”.
        Instance(InstanceFixType),
        Text(Text),
    }
    /// Data type for the xccdf:check element. The
    ///xccdf:check element identifies instructions for tests to determine
    ///compliance with the xccdf:Rule as well as parameters controlling the
    ///reporting of those test results. The xccdf:check element must have at least
    ///one child element.
    #[derive(Debug)]
    pub struct CheckType {
        ///The URI for a checking system. If the checking
        ///system uses XML namespaces, then the system attribute for the system should be
        ///its namespace.
        pub system: String,
        ///If set to true, the final result of the
        ///xccdf:check is negated according to the truth table given below.
        pub negate: bool,
        ///Unique identifier for this element. Optional, but
        ///must be globally unique if present.
        pub id: Option<String>,
        ///This may be referenced from xccdf:Profile
        ///selection elements or used during manual tailoring to refine the application of
        ///the xccdf:Rule. If no selector values are specified for a given
        ///xccdf:Rule by xccdf:Profile elements or manual tailoring, all
        ///xccdf:check elements with non-empty @selector attributes are ignored. If
        ///an xccdf:Rule has multiple xccdf:check elements with the same
        ///@selector attribute, each must employ a different checking system, as identified
        ///by the @system attribute of the xccdf:check element.
        pub selector: String,
        ///Applicable in cases where multiple checks are
        ///executed to determine compliance with a single xccdf:Rule. This
        ///situation can arise when an xccdf:check includes an
        ///xccdf:check-content-ref element that does not include a @name attribute.
        ///The default behavior of a nameless xccdf:check-content-ref is to execute
        ///all checks in the referenced check content location and AND their results
        ///together into a single xccdf:rule-result using the AND truth table
        ///below. This corresponds to a @multi-check attribute value of “false”. If,
        ///however, the @multi-check attribute is set to "true" and a nameless
        ///xccdf:check-content-ref is used, the xccdf:Rule produces a
        ///separate xccdf:rule-result for each check.
        pub multi_check: bool,
        pub base: Option<String>,
        ///Identifies a value to be retrieved from the
        ///checking system during testing of a target system. This element's body must
        ///be empty within an xccdf:check. After the associated check results
        ///have been collected, the result structure returned by the checking engine is
        ///processed to collect the named information. This information is then
        ///recorded in the check-import element in the corresponding
        ///xccdf:rule-result.
        pub check_import: Vec<CheckImportType>,
        ///A mapping from an xccdf:Value element
        ///to a checking system variable (i.e., external name or id for use by the
        ///checking system). This supports export of tailoring values from the XCCDF
        ///processing environment to the checking system.
        pub check_export: Vec<CheckExportType>,
        ///Points to code for a detached check in another
        ///location that uses the language or system specified by the
        ///xccdf:check element’s @system attribute. If multiple
        ///xccdf:check-content-ref elements appear, they represent alternative
        ///locations from which a benchmark consumer may obtain the check content.
        ///Benchmark consumers should process the alternatives in the order in which
        ///they appear in the XML. The first xccdf:check-content-ref from which
        ///content can be successfully retrieved should be used.
        pub check_content_ref: Vec<CheckContentRefType>,
        ///Holds the actual code of a check, in the
        ///language or system specified by the xccdf:check element’s @system
        ///attribute. If both xccdf:check-content-ref and
        ///xccdf:check-content elements appear in a single xccdf:check
        ///element, benchmark consumers should use the xccdf:check-content
        ///element only if none of the references can be resolved to provide
        ///content.
        pub check_content: Option<CheckContentType>,
    }
    /// The type for an element that contains a boolean
    ///combination of xccdf:checks. This element can have only
    ///xccdf:complex-check and xccdf:check elements as children. Child
    ///elements may appear in any order but at least one child element must be present. It
    ///has two attributes, @operator and @negate, which dictate how xccdf:check or
    ///xccdf:complex-check child elements are to be combined. Truth tables for
    ///these operations appear below.
    #[derive(Debug)]
    pub struct ComplexCheckType {
        ///Indicates whether the child xccdf:check
        ///and/or xccdf:complex-check elements of this xccdf:complex-check
        ///should be combined using an AND or OR operation
        pub operator: CcOperatorEnumType,
        ///If true, negate the final result of this
        ///xccdf:complex-check after the child elements are combined using the
        ///identified operator.
        pub negate: bool,
        pub content: Vec<ComplexCheckTypeContent>,
    }
    /// The type for an element that contains a boolean
    ///combination of xccdf:checks. This element can have only
    ///xccdf:complex-check and xccdf:check elements as children. Child
    ///elements may appear in any order but at least one child element must be present. It
    ///has two attributes, @operator and @negate, which dictate how xccdf:check or
    ///xccdf:complex-check child elements are to be combined. Truth tables for
    ///these operations appear below.
    #[derive(Debug)]
    pub enum ComplexCheckTypeContent {
        ///Instructions for a single
        ///test.
        Check(CheckType),
        ///A child xccdf:complex-check, allowing
        ///another level of logic in combining component checks.
        ComplexCheck(ComplexCheckType),
    }
    ///Type for a reference to the xccdf:Benchmark
    ///document.
    #[derive(Debug)]
    pub struct BenchmarkReferenceType {
        ///The URI of the xccdf:Benchmark document.
        pub href: String,
        ///The value of that xccdf:Benchmark element's
        ///@id attribute.
        pub id: Option<String>,
    }
    ///Type for the xccdf:tailoring element within an
    ///xccdf:TestResult. This element is used to indicate the identity and location
    ///of an xccdf:Tailoring file that was used to create the assessment results.
    #[derive(Debug)]
    pub struct TailoringReferenceType {
        ///The URI of the xccdf:Tailoring file's
        ///location.
        pub href: String,
        ///The xccdf:Tailoring element's @id value.
        pub id: String,
        ///The value of the xccdf:Tailoring element's
        ///xccdf:version property.
        pub version: String,
        ///The value of the @time attribute in the
        ///xccdf:Tailoring element's xccdf:version
        ///property.
        pub time: String,
    }
    ///Type for an xccdf:identity element in an
    ///xccdf:TestResult. It contains information about the system identity or user
    ///employed during application of the xccdf:Benchmark. If used, shall specify
    ///the name of the authenticated identity.
    #[derive(Debug)]
    pub struct IdentityType {
        ///Whether the identity was authenticated with
        ///the target system during the application of the xccdf:Benchmark.
        pub authenticated: bool,
        ///Whether the identity was granted
        ///administrative or other special privileges beyond those of a normal
        ///user.
        pub privileged: bool,
        pub content: String,
    }
    /// Data type for the xccdf:target-facts elements
    ///in xccdf:TestResult elements. A xccdf:target-facts element holds a
    ///list of named facts about the target system or platform. Each fact is an element of
    ///type factType. Each xccdf:fact must have a name, but duplicate names are
    ///allowed. (For example, if you had a fact about MAC addresses, and the target system
    ///had three NICs, then you'd need three instances of the "urn:xccdf:fact:ethernet:MAC"
    ///fact.)
    #[derive(Debug)]
    pub struct TargetFactsType {
        ///A named fact about the target system or
        ///platform.
        pub fact: Vec<FactType>,
    }
    ///Type for an xccdf:target-id-ref element in an
    ///xccdf:TestResult element. This element contains references to external
    ///structures with identifying information about the target of an
    ///assessment.
    #[derive(Debug)]
    pub struct TargetIdRefType {
        ///Indicates the language in which this identifying
        ///information is expressed. If the identifying language uses XML namespaces, then
        ///the @system attribute for the language should be its
        ///namespace.
        pub system: String,
        ///Points to the external resource (e.g., a file) that
        ///contains the identifying information.
        pub href: String,
        ///Identifies a specific structure within the
        ///referenced file. If the @name attribute is absent, the reference is to the
        ///entire resource indicated in the @href attribute.
        pub name: Option<String>,
    }
    ///Type for the xccdf:rule-result element within
    ///an xccdf:TestResult. An xccdf:rule-result holds the result of
    ///applying an xccdf:Rule from the xccdf:Benchmark to a target system
    ///or component of a target system.
    #[derive(Debug)]
    pub struct RuleResultType {
        ///The value of the @id property of an
        ///xccdf:Rule. This xccdf:rule-result reflects the result of
        ///applying this xccdf:Rule to a target or target component.
        pub idref: String,
        ///The value of the @role property of the referenced
        ///xccdf:Rule.
        pub role: Option<RoleEnumType>,
        ///The value of the @severity property of the
        ///referenced xccdf:Rule.
        pub severity: Option<SeverityEnumType>,
        ///Time when application of this instance of the
        ///referenced xccdf:Rule was completed.
        pub time: Option<String>,
        ///The value of the @version property of the
        ///referenced xccdf:Rule.
        pub version: Option<String>,
        ///The value of the @weight property of the referenced
        ///xccdf:Rule.
        pub weight: Option<f64>,
        pub content: Vec<RuleResultTypeContent>,
    }
    ///Type for the xccdf:rule-result element within
    ///an xccdf:TestResult. An xccdf:rule-result holds the result of
    ///applying an xccdf:Rule from the xccdf:Benchmark to a target system
    ///or component of a target system.
    #[derive(Debug)]
    pub enum RuleResultTypeContent {
        ///Result of applying the referenced
        ///xccdf:Rule to a target or target component. (E.g., Pass, Fail, etc.)
        Result(ResultEnumType),
        ///An XML block explaining how and why an auditor
        ///chose to override the result.
        Override(OverrideType),
        ///A long-term globally meaningful identifier for
        ///the issue, vulnerability, platform, etc. copied from the referenced
        ///xccdf:Rule.
        Ident(IdentType),
        ///XML metadata associated with this
        ///xccdf:rule-result.
        Metadata(MetadataType),
        ///Diagnostic messages from the checking engine.
        ///These elements do not affect scoring; they are present merely to convey
        ///diagnostic information from the checking engine.
        Message(MessageType),
        ///Name of the target subsystem or component to
        ///which this result applies, for a multiply instantiated xccdf:Rule.
        ///The element is important for an xccdf:Rule that applies to
        ///components of the target system, especially when a target might have several
        ///such components, and where the @multiple attribute of the xccdf:Rule
        ///is set to true.
        Instance(InstanceResultType),
        ///Fix script for this target platform, if
        ///available (would normally appear only for result values of “fail”). It is
        ///assumed to have been ‘instantiated’ by the testing tool and any
        ///substitutions or platform selections already made.
        Fix(FixType),
        ///Encapsulated or referenced results to
        ///detailed testing output from the checking engine (if
        ///any).
        Check(CheckType),
        ///A copy of the xccdf:Rule element’s
        ///xccdf:complex-check element where each component
        ///xccdf:check element of the xccdf:complex-check element
        ///is an encapsulated or referenced results to detailed testing output from
        ///the checking engine (if any) as described in the
        ///xccdf:rule-result xccdf:check
        ///property.
        ComplexCheck(ComplexCheckType),
    }
    ///Type for a score value in an xccdf:TestResult.
    #[derive(Debug)]
    pub struct ScoreType {
        ///A URI indicating the scoring model used to
        ///create this score.
        pub system: Option<String>,
        ///The maximum possible score value that could
        ///have been achieved under the named scoring system.
        pub maximum: Option<f64>,
        pub content: f64,
    }
    ///This holds the possible values of the @use attribute
    ///within an xccdf:sub element. The @use attribute is only applicable with the
    ///subType's @idref attribute holds the value of the @id of an xccdf:Value
    ///element.
    #[derive(Debug)]
    pub enum SubUseEnumType {
        ///Replace with the selected xccdf:value
        ///or xccdf:complex-value of an xccdf:Value.
        Value,
        ///Replace with the xccdf:title of the
        ///xccdf:Value.
        Title,
        ///Use the context-dependent processing of
        ///xccdf:sub elements outlined in XCCDF 1.1.4.
        Legacy,
    }
    /// Allowed warning category keywords for the
    ///xccdf:warning element used in xccdf:Rule elements.
    #[derive(Debug)]
    pub enum WarningCategoryEnumType {
        ///Broad or general-purpose warning
        ///(default)
        General,
        ///Warning about possible impacts to functionality
        ///or operational features
        Functionality,
        ///Warning about changes to target system
        ///performance or throughput
        Performance,
        ///Warning about hardware restrictions or possible
        ///impacts to hardware
        Hardware,
        ///Warning about legal
        ///implications
        Legal,
        ///Warning about regulatory obligations or
        ///compliance implications
        Regulatory,
        ///Warning about impacts to the management or
        ///administration of the target system
        Management,
        ///Warning about impacts to audit or
        ///logging
        Audit,
        ///Warning about dependencies between this element
        ///and other parts of the target system, or version
        ///dependencies
        Dependency,
    }
    ///Data type that supports values that are lists of simple
    ///types. Each element in the list is represented by an instance of the
    ///xccdf:item child element. If there are no xccdf:item child elements
    ///then this represents an empty list.
    #[derive(Debug)]
    pub struct ComplexValueType {
        ///A single item in the list of values.
        pub item: Vec<String>,
    }
    /// Allowed @strategy keyword values for an
    ///xccdf:Rule element's xccdf:fix or xccdf:fixtext elements.
    ///The values indicate the method or approach for fixing non-compliance with a
    ///particular xccdf:Rule.
    #[derive(Debug)]
    pub enum FixStrategyEnumType {
        ///Strategy not defined
        ///(default)
        Unknown,
        ///Adjust target
        ///configuration/settings
        Configure,
        ///Combination of two or more
        ///approaches
        Combination,
        ///Turn off or uninstall a target component
        Disable,
        ///Turn on or install a target
        ///component
        Enable,
        ///Apply a patch, hotfix, update,
        ///etc.
        Patch,
        ///Remediation requires out-of-band adjustments to
        ///policies or procedures
        Policy,
        ///Adjust permissions, access rights, filters, or
        ///other access restrictions
        Restrict,
        ///Install, upgrade or update the
        ///system
        Update,
    }
    /// This type enumerates allowed rating values the
    ///disruption and complexity properties of an xccdf:Rule element's
    ///xccdf:fix or xccdf:fixtext elements.
    #[derive(Debug)]
    pub enum RatingEnumType {
        ///Rating unknown or impossible to estimate
        ///(default)
        Unknown,
        ///Little or no potential for disruption, very
        ///modest complexity
        Low,
        ///Some chance of minor disruption, substantial
        ///complexity
        Medium,
        ///Likely to cause serious disruption, very
        ///complex
        High,
    }
    /// Type for an xccdf:instance element which may
    ///appear in an xccdf:fix element. The xccdf:instance element inside an
    ///xccdf:fix element designates a spot where the name of the instance should be
    ///substituted into the fix template to generate the final fix data.
    #[derive(Debug)]
    pub struct InstanceFixType {
        ///Describes the scope or significance of the instance
        ///content. The context attribute is intended to be informative and does not affect
        ///basic processing.
        pub context: String,
    }
    /// Data type for the xccdf:check-import element,
    ///which specifies a value that the xccdf:Benchmark author wishes to retrieve
    ///from the checking system during testing of a target system. The @import-name
    ///attribute identifies some structure in the checking system that is then retrieved.
    ///The mapping from the values of this attribute to specific checking system structures
    ///is beyond the scope of the XCCDF specification. When the xccdf:check-import
    ///element appears in the context of an xccdf:Rule, then it should be empty and
    ///any content must be ignored. When the xccdf:check-import element appears in
    ///the context of an xccdf:rule-result, then its body holds the imported value.
    #[derive(Debug)]
    pub struct CheckImportType {
        ///An identifier indicating some structure in the
        ///checking system to be collected.
        pub import_name: String,
        ///An XPath that is used to select specific values or
        ///structures from the imported structure. This allows further refinement of the
        ///collected data if the imported value takes the form of XML structures.
        pub import_xpath: Option<String>,
        pub text_before: Option<Text>,
        pub any: Option<AnyElement>,
        pub text_after_any_36: Option<Text>,
    }
    /// Data type for the xccdf:check-export element,
    ///which specifies a mapping from an xccdf:Value element to a checking system
    ///variable (i.e., external name or id for use by the checking system). This supports
    ///export of tailoring xccdf:Value elements from the XCCDF processing
    ///environment to the checking system. The interface between the XCCDF benchmark
    ///consumer and the checking system should support, at a minimum, passing the
    ///xccdf:value property of the xccdf:Value element, but may also
    ///support passing the xccdf:Value element's @type and @operator
    ///properties.
    #[derive(Debug)]
    pub struct CheckExportType {
        ///The id of the xccdf:Value element to
        ///export.
        pub value_id: String,
        ///An identifier indicating some structure in the
        ///checking system into which the identified xccdf:Value element's
        ///properties will be mapped.
        pub export_name: String,
    }
    /// Data type for the xccdf:check-content-ref
    ///element, which points to the code for a detached check in another file. This element
    ///has no body, just a couple of attributes: @href and @name. The @name is optional, if
    ///it does not appear then this reference is to the entire document.
    #[derive(Debug)]
    pub struct CheckContentRefType {
        ///Identifies the referenced document containing
        ///checking instructions.
        pub href: String,
        ///Identifies a particular part or element of the
        ///referenced check document.
        pub name: Option<String>,
    }
    /// Data type for the xccdf:check-content element.
    ///The body of this element holds the actual code of a check, in the language or system
    ///specified by the xccdf:check element’s @system attribute. The body of this
    ///element may be any XML, but cannot contain any XCCDF elements. XCCDF tools do not
    ///process its content directly but instead pass the content directly to checking
    ///engines.
    #[derive(Debug)]
    pub struct CheckContentType {
        pub content: Vec<CheckContentTypeContent>,
    }
    /// Data type for the xccdf:check-content element.
    ///The body of this element holds the actual code of a check, in the language or system
    ///specified by the xccdf:check element’s @system attribute. The body of this
    ///element may be any XML, but cannot contain any XCCDF elements. XCCDF tools do not
    ///process its content directly but instead pass the content directly to checking
    ///engines.
    #[derive(Debug)]
    pub enum CheckContentTypeContent {
        Any(AnyElement),
    }
    /// The type for the allowed @operator names for the
    ///xccdf:complex-check operator attribute. Only AND and OR operators are
    ///supported. (The xccdf:complex-check has a separate mechanism for negation.)
    #[derive(Debug)]
    pub enum CcOperatorEnumType {
        ///The logical OR of the component terms
        Or,
        ///The logical AND of the component
        ///terms
        And,
    }
    /// Data type for an xccdf:fact element, which
    ///holds information about a target system: a name-value pair with a type. The content
    ///of the element is the value, and the @name attribute indicates the name. The @name
    ///is in the form of a URI that indicates the nature of the fact. A table of defined
    ///fact URIs appears in section 6.6.3 of the XCCDF specification. Additional URIs may
    ///be defined by authors to indicate additional kinds of facts.
    #[derive(Debug)]
    pub struct FactType {
        ///A URI that indicates the name of the fact.
        pub name: String,
        ///The data type of the fact value.
        pub type_: ValueTypeType,
        pub content: String,
    }
    ///Allowed result indicators for a
    ///test.
    #[derive(Debug)]
    pub enum ResultEnumType {
        ///The target system or system component satisfied
        ///all the conditions of the xccdf:Rule.
        Pass,
        ///The target system or system component did not
        ///satisfy all the conditions of the xccdf:Rule.
        Fail,
        ///The checking engine could not complete the
        ///evaluation; therefore the status of the target’s compliance with the
        ///xccdf:Rule is not certain. This could happen, for example, if a
        ///testing tool was run with insufficient privileges and could not gather all
        ///of the necessary information.
        Error,
        ///The testing tool encountered some problem and
        ///the result is unknown. For example, a result of ‘unknown’ might be given if
        ///the testing tool was unable to interpret the output of the checking engine
        ///(the output has no meaning to the testing tool).
        Unknown,
        ///The xccdf:Rule was not applicable to
        ///the target of the test. For example, the xccdf:Rule might have been
        ///specific to a different version of the target OS, or it might have been a
        ///test against a platform feature that was not installed.
        Notapplicable,
        ///The xccdf:Rule was not evaluated by the
        ///checking engine. This status is designed for xccdf:Rule elements
        ///that have no check. It may also correspond to a status returned by a
        ///checking engine if the checking engine does not support the indicated check
        ///code.
        Notchecked,
        ///The xccdf:Rule was not selected in the
        ///xccdf:Benchmark.
        Notselected,
        ///The xccdf:Rule was checked, but the
        ///output from the checking engine is simply information for auditors or
        ///administrators; it is not a compliance category. This status value is
        ///designed for xccdf:Rule elements whose main purpose is to extract
        ///information from the target rather than test the target.
        Informational,
        ///The xccdf:Rule had failed, but was then
        ///fixed (possibly by a tool that can automatically apply remediation, or
        ///possibly by the human auditor).
        Fixed,
    }
    /// Type for an xccdf:override element in an
    ///xccdf:rule-result. This element is used to record manual modification or
    ///annotation of a particular xccdf:rule-result. All attributes and child
    ///elements are required. It will not always be the case that the
    ///xccdf:new-result value will differ from the xccdf:old-result value.
    ///They might match if an authority wished to make a remark on the result without
    ///changing it. If xccdf:new-result and xccdf:old-result differ, the
    ///xccdf:result element of the enclosing xccdf:rule-result must match
    ///the xccdf:new-result value.
    #[derive(Debug)]
    pub struct OverrideType {
        ///When the override was applied.
        pub time: String,
        ///Name or other identification for the human
        ///principal authorizing the override.
        pub authority: String,
        ///The xccdf:rule-result status before
        ///this override.
        pub old_result: ResultEnumType,
        ///The new, override xccdf:rule-result
        ///status.
        pub new_result: ResultEnumType,
        ///Rationale or explanation text for why or how
        ///the override was applied.
        pub remark: TextType,
    }
    /// Type for a message generated by the checking engine or
    ///XCCDF tool during xccdf:Benchmark testing. The message is contained in
    ///string format in the body of the element.
    #[derive(Debug)]
    pub struct MessageType {
        ///Denotes the seriousness of the
        ///message.
        pub severity: MsgSevEnumType,
        pub content: String,
    }
    ///Type for an xccdf:instance element in an
    ///xccdf:rule-result. The content is a string, but the element may also have
    ///two attributes: @context and @parentContext. Both attributes are intended to provide
    ///hints as to the nature of the substituted content. This body of this type records
    ///the details of the target system instance for multiply instantiated
    ///xccdf:Rule elements.
    #[derive(Debug)]
    pub struct InstanceResultType {
        ///Describes the scope or significance of the
        ///instance content.
        pub context: String,
        ///Used to express nested structure in
        ///instance context structures.
        pub parent_context: Option<String>,
        pub content: String,
    }
    /// Allowed values to indicate the severity of messages
    ///from the checking engine. These values don't affect scoring themselves but are
    ///present merely to convey diagnostic information from the checking engine. Benchmark
    ///consumers may choose to log these messages or display them to the user.
    #[derive(Debug)]
    pub enum MsgSevEnumType {
        ///Denotes a serious problem identified; test did
        ///not run.
        Error,
        ///Denotes a possible issue; test may not have
        ///run.
        Warning,
        ///Denotes important information about the tests.
        Info,
    }
}
pub mod cpe {
    #[derive(Debug)]
    pub struct PlatformSpecificationType {
        pub platform: Vec<PlatformType>,
    }
    #[derive(Debug)]
    pub struct PlatformType {
        ///A locally unique name for the platform. There is no defined
        ///format for this id; however, it must be unique within the containing CPE Applicability
        ///Language document.
        pub id: String,
        ///A human-readable title for a platform. To support uses intended for
        ///multiple languages, the title element supports the ‘xml:lang’ attribute. At most one title
        ///element can appear for each language.
        pub title: Vec<TextType>,
        ///An additional description. To support uses intended for multiple
        ///languages, the remark element supports the ‘xml:lang’ attribute. There can be multiple remarks
        ///for a single language.
        pub remark: Vec<TextType>,
        ///Definition of test using logical operators (AND, OR,
        ///negate).
        pub logical_test: LogicalTestType,
    }
    ///This type allows the xml:lang attribute to associate a specific language
    ///with an element's string content.
    #[derive(Debug)]
    pub struct TextType {
        pub lang: Option<String>,
        pub content: String,
    }
    ///The logical-test element appears as a child of a platform element, and may
    ///also be nested to create more complex logical tests. The content consists of one or more elements:
    ///fact-ref, check-fact-ref, and logical-test children are permitted. The operator to be applied, and
    ///optional negation of the test, are given as attributes.
    #[derive(Debug)]
    pub struct LogicalTestType {
        ///The operator applied to the results of evaluating the fact-ref,
        ///check-fact-ref, and logical-test elements. The permitted operators are "AND" and
        ///"OR".
        pub operator: OperatorEnumerationType,
        ///Whether the result of applying the operator should be negated. Possible
        ///values are "TRUE" and "FALSE". This does not apply if the initial result is
        ///ERROR.
        pub negate: bool,
        ///Definition of complex logical test using AND, OR, and/or negate
        ///operators. Evaluates to a TRUE, FALSE, or ERROR result.
        pub logical_test: Vec<LogicalTestType>,
        ///A reference to a bound form of a WFN; the reference always
        ///evaluates to a boolean result. The bound name contained within a fact-ref is meant to describe a
        ///possible set of products and is not meant to identify a unique product
        ///class.
        pub fact_ref: Vec<CpeFactRefType>,
        ///A reference to a check that always evaluates to TRUE, FALSE, or
        ///ERROR. Examples of types of checks are OVAL and OCIL checks.
        pub check_fact_ref: Vec<CheckFactRefType>,
    }
    ///The OperatorEnumeration simple type defines acceptable operators. Each
    ///operator defines how to evaluate multiple arguments.
    #[derive(Debug)]
    pub enum OperatorEnumerationType {
        And,
        Or,
    }
    ///A reference to a CPE Name that always evaluates to a Boolean
    ///result.
    #[derive(Debug)]
    pub struct CpeFactRefType {
        pub description: Option<String>,
        pub name: String,
    }
    ///A reference to a check that always evaluates to a TRUE, FALSE, or ERROR
    ///result.
    ///The CheckFactRefType complex type is used to define an element for holding
    ///information about an individual check. It includes a checking system specification URI, string content
    ///identifying the check content to invoke, and an external reference. The checking system specification
    ///should be the URI that uniquely identifies a revision of a check system language, and the id-ref will be
    ///an identifier of a test written in that language. The external reference should be used to point to the
    ///content in which the check identifier is defined.
    #[derive(Debug)]
    pub struct CheckFactRefType {
        pub description: Option<String>,
        pub system: String,
        pub href: String,
        pub id_ref: String,
    }
}
pub mod xs {
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<String>);
}
