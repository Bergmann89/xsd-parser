pub type Item = ItemType;
#[derive(Debug)]
pub struct ItemType {
    pub metadata: Option<ItemMetadataType>,
    pub name: String,
}
#[derive(Debug)]
pub struct ItemMetadataType {
    pub label: Option<String>,
    pub description: Option<String>,
}
