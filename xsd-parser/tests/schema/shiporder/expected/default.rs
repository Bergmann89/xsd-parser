pub type Shiporder = ShiporderType;
#[derive(Debug)]
pub struct ShiporderType {
    pub orderid: String,
    pub orderperson: String,
    pub shipto: ShiporderShiptoType,
    pub item: Vec<ShiporderItemType>,
}
#[derive(Debug)]
pub struct ShiporderShiptoType {
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
}
#[derive(Debug)]
pub struct ShiporderItemType {
    pub title: String,
    pub note: Option<String>,
    pub quantity: usize,
    pub price: f64,
}
