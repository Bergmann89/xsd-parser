use core::fmt::Debug;
use xsd_parser_types::AsAny;
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
pub type Scene = SceneType;
#[derive(Debug)]
pub struct SceneType {
    pub shape: Option<Shape>,
}
#[derive(Debug)]
pub struct Shape(pub Box<dyn ShapeTrait>);
pub trait ShapeTrait: Debug + AsAny {}
impl Shape {
    pub fn new<T: ShapeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct ShapeType {
    pub id: Option<i32>,
}
impl ShapeTrait for ShapeType {}
pub type Pick = PickType;
#[derive(Debug)]
pub enum PickType {
    X(String),
    Y(String),
}
pub type Unordered = UnorderedType;
#[derive(Debug)]
pub struct UnorderedType {
    pub x: String,
    pub y: Option<String>,
}
pub type Nothing = NothingType;
#[derive(Debug)]
pub struct NothingType;
