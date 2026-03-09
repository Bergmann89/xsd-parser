pub type Collection = CollectionType;
#[derive(Debug)]
pub struct CollectionType {
    pub entry: Vec<CollectionEntryType>,
    pub name: String,
}
#[derive(Debug)]
pub struct CollectionEntryType {
    pub code: String,
}
