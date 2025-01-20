#[derive(Debug, Clone)]
pub struct Shiporder {
    pub orderid: StringType,
    pub orderperson: StringType,
    pub shipto: ShiporderShipto,
    pub item: Vec<ShiporderItem>,
}
pub type StringType = String;
#[derive(Debug, Clone)]
pub struct ShiporderShipto {
    pub name: StringType,
    pub address: StringType,
    pub city: StringType,
    pub country: StringType,
}
#[derive(Debug, Clone)]
pub struct ShiporderItem {
    pub title: StringType,
    pub note: Option<StringType>,
    pub quantity: PositiveIntegerType,
    pub price: DecimalType,
}
pub type PositiveIntegerType = usize;
pub type DecimalType = f64;
