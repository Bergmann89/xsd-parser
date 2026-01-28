pub type OpenDrive = OpenDriveType;
#[derive(Debug)]
pub struct OpenDriveType {
    pub road: TRoadType,
}
#[derive(Debug)]
pub struct TRoadType {
    pub length: f64,
    pub id: String,
    pub name: Option<String>,
    pub g_additional_data_1: TRoadGAdditionalDataType,
    pub mid_section: Option<String>,
    pub g_additional_data_2: TRoadGAdditionalDataType,
}
#[derive(Debug)]
pub struct TRoadGAdditionalDataType {
    pub user_data: Vec<String>,
    pub include: Vec<String>,
    pub data_quality: Option<String>,
}
