pub type SomeDetails = WithFirstDataType;
#[derive(Debug)]
pub struct WithFirstDataType {
    pub create_date: Option<String>,
    pub first_info: Option<DataType>,
}
#[derive(Debug)]
pub struct DataType {
    pub create_date: Option<String>,
}
pub type Other = WithAttrType;
#[derive(Debug)]
pub struct WithAttrType {
    pub alt: Option<String>,
    pub content: String,
}
