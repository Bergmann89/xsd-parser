use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shiporder {
    #[serde(rename = "orderid")]
    pub orderid: StringType,
    #[serde(rename = "orderperson")]
    pub orderperson: StringType,
    #[serde(rename = "shipto")]
    pub shipto: ShiporderShipto,
    #[serde(default, rename = "item")]
    pub item: Vec<ShiporderItem>,
}
pub type StringType = String;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiporderShipto {
    #[serde(rename = "name")]
    pub name: StringType,
    #[serde(rename = "address")]
    pub address: StringType,
    #[serde(rename = "city")]
    pub city: StringType,
    #[serde(rename = "country")]
    pub country: StringType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiporderItem {
    #[serde(rename = "title")]
    pub title: StringType,
    #[serde(default, rename = "note")]
    pub note: Option<StringType>,
    #[serde(rename = "quantity")]
    pub quantity: PositiveIntegerType,
    #[serde(rename = "price")]
    pub price: DecimalType,
}
pub type PositiveIntegerType = usize;
pub type DecimalType = f64;
