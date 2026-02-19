pub type OpenDrive = OpenDriveXElementType;
#[derive(Debug)]
pub struct OpenDriveXElementType {
    pub header: OpenDriveHeaderXElementType,
    pub road: Vec<OpenDriveRoadXElementType>,
    pub controller: Vec<OpenDriveControllerXElementType>,
    pub junction: Vec<OpenDriveJunctionXElementType>,
    pub junction_group: Vec<OpenDriveJunctionGroupXElementType>,
    pub station: Vec<OpenDriveStationXElementType>,
}
#[derive(Debug)]
pub enum AccessXType {
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
pub enum BridgeTypeXType {
    Concrete,
    Steel,
    Brick,
    Wood,
}
#[derive(Debug)]
pub struct CenterLaneXType {
    pub id_attrib: Option<i32>,
    pub type_attrib: Option<LaneTypeXType>,
    pub level_attrib: Option<SingleSideXType>,
    pub link: Option<CenterLaneLinkXElementType>,
    pub road_mark: Vec<CenterLaneRoadMarkXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub enum ColorXType {
    Standard,
    Blue,
    Green,
    Red,
    White,
    Yellow,
}
#[derive(Debug)]
pub enum ContactPointXType {
    Start,
    End,
}
#[derive(Debug)]
pub enum CrossfallSideXType {
    Left,
    Right,
    Both,
}
#[derive(Debug)]
pub enum DirXType {
    Plus,
    Minus,
}
#[derive(Debug)]
pub enum DirectionXType {
    Same,
    Opposite,
}
#[derive(Debug)]
pub enum DynamicXType {
    Yes,
    No,
}
#[derive(Debug)]
pub enum ElementTypeXType {
    Road,
    Junction,
}
#[derive(Debug)]
pub struct IncludeXType {
    pub file_attrib: Option<String>,
}
#[derive(Debug)]
pub enum JunctionGroupTypeXType {
    Roundabout,
    Unknown,
}
#[derive(Debug)]
pub struct LaneXType {
    pub id_attrib: Option<i32>,
    pub type_attrib: Option<LaneTypeXType>,
    pub level_attrib: Option<SingleSideXType>,
    pub link: Option<LaneLinkXElementType>,
    pub content_73: LaneContent73XType,
    pub road_mark: Vec<LaneRoadMarkXElementType>,
    pub material: Vec<LaneMaterialXElementType>,
    pub visibility: Vec<LaneVisibilityXElementType>,
    pub speed: Vec<LaneSpeedXElementType>,
    pub access: Vec<LaneAccessXElementType>,
    pub height: Vec<LaneHeightXElementType>,
    pub rule: Vec<LaneRuleXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub enum LaneChangeXType {
    Increase,
    Decrease,
    Both,
    None,
}
#[derive(Debug)]
pub enum LaneTypeXType {
    None,
    Driving,
    Stop,
    Shoulder,
    Biking,
    Sidewalk,
    Border,
    Restricted,
    Parking,
    Bidirectional,
    Median,
    Special1,
    Special2,
    Special3,
    RoadWorks,
    Tram,
    Rail,
    Entry,
    Exit,
    OffRamp,
    OnRamp,
}
#[derive(Debug)]
pub struct LaneValidityXType {
    pub from_lane_attrib: Option<i32>,
    pub to_lane_attrib: Option<i32>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub enum MaxXType {
    Max1(Max1XType),
    I32(i32),
}
#[derive(Debug)]
pub enum ModeXType {
    Attached,
    Attached0,
    Genuine,
}
#[derive(Debug)]
pub enum OrientationXType {
    Plus,
    Minus,
    None,
}
#[derive(Debug)]
pub enum PRangeXType {
    ArcLength,
    Normalized,
}
#[derive(Debug)]
pub struct ParkingSpaceXType {
    pub access_attrib: Option<AccessXType>,
    pub restrictions_attrib: Option<String>,
    pub marking: Vec<ParkingSpaceMarkingXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub enum ParkingSpacemarkingSideXType {
    Front,
    Rear,
    Left,
    Right,
}
#[derive(Debug)]
pub enum PositionXType {
    Dynamic,
    Straight,
    Turn,
}
#[derive(Debug)]
pub enum PurposeXType {
    Elevation,
    Friction,
}
#[derive(Debug)]
pub enum RestrictionXType {
    Simulator,
    AutonomousTraffic,
    Pedestrian,
    None,
}
#[derive(Debug)]
pub enum RoadTypeXType {
    Unknown,
    Rural,
    Motorway,
    Town,
    LowSpeed,
    Pedestrian,
    Bicycle,
}
#[derive(Debug)]
pub enum RoadmarkTypeXType {
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
}
#[derive(Debug)]
pub enum RuleXType {
    NoPassing,
    Caution,
    None,
}
#[derive(Debug)]
pub enum SideXType {
    Left,
    Right,
}
#[derive(Debug)]
pub enum SingleSideXType {
    True,
    False,
}
#[derive(Debug)]
pub enum StationTypeXType {
    Small,
    Medium,
    Large,
}
#[derive(Debug)]
pub enum SurfaceOrientationXType {
    Same,
    Opposite,
}
#[derive(Debug)]
pub enum TunnelTypeXType {
    Standard,
    Underpass,
}
#[derive(Debug)]
pub enum UnitXType {
    M,
    Km,
    Ft,
    Mile,
    MS,
    Mph,
    KmH,
    Kg,
    T,
    Percent,
}
#[derive(Debug)]
pub struct UserDataXType {
    pub code_attrib: Option<String>,
    pub value_attrib: Option<String>,
}
#[derive(Debug)]
pub enum WeightXType {
    Standard,
    Bold,
}
#[derive(Debug, Default)]
pub struct EntitiesXType(pub Vec<String>);
pub type EntityXType = String;
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
pub struct OpenDriveHeaderXElementType {
    pub rev_major_attrib: Option<u16>,
    pub rev_minor_attrib: Option<u16>,
    pub name_attrib: Option<String>,
    pub version_attrib: Option<f32>,
    pub date_attrib: Option<String>,
    pub north_attrib: Option<f64>,
    pub south_attrib: Option<f64>,
    pub east_attrib: Option<f64>,
    pub west_attrib: Option<f64>,
    pub vendor_attrib: Option<String>,
    pub geo_reference: Option<String>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadXElementType {
    pub name_attrib: Option<String>,
    pub length_attrib: Option<f64>,
    pub id_attrib: Option<String>,
    pub junction_attrib: Option<String>,
    pub link: Option<OpenDriveRoadLinkXElementType>,
    pub type_: Vec<OpenDriveRoadTypeXElementType>,
    pub plan_view: OpenDriveRoadPlanViewXElementType,
    pub elevation_profile: Option<OpenDriveRoadElevationProfileXElementType>,
    pub lateral_profile: Option<OpenDriveRoadLateralProfileXElementType>,
    pub lanes: OpenDriveRoadLanesXElementType,
    pub objects: Option<OpenDriveRoadObjectsXElementType>,
    pub signals: Option<OpenDriveRoadSignalsXElementType>,
    pub surface: Option<OpenDriveRoadSurfaceXElementType>,
    pub railroad: Option<OpenDriveRoadRailroadXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveControllerXElementType {
    pub id_attrib: Option<String>,
    pub name_attrib: Option<String>,
    pub sequence_attrib: Option<i32>,
    pub control: Vec<OpenDriveControllerControlXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionXElementType {
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub connection: Vec<OpenDriveJunctionConnectionXElementType>,
    pub priority: Vec<OpenDriveJunctionPriorityXElementType>,
    pub controller: Vec<OpenDriveJunctionControllerXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionGroupXElementType {
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub type_attrib: Option<JunctionGroupTypeXType>,
    pub junction_reference: Vec<OpenDriveJunctionGroupJunctionReferenceXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveStationXElementType {
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub type_attrib: Option<StationTypeXType>,
    pub platform: Vec<OpenDriveStationPlatformXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct CenterLaneLinkXElementType {
    pub predecessor: Option<CenterLaneLinkPredecessorXElementType>,
    pub successor: Option<CenterLaneLinkSuccessorXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct CenterLaneRoadMarkXElementType {
    pub s_offset_attrib: Option<f64>,
    pub type_attrib: Option<RoadmarkTypeXType>,
    pub weight_attrib: Option<WeightXType>,
    pub color_attrib: Option<ColorXType>,
    pub material_attrib: Option<String>,
    pub width_attrib: Option<f64>,
    pub lane_change_attrib: Option<LaneChangeXType>,
    pub height_attrib: Option<f64>,
    pub type_: Option<CenterLaneRoadMarkTypeXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneLinkXElementType {
    pub predecessor: Option<LaneLinkPredecessorXElementType>,
    pub successor: Option<LaneLinkSuccessorXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneContent73XType {
    pub content: LaneContent73XTypeContent,
}
#[derive(Debug)]
pub enum LaneContent73XTypeContent {
    Width(Vec<LaneWidthXElementType>),
    Border(Vec<LaneBorderXElementType>),
}
#[derive(Debug)]
pub struct LaneRoadMarkXElementType {
    pub s_offset_attrib: Option<f64>,
    pub type_attrib: Option<RoadmarkTypeXType>,
    pub weight_attrib: Option<WeightXType>,
    pub color_attrib: Option<ColorXType>,
    pub material_attrib: Option<String>,
    pub width_attrib: Option<f64>,
    pub lane_change_attrib: Option<LaneChangeXType>,
    pub height_attrib: Option<f64>,
    pub type_: Option<LaneRoadMarkTypeXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneMaterialXElementType {
    pub s_offset_attrib: Option<f64>,
    pub surface_attrib: Option<String>,
    pub friction_attrib: Option<f64>,
    pub roughness_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneVisibilityXElementType {
    pub s_offset_attrib: Option<f64>,
    pub forward_attrib: Option<f64>,
    pub back_attrib: Option<f64>,
    pub left_attrib: Option<f64>,
    pub right_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneSpeedXElementType {
    pub s_offset_attrib: Option<f64>,
    pub max_attrib: Option<f64>,
    pub unit_attrib: Option<UnitXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneAccessXElementType {
    pub s_offset_attrib: Option<f64>,
    pub restriction_attrib: Option<RestrictionXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneHeightXElementType {
    pub s_offset_attrib: Option<f64>,
    pub inner_attrib: Option<f64>,
    pub outer_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneRuleXElementType {
    pub s_offset_attrib: Option<f64>,
    pub value_attrib: Option<String>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub enum Max1XType {
    NoLimit,
    Undefined,
}
#[derive(Debug)]
pub struct ParkingSpaceMarkingXElementType {
    pub side_attrib: Option<ParkingSpacemarkingSideXType>,
    pub type_attrib: Option<RoadmarkTypeXType>,
    pub width_attrib: Option<f64>,
    pub color_attrib: Option<ColorXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLinkXElementType {
    pub predecessor: Option<OpenDriveRoadLinkPredecessorXElementType>,
    pub successor: Option<OpenDriveRoadLinkSuccessorXElementType>,
    pub neighbor: Vec<OpenDriveRoadLinkNeighborXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadTypeXElementType {
    pub s_attrib: Option<f64>,
    pub type_attrib: Option<RoadTypeXType>,
    pub speed: Option<OpenDriveRoadTypeSpeedXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewXElementType {
    pub geometry: Vec<OpenDriveRoadPlanViewGeometryXElementType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadElevationProfileXElementType {
    pub elevation: Vec<OpenDriveRoadElevationProfileElevationXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLateralProfileXElementType {
    pub superelevation: Vec<OpenDriveRoadLateralProfileSuperelevationXElementType>,
    pub crossfall: Vec<OpenDriveRoadLateralProfileCrossfallXElementType>,
    pub shape: Vec<OpenDriveRoadLateralProfileShapeXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLanesXElementType {
    pub lane_offset: Vec<OpenDriveRoadLanesLaneOffsetXElementType>,
    pub lane_section: Vec<OpenDriveRoadLanesLaneSectionXElementType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsXElementType {
    pub object: Vec<OpenDriveRoadObjectsObjectXElementType>,
    pub object_reference: Vec<OpenDriveRoadObjectsObjectReferenceXElementType>,
    pub tunnel: Vec<OpenDriveRoadObjectsTunnelXElementType>,
    pub bridge: Vec<OpenDriveRoadObjectsBridgeXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadSignalsXElementType {
    pub signal: Vec<OpenDriveRoadSignalsSignalXElementType>,
    pub signal_reference: Vec<OpenDriveRoadSignalsSignalReferenceXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadSurfaceXElementType {
    pub crg: Vec<OpenDriveRoadSurfaceCrgXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadRailroadXElementType {
    pub switch: Vec<OpenDriveRoadRailroadSwitchXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveControllerControlXElementType {
    pub signal_id_attrib: Option<String>,
    pub type_attrib: Option<String>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionConnectionXElementType {
    pub id_attrib: Option<String>,
    pub incoming_road_attrib: Option<String>,
    pub connecting_road_attrib: Option<String>,
    pub contact_point_attrib: Option<ContactPointXType>,
    pub lane_link: Vec<OpenDriveJunctionConnectionLaneLinkXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionPriorityXElementType {
    pub high_attrib: Option<String>,
    pub low_attrib: Option<String>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionControllerXElementType {
    pub id_attrib: Option<String>,
    pub type_attrib: Option<String>,
    pub sequence_attrib: Option<i32>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionGroupJunctionReferenceXElementType {
    pub junction_attrib: Option<String>,
}
#[derive(Debug)]
pub struct OpenDriveStationPlatformXElementType {
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub segment: Vec<OpenDriveStationPlatformSegmentXElementType>,
}
#[derive(Debug)]
pub struct CenterLaneLinkPredecessorXElementType {
    pub id_attrib: Option<i32>,
}
#[derive(Debug)]
pub struct CenterLaneLinkSuccessorXElementType {
    pub id_attrib: Option<i32>,
}
#[derive(Debug)]
pub struct CenterLaneRoadMarkTypeXElementType {
    pub name_attrib: Option<String>,
    pub width_attrib: Option<f64>,
    pub line: Vec<CenterLaneRoadMarkTypeLineXElementType>,
}
#[derive(Debug)]
pub struct LaneLinkPredecessorXElementType {
    pub id_attrib: Option<i32>,
}
#[derive(Debug)]
pub struct LaneLinkSuccessorXElementType {
    pub id_attrib: Option<i32>,
}
#[derive(Debug)]
pub struct LaneWidthXElementType {
    pub s_offset_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneBorderXElementType {
    pub s_offset_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct LaneRoadMarkTypeXElementType {
    pub name_attrib: Option<String>,
    pub width_attrib: Option<f64>,
    pub line: Vec<LaneRoadMarkTypeLineXElementType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLinkPredecessorXElementType {
    pub element_type_attrib: Option<ElementTypeXType>,
    pub element_id_attrib: Option<String>,
    pub contact_point_attrib: Option<ContactPointXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLinkSuccessorXElementType {
    pub element_type_attrib: Option<ElementTypeXType>,
    pub element_id_attrib: Option<String>,
    pub contact_point_attrib: Option<ContactPointXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLinkNeighborXElementType {
    pub side_attrib: Option<SideXType>,
    pub element_id_attrib: Option<String>,
    pub direction_attrib: Option<DirectionXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadTypeSpeedXElementType {
    pub max_attrib: Option<MaxXType>,
    pub unit_attrib: Option<UnitXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewGeometryXElementType {
    pub s_attrib: Option<f64>,
    pub x_attrib: Option<f64>,
    pub y_attrib: Option<f64>,
    pub hdg_attrib: Option<f64>,
    pub length_attrib: Option<f64>,
    pub content: OpenDriveRoadPlanViewGeometryXElementTypeContent,
}
#[derive(Debug)]
pub enum OpenDriveRoadPlanViewGeometryXElementTypeContent {
    Line(OpenDriveRoadPlanViewGeometryLineXElementType),
    Spiral(OpenDriveRoadPlanViewGeometrySpiralXElementType),
    Arc(OpenDriveRoadPlanViewGeometryArcXElementType),
    Poly3(OpenDriveRoadPlanViewGeometryPoly3XElementType),
    ParamPoly3(OpenDriveRoadPlanViewGeometryParamPoly3XElementType),
    UserData(Vec<UserDataXType>),
    Include(Vec<IncludeXType>),
}
#[derive(Debug)]
pub struct OpenDriveRoadElevationProfileElevationXElementType {
    pub s_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLateralProfileSuperelevationXElementType {
    pub s_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLateralProfileCrossfallXElementType {
    pub side_attrib: Option<CrossfallSideXType>,
    pub s_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLateralProfileShapeXElementType {
    pub s_attrib: Option<f64>,
    pub t_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLanesLaneOffsetXElementType {
    pub s_attrib: Option<f64>,
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLanesLaneSectionXElementType {
    pub s_attrib: Option<f64>,
    pub single_side_attrib: Option<SingleSideXType>,
    pub left: Option<OpenDriveRoadLanesLaneSectionLeftXElementType>,
    pub center: OpenDriveRoadLanesLaneSectionCenterXElementType,
    pub right: Option<OpenDriveRoadLanesLaneSectionRightXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectXElementType {
    pub type_attrib: Option<String>,
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub s_attrib: Option<f64>,
    pub t_attrib: Option<f64>,
    pub z_offset_attrib: Option<f64>,
    pub valid_length_attrib: Option<f64>,
    pub orientation_attrib: Option<OrientationXType>,
    pub length_attrib: Option<f64>,
    pub width_attrib: Option<f64>,
    pub radius_attrib: Option<f64>,
    pub height_attrib: Option<f64>,
    pub hdg_attrib: Option<f64>,
    pub pitch_attrib: Option<f64>,
    pub roll_attrib: Option<f64>,
    pub repeat: Vec<OpenDriveRoadObjectsObjectRepeatXElementType>,
    pub outline: Option<OpenDriveRoadObjectsObjectOutlineXElementType>,
    pub material: Option<OpenDriveRoadObjectsObjectMaterialXElementType>,
    pub validity: Vec<LaneValidityXType>,
    pub parking_space: Option<ParkingSpaceXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectReferenceXElementType {
    pub s_attrib: Option<f64>,
    pub t_attrib: Option<f64>,
    pub id_attrib: Option<String>,
    pub z_offset_attrib: Option<f64>,
    pub valid_length_attrib: Option<f64>,
    pub orientation_attrib: Option<OrientationXType>,
    pub validity: Vec<LaneValidityXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsTunnelXElementType {
    pub s_attrib: Option<f64>,
    pub length_attrib: Option<f64>,
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub type_attrib: Option<TunnelTypeXType>,
    pub lighting_attrib: Option<f64>,
    pub daylight_attrib: Option<f64>,
    pub validity: Vec<LaneValidityXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsBridgeXElementType {
    pub s_attrib: Option<f64>,
    pub length_attrib: Option<f64>,
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub type_attrib: Option<BridgeTypeXType>,
    pub validity: Vec<LaneValidityXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadSignalsSignalXElementType {
    pub s_attrib: Option<f64>,
    pub t_attrib: Option<f64>,
    pub id_attrib: Option<String>,
    pub name_attrib: Option<String>,
    pub dynamic_attrib: Option<DynamicXType>,
    pub orientation_attrib: Option<OrientationXType>,
    pub z_offset_attrib: Option<f64>,
    pub country_attrib: Option<String>,
    pub type_attrib: Option<String>,
    pub subtype_attrib: Option<String>,
    pub value_attrib: Option<f64>,
    pub unit_attrib: Option<UnitXType>,
    pub height_attrib: Option<f64>,
    pub width_attrib: Option<f64>,
    pub text_attrib: Option<String>,
    pub h_offset_attrib: Option<f64>,
    pub pitch_attrib: Option<f64>,
    pub roll_attrib: Option<f64>,
    pub validity: Vec<LaneValidityXType>,
    pub dependency: Vec<OpenDriveRoadSignalsSignalDependencyXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadSignalsSignalReferenceXElementType {
    pub s_attrib: Option<f64>,
    pub t_attrib: Option<f64>,
    pub id_attrib: Option<String>,
    pub orientation_attrib: Option<OrientationXType>,
    pub validity: Vec<LaneValidityXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadSurfaceCrgXElementType {
    pub file_attrib: Option<String>,
    pub s_start_attrib: Option<f64>,
    pub s_end_attrib: Option<f64>,
    pub orientation_attrib: Option<SurfaceOrientationXType>,
    pub mode_attrib: Option<ModeXType>,
    pub purpose_attrib: Option<PurposeXType>,
    pub s_offset_attrib: Option<f64>,
    pub t_offset_attrib: Option<f64>,
    pub z_offset_attrib: Option<f64>,
    pub z_scale_attrib: Option<f64>,
    pub h_offset_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadRailroadSwitchXElementType {
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
    pub position_attrib: Option<PositionXType>,
    pub main_track: OpenDriveRoadRailroadSwitchMainTrackXElementType,
    pub side_track: OpenDriveRoadRailroadSwitchSideTrackXElementType,
    pub partner: Option<OpenDriveRoadRailroadSwitchPartnerXElementType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveJunctionConnectionLaneLinkXElementType {
    pub from_attrib: Option<i32>,
    pub to_attrib: Option<i32>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveStationPlatformSegmentXElementType {
    pub road_id_attrib: Option<String>,
    pub s_start_attrib: Option<f64>,
    pub s_end_attrib: Option<f64>,
    pub side_attrib: Option<SideXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct CenterLaneRoadMarkTypeLineXElementType {
    pub length_attrib: Option<f64>,
    pub space_attrib: Option<f64>,
    pub t_offset_attrib: Option<f64>,
    pub s_offset_attrib: Option<f64>,
    pub rule_attrib: Option<RuleXType>,
    pub width_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct LaneRoadMarkTypeLineXElementType {
    pub length_attrib: Option<f64>,
    pub space_attrib: Option<f64>,
    pub t_offset_attrib: Option<f64>,
    pub s_offset_attrib: Option<f64>,
    pub rule_attrib: Option<RuleXType>,
    pub width_attrib: Option<f64>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewGeometryLineXElementType {
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewGeometrySpiralXElementType {
    pub curv_start_attrib: Option<f64>,
    pub curv_end_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewGeometryArcXElementType {
    pub curvature_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewGeometryPoly3XElementType {
    pub a_attrib: Option<f64>,
    pub b_attrib: Option<f64>,
    pub c_attrib: Option<f64>,
    pub d_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadPlanViewGeometryParamPoly3XElementType {
    pub au_attrib: Option<f64>,
    pub bu_attrib: Option<f64>,
    pub cu_attrib: Option<f64>,
    pub du_attrib: Option<f64>,
    pub av_attrib: Option<f64>,
    pub bv_attrib: Option<f64>,
    pub cv_attrib: Option<f64>,
    pub dv_attrib: Option<f64>,
    pub p_range_attrib: Option<PRangeXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLanesLaneSectionLeftXElementType {
    pub lane: Vec<LaneXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLanesLaneSectionCenterXElementType {
    pub lane: Option<CenterLaneXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadLanesLaneSectionRightXElementType {
    pub lane: Vec<LaneXType>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectRepeatXElementType {
    pub s_attrib: Option<f64>,
    pub length_attrib: Option<f64>,
    pub distance_attrib: Option<f64>,
    pub t_start_attrib: Option<f64>,
    pub t_end_attrib: Option<f64>,
    pub width_start_attrib: Option<f64>,
    pub width_end_attrib: Option<f64>,
    pub height_start_attrib: Option<f64>,
    pub height_end_attrib: Option<f64>,
    pub z_offset_start_attrib: Option<f64>,
    pub z_offset_end_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectOutlineXElementType {
    pub content_29: Option<OpenDriveRoadObjectsObjectOutlineContent29XType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectMaterialXElementType {
    pub surface_attrib: Option<String>,
    pub friction_attrib: Option<f64>,
    pub roughness_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadSignalsSignalDependencyXElementType {
    pub id_attrib: Option<String>,
    pub type_attrib: Option<String>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadRailroadSwitchMainTrackXElementType {
    pub id_attrib: Option<String>,
    pub s_attrib: Option<f64>,
    pub dir_attrib: Option<DirXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadRailroadSwitchSideTrackXElementType {
    pub id_attrib: Option<String>,
    pub s_attrib: Option<f64>,
    pub dir_attrib: Option<DirXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadRailroadSwitchPartnerXElementType {
    pub name_attrib: Option<String>,
    pub id_attrib: Option<String>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectOutlineContent29XType {
    pub content: OpenDriveRoadObjectsObjectOutlineContent29XTypeContent,
}
#[derive(Debug)]
pub enum OpenDriveRoadObjectsObjectOutlineContent29XTypeContent {
    CornerRoad(Vec<OpenDriveRoadObjectsObjectOutlineCornerRoadXElementType>),
    CornerLocal(Vec<OpenDriveRoadObjectsObjectOutlineCornerLocalXElementType>),
    UserData(Vec<UserDataXType>),
    Include(Vec<IncludeXType>),
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectOutlineCornerRoadXElementType {
    pub s_attrib: Option<f64>,
    pub t_attrib: Option<f64>,
    pub dz_attrib: Option<f64>,
    pub height_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
#[derive(Debug)]
pub struct OpenDriveRoadObjectsObjectOutlineCornerLocalXElementType {
    pub u_attrib: Option<f64>,
    pub v_attrib: Option<f64>,
    pub z_attrib: Option<f64>,
    pub height_attrib: Option<f64>,
    pub user_data: Vec<UserDataXType>,
    pub include: Vec<IncludeXType>,
}
