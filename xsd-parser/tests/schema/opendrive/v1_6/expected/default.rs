pub type OpenDrive = OpenDriveXElementType;
#[derive(Debug)]
pub struct OpenDriveXElementType {
    pub header: THeaderXType,
    pub road: Vec<TRoadXType>,
    pub controller: Vec<TControllerXType>,
    pub junction: Vec<TJunctionXType>,
    pub junction_group: Vec<TJunctionGroupXType>,
    pub station: Vec<TStationXType>,
    pub g_additional_data: Vec<OpenDriveGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct OpenDriveElementXType;
#[derive(Debug)]
pub enum EDataQualityRawDataPostProcessingXType {
    Raw,
    Cleaned,
    Processed,
    Fused,
}
#[derive(Debug)]
pub enum EDataQualityRawDataSourceXType {
    Sensor,
    Cadaster,
    Custom,
}
#[derive(Debug)]
pub enum EUnitXType {
    EUnitDistance(EUnitDistanceXType),
    EUnitSpeed(EUnitSpeedXType),
    EUnitMass(EUnitMassXType),
    EUnitSlope(EUnitSlopeXType),
}
#[derive(Debug)]
pub enum EUnitDistanceXType {
    M,
    Km,
    Ft,
    Mile,
}
#[derive(Debug)]
pub enum EUnitMassXType {
    Kg,
    T,
}
#[derive(Debug)]
pub enum EUnitSlopeXType {
    Percent,
}
#[derive(Debug)]
pub enum EUnitSpeedXType {
    MS,
    Mph,
    KmH,
}
#[derive(Debug)]
pub struct TDataQualityXType {
    pub error: Option<TDataQualityErrorXType>,
    pub raw_data: Option<TDataQualityRawDataXType>,
}
#[derive(Debug)]
pub struct TDataQualityErrorXType {
    pub xy_absolute_attrib: f64,
    pub z_absolute_attrib: f64,
    pub xy_relative_attrib: f64,
    pub z_relative_attrib: f64,
}
#[derive(Debug)]
pub struct TDataQualityRawDataXType {
    pub date_attrib: String,
    pub source_attrib: EDataQualityRawDataSourceXType,
    pub source_comment_attrib: Option<String>,
    pub post_processing_attrib: EDataQualityRawDataPostProcessingXType,
    pub post_processing_comment_attrib: Option<String>,
}
pub type TGrEqZeroXType = f64;
pub type TGrZeroXType = f64;
#[derive(Debug)]
pub struct THeaderXType {
    pub rev_major_attrib: i32,
    pub rev_minor_attrib: i32,
    pub name_attrib: Option<String>,
    pub version_attrib: Option<String>,
    pub date_attrib: Option<String>,
    pub north_attrib: Option<f64>,
    pub south_attrib: Option<f64>,
    pub east_attrib: Option<f64>,
    pub west_attrib: Option<f64>,
    pub vendor_attrib: Option<String>,
    pub geo_reference: Option<THeaderGeoReferenceXType>,
    pub offset: Option<THeaderOffsetXType>,
    pub g_additional_data: Vec<THeaderGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct THeaderGeoReferenceXType {
    pub g_additional_data: Vec<THeaderGeoReferenceGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct THeaderOffsetXType {
    pub x_attrib: f64,
    pub y_attrib: f64,
    pub z_attrib: f64,
    pub hdg_attrib: f32,
    pub g_additional_data: Vec<THeaderOffsetGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TIncludeXType {
    pub file_attrib: String,
}
#[derive(Debug)]
pub struct TUserDataXType {
    pub code_attrib: String,
    pub value_attrib: Option<String>,
}
#[derive(Debug)]
pub enum TYesNoXType {
    Yes,
    No,
}
pub type TZeroOneXType = f64;
#[derive(Debug)]
pub enum ERoadRailroadSwitchPositionXType {
    Dynamic,
    Straight,
    Turn,
}
#[derive(Debug)]
pub enum EStationPlatformSegmentSideXType {
    Left,
    Right,
}
#[derive(Debug)]
pub enum EStationTypeXType {
    Small,
    Medium,
    Large,
}
#[derive(Debug)]
pub struct TRoadRailroadXType {
    pub switch: Vec<TRoadRailroadSwitchXType>,
    pub g_additional_data: Vec<TRoadRailroadGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchXType {
    pub name_attrib: String,
    pub id_attrib: String,
    pub position_attrib: ERoadRailroadSwitchPositionXType,
    pub main_track: TRoadRailroadSwitchMainTrackXType,
    pub side_track: TRoadRailroadSwitchSideTrackXType,
    pub partner: Option<TRoadRailroadSwitchPartnerXType>,
    pub g_additional_data: Vec<TRoadRailroadSwitchGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchMainTrackXType {
    pub id_attrib: String,
    pub s_attrib: f64,
    pub dir_attrib: EElementDirXType,
    pub g_additional_data: Vec<TRoadRailroadSwitchMainTrackGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchPartnerXType {
    pub name_attrib: Option<String>,
    pub id_attrib: String,
    pub g_additional_data: Vec<TRoadRailroadSwitchPartnerGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchSideTrackXType {
    pub id_attrib: String,
    pub s_attrib: f64,
    pub dir_attrib: EElementDirXType,
    pub g_additional_data: Vec<TRoadRailroadSwitchSideTrackGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TStationXType {
    pub name_attrib: String,
    pub id_attrib: String,
    pub type_attrib: Option<EStationTypeXType>,
    pub platform: Vec<TStationPlatformXType>,
    pub g_additional_data: Vec<TStationGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TStationPlatformXType {
    pub name_attrib: Option<String>,
    pub id_attrib: String,
    pub segment: Vec<TStationPlatformSegmentXType>,
    pub g_additional_data: Vec<TStationPlatformGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TStationPlatformSegmentXType {
    pub road_id_attrib: String,
    pub s_start_attrib: f64,
    pub s_end_attrib: f64,
    pub side_attrib: EStationPlatformSegmentSideXType,
}
#[derive(Debug)]
pub enum EContactPointXType {
    Start,
    End,
}
#[derive(Debug)]
pub enum EElementDirXType {
    Plus,
    Minus,
}
#[derive(Debug)]
pub enum EJunctionGroupTypeXType {
    Roundabout,
    Unknown,
}
#[derive(Debug)]
pub enum EJunctionTypeXType {
    Default,
    Virtual,
}
#[derive(Debug)]
pub enum ERoadSurfaceCrgModeXType {
    Attached,
    Attached0,
    Genuine,
    Global,
}
#[derive(Debug)]
pub enum ERoadSurfaceCrgPurposeXType {
    Elevation,
    Friction,
}
#[derive(Debug)]
pub struct TJunctionXType {
    pub name_attrib: Option<String>,
    pub id_attrib: String,
    pub type_attrib: Option<EJunctionTypeXType>,
    pub connection: Vec<TJunctionConnectionXType>,
    pub priority: Vec<TJunctionPriorityXType>,
    pub controller: Vec<TJunctionControllerXType>,
    pub surface: Option<TJunctionSurfaceXType>,
    pub g_additional_data: Vec<TJunctionGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TJunctionGroupXType {
    pub name_attrib: Option<String>,
    pub id_attrib: String,
    pub type_attrib: EJunctionGroupTypeXType,
    pub junction_reference: Vec<TJunctionGroupJunctionReferenceXType>,
    pub g_additional_data: Vec<TJunctionGroupGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TJunctionGroupJunctionReferenceXType {
    pub junction_attrib: String,
}
#[derive(Debug)]
pub struct TJunctionConnectionXType {
    pub id_attrib: String,
    pub type_attrib: Option<EJunctionTypeXType>,
    pub incoming_road_attrib: Option<String>,
    pub connecting_road_attrib: Option<String>,
    pub contact_point_attrib: Option<String>,
    pub predecessor: Option<TJunctionPredecessorSuccessorXType>,
    pub successor: Option<TJunctionPredecessorSuccessorXType>,
    pub lane_link: Vec<TJunctionConnectionLaneLinkXType>,
}
#[derive(Debug)]
pub struct TJunctionConnectionLaneLinkXType {
    pub from_attrib: i32,
    pub to_attrib: i32,
}
#[derive(Debug)]
pub struct TJunctionControllerXType {
    pub id_attrib: String,
    pub type_attrib: Option<String>,
    pub sequence_attrib: Option<usize>,
}
#[derive(Debug)]
pub struct TJunctionPredecessorSuccessorXType {
    pub element_type_attrib: String,
    pub element_id_attrib: String,
    pub element_s_attrib: f64,
    pub element_dir_attrib: EElementDirXType,
}
#[derive(Debug)]
pub struct TJunctionPriorityXType {
    pub high_attrib: Option<String>,
    pub low_attrib: Option<String>,
}
#[derive(Debug)]
pub struct TJunctionSurfaceXType {
    pub crg: Vec<TJunctionSurfaceCrgXType>,
    pub g_additional_data: Vec<TJunctionSurfaceGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TJunctionSurfaceCrgXType {
    pub file_attrib: String,
    pub mode_attrib: ERoadSurfaceCrgModeXType,
    pub purpose_attrib: Option<ERoadSurfaceCrgPurposeXType>,
    pub z_offset_attrib: Option<f64>,
    pub z_scale_attrib: Option<f64>,
}
#[derive(Debug)]
pub enum ERoadSignalsSignalReferenceElementTypeXType {
    Object,
    Signal,
}
#[derive(Debug)]
pub struct TControllerXType {
    pub id_attrib: String,
    pub name_attrib: Option<String>,
    pub sequence_attrib: Option<usize>,
    pub control: Vec<TControllerControlXType>,
    pub g_additional_data: Vec<TControllerGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TControllerControlXType {
    pub signal_id_attrib: String,
    pub type_attrib: Option<String>,
}
#[derive(Debug)]
pub struct TRoadSignalsXType {
    pub signal: Vec<TRoadSignalsSignalXType>,
    pub signal_reference: Vec<TRoadSignalsSignalReference2>,
    pub g_additional_data: Vec<TRoadSignalsGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalXType {
    pub s_attrib: f64,
    pub t_attrib: f64,
    pub id_attrib: String,
    pub name_attrib: Option<String>,
    pub dynamic_attrib: TYesNoXType,
    pub orientation_attrib: EOrientationXType,
    pub z_offset_attrib: f64,
    pub country_attrib: Option<ECountryCodeXType>,
    pub country_revision_attrib: Option<String>,
    pub type_attrib: String,
    pub subtype_attrib: String,
    pub value_attrib: Option<f64>,
    pub unit_attrib: Option<EUnitXType>,
    pub height_attrib: Option<f64>,
    pub width_attrib: Option<f64>,
    pub text_attrib: Option<String>,
    pub h_offset_attrib: Option<f64>,
    pub pitch_attrib: Option<f64>,
    pub roll_attrib: Option<f64>,
    pub validity: Vec<TRoadObjectsObjectLaneValidityXType>,
    pub dependency: Vec<TRoadSignalsSignalDependencyXType>,
    pub reference: Vec<TRoadSignalsSignalReferenceXType>,
    pub content_34: Option<TRoadSignalsSignalContent34XType>,
    pub g_additional_data: Vec<TRoadSignalsSignalGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalReference2 {
    pub s_attrib: f64,
    pub t_attrib: f64,
    pub id_attrib: String,
    pub orientation_attrib: EOrientationXType,
    pub validity: Vec<TRoadObjectsObjectLaneValidityXType>,
    pub g_additional_data: Vec<TRoadSignalsSignalReferenceGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalDependencyXType {
    pub id_attrib: String,
    pub type_attrib: Option<String>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalPositionInertialXType {
    pub x_attrib: f64,
    pub y_attrib: f64,
    pub z_attrib: f64,
    pub hdg_attrib: f64,
    pub pitch_attrib: Option<f64>,
    pub roll_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalPositionRoadXType {
    pub road_id_attrib: String,
    pub s_attrib: f64,
    pub t_attrib: f64,
    pub z_offset_attrib: f64,
    pub h_offset_attrib: f64,
    pub pitch_attrib: Option<f64>,
    pub roll_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalReferenceXType {
    pub element_type_attrib: ERoadSignalsSignalReferenceElementTypeXType,
    pub element_id_attrib: String,
    pub type_attrib: Option<String>,
}
#[derive(Debug)]
pub enum ECountryCodeXType {
    String(String),
    ECountryCodeDeprecated(ECountryCodeDeprecatedXType),
}
#[derive(Debug)]
pub enum ECountryCodeDeprecatedXType {
    OpenDrive,
    Austria,
    Brazil,
    China,
    France,
    Germany,
    Italy,
    Switzerland,
    Usa,
}
pub type ECountryCodeIso3166Alpha2XType = String;
pub type ECountryCodeIso3166Alpha3DeprecatedXType = String;
#[derive(Debug)]
pub enum EDirectionXType {
    Same,
    Opposite,
}
#[derive(Debug)]
pub enum EMaxSpeedStringXType {
    NoLimit,
    Undefined,
}
#[derive(Debug)]
pub enum EParamPoly3PRangeXType {
    ArcLength,
    Normalized,
}
#[derive(Debug)]
pub enum ERoadTypeXType {
    Unknown,
    Rural,
    Motorway,
    Town,
    LowSpeed,
    Pedestrian,
    Bicycle,
    TownExpressway,
    TownCollector,
    TownArterial,
    TownPrivate,
    TownLocal,
    TownPlayStreet,
}
#[derive(Debug)]
pub enum ERoadLinkElementTypeXType {
    Road,
    Junction,
}
#[derive(Debug)]
pub enum ETrafficRuleXType {
    Rht,
    Lht,
}
#[derive(Debug)]
pub enum TMaxSpeedXType {
    F64(f64),
    EMaxSpeedString(EMaxSpeedStringXType),
}
#[derive(Debug)]
pub struct TRoadXType {
    pub name_attrib: Option<String>,
    pub length_attrib: String,
    pub id_attrib: String,
    pub junction_attrib: String,
    pub rule_attrib: Option<ETrafficRuleXType>,
    pub link: Option<TRoadLinkXType>,
    pub type_: Vec<TRoadTypeXType>,
    pub plan_view: TRoadPlanViewXType,
    pub elevation_profile: Option<TRoadElevationProfileXType>,
    pub lateral_profile: Option<TRoadLateralProfileXType>,
    pub lanes: TRoadLanesXType,
    pub objects: Option<TRoadObjectsXType>,
    pub signals: Option<TRoadSignalsXType>,
    pub surface: Option<TRoadSurfaceXType>,
    pub railroad: Option<TRoadRailroadXType>,
    pub g_additional_data: Vec<TRoadGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadElevationProfileXType {
    pub elevation: Vec<TRoadElevationProfileElevationXType>,
    pub g_additional_data: Vec<TRoadElevationProfileGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadElevationProfileElevationXType {
    pub s_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLateralProfileXType {
    pub superelevation: Vec<TRoadLateralProfileSuperelevationXType>,
    pub shape: Vec<TRoadLateralProfileShapeXType>,
    pub g_additional_data: Vec<TRoadLateralProfileGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLateralProfileShapeXType {
    pub s_attrib: f64,
    pub t_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLateralProfileSuperelevationXType {
    pub s_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLinkXType {
    pub predecessor: Option<TRoadLinkPredecessorSuccessorXType>,
    pub successor: Option<TRoadLinkPredecessorSuccessorXType>,
    pub g_additional_data: Vec<TRoadLinkGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLinkPredecessorSuccessorXType {
    pub element_id_attrib: String,
    pub element_type_attrib: Option<String>,
    pub contact_point_attrib: Option<String>,
    pub element_s_attrib: Option<f64>,
    pub element_dir_attrib: Option<String>,
}
#[derive(Debug)]
pub struct TRoadPlanViewXType {
    pub geometry: Vec<TRoadPlanViewGeometryXType>,
    pub g_additional_data: Vec<TRoadPlanViewGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadPlanViewGeometryXType {
    pub s_attrib: f64,
    pub x_attrib: f64,
    pub y_attrib: f64,
    pub hdg_attrib: f64,
    pub length_attrib: String,
    pub content: TRoadPlanViewGeometryXTypeContent,
}
#[derive(Debug)]
pub enum TRoadPlanViewGeometryXTypeContent {
    Line(TRoadPlanViewGeometryLineXType),
    Spiral(TRoadPlanViewGeometrySpiralXType),
    Arc(TRoadPlanViewGeometryArcXType),
    Poly3(TRoadPlanViewGeometryPoly3XType),
    ParamPoly3(TRoadPlanViewGeometryParamPoly3XType),
    GAdditionalData(Vec<TRoadPlanViewGeometryGAdditionalDataXType>),
}
#[derive(Debug)]
pub struct TRoadPlanViewGeometryArcXType {
    pub curvature_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadPlanViewGeometryLineXType;
#[derive(Debug)]
pub struct TRoadPlanViewGeometryParamPoly3XType {
    pub au_attrib: f64,
    pub bu_attrib: f64,
    pub cu_attrib: f64,
    pub du_attrib: f64,
    pub av_attrib: f64,
    pub bv_attrib: f64,
    pub cv_attrib: f64,
    pub dv_attrib: f64,
    pub p_range_attrib: EParamPoly3PRangeXType,
}
#[derive(Debug)]
pub struct TRoadPlanViewGeometryPoly3XType {
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadPlanViewGeometrySpiralXType {
    pub curv_start_attrib: f64,
    pub curv_end_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadSurfaceXType {
    pub crg: Vec<TRoadSurfaceCrgXType>,
    pub g_additional_data: Vec<TRoadSurfaceGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadSurfaceCrgXType {
    pub file_attrib: String,
    pub s_start_attrib: f64,
    pub s_end_attrib: f64,
    pub orientation_attrib: EDirectionXType,
    pub mode_attrib: ERoadSurfaceCrgModeXType,
    pub purpose_attrib: Option<ERoadSurfaceCrgPurposeXType>,
    pub s_offset_attrib: Option<f64>,
    pub t_offset_attrib: Option<f64>,
    pub z_offset_attrib: Option<f64>,
    pub z_scale_attrib: Option<f64>,
    pub h_offset_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct TRoadTypeXType {
    pub s_attrib: f64,
    pub type_attrib: ERoadTypeXType,
    pub country_attrib: Option<ECountryCodeXType>,
    pub speed: Option<TRoadTypeSpeedXType>,
    pub g_additional_data: Vec<TRoadTypeGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadTypeSpeedXType {
    pub max_attrib: TMaxSpeedXType,
    pub unit_attrib: Option<EUnitSpeedXType>,
}
#[derive(Debug)]
pub enum EBorderTypeXType {
    Concrete,
    Curb,
}
#[derive(Debug)]
pub enum EBridgeTypeXType {
    Concrete,
    Steel,
    Brick,
    Wood,
}
#[derive(Debug)]
pub enum EObjectTypeXType {
    None,
    Obstacle,
    Car,
    Pole,
    Tree,
    Vegetation,
    Barrier,
    Building,
    ParkingSpace,
    Patch,
    Railing,
    TrafficIsland,
    Crosswalk,
    StreetLamp,
    Gantry,
    SoundBarrier,
    Van,
    Bus,
    Trailer,
    Bike,
    Motorbike,
    Tram,
    Train,
    Pedestrian,
    Wind,
    RoadMark,
}
#[derive(Debug)]
pub enum EOrientationXType {
    Plus,
    Minus,
    None,
}
#[derive(Debug)]
pub enum EOutlineFillTypeXType {
    Grass,
    Concrete,
    Cobble,
    Asphalt,
    Pavement,
    Gravel,
    Soil,
}
#[derive(Debug)]
pub enum ERoadObjectsObjectParkingSpaceAccessXType {
    All,
    Car,
    Women,
    Handicapped,
    Bus,
    Truck,
    Electric,
    Residents,
}
#[derive(Debug)]
pub enum ESideTypeXType {
    Left,
    Right,
    Front,
    Rear,
}
#[derive(Debug)]
pub enum ETunnelTypeXType {
    Standard,
    Underpass,
}
#[derive(Debug)]
pub struct TRoadObjectsXType {
    pub object: Vec<TRoadObjectsObjectXType>,
    pub object_reference: Vec<TRoadObjectsObjectReferenceXType>,
    pub tunnel: Vec<TRoadObjectsTunnelXType>,
    pub bridge: Vec<TRoadObjectsBridgeXType>,
    pub g_additional_data: Vec<TRoadObjectsGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsBridgeXType {
    pub s_attrib: f64,
    pub length_attrib: f64,
    pub name_attrib: Option<String>,
    pub id_attrib: String,
    pub type_attrib: EBridgeTypeXType,
    pub validity: Vec<TRoadObjectsObjectLaneValidityXType>,
    pub g_additional_data: Vec<TRoadObjectsBridgeGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectXType {
    pub t_attrib: f64,
    pub z_offset_attrib: f64,
    pub type_attrib: Option<EObjectTypeXType>,
    pub valid_length_attrib: Option<f64>,
    pub orientation_attrib: Option<EOrientationXType>,
    pub subtype_attrib: Option<String>,
    pub dynamic_attrib: Option<TYesNoXType>,
    pub hdg_attrib: Option<f64>,
    pub name_attrib: Option<String>,
    pub pitch_attrib: Option<f64>,
    pub id_attrib: String,
    pub roll_attrib: Option<f64>,
    pub height_attrib: Option<f64>,
    pub s_attrib: f64,
    pub length_attrib: Option<f64>,
    pub width_attrib: Option<f64>,
    pub radius_attrib: Option<f64>,
    pub repeat: Vec<TRoadObjectsObjectRepeatXType>,
    pub outline: Option<TRoadObjectsObjectOutlinesOutlineXType>,
    pub outlines: Option<TRoadObjectsObjectOutlinesXType>,
    pub material: Vec<TRoadObjectsObjectMaterialXType>,
    pub validity: Vec<TRoadObjectsObjectLaneValidityXType>,
    pub parking_space: Option<TRoadObjectsObjectParkingSpaceXType>,
    pub markings: Option<TRoadObjectsObjectMarkingsXType>,
    pub borders: Option<TRoadObjectsObjectBordersXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectReferenceXType {
    pub s_attrib: f64,
    pub t_attrib: f64,
    pub id_attrib: String,
    pub z_offset_attrib: Option<f64>,
    pub valid_length_attrib: Option<f64>,
    pub orientation_attrib: EOrientationXType,
    pub validity: Vec<TRoadObjectsObjectLaneValidityXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectReferenceGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectBordersXType {
    pub border: Vec<TRoadObjectsObjectBordersBorderXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectBordersGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectBordersBorderXType {
    pub width_attrib: f64,
    pub type_attrib: EBorderTypeXType,
    pub outline_id_attrib: usize,
    pub use_complete_outline_attrib: Option<TBoolXType>,
    pub corner_reference: Vec<TRoadObjectsObjectMarkingsMarkingCornerReferenceXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectBordersBorderGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectMarkingsXType {
    pub marking: Vec<TRoadObjectsObjectMarkingsMarkingXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectMarkingsGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectMarkingsMarkingXType {
    pub side_attrib: Option<ESideTypeXType>,
    pub weight_attrib: Option<ERoadMarkWeightXType>,
    pub width_attrib: Option<String>,
    pub color_attrib: ERoadMarkColorXType,
    pub z_offset_attrib: Option<f64>,
    pub space_length_attrib: f64,
    pub line_length_attrib: String,
    pub start_offset_attrib: f64,
    pub stop_offset_attrib: f64,
    pub corner_reference: Vec<TRoadObjectsObjectMarkingsMarkingCornerReferenceXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectMarkingsMarkingGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectMarkingsMarkingCornerReferenceXType {
    pub id_attrib: usize,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectMaterialXType {
    pub surface_attrib: Option<String>,
    pub friction_attrib: Option<f64>,
    pub roughness_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesXType {
    pub outline: Vec<TRoadObjectsObjectOutlinesOutlineXType>,
    pub g_additional_data: Vec<TRoadObjectsObjectOutlinesGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesOutlineXType {
    pub id_attrib: Option<usize>,
    pub fill_type_attrib: Option<EOutlineFillTypeXType>,
    pub outer_attrib: Option<TBoolXType>,
    pub closed_attrib: Option<TBoolXType>,
    pub lane_type_attrib: Option<ELaneTypeXType>,
    pub content_70: Option<TRoadObjectsObjectOutlinesOutlineContent70XType>,
    pub g_additional_data: Vec<TRoadObjectsObjectOutlinesOutlineGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesOutlineCornerLocalXType {
    pub u_attrib: f64,
    pub v_attrib: f64,
    pub z_attrib: f64,
    pub height_attrib: f64,
    pub id_attrib: Option<usize>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesOutlineCornerRoadXType {
    pub s_attrib: f64,
    pub t_attrib: f64,
    pub dz_attrib: f64,
    pub height_attrib: f64,
    pub id_attrib: Option<usize>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectParkingSpaceXType {
    pub access_attrib: ERoadObjectsObjectParkingSpaceAccessXType,
    pub restrictions_attrib: Option<String>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectRepeatXType {
    pub s_attrib: f64,
    pub length_attrib: f64,
    pub distance_attrib: f64,
    pub t_start_attrib: f64,
    pub t_end_attrib: f64,
    pub height_start_attrib: f64,
    pub height_end_attrib: f64,
    pub z_offset_start_attrib: f64,
    pub z_offset_end_attrib: f64,
    pub width_start_attrib: Option<f64>,
    pub width_end_attrib: Option<f64>,
    pub length_start_attrib: Option<f64>,
    pub length_end_attrib: Option<f64>,
    pub radius_start_attrib: Option<f64>,
    pub radius_end_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct TRoadObjectsTunnelXType {
    pub s_attrib: f64,
    pub length_attrib: f64,
    pub name_attrib: Option<String>,
    pub id_attrib: String,
    pub type_attrib: ETunnelTypeXType,
    pub lighting_attrib: Option<f64>,
    pub daylight_attrib: Option<f64>,
    pub validity: Vec<TRoadObjectsObjectLaneValidityXType>,
    pub g_additional_data: Vec<TRoadObjectsTunnelGAdditionalDataXType>,
}
#[derive(Debug)]
pub enum EAccessRestrictionTypeXType {
    Simulator,
    AutonomousTraffic,
    Pedestrian,
    PassengerCar,
    Bus,
    Delivery,
    Emergency,
    Taxi,
    ThroughTraffic,
    Truck,
    Bicycle,
    Motorcycle,
    None,
    Trucks,
}
#[derive(Debug)]
pub enum ELaneTypeXType {
    Shoulder,
    Border,
    Driving,
    Stop,
    None,
    Restricted,
    Parking,
    Median,
    Biking,
    Sidewalk,
    Curb,
    Exit,
    Entry,
    OnRamp,
    OffRamp,
    ConnectingRamp,
    Bidirectional,
    Special1,
    Special2,
    Special3,
    RoadWorks,
    Tram,
    Rail,
    Bus,
    Taxi,
    Hov,
    MwyEntry,
    MwyExit,
}
#[derive(Debug)]
pub enum ERoadMarkColorXType {
    Standard,
    Blue,
    Green,
    Red,
    White,
    Yellow,
    Orange,
}
#[derive(Debug)]
pub enum ERoadMarkRuleXType {
    NoPassing,
    Caution,
    None,
}
#[derive(Debug)]
pub enum ERoadMarkTypeXType {
    None,
    Solid,
    Broken,
    SolidSolid,
    SolidBroken,
    BrokenSolid,
    BrokenBroken,
    BottsDots,
    Grass,
    Curb,
    Custom,
    Edge,
}
#[derive(Debug)]
pub enum ERoadMarkWeightXType {
    Standard,
    Bold,
}
#[derive(Debug)]
pub enum ERoadLanesLaneSectionLcrLaneRoadMarkLaneChangeXType {
    Increase,
    Decrease,
    Both,
    None,
}
#[derive(Debug)]
pub enum ERoadLanesLaneSectionLrLaneAccessRuleXType {
    Allow,
    Deny,
}
#[derive(Debug)]
pub enum TBoolXType {
    True,
    False,
}
#[derive(Debug)]
pub struct TRoadLanesXType {
    pub lane_offset: Vec<TRoadLanesLaneOffsetXType>,
    pub lane_section: Vec<TRoadLanesLaneSectionXType>,
    pub g_additional_data: Vec<TRoadLanesGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneOffsetXType {
    pub s_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionXType {
    pub s_attrib: f64,
    pub single_side_attrib: Option<TBoolXType>,
    pub left: Option<TRoadLanesLaneSectionLeftXType>,
    pub center: TRoadLanesLaneSectionCenterXType,
    pub right: Option<TRoadLanesLaneSectionRightXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionCenterXType {
    pub lane: Vec<TRoadLanesLaneSectionCenterLaneXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionCenterGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionCenterLaneXType {
    pub type_attrib: ELaneTypeXType,
    pub level_attrib: Option<TBoolXType>,
    pub id_attrib: i32,
    pub link: Option<TRoadLanesLaneSectionLcrLaneLinkXType>,
    pub content_82: Vec<TRoadLanesLaneSectionLrLaneContent82XType>,
    pub road_mark: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkXType>,
    pub material: Vec<TRoadLanesLaneSectionLrLaneMaterialXType>,
    pub speed: Vec<TRoadLanesLaneSectionLrLaneSpeedXType>,
    pub access: Vec<TRoadLanesLaneSectionLrLaneAccessXType>,
    pub height: Vec<TRoadLanesLaneSectionLrLaneHeightXType>,
    pub rule: Vec<TRoadLanesLaneSectionLrLaneRuleXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLrLaneGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneLinkXType {
    pub predecessor: Vec<TRoadLanesLaneSectionLcrLaneLinkPredecessorSuccessorXType>,
    pub successor: Vec<TRoadLanesLaneSectionLcrLaneLinkPredecessorSuccessorXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLcrLaneLinkGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneLinkPredecessorSuccessorXType {
    pub id_attrib: i32,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkXType {
    pub s_offset_attrib: f64,
    pub type_attrib: ERoadMarkTypeXType,
    pub weight_attrib: Option<ERoadMarkWeightXType>,
    pub color_attrib: ERoadMarkColorXType,
    pub material_attrib: Option<String>,
    pub width_attrib: Option<f64>,
    pub lane_change_attrib: Option<ERoadLanesLaneSectionLcrLaneRoadMarkLaneChangeXType>,
    pub height_attrib: Option<f64>,
    pub sway: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkSwayXType>,
    pub type_: Option<TRoadLanesLaneSectionLcrLaneRoadMarkTypeXType>,
    pub explicit: Option<TRoadLanesLaneSectionLcrLaneRoadMarkExplicitXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkExplicitXType {
    pub line: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkExplicitLineXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkExplicitGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkExplicitLineXType {
    pub length_attrib: String,
    pub t_offset_attrib: f64,
    pub s_offset_attrib: f64,
    pub rule_attrib: Option<ERoadMarkRuleXType>,
    pub width_attrib: Option<String>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkSwayXType {
    pub ds_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkTypeXType {
    pub name_attrib: String,
    pub width_attrib: f64,
    pub line: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkTypeLineXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkTypeGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkTypeLineXType {
    pub length_attrib: String,
    pub space_attrib: f64,
    pub t_offset_attrib: f64,
    pub s_offset_attrib: f64,
    pub rule_attrib: Option<ERoadMarkRuleXType>,
    pub width_attrib: Option<String>,
    pub color_attrib: Option<ERoadMarkColorXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLeftXType {
    pub lane: Vec<TRoadLanesLaneSectionLeftLaneXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLeftGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLeftLaneXType {
    pub type_attrib: ELaneTypeXType,
    pub level_attrib: Option<TBoolXType>,
    pub id_attrib: usize,
    pub link: Option<TRoadLanesLaneSectionLcrLaneLinkXType>,
    pub content_82: Vec<TRoadLanesLaneSectionLrLaneContent82XType>,
    pub road_mark: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkXType>,
    pub material: Vec<TRoadLanesLaneSectionLrLaneMaterialXType>,
    pub speed: Vec<TRoadLanesLaneSectionLrLaneSpeedXType>,
    pub access: Vec<TRoadLanesLaneSectionLrLaneAccessXType>,
    pub height: Vec<TRoadLanesLaneSectionLrLaneHeightXType>,
    pub rule: Vec<TRoadLanesLaneSectionLrLaneRuleXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLrLaneGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneXType {
    pub type_attrib: ELaneTypeXType,
    pub level_attrib: Option<TBoolXType>,
    pub link: Option<TRoadLanesLaneSectionLcrLaneLinkXType>,
    pub content_82: Vec<TRoadLanesLaneSectionLrLaneContent82XType>,
    pub road_mark: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkXType>,
    pub material: Vec<TRoadLanesLaneSectionLrLaneMaterialXType>,
    pub speed: Vec<TRoadLanesLaneSectionLrLaneSpeedXType>,
    pub access: Vec<TRoadLanesLaneSectionLrLaneAccessXType>,
    pub height: Vec<TRoadLanesLaneSectionLrLaneHeightXType>,
    pub rule: Vec<TRoadLanesLaneSectionLrLaneRuleXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLrLaneGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneAccessXType {
    pub s_offset_attrib: f64,
    pub rule_attrib: Option<ERoadLanesLaneSectionLrLaneAccessRuleXType>,
    pub restriction_attrib: EAccessRestrictionTypeXType,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneBorderXType {
    pub s_offset_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneHeightXType {
    pub s_offset_attrib: f64,
    pub inner_attrib: f64,
    pub outer_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneMaterialXType {
    pub s_offset_attrib: f64,
    pub surface_attrib: Option<String>,
    pub friction_attrib: f64,
    pub roughness_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneRuleXType {
    pub s_offset_attrib: f64,
    pub value_attrib: String,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneSpeedXType {
    pub s_offset_attrib: f64,
    pub max_attrib: f64,
    pub unit_attrib: Option<EUnitSpeedXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneWidthXType {
    pub s_offset_attrib: f64,
    pub a_attrib: f64,
    pub b_attrib: f64,
    pub c_attrib: f64,
    pub d_attrib: f64,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionRightXType {
    pub lane: Vec<TRoadLanesLaneSectionRightLaneXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionRightGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionRightLaneXType {
    pub type_attrib: ELaneTypeXType,
    pub level_attrib: Option<TBoolXType>,
    pub id_attrib: isize,
    pub link: Option<TRoadLanesLaneSectionLcrLaneLinkXType>,
    pub content_82: Vec<TRoadLanesLaneSectionLrLaneContent82XType>,
    pub road_mark: Vec<TRoadLanesLaneSectionLcrLaneRoadMarkXType>,
    pub material: Vec<TRoadLanesLaneSectionLrLaneMaterialXType>,
    pub speed: Vec<TRoadLanesLaneSectionLrLaneSpeedXType>,
    pub access: Vec<TRoadLanesLaneSectionLrLaneAccessXType>,
    pub height: Vec<TRoadLanesLaneSectionLrLaneHeightXType>,
    pub rule: Vec<TRoadLanesLaneSectionLrLaneRuleXType>,
    pub g_additional_data: Vec<TRoadLanesLaneSectionLrLaneGAdditionalDataXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectLaneValidityXType {
    pub from_lane_attrib: i32,
    pub to_lane_attrib: i32,
}
#[derive(Debug, Default)]
pub struct EntitiesXType(pub Vec<String>);
#[derive(Debug, Default)]
pub struct EntityXType(pub Vec<String>);
pub type IdXType = String;
pub type IdrefXType = String;
#[derive(Debug, Default)]
pub struct IdrefsXType(pub Vec<String>);
pub type NcNameXType = String;
pub type NmtokenXType = String;
#[derive(Debug, Default)]
pub struct NmtokensXType(pub Vec<String>);
pub type NotationXType = String;
pub type NameXType = String;
pub type QNameXType = String;
pub type AnySimpleTypeXType = String;
#[derive(Debug)]
pub struct AnyTypeXType;
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
pub type IntegerXType = i32;
pub type LanguageXType = String;
pub type LongXType = i64;
pub type NegativeIntegerXType = isize;
pub type NonNegativeIntegerXType = usize;
pub type NonPositiveIntegerXType = isize;
pub type NormalizedStringXType = String;
pub type PositiveIntegerXType = usize;
pub type ShortXType = i16;
pub type StringXType = String;
pub type TimeXType = String;
pub type TokenXType = String;
pub type UnsignedByteXType = u8;
pub type UnsignedIntXType = u32;
pub type UnsignedLongXType = u64;
pub type UnsignedShortXType = u16;
#[derive(Debug)]
pub struct OpenDriveGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct THeaderGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct THeaderGeoReferenceGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct THeaderOffsetGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchMainTrackGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchPartnerGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadRailroadSwitchSideTrackGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TStationGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TStationPlatformGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TJunctionGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TJunctionGroupGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TJunctionSurfaceGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TControllerGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadSignalsGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalContent34XType {
    pub content: TRoadSignalsSignalContent34XTypeContent,
}
#[derive(Debug)]
pub enum TRoadSignalsSignalContent34XTypeContent {
    PositionRoad(TRoadSignalsSignalPositionRoadXType),
    PositionInertial(TRoadSignalsSignalPositionInertialXType),
}
#[derive(Debug)]
pub struct TRoadSignalsSignalGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadSignalsSignalReferenceGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadElevationProfileGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLateralProfileGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLinkGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadPlanViewGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadPlanViewGeometryGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadSurfaceGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadTypeGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsBridgeGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectReferenceGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectBordersGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectBordersBorderGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectMarkingsGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectMarkingsMarkingGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesOutlineContent70XType {
    pub content: TRoadObjectsObjectOutlinesOutlineContent70XTypeContent,
}
#[derive(Debug)]
pub enum TRoadObjectsObjectOutlinesOutlineContent70XTypeContent {
    CornerRoad(Vec<TRoadObjectsObjectOutlinesOutlineCornerRoadXType>),
    CornerLocal(Vec<TRoadObjectsObjectOutlinesOutlineCornerLocalXType>),
}
#[derive(Debug)]
pub struct TRoadObjectsObjectOutlinesOutlineGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadObjectsTunnelGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionCenterGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneContent82XType {
    pub content: TRoadLanesLaneSectionLrLaneContent82XTypeContent,
}
#[derive(Debug)]
pub enum TRoadLanesLaneSectionLrLaneContent82XTypeContent {
    Border(Vec<TRoadLanesLaneSectionLrLaneBorderXType>),
    Width(Vec<TRoadLanesLaneSectionLrLaneWidthXType>),
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLrLaneGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneLinkGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkExplicitGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkTypeGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionLeftGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
#[derive(Debug)]
pub struct TRoadLanesLaneSectionRightGAdditionalDataXType {
    pub include: Vec<TIncludeXType>,
    pub user_data: Vec<TUserDataXType>,
    pub data_quality: Option<TDataQualityXType>,
}
