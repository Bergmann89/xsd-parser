pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub au: Option<i32>,
    pub user_i_ds: String,
    pub mp_3s: String,
    pub i_pv_4s: String,
    pub x509_ski: String,
    pub chord: ChordType,
}
#[derive(Debug)]
pub enum ChordType {
    Dominant11Th,
    _12,
}
