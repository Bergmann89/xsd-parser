pub type CrossIndustryInvoice = CrossIndustryInvoiceType;
#[derive(Debug)]
pub struct CrossIndustryInvoiceType {
    pub exchanged_document_context: ExchangedDocumentContextType,
    pub exchanged_document: ExchangedDocumentType,
    pub supply_chain_trade_transaction: SupplyChainTradeTransactionType,
}
#[derive(Debug)]
pub struct ExchangedDocumentContextType {
    pub business_process_specified_document_context_parameter: Option<DocumentContextParameterType>,
    pub guideline_specified_document_context_parameter: DocumentContextParameterType,
}
#[derive(Debug)]
pub struct ExchangedDocumentType {
    pub id: IdType,
    pub type_code: DocumentCodeType,
    pub issue_date_time: DateTimeType,
}
#[derive(Debug)]
pub struct SupplyChainTradeTransactionType {
    pub applicable_header_trade_agreement: HeaderTradeAgreementType,
    pub applicable_header_trade_delivery: HeaderTradeDeliveryType,
    pub applicable_header_trade_settlement: HeaderTradeSettlementType,
}
#[derive(Debug)]
pub struct DocumentContextParameterType {
    pub id: IdType,
}
#[derive(Debug)]
pub struct IdType {
    pub scheme_id: Option<String>,
    pub content: String,
}
#[derive(Debug)]
pub struct DocumentCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct DateTimeType {
    pub content: DateTimeTypeContent,
}
#[derive(Debug)]
pub enum DateTimeTypeContent {
    DateTimeString(DateTimeTypeDateTimeStringType),
}
#[derive(Debug)]
pub struct HeaderTradeAgreementType {
    pub buyer_reference: Option<TextType>,
    pub seller_trade_party: TradePartyType,
    pub buyer_trade_party: TradePartyType,
    pub buyer_order_referenced_document: Option<ReferencedDocumentType>,
}
#[derive(Debug)]
pub struct HeaderTradeDeliveryType;
#[derive(Debug)]
pub struct HeaderTradeSettlementType {
    pub invoice_currency_code: CurrencyCodeType,
    pub specified_trade_settlement_header_monetary_summation:
        TradeSettlementHeaderMonetarySummationType,
}
#[derive(Debug)]
pub struct DateTimeTypeDateTimeStringType {
    pub format: String,
    pub content: String,
}
#[derive(Debug)]
pub struct TextType {
    pub content: String,
}
#[derive(Debug)]
pub struct TradePartyType {
    pub name: TextType,
    pub specified_legal_organization: Option<LegalOrganizationType>,
    pub postal_trade_address: Option<TradeAddressType>,
    pub specified_tax_registration: Vec<TaxRegistrationType>,
}
#[derive(Debug)]
pub struct ReferencedDocumentType {
    pub issuer_assigned_id: IdType,
}
#[derive(Debug)]
pub struct CurrencyCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct TradeSettlementHeaderMonetarySummationType {
    pub tax_basis_total_amount: AmountType,
    pub tax_total_amount: Vec<AmountType>,
    pub grand_total_amount: AmountType,
    pub due_payable_amount: AmountType,
}
#[derive(Debug)]
pub struct LegalOrganizationType {
    pub id: Option<IdType>,
}
#[derive(Debug)]
pub struct TradeAddressType {
    pub country_id: CountryIdType,
}
#[derive(Debug)]
pub struct TaxRegistrationType {
    pub id: IdType,
}
#[derive(Debug)]
pub struct AmountType {
    pub currency_id: Option<String>,
    pub content: f64,
}
#[derive(Debug)]
pub struct CountryIdType {
    pub content: String,
}
