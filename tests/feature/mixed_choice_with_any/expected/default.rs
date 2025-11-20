pub mod tns {
    use xsd_parser::xml::Text;
    pub type Sdl = RootType;
    #[derive(Debug)]
    pub struct RootType {
        pub container: ContainerType,
    }
    #[derive(Debug)]
    pub struct ContainerType {
        pub content: Vec<ContainerTypeContent>,
    }
    #[derive(Debug)]
    pub enum ContainerTypeContent {
        Known(KnownType),
        Text(Text),
    }
    #[derive(Debug)]
    pub struct KnownType {
        pub name: Option<String>,
    }
}
