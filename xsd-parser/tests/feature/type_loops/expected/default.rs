#[derive(Debug)]
pub struct EntityType {
    pub content: EntityTypeContent,
}
#[derive(Debug)]
pub enum EntityTypeContent {
    EntityA(EntityAType),
    EntityB(EntityBType),
}
#[derive(Debug)]
pub struct EntityAType {
    pub inner: EntityAHelperType,
}
#[derive(Debug)]
pub struct EntityBType {
    pub inner: EntityBHelperType,
}
#[derive(Debug)]
pub struct EntityAHelperType {
    pub to_b: EntityBType,
}
#[derive(Debug)]
pub struct EntityBHelperType {
    pub to_a: Box<EntityAType>,
}
