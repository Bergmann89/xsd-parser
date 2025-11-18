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
    pub included_note: Vec<NoteType>,
}
#[derive(Debug)]
pub struct SupplyChainTradeTransactionType {
    pub included_supply_chain_trade_line_item: Vec<SupplyChainTradeLineItemType>,
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
pub struct NoteType {
    pub content: TextType,
    pub subject_code: Option<CodeType>,
}
#[derive(Debug)]
pub struct SupplyChainTradeLineItemType {
    pub associated_document_line_document: DocumentLineDocumentType,
    pub specified_trade_product: TradeProductType,
    pub specified_line_trade_agreement: LineTradeAgreementType,
    pub specified_line_trade_delivery: LineTradeDeliveryType,
    pub specified_line_trade_settlement: LineTradeSettlementType,
}
#[derive(Debug)]
pub struct HeaderTradeAgreementType {
    pub buyer_reference: Option<TextType>,
    pub seller_trade_party: TradePartyType,
    pub buyer_trade_party: TradePartyType,
    pub seller_tax_representative_trade_party: Option<TradePartyType>,
    pub buyer_order_referenced_document: Option<ReferencedDocumentType>,
    pub contract_referenced_document: Option<ReferencedDocumentType>,
}
#[derive(Debug)]
pub struct HeaderTradeDeliveryType {
    pub ship_to_trade_party: Option<TradePartyType>,
    pub actual_delivery_supply_chain_event: Option<SupplyChainEventType>,
    pub despatch_advice_referenced_document: Option<ReferencedDocumentType>,
}
#[derive(Debug)]
pub struct HeaderTradeSettlementType {
    pub creditor_reference_id: Option<IdType>,
    pub payment_reference: Option<TextType>,
    pub tax_currency_code: Option<CurrencyCodeType>,
    pub invoice_currency_code: CurrencyCodeType,
    pub payee_trade_party: Option<TradePartyType>,
    pub specified_trade_settlement_payment_means: Vec<TradeSettlementPaymentMeansType>,
    pub applicable_trade_tax: Vec<TradeTaxType>,
    pub billing_specified_period: Option<SpecifiedPeriodType>,
    pub specified_trade_allowance_charge: Vec<TradeAllowanceChargeType>,
    pub specified_trade_payment_terms: Option<TradePaymentTermsType>,
    pub specified_trade_settlement_header_monetary_summation:
        TradeSettlementHeaderMonetarySummationType,
    pub invoice_referenced_document: Vec<ReferencedDocumentType>,
    pub receivable_specified_trade_accounting_account: Option<TradeAccountingAccountType>,
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
pub struct CodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct DocumentLineDocumentType {
    pub line_id: IdType,
    pub included_note: Option<NoteType>,
}
#[derive(Debug)]
pub struct TradeProductType {
    pub global_id: Option<IdType>,
    pub name: TextType,
}
#[derive(Debug)]
pub struct LineTradeAgreementType {
    pub gross_price_product_trade_price: Option<TradePriceType>,
    pub net_price_product_trade_price: TradePriceType,
}
#[derive(Debug)]
pub struct LineTradeDeliveryType {
    pub billed_quantity: QuantityType,
}
#[derive(Debug)]
pub struct LineTradeSettlementType {
    pub applicable_trade_tax: TradeTaxType,
    pub billing_specified_period: Option<SpecifiedPeriodType>,
    pub specified_trade_allowance_charge: Vec<TradeAllowanceChargeType>,
    pub specified_trade_settlement_line_monetary_summation:
        TradeSettlementLineMonetarySummationType,
}
#[derive(Debug)]
pub struct TradePartyType {
    pub id: Vec<IdType>,
    pub global_id: Vec<IdType>,
    pub name: Option<TextType>,
    pub specified_legal_organization: Option<LegalOrganizationType>,
    pub postal_trade_address: Option<TradeAddressType>,
    pub uri_universal_communication: Option<UniversalCommunicationType>,
    pub specified_tax_registration: Vec<TaxRegistrationType>,
}
#[derive(Debug)]
pub struct ReferencedDocumentType {
    pub issuer_assigned_id: IdType,
    pub formatted_issue_date_time: Option<FormattedDateTimeType>,
}
#[derive(Debug)]
pub struct SupplyChainEventType {
    pub occurrence_date_time: DateTimeType,
}
#[derive(Debug)]
pub struct CurrencyCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct TradeSettlementPaymentMeansType {
    pub type_code: PaymentMeansCodeType,
    pub payer_party_debtor_financial_account: Option<DebtorFinancialAccountType>,
    pub payee_party_creditor_financial_account: Option<CreditorFinancialAccountType>,
}
#[derive(Debug)]
pub struct TradeTaxType {
    pub calculated_amount: Option<AmountType>,
    pub type_code: TaxTypeCodeType,
    pub exemption_reason: Option<TextType>,
    pub basis_amount: Option<AmountType>,
    pub category_code: TaxCategoryCodeType,
    pub exemption_reason_code: Option<CodeType>,
    pub due_date_type_code: Option<TimeReferenceCodeType>,
    pub rate_applicable_percent: Option<PercentType>,
}
#[derive(Debug)]
pub struct SpecifiedPeriodType {
    pub start_date_time: Option<DateTimeType>,
    pub end_date_time: Option<DateTimeType>,
}
#[derive(Debug)]
pub struct TradeAllowanceChargeType {
    pub charge_indicator: IndicatorType,
    pub calculation_percent: Option<PercentType>,
    pub basis_amount: Option<AmountType>,
    pub actual_amount: AmountType,
    pub reason_code: Option<AllowanceChargeReasonCodeType>,
    pub reason: Option<TextType>,
    pub category_trade_tax: Option<TradeTaxType>,
}
#[derive(Debug)]
pub struct TradePaymentTermsType {
    pub description: Option<TextType>,
    pub due_date_date_time: Option<DateTimeType>,
    pub direct_debit_mandate_id: Option<IdType>,
}
#[derive(Debug)]
pub struct TradeSettlementHeaderMonetarySummationType {
    pub line_total_amount: AmountType,
    pub charge_total_amount: Option<AmountType>,
    pub allowance_total_amount: Option<AmountType>,
    pub tax_basis_total_amount: AmountType,
    pub tax_total_amount: Vec<AmountType>,
    pub grand_total_amount: AmountType,
    pub total_prepaid_amount: Option<AmountType>,
    pub due_payable_amount: AmountType,
}
#[derive(Debug)]
pub struct TradeAccountingAccountType {
    pub id: IdType,
}
#[derive(Debug)]
pub struct TradePriceType {
    pub charge_amount: AmountType,
    pub basis_quantity: Option<QuantityType>,
    pub applied_trade_allowance_charge: Option<TradeAllowanceChargeType>,
}
#[derive(Debug)]
pub struct QuantityType {
    pub unit_code: Option<String>,
    pub content: f64,
}
#[derive(Debug)]
pub struct TradeSettlementLineMonetarySummationType {
    pub line_total_amount: AmountType,
}
#[derive(Debug)]
pub struct LegalOrganizationType {
    pub id: Option<IdType>,
    pub trading_business_name: Option<TextType>,
}
#[derive(Debug)]
pub struct TradeAddressType {
    pub postcode_code: Option<CodeType>,
    pub line_one: Option<TextType>,
    pub line_two: Option<TextType>,
    pub line_three: Option<TextType>,
    pub city_name: Option<TextType>,
    pub country_id: CountryIdType,
    pub country_sub_division_name: Option<TextType>,
}
#[derive(Debug)]
pub struct UniversalCommunicationType {
    pub uriid: IdType,
}
#[derive(Debug)]
pub struct TaxRegistrationType {
    pub id: IdType,
}
#[derive(Debug)]
pub struct FormattedDateTimeType {
    pub date_time_string: FormattedDateTimeTypeDateTimeStringType,
}
#[derive(Debug)]
pub struct PaymentMeansCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct DebtorFinancialAccountType {
    pub ibanid: IdType,
}
#[derive(Debug)]
pub struct CreditorFinancialAccountType {
    pub ibanid: Option<IdType>,
    pub proprietary_id: Option<IdType>,
}
#[derive(Debug)]
pub struct AmountType {
    pub currency_id: Option<String>,
    pub content: f64,
}
#[derive(Debug)]
pub struct TaxTypeCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct TaxCategoryCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct TimeReferenceCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct PercentType {
    pub content: f64,
}
#[derive(Debug)]
pub struct IndicatorType {
    pub content: IndicatorTypeContent,
}
#[derive(Debug)]
pub enum IndicatorTypeContent {
    Indicator(bool),
}
#[derive(Debug)]
pub struct AllowanceChargeReasonCodeType {
    pub content: String,
}
#[derive(Debug)]
pub struct CountryIdType {
    pub content: String,
}
#[derive(Debug)]
pub struct FormattedDateTimeTypeDateTimeStringType {
    pub format: String,
    pub content: String,
}
