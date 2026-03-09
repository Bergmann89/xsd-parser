pub type Person = PersonType;
#[derive(Debug)]
pub struct PersonType {
    pub address: Vec<PersonAddressType>,
    pub name: String,
}
#[derive(Debug)]
pub struct PersonAddressType {
    pub street: String,
    pub city: String,
}
