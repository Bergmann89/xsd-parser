use serde::{Deserialize, Serialize};
pub type Shiporder = ShiporderType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ShiporderType {
    #[serde(rename = "@orderid")]
    pub orderid: String,
    #[serde(rename = "orderperson")]
    pub orderperson: String,
    #[serde(rename = "shipto")]
    pub shipto: ShiporderShiptoType,
    #[serde(default, rename = "item")]
    pub item: Vec<ShiporderItemType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ShiporderShiptoType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "country")]
    pub country: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ShiporderItemType {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(default, rename = "note")]
    pub note: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: usize,
    #[serde(rename = "price")]
    pub price: f64,
}
