pub type RootElement = RootElementType;
#[derive(Debug)]
pub struct RootElementType {
    pub my_element: RootElementMyElementType,
}
#[derive(Debug)]
pub struct RootElementMyElementType {
    pub my_element: RootElementMyElementMyElementType,
}
#[derive(Debug)]
pub struct RootElementMyElementMyElementType {
    pub created_date: RecordDateType,
}
#[derive(Debug)]
pub struct RecordDateType {
    pub record_date: String,
}
