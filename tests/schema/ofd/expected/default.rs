pub mod annotations {
    pub type Annotations = AnnotationsXElementType;
    #[derive(Debug)]
    pub struct AnnotationsXElementType {
        pub page: Vec<AnnotationsPageXElementType>,
    }
    #[derive(Debug)]
    pub struct AnnotationsPageXElementType {
        pub page_id: ::core::primitive::u32,
        pub file_loc: ::std::string::String,
    }
}
pub mod annotion {
    pub type PageAnnot = PageAnnotXElementType;
    #[derive(Debug)]
    pub struct PageAnnotXElementType {
        pub annot: Vec<PageAnnotAnnotXElementType>,
    }
    #[derive(Debug)]
    pub struct PageAnnotAnnotXElementType {
        pub id: ::core::primitive::u32,
        pub type_: PageAnnotAnnotTypeXType,
        pub creator: ::std::string::String,
        pub last_mod_date: ::std::string::String,
        pub visible: ::core::primitive::bool,
        pub subtype: Option<::std::string::String>,
        pub print: ::core::primitive::bool,
        pub no_zoom: ::core::primitive::bool,
        pub no_rotate: ::core::primitive::bool,
        pub read_only: ::core::primitive::bool,
        pub remark: Option<::std::string::String>,
        pub parameters: Option<PageAnnotAnnotParametersXElementType>,
        pub appearance: PageAnnotAnnotAppearanceXElementType,
    }
    #[derive(Debug)]
    pub enum PageAnnotAnnotTypeXType {
        Link,
        Path,
        Highlight,
        Stamp,
        Watermark,
    }
    #[derive(Debug)]
    pub struct PageAnnotAnnotParametersXElementType {
        pub parameter: Vec<super::ofd::CtDocInfoCustomDatasCustomDataXElementType>,
    }
    #[derive(Debug)]
    pub struct PageAnnotAnnotAppearanceXElementType {
        pub boundary: Option<::std::string::String>,
        pub content: Vec<PageAnnotAnnotAppearanceXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum PageAnnotAnnotAppearanceXElementTypeContent {
        TextObject(super::page::CtPageBlockTextObjectXElementType),
        PathObject(super::page::CtPageBlockPathObjectXElementType),
        ImageObject(super::page::CtPageBlockImageObjectXElementType),
        CompositeObject(super::page::CtPageBlockCompositeObjectXElementType),
        PageBlock(super::page::CtPageBlockPageBlockXElementType),
    }
}
pub mod attachments {
    pub type Attachments = AttachmentsXElementType;
    #[derive(Debug)]
    pub struct AttachmentsXElementType {
        pub attachment: Vec<CtAttachmentXType>,
    }
    #[derive(Debug)]
    pub struct CtAttachmentXType {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub format: Option<::std::string::String>,
        pub creation_date: Option<::std::string::String>,
        pub mod_date: Option<::std::string::String>,
        pub size: Option<::core::primitive::f64>,
        pub visible: ::core::primitive::bool,
        pub usage: ::std::string::String,
        pub file_loc: ::std::string::String,
    }
}
pub mod custom_tags {
    pub type CustomTags = CustomTagsXElementType;
    #[derive(Debug)]
    pub struct CustomTagsXElementType {
        pub custom_tag: Vec<CustomTagsCustomTagXElementType>,
    }
    #[derive(Debug)]
    pub struct CustomTagsCustomTagXElementType {
        pub name_space: ::std::string::String,
        pub schema_loc: Option<::std::string::String>,
        pub file_loc: ::std::string::String,
    }
}
pub mod definition {
    #[derive(Debug)]
    pub struct CtActionXType {
        pub event: CtActionEventXType,
        pub content: Vec<CtActionXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtActionXTypeContent {
        Region(CtRegionXType),
        Goto(CtActionGotoXElementType),
        Uri(CtActionUriXElementType),
        GotoA(CtActionGotoAxElementType),
        Sound(CtActionSoundXElementType),
        Movie(CtActionMovieXElementType),
    }
    #[derive(Debug)]
    pub struct CtDestXType {
        pub type_: CtDestTypeXType,
        pub page_id: ::core::primitive::u32,
        pub left: Option<::core::primitive::f64>,
        pub top: Option<::core::primitive::f64>,
        pub right: Option<::core::primitive::f64>,
        pub bottom: Option<::core::primitive::f64>,
        pub zoom: Option<::core::primitive::f64>,
    }
    #[derive(Debug)]
    pub struct CtPageAreaXType {
        pub physical_box: ::std::string::String,
        pub application_box: Option<::std::string::String>,
        pub content_box: Option<::std::string::String>,
        pub bleed_box: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtRegionXType {
        pub area: Vec<CtRegionAreaXElementType>,
    }
    pub type StArrayXType = ::std::string::String;
    pub type StBoxXType = ::std::string::String;
    pub type StIdXType = ::core::primitive::u32;
    pub type StLocXType = ::std::string::String;
    pub type StPosXType = ::std::string::String;
    pub type StRefIdXType = ::core::primitive::u32;
    #[derive(Debug)]
    pub enum CtActionEventXType {
        Do,
        Po,
        Click,
    }
    #[derive(Debug)]
    pub enum CtActionGotoXElementType {
        Dest(CtDestXType),
        Bookmark(CtActionGotoBookmarkXElementType),
    }
    #[derive(Debug)]
    pub struct CtActionUriXElementType {
        pub uri: ::std::string::String,
        pub base: Option<::std::string::String>,
        pub target: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtActionGotoAxElementType {
        pub attach_id: ::std::string::String,
        pub new_window: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub struct CtActionSoundXElementType {
        pub resource_id: ::core::primitive::u32,
        pub volume: Option<::core::primitive::i32>,
        pub repeat: Option<::core::primitive::bool>,
        pub synchronous: Option<::core::primitive::bool>,
    }
    #[derive(Debug)]
    pub struct CtActionMovieXElementType {
        pub resource_id: ::core::primitive::u32,
        pub operator: CtActionMovieOperatorXType,
    }
    #[derive(Debug)]
    pub enum CtDestTypeXType {
        Xyz,
        Fit,
        FitH,
        FitV,
        FitR,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaXElementType {
        pub start: ::std::string::String,
        pub content: Vec<CtRegionAreaXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtRegionAreaXElementTypeContent {
        Move(CtRegionAreaLineXElementType),
        Line(CtRegionAreaLineXElementType),
        OuadraticBezier(CtRegionAreaOuadraticBezierXElementType),
        CubicBezier(CtRegionAreaCubicBezierXElementType),
        Arc(CtRegionAreaArcXElementType),
        Close(super::xs::AnyTypeXType),
    }
    #[derive(Debug)]
    pub struct CtActionGotoBookmarkXElementType {
        pub name: ::std::string::String,
    }
    #[derive(Debug)]
    pub enum CtActionMovieOperatorXType {
        Play,
        Stop,
        Pause,
        Resume,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaLineXElementType {
        pub point_1: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaOuadraticBezierXElementType {
        pub pointl: ::std::string::String,
        pub point_2: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaCubicBezierXElementType {
        pub point_1: Option<::std::string::String>,
        pub point_2: Option<::std::string::String>,
        pub point_3: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaArcXElementType {
        pub sweep_direction: ::core::primitive::bool,
        pub large_arc: ::core::primitive::bool,
        pub rotation_anglet: ::core::primitive::f64,
        pub ellipse_size: ::std::string::String,
        pub end_point: ::std::string::String,
    }
}
pub mod document {
    #[derive(Debug)]
    pub struct CtBookmarkXType {
        pub name: ::std::string::String,
        pub dest: super::definition::CtDestXType,
    }
    #[derive(Debug)]
    pub struct CtOutlineElemXType {
        pub title: ::std::string::String,
        pub count: Option<::core::primitive::i32>,
        pub expanded: ::core::primitive::bool,
        pub actions: Option<super::page::CtGraphicUnitActionsXElementType>,
        pub outline_elem: Vec<CtOutlineElemXType>,
    }
    #[derive(Debug)]
    pub struct CtPermissionXType {
        pub edit: Option<::core::primitive::bool>,
        pub annot: Option<::core::primitive::bool>,
        pub export: Option<::core::primitive::bool>,
        pub signature: Option<::core::primitive::bool>,
        pub watermark: Option<::core::primitive::bool>,
        pub print_screen: Option<::core::primitive::bool>,
        pub print: Option<CtPermissionPrintXElementType>,
        pub valid_period: Option<CtPermissionValidPeriodXElementType>,
    }
    #[derive(Debug)]
    pub struct CtVPreferencesXType {
        pub content: Vec<CtVPreferencesXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtVPreferencesXTypeContent {
        PageMode(CtVPreferencesPageModeXElementType),
        PageLayout(CtVPreferencesPageLayoutXElementType),
        TabDisplay(CtVPreferencesTabDisplayXElementType),
        HideToolbar(::core::primitive::bool),
        HideMenubar(::core::primitive::bool),
        HideWindowUi(::core::primitive::bool),
        ZoomMode(CtVPreferencesZoomModeXElementType),
        Zoom(::core::primitive::f64),
    }
    pub type Document = DocumentXElementType;
    #[derive(Debug)]
    pub struct DocumentXElementType {
        pub common_data: DocumentCommonDataXElementType,
        pub pages: DocumentPagesXElementType,
        pub outlines: Option<DocumentOutlinesXElementType>,
        pub permissions: Option<CtPermissionXType>,
        pub actions: Option<super::page::CtGraphicUnitActionsXElementType>,
        pub v_preferences: Option<CtVPreferencesXType>,
        pub bookmarks: Option<DocumentBookmarksXElementType>,
        pub annotations: Option<::std::string::String>,
        pub custom_tags: Option<::std::string::String>,
        pub attachments: Option<::std::string::String>,
        pub extensions: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtPermissionPrintXElementType {
        pub printable: ::core::primitive::bool,
        pub copies: ::core::primitive::i32,
    }
    #[derive(Debug)]
    pub struct CtPermissionValidPeriodXElementType {
        pub start_date: Option<::std::string::String>,
        pub end_date: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub enum CtVPreferencesPageModeXElementType {
        None,
        FullScreen,
        UseOutlines,
        UseThumbs,
        UseCustomTags,
        UseLayers,
        UseAttatchs,
        UseBookmarks,
    }
    #[derive(Debug)]
    pub enum CtVPreferencesPageLayoutXElementType {
        OnePage,
        OneColumn,
        TwoPageL,
        TwoColumnL,
        TwoPageR,
        TwoColumnR,
    }
    #[derive(Debug)]
    pub enum CtVPreferencesTabDisplayXElementType {
        DocTitle,
        FileName,
    }
    #[derive(Debug)]
    pub enum CtVPreferencesZoomModeXElementType {
        Default,
        FitHeight,
        FitWidth,
        FitRect,
    }
    #[derive(Debug)]
    pub struct DocumentCommonDataXElementType {
        pub max_unit_id: ::core::primitive::u32,
        pub page_area: super::definition::CtPageAreaXType,
        pub public_res: Vec<::std::string::String>,
        pub document_res: Vec<::std::string::String>,
        pub template_page: Vec<DocumentCommonDataTemplatePageXElementType>,
        pub default_cs: Option<::core::primitive::u32>,
    }
    #[derive(Debug)]
    pub struct DocumentPagesXElementType {
        pub page: Vec<DocumentPagesPageXElementType>,
    }
    #[derive(Debug)]
    pub struct DocumentOutlinesXElementType {
        pub outline_elem: Vec<CtOutlineElemXType>,
    }
    #[derive(Debug)]
    pub struct DocumentBookmarksXElementType {
        pub bookmark: Vec<CtBookmarkXType>,
    }
    #[derive(Debug)]
    pub struct DocumentCommonDataTemplatePageXElementType {
        pub id: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub z_order: Option<DocumentCommonDataTemplatePageZOrderXType>,
        pub base_loc: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct DocumentPagesPageXElementType {
        pub id: ::core::primitive::u32,
        pub base_loc: ::std::string::String,
    }
    #[derive(Debug)]
    pub enum DocumentCommonDataTemplatePageZOrderXType {
        Background,
        Foreground,
    }
}
pub mod extensions {
    #[derive(Debug)]
    pub struct CtExtensionXType {
        pub app_name: ::std::string::String,
        pub company: Option<::std::string::String>,
        pub app_version: Option<::std::string::String>,
        pub date: Option<::std::string::String>,
        pub ref_id: ::core::primitive::u32,
        pub content: Vec<CtExtensionXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtExtensionXTypeContent {
        Property(CtExtensionPropertyXElementType),
        Data(super::xs::AnyTypeXType),
        ExtendData(::std::string::String),
    }
    pub type Extensions = ExtensionsXElementType;
    #[derive(Debug)]
    pub struct ExtensionsXElementType {
        pub extension: Vec<CtExtensionXType>,
    }
    #[derive(Debug)]
    pub struct CtExtensionPropertyXElementType {
        pub name: ::std::string::String,
        pub type_: Option<::std::string::String>,
        pub content: ::std::string::String,
    }
}
pub mod ofd {
    #[derive(Debug)]
    pub struct CtDocInfoXType {
        pub doc_id: ::std::string::String,
        pub title: Option<::std::string::String>,
        pub author: Option<::std::string::String>,
        pub subject: Option<::std::string::String>,
        pub abstract_: Option<::std::string::String>,
        pub creation_date: Option<::std::string::String>,
        pub mod_date: Option<::std::string::String>,
        pub doc_usage: Option<::std::string::String>,
        pub cover: Option<::std::string::String>,
        pub keywords: Option<CtDocInfoKeywordsXElementType>,
        pub creator: Option<::std::string::String>,
        pub creator_version: Option<::std::string::String>,
        pub custom_datas: Option<CtDocInfoCustomDatasXElementType>,
    }
    pub type Ofd = OfdXElementType;
    #[derive(Debug)]
    pub struct OfdXElementType {
        pub version: ::std::string::String,
        pub doc_type: OfdDocTypeXType,
        pub doc_body: Vec<OfdDocBodyXElementType>,
    }
    #[derive(Debug)]
    pub struct CtDocInfoKeywordsXElementType {
        pub keyword: Vec<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtDocInfoCustomDatasXElementType {
        pub custom_data: Vec<CtDocInfoCustomDatasCustomDataXElementType>,
    }
    #[derive(Debug)]
    pub enum OfdDocTypeXType {
        Ofd,
    }
    #[derive(Debug)]
    pub struct OfdDocBodyXElementType {
        pub doc_info: CtDocInfoXType,
        pub doc_root: ::std::string::String,
        pub versions: Option<OfdDocBodyVersionsXElementType>,
        pub signatures: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtDocInfoCustomDatasCustomDataXElementType {
        pub name: ::std::string::String,
        pub content: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct OfdDocBodyVersionsXElementType {
        pub version: Vec<OfdDocBodyVersionsVersionXElementType>,
    }
    #[derive(Debug)]
    pub struct OfdDocBodyVersionsVersionXElementType {
        pub id: ::std::string::String,
        pub index: ::core::primitive::i32,
        pub current: ::core::primitive::bool,
        pub base_loc: ::std::string::String,
    }
}
pub mod page {
    #[derive(Debug)]
    pub struct CtAxialShdXType {
        pub map_type: CtAxialShdMapTypeXType,
        pub map_unit: Option<::core::primitive::f64>,
        pub extend: CtAxialShdExtendXType,
        pub start_point: ::std::string::String,
        pub end_point: ::std::string::String,
        pub segment: Vec<CtAxialShdSegmentXElementType>,
    }
    #[derive(Debug)]
    pub struct CtCgTransformXType {
        pub code_position: ::core::primitive::i32,
        pub code_count: ::core::primitive::i32,
        pub glyph_count: ::core::primitive::i32,
        pub glyphs: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtClipXType {
        pub area: Vec<CtClipAreaXElementType>,
    }
    #[derive(Debug)]
    pub struct CtColorXType {
        pub value: Option<::std::string::String>,
        pub index: Option<::core::primitive::i32>,
        pub color_space: Option<::core::primitive::u32>,
        pub alpha: ::core::primitive::i32,
        pub content: Option<CtColorXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtColorXTypeContent {
        Pattern(CtPatternXType),
        AxialShd(CtAxialShdXType),
        RadialShd(CtRadialShdXType),
        GouraudShd(Box<CtGouraudShdXType>),
        LaGourandShd(Box<CtLaGouraudShdXType>),
    }
    #[derive(Debug)]
    pub struct CtCompositeXType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub resource_id: ::core::primitive::u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
    }
    #[derive(Debug)]
    pub struct CtGouraudShdXType {
        pub extend: Option<::core::primitive::i32>,
        pub point: Vec<CtGouraudShdPointXElementType>,
        pub back_color: Option<Box<CtColorXType>>,
    }
    #[derive(Debug)]
    pub struct CtGraphicUnitXType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
    }
    #[derive(Debug)]
    pub struct CtImageXType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub resource_id: ::core::primitive::u32,
        pub substitution: Option<::core::primitive::u32>,
        pub image_mask: Option<::core::primitive::u32>,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub border: Option<CtImageBorderXElementType>,
    }
    #[derive(Debug)]
    pub struct CtLaGouraudShdXType {
        pub vertices_per_row: ::core::primitive::i32,
        pub extend: Option<::core::primitive::i32>,
        pub point: Vec<CtLaGouraudShdPointXElementType>,
        pub back_color: Option<Box<CtColorXType>>,
    }
    #[derive(Debug)]
    pub struct CtLayerXType {
        pub type_: CtLayerTypeXType,
        pub draw_param: Option<::core::primitive::u32>,
        pub content: Vec<CtLayerXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtLayerXTypeContent {
        TextObject(CtPageBlockTextObjectXElementType),
        PathObject(CtPageBlockPathObjectXElementType),
        ImageObject(CtPageBlockImageObjectXElementType),
        CompositeObject(CtPageBlockCompositeObjectXElementType),
        PageBlock(CtPageBlockPageBlockXElementType),
    }
    #[derive(Debug)]
    pub struct CtPageBlockXType {
        pub content: Vec<CtPageBlockXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtPageBlockXTypeContent {
        TextObject(CtPageBlockTextObjectXElementType),
        PathObject(CtPageBlockPathObjectXElementType),
        ImageObject(CtPageBlockImageObjectXElementType),
        CompositeObject(CtPageBlockCompositeObjectXElementType),
        PageBlock(CtPageBlockPageBlockXElementType),
    }
    #[derive(Debug)]
    pub struct CtPathXType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub stroke: ::core::primitive::bool,
        pub fill: ::core::primitive::bool,
        pub rule: CtPathRuleXType,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub stroke_color: Option<CtColorXType>,
        pub fill_color: Option<CtColorXType>,
        pub abbreviated_data: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct CtPatternXType {
        pub width: ::core::primitive::f64,
        pub height: ::core::primitive::f64,
        pub x_step: Option<::core::primitive::f64>,
        pub y_step: Option<::core::primitive::f64>,
        pub reflect_method: CtPatternReflectMethodXType,
        pub relative_to: CtPatternRelativeToXType,
        pub ctm: Option<::std::string::String>,
        pub cell_content: CtPatternCellContentXElementType,
    }
    #[derive(Debug)]
    pub struct CtRadialShdXType {
        pub map_type: CtAxialShdMapTypeXType,
        pub map_unit: Option<::core::primitive::f64>,
        pub eccentricity: ::core::primitive::f64,
        pub angle: ::core::primitive::f64,
        pub start_point: ::std::string::String,
        pub start_radius: ::core::primitive::f64,
        pub end_point: ::std::string::String,
        pub end_radius: ::core::primitive::f64,
        pub extend: ::core::primitive::i32,
        pub seqment: Vec<CtAxialShdSegmentXElementType>,
    }
    #[derive(Debug)]
    pub struct CtTextXType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub font: ::core::primitive::u32,
        pub size: ::core::primitive::f64,
        pub stroke: ::core::primitive::bool,
        pub fill: ::core::primitive::bool,
        pub h_scale: ::core::primitive::f64,
        pub read_direction: ::core::primitive::i32,
        pub char_direction: ::core::primitive::i32,
        pub weight: CtTextWeightXType,
        pub italic: ::core::primitive::bool,
        pub content: Vec<CtTextXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtTextXTypeContent {
        Actions(CtGraphicUnitActionsXElementType),
        Clips(CtGraphicUnitClipsXElementType),
        FillColor(CtColorXType),
        StrokeColor(CtColorXType),
        CgTransform(CtCgTransformXType),
        TextCode(CtTextTextCodeXElementType),
    }
    pub type Page = PageXElementType;
    #[derive(Debug)]
    pub struct PageXElementType {
        pub template: Vec<PageTemplateXElementType>,
        pub page_res: Vec<::std::string::String>,
        pub area: Option<super::definition::CtPageAreaXType>,
        pub content: Option<PageContentXElementType>,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
    }
    #[derive(Debug)]
    pub enum CtAxialShdMapTypeXType {
        Direct,
        Repeat,
        Reflect,
    }
    #[derive(Debug)]
    pub enum CtAxialShdExtendXType {
        _0,
        _1,
        _2,
        _3,
    }
    #[derive(Debug)]
    pub struct CtAxialShdSegmentXElementType {
        pub position: Option<::core::primitive::f64>,
        pub color: CtColorXType,
    }
    #[derive(Debug)]
    pub struct CtClipAreaXElementType {
        pub draw_param: Option<::core::primitive::u32>,
        pub ctm: Option<::std::string::String>,
        pub content: CtClipAreaXElementTypeContent,
    }
    #[derive(Debug)]
    pub enum CtClipAreaXElementTypeContent {
        Path(CtPathXType),
        Text(CtTextXType),
    }
    #[derive(Debug)]
    pub enum CtGraphicUnitCapXType {
        Butt,
        Round,
        Square,
    }
    #[derive(Debug)]
    pub enum CtGraphicUnitJoinXType {
        Miter,
        Round,
        Bevel,
    }
    #[derive(Debug)]
    pub struct CtGraphicUnitActionsXElementType {
        pub action: Vec<super::definition::CtActionXType>,
    }
    #[derive(Debug)]
    pub struct CtGraphicUnitClipsXElementType {
        pub clip: Vec<CtClipXType>,
    }
    #[derive(Debug)]
    pub struct CtGouraudShdPointXElementType {
        pub x: ::core::primitive::f64,
        pub y: ::core::primitive::f64,
        pub edge_flag: Option<CtGouraudShdPointEdgeFlagXType>,
        pub color: CtColorXType,
    }
    #[derive(Debug)]
    pub struct CtImageBorderXElementType {
        pub line_width: ::core::primitive::f64,
        pub horizonal_corner_radius: ::core::primitive::f64,
        pub vertical_corner_radius: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub border_color: Option<CtColorXType>,
    }
    #[derive(Debug)]
    pub struct CtLaGouraudShdPointXElementType {
        pub x: Option<::core::primitive::f64>,
        pub y: Option<::core::primitive::f64>,
        pub color: CtColorXType,
    }
    #[derive(Debug)]
    pub enum CtLayerTypeXType {
        Body,
        Background,
        Foreground,
        Custom,
    }
    #[derive(Debug)]
    pub struct CtPageBlockTextObjectXElementType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub font: ::core::primitive::u32,
        pub size: ::core::primitive::f64,
        pub stroke: ::core::primitive::bool,
        pub fill: ::core::primitive::bool,
        pub h_scale: ::core::primitive::f64,
        pub read_direction: ::core::primitive::i32,
        pub char_direction: ::core::primitive::i32,
        pub weight: CtTextWeightXType,
        pub italic: ::core::primitive::bool,
        pub id: ::core::primitive::u32,
        pub content: Vec<CtPageBlockTextObjectXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtPageBlockTextObjectXElementTypeContent {
        Actions(CtGraphicUnitActionsXElementType),
        Clips(CtGraphicUnitClipsXElementType),
        FillColor(CtColorXType),
        StrokeColor(CtColorXType),
        CgTransform(CtCgTransformXType),
        TextCode(CtTextTextCodeXElementType),
    }
    #[derive(Debug)]
    pub struct CtPageBlockPathObjectXElementType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub stroke: ::core::primitive::bool,
        pub fill: ::core::primitive::bool,
        pub rule: CtPathRuleXType,
        pub id: ::core::primitive::u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub stroke_color: Option<CtColorXType>,
        pub fill_color: Option<CtColorXType>,
        pub abbreviated_data: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct CtPageBlockImageObjectXElementType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub resource_id: ::core::primitive::u32,
        pub substitution: Option<::core::primitive::u32>,
        pub image_mask: Option<::core::primitive::u32>,
        pub id: ::core::primitive::u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub border: Option<CtImageBorderXElementType>,
    }
    #[derive(Debug)]
    pub struct CtPageBlockCompositeObjectXElementType {
        pub boundary: ::std::string::String,
        pub name: Option<::std::string::String>,
        pub visible: ::core::primitive::bool,
        pub ctm: Option<::std::string::String>,
        pub draw_param: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: ::core::primitive::f64,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub alpha: ::core::primitive::i32,
        pub resource_id: ::core::primitive::u32,
        pub id: ::core::primitive::u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
    }
    #[derive(Debug)]
    pub struct CtPageBlockPageBlockXElementType {
        pub id: ::core::primitive::u32,
        pub content: Vec<CtPageBlockPageBlockXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtPageBlockPageBlockXElementTypeContent {
        TextObject(CtPageBlockTextObjectXElementType),
        PathObject(CtPageBlockPathObjectXElementType),
        ImageObject(CtPageBlockImageObjectXElementType),
        CompositeObject(CtPageBlockCompositeObjectXElementType),
        PageBlock(CtPageBlockPageBlockXElementType),
    }
    #[derive(Debug)]
    pub enum CtPathRuleXType {
        NonZero,
        EvenOdd,
    }
    #[derive(Debug)]
    pub enum CtPatternReflectMethodXType {
        Normal,
        Row,
        Column,
        RowAndColumn,
    }
    #[derive(Debug)]
    pub enum CtPatternRelativeToXType {
        Page,
        Object,
    }
    #[derive(Debug)]
    pub struct CtPatternCellContentXElementType {
        pub thumbnail: Option<::core::primitive::u32>,
        pub content: Vec<CtPatternCellContentXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtPatternCellContentXElementTypeContent {
        TextObject(CtPageBlockTextObjectXElementType),
        PathObject(CtPageBlockPathObjectXElementType),
        ImageObject(CtPageBlockImageObjectXElementType),
        CompositeObject(CtPageBlockCompositeObjectXElementType),
        PageBlock(CtPageBlockPageBlockXElementType),
    }
    #[derive(Debug)]
    pub enum CtTextWeightXType {
        _0,
        _100,
        _200,
        _300,
        _400,
        _500,
        _600,
        _700,
        _800,
        _900,
        _1000,
    }
    #[derive(Debug)]
    pub struct CtTextTextCodeXElementType {
        pub x: Option<::core::primitive::f64>,
        pub y: Option<::core::primitive::f64>,
        pub delta_x: Option<::std::string::String>,
        pub deltay: Option<::std::string::String>,
        pub content: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct PageTemplateXElementType {
        pub template_id: ::core::primitive::u32,
        pub z_order: PageTemplateZOrderXType,
    }
    #[derive(Debug)]
    pub struct PageContentXElementType {
        pub layer: Vec<PageContentLayerXElementType>,
    }
    #[derive(Debug)]
    pub enum CtGouraudShdPointEdgeFlagXType {
        _0,
        _1,
        _2,
    }
    #[derive(Debug)]
    pub enum PageTemplateZOrderXType {
        Backqround,
        Foreground,
    }
    #[derive(Debug)]
    pub struct PageContentLayerXElementType {
        pub type_: CtLayerTypeXType,
        pub draw_param: Option<::core::primitive::u32>,
        pub id: ::core::primitive::u32,
        pub content: Vec<PageContentLayerXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum PageContentLayerXElementTypeContent {
        TextObject(CtPageBlockTextObjectXElementType),
        PathObject(CtPageBlockPathObjectXElementType),
        ImageObject(CtPageBlockImageObjectXElementType),
        CompositeObject(CtPageBlockCompositeObjectXElementType),
        PageBlock(CtPageBlockPageBlockXElementType),
    }
}
pub mod res {
    #[derive(Debug)]
    pub struct CtColorSpaceXType {
        pub type_: CtColorSpaceTypeXType,
        pub bits_per_component: ::core::primitive::i32,
        pub profile: Option<::std::string::String>,
        pub palette: Option<CtColorSpacePaletteXElementType>,
    }
    #[derive(Debug)]
    pub struct CtDrawParamXType {
        pub relative: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub join: ::std::string::String,
        pub cap: ::std::string::String,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub miter_limit: ::core::primitive::f64,
        pub fill_color: Option<super::page::CtColorXType>,
        pub stroke_color: Option<super::page::CtColorXType>,
    }
    #[derive(Debug)]
    pub struct CtFontXType {
        pub font_name: ::std::string::String,
        pub family_name: Option<::std::string::String>,
        pub charset: CtFontCharsetXType,
        pub italic: ::core::primitive::bool,
        pub bold: ::core::primitive::bool,
        pub serif: ::core::primitive::bool,
        pub fixed_width: ::core::primitive::bool,
        pub font_file: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct CtMultiMediaXType {
        pub type_: CtMultiMediaTypeXType,
        pub format: Option<::std::string::String>,
        pub media_file: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct CtVectorGxType {
        pub width: ::core::primitive::f64,
        pub height: ::core::primitive::f64,
        pub thumbnail: Option<::core::primitive::u32>,
        pub substitution: Option<::core::primitive::u32>,
        pub content: super::page::CtPageBlockXType,
    }
    pub type Res = ResXElementType;
    #[derive(Debug)]
    pub struct ResXElementType {
        pub base_loc: ::std::string::String,
        pub content: Vec<ResXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum ResXElementTypeContent {
        ColorSpaces(ResColorSpacesXElementType),
        DrawParams(ResDrawParamsXElementType),
        Fonts(ResFontsXElementType),
        MultiMedias(ResMultiMediasXElementType),
        CompositeGraphicUnits(ResCompositeGraphicUnitsXElementType),
    }
    #[derive(Debug)]
    pub enum CtColorSpaceTypeXType {
        Gray,
        Rgb,
        Cmyk,
    }
    #[derive(Debug)]
    pub struct CtColorSpacePaletteXElementType {
        pub cv: Vec<::std::string::String>,
    }
    #[derive(Debug)]
    pub enum CtFontCharsetXType {
        Symbol,
        Prc,
        Big5,
        ShiftIis,
        Wansung,
        Johab,
        Unicode,
    }
    #[derive(Debug)]
    pub enum CtMultiMediaTypeXType {
        Image,
        Audio,
        Video,
    }
    #[derive(Debug)]
    pub struct ResColorSpacesXElementType {
        pub color_space: Vec<ResColorSpacesColorSpaceXElementType>,
    }
    #[derive(Debug)]
    pub struct ResDrawParamsXElementType {
        pub draw_param: Vec<ResDrawParamsDrawParamXElementType>,
    }
    #[derive(Debug)]
    pub struct ResFontsXElementType {
        pub font: Vec<ResFontsFontXElementType>,
    }
    #[derive(Debug)]
    pub struct ResMultiMediasXElementType {
        pub multi_media: Vec<ResMultiMediasMultiMediaXElementType>,
    }
    #[derive(Debug)]
    pub struct ResCompositeGraphicUnitsXElementType {
        pub composite_graphic_unit: Vec<ResCompositeGraphicUnitsCompositeGraphicUnitXElementType>,
    }
    #[derive(Debug)]
    pub struct ResColorSpacesColorSpaceXElementType {
        pub type_: CtColorSpaceTypeXType,
        pub bits_per_component: ::core::primitive::i32,
        pub profile: Option<::std::string::String>,
        pub id: ::core::primitive::u32,
        pub palette: Option<CtColorSpacePaletteXElementType>,
    }
    #[derive(Debug)]
    pub struct ResDrawParamsDrawParamXElementType {
        pub relative: Option<::core::primitive::u32>,
        pub line_width: ::core::primitive::f64,
        pub join: ::std::string::String,
        pub cap: ::std::string::String,
        pub dash_offset: ::core::primitive::f64,
        pub dash_pattern: Option<::std::string::String>,
        pub miter_limit: ::core::primitive::f64,
        pub id: ::core::primitive::u32,
        pub fill_color: Option<super::page::CtColorXType>,
        pub stroke_color: Option<super::page::CtColorXType>,
    }
    #[derive(Debug)]
    pub struct ResFontsFontXElementType {
        pub font_name: ::std::string::String,
        pub family_name: Option<::std::string::String>,
        pub charset: CtFontCharsetXType,
        pub italic: ::core::primitive::bool,
        pub bold: ::core::primitive::bool,
        pub serif: ::core::primitive::bool,
        pub fixed_width: ::core::primitive::bool,
        pub id: ::core::primitive::u32,
        pub font_file: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct ResMultiMediasMultiMediaXElementType {
        pub type_: CtMultiMediaTypeXType,
        pub format: Option<::std::string::String>,
        pub id: ::core::primitive::u32,
        pub media_file: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct ResCompositeGraphicUnitsCompositeGraphicUnitXElementType {
        pub width: ::core::primitive::f64,
        pub height: ::core::primitive::f64,
        pub id: ::core::primitive::u32,
        pub thumbnail: Option<::core::primitive::u32>,
        pub substitution: Option<::core::primitive::u32>,
        pub content: super::page::CtPageBlockXType,
    }
}
pub mod signature {
    pub type Sianature = SianatureXElementType;
    #[derive(Debug)]
    pub struct SianatureXElementType {
        pub siqned_info: SianatureSiqnedInfoXElementType,
        pub signed_value: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoXElementType {
        pub provider: SianatureSiqnedInfoProviderXElementType,
        pub signature_method: Option<::std::string::String>,
        pub sianature_date_time: Option<::std::string::String>,
        pub references: SianatureSiqnedInfoReferencesXElementType,
        pub stamp_annot: Vec<SianatureSiqnedInfoStampAnnotXElementType>,
        pub seal: Option<SianatureSiqnedInfoSealXElementType>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoProviderXElementType {
        pub provider_name: ::std::string::String,
        pub version: Option<::std::string::String>,
        pub company: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoReferencesXElementType {
        pub check_method: SianatureSiqnedInfoReferencesCheckMethodXType,
        pub reference: Vec<SianatureSiqnedInfoReferencesReferenceXElementType>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoStampAnnotXElementType {
        pub id: ::std::string::String,
        pub page_ref: ::core::primitive::u32,
        pub boundary: ::std::string::String,
        pub clip: Option<::std::string::String>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoSealXElementType {
        pub base_loc: ::std::string::String,
    }
    #[derive(Debug)]
    pub enum SianatureSiqnedInfoReferencesCheckMethodXType {
        Md5,
        Sha1,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoReferencesReferenceXElementType {
        pub file_ref: ::std::string::String,
        pub check_value: ::std::string::String,
    }
}
pub mod signatures {
    pub type Siqnatures = SiqnaturesXElementType;
    #[derive(Debug)]
    pub struct SiqnaturesXElementType {
        pub max_sign_id: Option<::std::string::String>,
        pub signature: Vec<SiqnaturesSignatureXElementType>,
    }
    #[derive(Debug)]
    pub struct SiqnaturesSignatureXElementType {
        pub id: ::std::string::String,
        pub type_: SiqnaturesSignatureTypeXType,
        pub base_loc: ::std::string::String,
    }
    #[derive(Debug)]
    pub enum SiqnaturesSignatureTypeXType {
        Seal,
        Siqn,
    }
}
pub mod version {
    pub type DocVersion = DocVersionXElementType;
    #[derive(Debug)]
    pub struct DocVersionXElementType {
        pub id: ::std::string::String,
        pub version: Option<::std::string::String>,
        pub name: Option<::std::string::String>,
        pub creation_date: Option<::std::string::String>,
        pub file_list: DocVersionFileListXElementType,
        pub doc_root: ::std::string::String,
    }
    #[derive(Debug)]
    pub struct DocVersionFileListXElementType {
        pub file: Vec<DocVersionFileListFileXElementType>,
    }
    #[derive(Debug)]
    pub struct DocVersionFileListFileXElementType {
        pub id: ::std::string::String,
        pub content: ::std::string::String,
    }
}
pub mod xs {
    use num::{BigInt, BigUint};
    #[derive(Debug, Default)]
    pub struct EntitiesXType(pub Vec<::std::string::String>);
    pub type EntityXType = EntitiesXType;
    pub type IdXType = ::std::string::String;
    pub type IdrefXType = ::std::string::String;
    pub type IdrefsXType = EntitiesXType;
    pub type NcNameXType = ::std::string::String;
    pub type NmtokenXType = ::std::string::String;
    pub type NmtokensXType = EntitiesXType;
    pub type NotationXType = ::std::string::String;
    pub type NameXType = ::std::string::String;
    pub type QNameXType = ::std::string::String;
    pub type AnySimpleTypeXType = ::std::string::String;
    #[derive(Debug)]
    pub struct AnyTypeXType;
    pub type AnyUriXType = ::std::string::String;
    pub type Base64BinaryXType = ::std::string::String;
    pub type BooleanXType = ::core::primitive::bool;
    pub type ByteXType = ::core::primitive::i8;
    pub type DateXType = ::std::string::String;
    pub type DateTimeXType = ::std::string::String;
    pub type DecimalXType = ::core::primitive::f64;
    pub type DoubleXType = ::core::primitive::f64;
    pub type DurationXType = ::std::string::String;
    pub type FloatXType = ::core::primitive::f32;
    pub type GDayXType = ::std::string::String;
    pub type GMonthXType = ::std::string::String;
    pub type GMonthDayXType = ::std::string::String;
    pub type GYearXType = ::std::string::String;
    pub type GYearMonthXType = ::std::string::String;
    pub type HexBinaryXType = ::std::string::String;
    pub type IntXType = ::core::primitive::i32;
    pub type IntegerXType = BigInt;
    pub type LanguageXType = ::std::string::String;
    pub type LongXType = ::core::primitive::i64;
    pub type NegativeIntegerXType = BigInt;
    pub type NonNegativeIntegerXType = BigUint;
    pub type NonPositiveIntegerXType = BigInt;
    pub type NormalizedStringXType = ::std::string::String;
    pub type PositiveIntegerXType = BigUint;
    pub type ShortXType = ::core::primitive::i16;
    pub type StringXType = ::std::string::String;
    pub type TimeXType = ::std::string::String;
    pub type TokenXType = ::std::string::String;
    pub type UnsignedByteXType = ::core::primitive::u8;
    pub type UnsignedIntXType = ::core::primitive::u32;
    pub type UnsignedLongXType = ::core::primitive::u64;
    pub type UnsignedShortXType = ::core::primitive::u16;
}
