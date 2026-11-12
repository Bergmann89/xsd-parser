pub type Packet = PacketType;
#[derive(Debug, PartialEq)]
pub struct PacketType {
    pub sample: [String; 64usize],
    pub small: [u32; 32usize],
    pub threshold: [u32; 33usize],
    pub choice: ChoiceType,
}
#[derive(Debug, PartialEq)]
pub enum ChoiceType {
    Number([u32; 64usize]),
    Text(String),
}
#[derive(Debug, PartialEq)]
pub struct NamedArrayType {
    pub sample: [String; 64usize],
    pub threshold: [u32; 33usize],
}
