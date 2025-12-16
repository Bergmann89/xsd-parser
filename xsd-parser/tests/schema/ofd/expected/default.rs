pub mod annotations {
    pub type Annotations = AnnotationsXElementType;
    #[derive(Debug)]
    pub struct AnnotationsXElementType {
        pub page: Vec<AnnotationsPageXElementType>,
    }
    #[derive(Debug)]
    pub struct AnnotationsPageXElementType {
        pub page_id: u32,
        pub file_loc: String,
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
        pub id: u32,
        pub type_: PageAnnotAnnotTypeXType,
        pub creator: String,
        pub last_mod_date: String,
        pub visible: bool,
        pub subtype: Option<String>,
        pub print: bool,
        pub no_zoom: bool,
        pub no_rotate: bool,
        pub read_only: bool,
        pub remark: Option<String>,
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
        pub boundary: Option<String>,
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
        pub id: String,
        pub name: String,
        pub format: Option<String>,
        pub creation_date: Option<String>,
        pub mod_date: Option<String>,
        pub size: Option<f64>,
        pub visible: bool,
        pub usage: String,
        pub file_loc: String,
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
        pub name_space: String,
        pub schema_loc: Option<String>,
        pub file_loc: String,
    }
}
pub mod definition {
    use xsd_parser_types::xml::AnyElement;
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
        pub page_id: u32,
        pub left: Option<f64>,
        pub top: Option<f64>,
        pub right: Option<f64>,
        pub bottom: Option<f64>,
        pub zoom: Option<f64>,
    }
    #[derive(Debug)]
    pub struct CtPageAreaXType {
        pub physical_box: String,
        pub application_box: Option<String>,
        pub content_box: Option<String>,
        pub bleed_box: Option<String>,
    }
    #[derive(Debug)]
    pub struct CtRegionXType {
        pub area: Vec<CtRegionAreaXElementType>,
    }
    pub type StArrayXType = String;
    pub type StBoxXType = String;
    pub type StIdXType = u32;
    pub type StLocXType = String;
    pub type StPosXType = String;
    pub type StRefIdXType = u32;
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
        pub uri: String,
        pub base: Option<String>,
        pub target: Option<String>,
    }
    #[derive(Debug)]
    pub struct CtActionGotoAxElementType {
        pub attach_id: String,
        pub new_window: bool,
    }
    #[derive(Debug)]
    pub struct CtActionSoundXElementType {
        pub resource_id: u32,
        pub volume: Option<i32>,
        pub repeat: Option<bool>,
        pub synchronous: Option<bool>,
    }
    #[derive(Debug)]
    pub struct CtActionMovieXElementType {
        pub resource_id: u32,
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
        pub start: String,
        pub content: Vec<CtRegionAreaXElementTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtRegionAreaXElementTypeContent {
        Move(CtRegionAreaLineXElementType),
        Line(CtRegionAreaLineXElementType),
        OuadraticBezier(CtRegionAreaOuadraticBezierXElementType),
        CubicBezier(CtRegionAreaCubicBezierXElementType),
        Arc(CtRegionAreaArcXElementType),
        Close(AnyElement),
    }
    #[derive(Debug)]
    pub struct CtActionGotoBookmarkXElementType {
        pub name: String,
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
        pub point_1: String,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaOuadraticBezierXElementType {
        pub pointl: String,
        pub point_2: String,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaCubicBezierXElementType {
        pub point_1: Option<String>,
        pub point_2: Option<String>,
        pub point_3: String,
    }
    #[derive(Debug)]
    pub struct CtRegionAreaArcXElementType {
        pub sweep_direction: bool,
        pub large_arc: bool,
        pub rotation_anglet: f64,
        pub ellipse_size: String,
        pub end_point: String,
    }
}
pub mod document {
    #[derive(Debug)]
    pub struct CtBookmarkXType {
        pub name: String,
        pub dest: super::definition::CtDestXType,
    }
    #[derive(Debug)]
    pub struct CtOutlineElemXType {
        pub title: String,
        pub count: Option<i32>,
        pub expanded: bool,
        pub actions: Option<super::page::CtGraphicUnitActionsXElementType>,
        pub outline_elem: Vec<CtOutlineElemXType>,
    }
    #[derive(Debug)]
    pub struct CtPermissionXType {
        pub edit: Option<bool>,
        pub annot: Option<bool>,
        pub export: Option<bool>,
        pub signature: Option<bool>,
        pub watermark: Option<bool>,
        pub print_screen: Option<bool>,
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
        HideToolbar(bool),
        HideMenubar(bool),
        HideWindowUi(bool),
        ZoomMode(CtVPreferencesZoomModeXElementType),
        Zoom(f64),
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
        pub annotations: Option<String>,
        pub custom_tags: Option<String>,
        pub attachments: Option<String>,
        pub extensions: Option<String>,
    }
    #[derive(Debug)]
    pub struct CtPermissionPrintXElementType {
        pub printable: bool,
        pub copies: i32,
    }
    #[derive(Debug)]
    pub struct CtPermissionValidPeriodXElementType {
        pub start_date: Option<String>,
        pub end_date: Option<String>,
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
        pub max_unit_id: u32,
        pub page_area: super::definition::CtPageAreaXType,
        pub public_res: Vec<String>,
        pub document_res: Vec<String>,
        pub template_page: Vec<DocumentCommonDataTemplatePageXElementType>,
        pub default_cs: Option<u32>,
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
        pub id: String,
        pub name: Option<String>,
        pub z_order: Option<DocumentCommonDataTemplatePageZOrderXType>,
        pub base_loc: String,
    }
    #[derive(Debug)]
    pub struct DocumentPagesPageXElementType {
        pub id: u32,
        pub base_loc: String,
    }
    #[derive(Debug)]
    pub enum DocumentCommonDataTemplatePageZOrderXType {
        Background,
        Foreground,
    }
}
pub mod extensions {
    use xsd_parser_types::xml::AnyElement;
    #[derive(Debug)]
    pub struct CtExtensionXType {
        pub app_name: String,
        pub company: Option<String>,
        pub app_version: Option<String>,
        pub date: Option<String>,
        pub ref_id: u32,
        pub content: Vec<CtExtensionXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtExtensionXTypeContent {
        Property(CtExtensionPropertyXElementType),
        Data(AnyElement),
        ExtendData(String),
    }
    pub type Extensions = ExtensionsXElementType;
    #[derive(Debug)]
    pub struct ExtensionsXElementType {
        pub extension: Vec<CtExtensionXType>,
    }
    #[derive(Debug)]
    pub struct CtExtensionPropertyXElementType {
        pub name: String,
        pub type_: Option<String>,
        pub content: String,
    }
}
pub mod ofd {
    #[derive(Debug)]
    pub struct CtDocInfoXType {
        pub doc_id: String,
        pub title: Option<String>,
        pub author: Option<String>,
        pub subject: Option<String>,
        pub abstract_: Option<String>,
        pub creation_date: Option<String>,
        pub mod_date: Option<String>,
        pub doc_usage: Option<String>,
        pub cover: Option<String>,
        pub keywords: Option<CtDocInfoKeywordsXElementType>,
        pub creator: Option<String>,
        pub creator_version: Option<String>,
        pub custom_datas: Option<CtDocInfoCustomDatasXElementType>,
    }
    pub type Ofd = OfdXElementType;
    #[derive(Debug)]
    pub struct OfdXElementType {
        pub version: String,
        pub doc_type: OfdDocTypeXType,
        pub doc_body: Vec<OfdDocBodyXElementType>,
    }
    #[derive(Debug)]
    pub struct CtDocInfoKeywordsXElementType {
        pub keyword: Vec<String>,
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
        pub doc_root: String,
        pub versions: Option<OfdDocBodyVersionsXElementType>,
        pub signatures: Option<String>,
    }
    #[derive(Debug)]
    pub struct CtDocInfoCustomDatasCustomDataXElementType {
        pub name: String,
        pub content: String,
    }
    #[derive(Debug)]
    pub struct OfdDocBodyVersionsXElementType {
        pub version: Vec<OfdDocBodyVersionsVersionXElementType>,
    }
    #[derive(Debug)]
    pub struct OfdDocBodyVersionsVersionXElementType {
        pub id: String,
        pub index: i32,
        pub current: bool,
        pub base_loc: String,
    }
}
pub mod page {
    #[derive(Debug)]
    pub struct CtAxialShdXType {
        pub map_type: CtAxialShdMapTypeXType,
        pub map_unit: Option<f64>,
        pub extend: CtAxialShdExtendXType,
        pub start_point: String,
        pub end_point: String,
        pub segment: Vec<CtAxialShdSegmentXElementType>,
    }
    #[derive(Debug)]
    pub struct CtCgTransformXType {
        pub code_position: i32,
        pub code_count: i32,
        pub glyph_count: i32,
        pub glyphs: Option<String>,
    }
    #[derive(Debug)]
    pub struct CtClipXType {
        pub area: Vec<CtClipAreaXElementType>,
    }
    #[derive(Debug)]
    pub struct CtColorXType {
        pub value: Option<String>,
        pub index: Option<i32>,
        pub color_space: Option<u32>,
        pub alpha: i32,
        pub content: Option<CtColorXTypeContent>,
    }
    #[derive(Debug)]
    pub enum CtColorXTypeContent {
        Pattern(CtPatternXType),
        AxialShd(CtAxialShdXType),
        RadialShd(CtRadialShdXType),
        GouraudShd(CtGouraudShdXType),
        LaGourandShd(CtLaGouraudShdXType),
    }
    #[derive(Debug)]
    pub struct CtCompositeXType {
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub resource_id: u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
    }
    #[derive(Debug)]
    pub struct CtGouraudShdXType {
        pub extend: Option<i32>,
        pub point: Vec<CtGouraudShdPointXElementType>,
        pub back_color: Option<Box<CtColorXType>>,
    }
    #[derive(Debug)]
    pub struct CtGraphicUnitXType {
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
    }
    #[derive(Debug)]
    pub struct CtImageXType {
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub resource_id: u32,
        pub substitution: Option<u32>,
        pub image_mask: Option<u32>,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub border: Option<CtImageBorderXElementType>,
    }
    #[derive(Debug)]
    pub struct CtLaGouraudShdXType {
        pub vertices_per_row: i32,
        pub extend: Option<i32>,
        pub point: Vec<CtLaGouraudShdPointXElementType>,
        pub back_color: Option<Box<CtColorXType>>,
    }
    #[derive(Debug)]
    pub struct CtLayerXType {
        pub type_: CtLayerTypeXType,
        pub draw_param: Option<u32>,
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
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub stroke: bool,
        pub fill: bool,
        pub rule: CtPathRuleXType,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub stroke_color: Option<CtColorXType>,
        pub fill_color: Option<CtColorXType>,
        pub abbreviated_data: String,
    }
    #[derive(Debug)]
    pub struct CtPatternXType {
        pub width: f64,
        pub height: f64,
        pub x_step: Option<f64>,
        pub y_step: Option<f64>,
        pub reflect_method: CtPatternReflectMethodXType,
        pub relative_to: CtPatternRelativeToXType,
        pub ctm: Option<String>,
        pub cell_content: CtPatternCellContentXElementType,
    }
    #[derive(Debug)]
    pub struct CtRadialShdXType {
        pub map_type: CtAxialShdMapTypeXType,
        pub map_unit: Option<f64>,
        pub eccentricity: f64,
        pub angle: f64,
        pub start_point: String,
        pub start_radius: f64,
        pub end_point: String,
        pub end_radius: f64,
        pub extend: i32,
        pub seqment: Vec<CtAxialShdSegmentXElementType>,
    }
    #[derive(Debug)]
    pub struct CtTextXType {
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub font: u32,
        pub size: f64,
        pub stroke: bool,
        pub fill: bool,
        pub h_scale: f64,
        pub read_direction: i32,
        pub char_direction: i32,
        pub weight: CtTextWeightXType,
        pub italic: bool,
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
        pub page_res: Vec<String>,
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
        pub position: Option<f64>,
        pub color: Box<CtColorXType>,
    }
    #[derive(Debug)]
    pub struct CtClipAreaXElementType {
        pub draw_param: Option<u32>,
        pub ctm: Option<String>,
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
        pub x: f64,
        pub y: f64,
        pub edge_flag: Option<CtGouraudShdPointEdgeFlagXType>,
        pub color: Box<CtColorXType>,
    }
    #[derive(Debug)]
    pub struct CtImageBorderXElementType {
        pub line_width: f64,
        pub horizonal_corner_radius: f64,
        pub vertical_corner_radius: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub border_color: Option<CtColorXType>,
    }
    #[derive(Debug)]
    pub struct CtLaGouraudShdPointXElementType {
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub color: Box<CtColorXType>,
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
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub font: u32,
        pub size: f64,
        pub stroke: bool,
        pub fill: bool,
        pub h_scale: f64,
        pub read_direction: i32,
        pub char_direction: i32,
        pub weight: CtTextWeightXType,
        pub italic: bool,
        pub id: u32,
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
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub stroke: bool,
        pub fill: bool,
        pub rule: CtPathRuleXType,
        pub id: u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub stroke_color: Option<CtColorXType>,
        pub fill_color: Option<CtColorXType>,
        pub abbreviated_data: String,
    }
    #[derive(Debug)]
    pub struct CtPageBlockImageObjectXElementType {
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub resource_id: u32,
        pub substitution: Option<u32>,
        pub image_mask: Option<u32>,
        pub id: u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
        pub border: Option<CtImageBorderXElementType>,
    }
    #[derive(Debug)]
    pub struct CtPageBlockCompositeObjectXElementType {
        pub boundary: String,
        pub name: Option<String>,
        pub visible: bool,
        pub ctm: Option<String>,
        pub draw_param: Option<u32>,
        pub line_width: f64,
        pub cap: CtGraphicUnitCapXType,
        pub join: CtGraphicUnitJoinXType,
        pub miter_limit: f64,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub alpha: i32,
        pub resource_id: u32,
        pub id: u32,
        pub actions: Option<CtGraphicUnitActionsXElementType>,
        pub clips: Option<CtGraphicUnitClipsXElementType>,
    }
    #[derive(Debug)]
    pub struct CtPageBlockPageBlockXElementType {
        pub id: u32,
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
        pub thumbnail: Option<u32>,
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
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub delta_x: Option<String>,
        pub deltay: Option<String>,
        pub content: String,
    }
    #[derive(Debug)]
    pub struct PageTemplateXElementType {
        pub template_id: u32,
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
        pub draw_param: Option<u32>,
        pub id: u32,
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
        pub bits_per_component: i32,
        pub profile: Option<String>,
        pub palette: Option<CtColorSpacePaletteXElementType>,
    }
    #[derive(Debug)]
    pub struct CtDrawParamXType {
        pub relative: Option<u32>,
        pub line_width: f64,
        pub join: String,
        pub cap: String,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub miter_limit: f64,
        pub fill_color: Option<super::page::CtColorXType>,
        pub stroke_color: Option<super::page::CtColorXType>,
    }
    #[derive(Debug)]
    pub struct CtFontXType {
        pub font_name: String,
        pub family_name: Option<String>,
        pub charset: CtFontCharsetXType,
        pub italic: bool,
        pub bold: bool,
        pub serif: bool,
        pub fixed_width: bool,
        pub font_file: Option<String>,
    }
    #[derive(Debug)]
    pub struct CtMultiMediaXType {
        pub type_: CtMultiMediaTypeXType,
        pub format: Option<String>,
        pub media_file: String,
    }
    #[derive(Debug)]
    pub struct CtVectorGxType {
        pub width: f64,
        pub height: f64,
        pub thumbnail: Option<u32>,
        pub substitution: Option<u32>,
        pub content: super::page::CtPageBlockXType,
    }
    pub type Res = ResXElementType;
    #[derive(Debug)]
    pub struct ResXElementType {
        pub base_loc: String,
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
        pub cv: Vec<String>,
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
        pub bits_per_component: i32,
        pub profile: Option<String>,
        pub id: u32,
        pub palette: Option<CtColorSpacePaletteXElementType>,
    }
    #[derive(Debug)]
    pub struct ResDrawParamsDrawParamXElementType {
        pub relative: Option<u32>,
        pub line_width: f64,
        pub join: String,
        pub cap: String,
        pub dash_offset: f64,
        pub dash_pattern: Option<String>,
        pub miter_limit: f64,
        pub id: u32,
        pub fill_color: Option<super::page::CtColorXType>,
        pub stroke_color: Option<super::page::CtColorXType>,
    }
    #[derive(Debug)]
    pub struct ResFontsFontXElementType {
        pub font_name: String,
        pub family_name: Option<String>,
        pub charset: CtFontCharsetXType,
        pub italic: bool,
        pub bold: bool,
        pub serif: bool,
        pub fixed_width: bool,
        pub id: u32,
        pub font_file: Option<String>,
    }
    #[derive(Debug)]
    pub struct ResMultiMediasMultiMediaXElementType {
        pub type_: CtMultiMediaTypeXType,
        pub format: Option<String>,
        pub id: u32,
        pub media_file: String,
    }
    #[derive(Debug)]
    pub struct ResCompositeGraphicUnitsCompositeGraphicUnitXElementType {
        pub width: f64,
        pub height: f64,
        pub id: u32,
        pub thumbnail: Option<u32>,
        pub substitution: Option<u32>,
        pub content: super::page::CtPageBlockXType,
    }
}
pub mod signature {
    pub type Sianature = SianatureXElementType;
    #[derive(Debug)]
    pub struct SianatureXElementType {
        pub siqned_info: SianatureSiqnedInfoXElementType,
        pub signed_value: String,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoXElementType {
        pub provider: SianatureSiqnedInfoProviderXElementType,
        pub signature_method: Option<String>,
        pub sianature_date_time: Option<String>,
        pub references: SianatureSiqnedInfoReferencesXElementType,
        pub stamp_annot: Vec<SianatureSiqnedInfoStampAnnotXElementType>,
        pub seal: Option<SianatureSiqnedInfoSealXElementType>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoProviderXElementType {
        pub provider_name: String,
        pub version: Option<String>,
        pub company: Option<String>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoReferencesXElementType {
        pub check_method: SianatureSiqnedInfoReferencesCheckMethodXType,
        pub reference: Vec<SianatureSiqnedInfoReferencesReferenceXElementType>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoStampAnnotXElementType {
        pub id: String,
        pub page_ref: u32,
        pub boundary: String,
        pub clip: Option<String>,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoSealXElementType {
        pub base_loc: String,
    }
    #[derive(Debug)]
    pub enum SianatureSiqnedInfoReferencesCheckMethodXType {
        Md5,
        Sha1,
    }
    #[derive(Debug)]
    pub struct SianatureSiqnedInfoReferencesReferenceXElementType {
        pub file_ref: String,
        pub check_value: String,
    }
}
pub mod signatures {
    pub type Siqnatures = SiqnaturesXElementType;
    #[derive(Debug)]
    pub struct SiqnaturesXElementType {
        pub max_sign_id: Option<String>,
        pub signature: Vec<SiqnaturesSignatureXElementType>,
    }
    #[derive(Debug)]
    pub struct SiqnaturesSignatureXElementType {
        pub id: String,
        pub type_: SiqnaturesSignatureTypeXType,
        pub base_loc: String,
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
        pub id: String,
        pub version: Option<String>,
        pub name: Option<String>,
        pub creation_date: Option<String>,
        pub file_list: DocVersionFileListXElementType,
        pub doc_root: String,
    }
    #[derive(Debug)]
    pub struct DocVersionFileListXElementType {
        pub file: Vec<DocVersionFileListFileXElementType>,
    }
    #[derive(Debug)]
    pub struct DocVersionFileListFileXElementType {
        pub id: String,
        pub content: String,
    }
}
pub mod xs {
    use core::num::{NonZeroIsize, NonZeroUsize};
    use num::{BigInt, BigUint};
    #[derive(Debug, Default)]
    pub struct EntitiesXType(pub Vec<String>);
    pub type EntityXType = EntitiesXType;
    pub type IdXType = String;
    pub type IdrefXType = String;
    pub type IdrefsXType = EntitiesXType;
    pub type NcNameXType = String;
    pub type NmtokenXType = String;
    pub type NmtokensXType = EntitiesXType;
    pub type NotationXType = String;
    pub type NameXType = String;
    pub type QNameXType = String;
    pub type AnySimpleTypeXType = String;
    pub type AnyUriXType = String;
    pub type Base64BinaryXType = String;
    pub type BooleanXType = bool;
    pub type ByteXType = i8;
    pub type DateXType = String;
    pub type DateTimeXType = String;
    pub type DecimalXType = f64;
    pub type DoubleXType = f64;
    pub type DurationXType = String;
    pub type FloatXType = f32;
    pub type GDayXType = String;
    pub type GMonthXType = String;
    pub type GMonthDayXType = String;
    pub type GYearXType = String;
    pub type GYearMonthXType = String;
    pub type HexBinaryXType = String;
    pub type IntXType = i32;
    pub type IntegerXType = BigInt;
    pub type LanguageXType = String;
    pub type LongXType = i64;
    pub type NegativeIntegerXType = NonZeroIsize;
    pub type NonNegativeIntegerXType = BigUint;
    pub type NonPositiveIntegerXType = BigInt;
    pub type NormalizedStringXType = String;
    pub type PositiveIntegerXType = NonZeroUsize;
    pub type ShortXType = i16;
    pub type StringXType = String;
    pub type TimeXType = String;
    pub type TokenXType = String;
    pub type UnsignedByteXType = u8;
    pub type UnsignedIntXType = u32;
    pub type UnsignedLongXType = u64;
    pub type UnsignedShortXType = u16;
}
