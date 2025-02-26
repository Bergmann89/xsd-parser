pub type Shiporder = ShiporderType;
#[derive(Debug, Clone)]
pub struct ShiporderType {
    pub orderid: String,
    pub orderperson: String,
    pub shipto: ShiporderShiptoType,
    pub item: Vec<ShiporderItemType>,
}
#[derive(Debug, Clone)]
pub struct ShiporderShiptoType {
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
}
#[derive(Debug, Clone)]
pub struct ShiporderItemType {
    pub title: String,
    pub note: Option<String>,
    pub quantity: usize,
    pub price: f64,
}
