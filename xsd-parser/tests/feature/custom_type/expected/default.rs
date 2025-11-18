pub type Amount = CurrencyAmountType;
#[derive(Debug)]
pub struct CurrencyAmountType {
    pub ccy: String,
    pub content: Decimal,
}
