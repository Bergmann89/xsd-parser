pub type Catalog = CatalogType;
#[derive(Debug)]
pub struct CatalogType {
    pub title: String,
    pub entry: Vec<CatalogEntryType>,
}
#[derive(Debug)]
pub struct CatalogEntryType {
    pub key: String,
    pub value: Option<String>,
}
