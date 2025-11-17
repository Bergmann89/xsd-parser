pub type Container = ContainerType;
#[derive(Debug)]
pub struct ContainerType {
    pub content_2: Vec<ContainerContent2Type>,
}
#[derive(Debug)]
pub enum ContainerContent2Type {
    Foo(String),
    Content3(ContainerContent3Type),
}
#[derive(Debug)]
pub struct ContainerContent3Type {
    pub content_4: Vec<ContainerContent4Type>,
}
#[derive(Debug)]
pub enum ContainerContent4Type {
    Bar(String),
}
