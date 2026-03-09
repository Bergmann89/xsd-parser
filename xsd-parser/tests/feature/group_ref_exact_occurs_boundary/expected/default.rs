pub type Collection = CollectionType;
#[derive(Debug)]
pub struct CollectionType {
    pub entry: [CollectionEntryType; 2usize],
    pub name: String,
}
#[derive(Debug)]
pub struct CollectionEntryType {
    pub code: String,
}
