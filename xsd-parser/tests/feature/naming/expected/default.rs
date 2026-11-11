pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub a_u: Option<i32>,
    pub user_ids: String,
    pub mp3s: String,
    pub ip_v4s: String,
    pub x509_ski: String,
    pub chord: ChordType,
}
#[derive(Debug)]
pub enum ChordType {
    Dominant11th,
    _1_2,
}
