pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub a: String,
    pub pos: Option<String>,
}
pub type Max = MaxType;
#[derive(Debug)]
pub struct MaxType {
    pub a: Vec<String>,
}
pub type Outer = OuterType;
#[derive(Debug)]
pub struct OuterType {
    pub child: ChildType,
    pub b: Option<String>,
}
#[derive(Debug)]
pub struct ChildType {
    pub inner: String,
    pub extra: Option<String>,
}
pub type Node = NodeType;
#[derive(Debug)]
pub struct NodeType {
    pub name: String,
    pub kid: Option<Box<NodeType>>,
}
pub type GRoot = GRootType;
#[derive(Debug)]
pub struct GRootType {
    pub rec_group: GRootRecGroupType,
}
#[derive(Debug)]
pub struct GRootRecGroupType {
    pub v: String,
    pub via: Option<ViaType>,
}
#[derive(Debug)]
pub struct ViaType {
    pub rec_group: ViaRecGroupType,
}
#[derive(Debug)]
pub struct ViaRecGroupType {
    pub v: String,
    pub via: Option<Box<ViaType>>,
}
