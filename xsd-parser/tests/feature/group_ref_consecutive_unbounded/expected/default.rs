pub type Record = RecordType;
#[derive(Debug)]
pub struct RecordType {
    pub contact: Vec<RecordContactType>,
    pub address: Vec<RecordAddressType>,
    pub name: String,
}
#[derive(Debug)]
pub struct RecordContactType {
    pub email: String,
}
#[derive(Debug)]
pub struct RecordAddressType {
    pub street: String,
    pub city: String,
}
