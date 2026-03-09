pub type Widget = WidgetType;
#[derive(Debug)]
pub struct WidgetType {
    pub identity: WidgetIdentityType,
    pub name: String,
    pub description: Option<String>,
}
#[derive(Debug)]
pub struct WidgetIdentityType {
    pub identifier: String,
    pub tag: Vec<String>,
}
pub type Container = ContainerType;
#[derive(Debug)]
pub struct ContainerType {
    pub identity: Vec<ContainerIdentityType>,
    pub name: String,
}
#[derive(Debug)]
pub struct ContainerIdentityType {
    pub identifier: String,
    pub tag: Vec<String>,
}
