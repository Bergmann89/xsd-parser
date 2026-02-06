use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_RSM: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100");
pub const NS_QDT: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:QualifiedDataType:100");
pub const NS_RAM: Namespace = Namespace::new_const(
    b"urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100",
);
pub const NS_UDT: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_RSM: NamespacePrefix = NamespacePrefix::new_const(b"rsm");
pub const PREFIX_QDT: NamespacePrefix = NamespacePrefix::new_const(b"qdt");
pub const PREFIX_RAM: NamespacePrefix = NamespacePrefix::new_const(b"ram");
pub const PREFIX_UDT: NamespacePrefix = NamespacePrefix::new_const(b"udt");
pub type CrossIndustryInvoice = CrossIndustryInvoiceType;
#[derive(Debug)]
pub struct CrossIndustryInvoiceType {
    pub exchanged_document_context: ExchangedDocumentContextType,
    pub exchanged_document: ExchangedDocumentType,
    pub supply_chain_trade_transaction: SupplyChainTradeTransactionType,
}
impl WithSerializer for CrossIndustryInvoiceType {
    type Serializer<'x> = quick_xml_serialize::CrossIndustryInvoiceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CrossIndustryInvoiceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CrossIndustryInvoiceTypeSerializerState::Init__),
            name: name.unwrap_or("rsm:CrossIndustryInvoiceType"),
            is_root,
        })
    }
}
impl WithDeserializer for CrossIndustryInvoiceType {
    type Deserializer = quick_xml_deserialize::CrossIndustryInvoiceTypeDeserializer;
}
#[derive(Debug)]
pub struct ExchangedDocumentContextType {
    pub business_process_specified_document_context_parameter: Option<DocumentContextParameterType>,
    pub guideline_specified_document_context_parameter: DocumentContextParameterType,
}
impl WithSerializer for ExchangedDocumentContextType {
    type Serializer<'x> = quick_xml_serialize::ExchangedDocumentContextTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::ExchangedDocumentContextTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::ExchangedDocumentContextTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:ExchangedDocumentContextType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for ExchangedDocumentContextType {
    type Deserializer = quick_xml_deserialize::ExchangedDocumentContextTypeDeserializer;
}
#[derive(Debug)]
pub struct ExchangedDocumentType {
    pub id: IdType,
    pub type_code: DocumentCodeType,
    pub issue_date_time: DateTimeType,
    pub included_note: Vec<NoteType>,
}
impl WithSerializer for ExchangedDocumentType {
    type Serializer<'x> = quick_xml_serialize::ExchangedDocumentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ExchangedDocumentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ExchangedDocumentTypeSerializerState::Init__),
            name: name.unwrap_or("ram:ExchangedDocumentType"),
            is_root,
        })
    }
}
impl WithDeserializer for ExchangedDocumentType {
    type Deserializer = quick_xml_deserialize::ExchangedDocumentTypeDeserializer;
}
#[derive(Debug)]
pub struct SupplyChainTradeTransactionType {
    pub applicable_header_trade_agreement: HeaderTradeAgreementType,
    pub applicable_header_trade_delivery: HeaderTradeDeliveryType,
    pub applicable_header_trade_settlement: HeaderTradeSettlementType,
}
impl WithSerializer for SupplyChainTradeTransactionType {
    type Serializer<'x> = quick_xml_serialize::SupplyChainTradeTransactionTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::SupplyChainTradeTransactionTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::SupplyChainTradeTransactionTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:SupplyChainTradeTransactionType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for SupplyChainTradeTransactionType {
    type Deserializer = quick_xml_deserialize::SupplyChainTradeTransactionTypeDeserializer;
}
#[derive(Debug)]
pub struct DocumentContextParameterType {
    pub id: IdType,
}
impl WithSerializer for DocumentContextParameterType {
    type Serializer<'x> = quick_xml_serialize::DocumentContextParameterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::DocumentContextParameterTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::DocumentContextParameterTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:DocumentContextParameterType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for DocumentContextParameterType {
    type Deserializer = quick_xml_deserialize::DocumentContextParameterTypeDeserializer;
}
#[derive(Debug)]
pub struct IdType {
    pub scheme_id: Option<String>,
    pub content: String,
}
impl WithSerializer for IdType {
    type Serializer<'x> = quick_xml_serialize::IdTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::IdTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IdTypeSerializerState::Init__),
            name: name.unwrap_or("udt:IDType"),
            is_root,
        })
    }
}
impl WithDeserializer for IdType {
    type Deserializer = quick_xml_deserialize::IdTypeDeserializer;
}
#[derive(Debug)]
pub struct DocumentCodeType {
    pub content: String,
}
impl WithSerializer for DocumentCodeType {
    type Serializer<'x> = quick_xml_serialize::DocumentCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DocumentCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocumentCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:DocumentCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for DocumentCodeType {
    type Deserializer = quick_xml_deserialize::DocumentCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct DateTimeType {
    pub content: DateTimeTypeContent,
}
#[derive(Debug)]
pub enum DateTimeTypeContent {
    DateTimeString(DateTimeTypeDateTimeStringType),
}
impl WithSerializer for DateTimeType {
    type Serializer<'x> = quick_xml_serialize::DateTimeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DateTimeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DateTimeTypeSerializerState::Init__),
            name: name.unwrap_or("udt:DateTimeType"),
            is_root,
        })
    }
}
impl WithSerializer for DateTimeTypeContent {
    type Serializer<'x> = quick_xml_serialize::DateTimeTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DateTimeTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DateTimeTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for DateTimeType {
    type Deserializer = quick_xml_deserialize::DateTimeTypeDeserializer;
}
impl WithDeserializer for DateTimeTypeContent {
    type Deserializer = quick_xml_deserialize::DateTimeTypeContentDeserializer;
}
#[derive(Debug)]
pub struct NoteType {
    pub content: TextType,
    pub subject_code: Option<CodeType>,
}
impl WithSerializer for NoteType {
    type Serializer<'x> = quick_xml_serialize::NoteTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::NoteTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NoteTypeSerializerState::Init__),
            name: name.unwrap_or("ram:NoteType"),
            is_root,
        })
    }
}
impl WithDeserializer for NoteType {
    type Deserializer = quick_xml_deserialize::NoteTypeDeserializer;
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
impl WithSerializer for HeaderTradeAgreementType {
    type Serializer<'x> = quick_xml_serialize::HeaderTradeAgreementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HeaderTradeAgreementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeaderTradeAgreementTypeSerializerState::Init__),
            name: name.unwrap_or("ram:HeaderTradeAgreementType"),
            is_root,
        })
    }
}
impl WithDeserializer for HeaderTradeAgreementType {
    type Deserializer = quick_xml_deserialize::HeaderTradeAgreementTypeDeserializer;
}
#[derive(Debug)]
pub struct HeaderTradeDeliveryType {
    pub ship_to_trade_party: Option<TradePartyType>,
    pub actual_delivery_supply_chain_event: Option<SupplyChainEventType>,
    pub despatch_advice_referenced_document: Option<ReferencedDocumentType>,
}
impl WithSerializer for HeaderTradeDeliveryType {
    type Serializer<'x> = quick_xml_serialize::HeaderTradeDeliveryTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HeaderTradeDeliveryTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeaderTradeDeliveryTypeSerializerState::Init__),
            name: name.unwrap_or("ram:HeaderTradeDeliveryType"),
            is_root,
        })
    }
}
impl WithDeserializer for HeaderTradeDeliveryType {
    type Deserializer = quick_xml_deserialize::HeaderTradeDeliveryTypeDeserializer;
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
impl WithSerializer for HeaderTradeSettlementType {
    type Serializer<'x> = quick_xml_serialize::HeaderTradeSettlementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HeaderTradeSettlementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeaderTradeSettlementTypeSerializerState::Init__),
            name: name.unwrap_or("ram:HeaderTradeSettlementType"),
            is_root,
        })
    }
}
impl WithDeserializer for HeaderTradeSettlementType {
    type Deserializer = quick_xml_deserialize::HeaderTradeSettlementTypeDeserializer;
}
#[derive(Debug)]
pub struct DateTimeTypeDateTimeStringType {
    pub format: String,
    pub content: String,
}
impl WithSerializer for DateTimeTypeDateTimeStringType {
    type Serializer<'x> = quick_xml_serialize::DateTimeTypeDateTimeStringTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::DateTimeTypeDateTimeStringTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::DateTimeTypeDateTimeStringTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("udt:DateTimeTypeDateTimeString"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for DateTimeTypeDateTimeStringType {
    type Deserializer = quick_xml_deserialize::DateTimeTypeDateTimeStringTypeDeserializer;
}
#[derive(Debug)]
pub struct TextType {
    pub content: String,
}
impl WithSerializer for TextType {
    type Serializer<'x> = quick_xml_serialize::TextTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TextTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TextTypeSerializerState::Init__),
            name: name.unwrap_or("udt:TextType"),
            is_root,
        })
    }
}
impl WithDeserializer for TextType {
    type Deserializer = quick_xml_deserialize::TextTypeDeserializer;
}
#[derive(Debug)]
pub struct CodeType {
    pub content: String,
}
impl WithSerializer for CodeType {
    type Serializer<'x> = quick_xml_serialize::CodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CodeTypeSerializerState::Init__),
            name: name.unwrap_or("udt:CodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for CodeType {
    type Deserializer = quick_xml_deserialize::CodeTypeDeserializer;
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
impl WithSerializer for TradePartyType {
    type Serializer<'x> = quick_xml_serialize::TradePartyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradePartyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradePartyTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradePartyType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradePartyType {
    type Deserializer = quick_xml_deserialize::TradePartyTypeDeserializer;
}
#[derive(Debug)]
pub struct ReferencedDocumentType {
    pub issuer_assigned_id: IdType,
    pub formatted_issue_date_time: Option<FormattedDateTimeType>,
}
impl WithSerializer for ReferencedDocumentType {
    type Serializer<'x> = quick_xml_serialize::ReferencedDocumentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ReferencedDocumentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ReferencedDocumentTypeSerializerState::Init__),
            name: name.unwrap_or("ram:ReferencedDocumentType"),
            is_root,
        })
    }
}
impl WithDeserializer for ReferencedDocumentType {
    type Deserializer = quick_xml_deserialize::ReferencedDocumentTypeDeserializer;
}
#[derive(Debug)]
pub struct SupplyChainEventType {
    pub occurrence_date_time: DateTimeType,
}
impl WithSerializer for SupplyChainEventType {
    type Serializer<'x> = quick_xml_serialize::SupplyChainEventTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SupplyChainEventTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SupplyChainEventTypeSerializerState::Init__),
            name: name.unwrap_or("ram:SupplyChainEventType"),
            is_root,
        })
    }
}
impl WithDeserializer for SupplyChainEventType {
    type Deserializer = quick_xml_deserialize::SupplyChainEventTypeDeserializer;
}
#[derive(Debug)]
pub struct CurrencyCodeType {
    pub content: String,
}
impl WithSerializer for CurrencyCodeType {
    type Serializer<'x> = quick_xml_serialize::CurrencyCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CurrencyCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CurrencyCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:CurrencyCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for CurrencyCodeType {
    type Deserializer = quick_xml_deserialize::CurrencyCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeSettlementPaymentMeansType {
    pub type_code: PaymentMeansCodeType,
    pub payer_party_debtor_financial_account: Option<DebtorFinancialAccountType>,
    pub payee_party_creditor_financial_account: Option<CreditorFinancialAccountType>,
}
impl WithSerializer for TradeSettlementPaymentMeansType {
    type Serializer<'x> = quick_xml_serialize::TradeSettlementPaymentMeansTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::TradeSettlementPaymentMeansTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::TradeSettlementPaymentMeansTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:TradeSettlementPaymentMeansType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for TradeSettlementPaymentMeansType {
    type Deserializer = quick_xml_deserialize::TradeSettlementPaymentMeansTypeDeserializer;
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
impl WithSerializer for TradeTaxType {
    type Serializer<'x> = quick_xml_serialize::TradeTaxTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeTaxTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeTaxTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeTaxType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeTaxType {
    type Deserializer = quick_xml_deserialize::TradeTaxTypeDeserializer;
}
#[derive(Debug)]
pub struct SpecifiedPeriodType {
    pub start_date_time: Option<DateTimeType>,
    pub end_date_time: Option<DateTimeType>,
}
impl WithSerializer for SpecifiedPeriodType {
    type Serializer<'x> = quick_xml_serialize::SpecifiedPeriodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SpecifiedPeriodTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SpecifiedPeriodTypeSerializerState::Init__),
            name: name.unwrap_or("ram:SpecifiedPeriodType"),
            is_root,
        })
    }
}
impl WithDeserializer for SpecifiedPeriodType {
    type Deserializer = quick_xml_deserialize::SpecifiedPeriodTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeAllowanceChargeType {
    pub charge_indicator: IndicatorType,
    pub calculation_percent: Option<PercentType>,
    pub basis_amount: Option<AmountType>,
    pub actual_amount: AmountType,
    pub reason_code: Option<AllowanceChargeReasonCodeType>,
    pub reason: Option<TextType>,
    pub category_trade_tax: TradeTaxType,
}
impl WithSerializer for TradeAllowanceChargeType {
    type Serializer<'x> = quick_xml_serialize::TradeAllowanceChargeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeAllowanceChargeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeAllowanceChargeTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeAllowanceChargeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeAllowanceChargeType {
    type Deserializer = quick_xml_deserialize::TradeAllowanceChargeTypeDeserializer;
}
#[derive(Debug)]
pub struct TradePaymentTermsType {
    pub description: Option<TextType>,
    pub due_date_date_time: Option<DateTimeType>,
    pub direct_debit_mandate_id: Option<IdType>,
}
impl WithSerializer for TradePaymentTermsType {
    type Serializer<'x> = quick_xml_serialize::TradePaymentTermsTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradePaymentTermsTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradePaymentTermsTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradePaymentTermsType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradePaymentTermsType {
    type Deserializer = quick_xml_deserialize::TradePaymentTermsTypeDeserializer;
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
impl WithSerializer for TradeSettlementHeaderMonetarySummationType {
    type Serializer<'x> =
        quick_xml_serialize::TradeSettlementHeaderMonetarySummationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok (quick_xml_serialize :: TradeSettlementHeaderMonetarySummationTypeSerializer { value : self , state : Box :: new (quick_xml_serialize :: TradeSettlementHeaderMonetarySummationTypeSerializerState :: Init__) , name : name . unwrap_or ("ram:TradeSettlementHeaderMonetarySummationType") , is_root , })
    }
}
impl WithDeserializer for TradeSettlementHeaderMonetarySummationType {
    type Deserializer =
        quick_xml_deserialize::TradeSettlementHeaderMonetarySummationTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeAccountingAccountType {
    pub id: IdType,
}
impl WithSerializer for TradeAccountingAccountType {
    type Serializer<'x> = quick_xml_serialize::TradeAccountingAccountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeAccountingAccountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeAccountingAccountTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeAccountingAccountType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeAccountingAccountType {
    type Deserializer = quick_xml_deserialize::TradeAccountingAccountTypeDeserializer;
}
#[derive(Debug)]
pub struct LegalOrganizationType {
    pub id: Option<IdType>,
    pub trading_business_name: Option<TextType>,
}
impl WithSerializer for LegalOrganizationType {
    type Serializer<'x> = quick_xml_serialize::LegalOrganizationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::LegalOrganizationTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::LegalOrganizationTypeSerializerState::Init__),
            name: name.unwrap_or("ram:LegalOrganizationType"),
            is_root,
        })
    }
}
impl WithDeserializer for LegalOrganizationType {
    type Deserializer = quick_xml_deserialize::LegalOrganizationTypeDeserializer;
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
impl WithSerializer for TradeAddressType {
    type Serializer<'x> = quick_xml_serialize::TradeAddressTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeAddressTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeAddressTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeAddressType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeAddressType {
    type Deserializer = quick_xml_deserialize::TradeAddressTypeDeserializer;
}
#[derive(Debug)]
pub struct UniversalCommunicationType {
    pub uriid: IdType,
}
impl WithSerializer for UniversalCommunicationType {
    type Serializer<'x> = quick_xml_serialize::UniversalCommunicationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::UniversalCommunicationTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::UniversalCommunicationTypeSerializerState::Init__),
            name: name.unwrap_or("ram:UniversalCommunicationType"),
            is_root,
        })
    }
}
impl WithDeserializer for UniversalCommunicationType {
    type Deserializer = quick_xml_deserialize::UniversalCommunicationTypeDeserializer;
}
#[derive(Debug)]
pub struct TaxRegistrationType {
    pub id: IdType,
}
impl WithSerializer for TaxRegistrationType {
    type Serializer<'x> = quick_xml_serialize::TaxRegistrationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TaxRegistrationTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TaxRegistrationTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TaxRegistrationType"),
            is_root,
        })
    }
}
impl WithDeserializer for TaxRegistrationType {
    type Deserializer = quick_xml_deserialize::TaxRegistrationTypeDeserializer;
}
#[derive(Debug)]
pub struct FormattedDateTimeType {
    pub date_time_string: FormattedDateTimeTypeDateTimeStringType,
}
impl WithSerializer for FormattedDateTimeType {
    type Serializer<'x> = quick_xml_serialize::FormattedDateTimeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FormattedDateTimeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FormattedDateTimeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:FormattedDateTimeType"),
            is_root,
        })
    }
}
impl WithDeserializer for FormattedDateTimeType {
    type Deserializer = quick_xml_deserialize::FormattedDateTimeTypeDeserializer;
}
#[derive(Debug)]
pub struct PaymentMeansCodeType {
    pub content: String,
}
impl WithSerializer for PaymentMeansCodeType {
    type Serializer<'x> = quick_xml_serialize::PaymentMeansCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PaymentMeansCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PaymentMeansCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:PaymentMeansCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for PaymentMeansCodeType {
    type Deserializer = quick_xml_deserialize::PaymentMeansCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct DebtorFinancialAccountType {
    pub ibanid: IdType,
}
impl WithSerializer for DebtorFinancialAccountType {
    type Serializer<'x> = quick_xml_serialize::DebtorFinancialAccountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DebtorFinancialAccountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DebtorFinancialAccountTypeSerializerState::Init__),
            name: name.unwrap_or("ram:DebtorFinancialAccountType"),
            is_root,
        })
    }
}
impl WithDeserializer for DebtorFinancialAccountType {
    type Deserializer = quick_xml_deserialize::DebtorFinancialAccountTypeDeserializer;
}
#[derive(Debug)]
pub struct CreditorFinancialAccountType {
    pub ibanid: Option<IdType>,
    pub proprietary_id: Option<IdType>,
}
impl WithSerializer for CreditorFinancialAccountType {
    type Serializer<'x> = quick_xml_serialize::CreditorFinancialAccountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::CreditorFinancialAccountTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::CreditorFinancialAccountTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:CreditorFinancialAccountType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for CreditorFinancialAccountType {
    type Deserializer = quick_xml_deserialize::CreditorFinancialAccountTypeDeserializer;
}
#[derive(Debug)]
pub struct AmountType {
    pub currency_id: Option<String>,
    pub content: f64,
}
impl WithSerializer for AmountType {
    type Serializer<'x> = quick_xml_serialize::AmountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AmountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AmountTypeSerializerState::Init__),
            name: name.unwrap_or("udt:AmountType"),
            is_root,
        })
    }
}
impl WithDeserializer for AmountType {
    type Deserializer = quick_xml_deserialize::AmountTypeDeserializer;
}
#[derive(Debug)]
pub struct TaxTypeCodeType {
    pub content: String,
}
impl WithSerializer for TaxTypeCodeType {
    type Serializer<'x> = quick_xml_serialize::TaxTypeCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TaxTypeCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TaxTypeCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:TaxTypeCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TaxTypeCodeType {
    type Deserializer = quick_xml_deserialize::TaxTypeCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TaxCategoryCodeType {
    pub content: String,
}
impl WithSerializer for TaxCategoryCodeType {
    type Serializer<'x> = quick_xml_serialize::TaxCategoryCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TaxCategoryCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TaxCategoryCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:TaxCategoryCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TaxCategoryCodeType {
    type Deserializer = quick_xml_deserialize::TaxCategoryCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TimeReferenceCodeType {
    pub content: String,
}
impl WithSerializer for TimeReferenceCodeType {
    type Serializer<'x> = quick_xml_serialize::TimeReferenceCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TimeReferenceCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TimeReferenceCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:TimeReferenceCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TimeReferenceCodeType {
    type Deserializer = quick_xml_deserialize::TimeReferenceCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct PercentType {
    pub content: f64,
}
impl WithSerializer for PercentType {
    type Serializer<'x> = quick_xml_serialize::PercentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PercentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PercentTypeSerializerState::Init__),
            name: name.unwrap_or("udt:PercentType"),
            is_root,
        })
    }
}
impl WithDeserializer for PercentType {
    type Deserializer = quick_xml_deserialize::PercentTypeDeserializer;
}
#[derive(Debug)]
pub struct IndicatorType {
    pub content: IndicatorTypeContent,
}
#[derive(Debug)]
pub enum IndicatorTypeContent {
    Indicator(bool),
}
impl WithSerializer for IndicatorType {
    type Serializer<'x> = quick_xml_serialize::IndicatorTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::IndicatorTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IndicatorTypeSerializerState::Init__),
            name: name.unwrap_or("udt:IndicatorType"),
            is_root,
        })
    }
}
impl WithSerializer for IndicatorTypeContent {
    type Serializer<'x> = quick_xml_serialize::IndicatorTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::IndicatorTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IndicatorTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for IndicatorType {
    type Deserializer = quick_xml_deserialize::IndicatorTypeDeserializer;
}
impl WithDeserializer for IndicatorTypeContent {
    type Deserializer = quick_xml_deserialize::IndicatorTypeContentDeserializer;
}
#[derive(Debug)]
pub struct AllowanceChargeReasonCodeType {
    pub content: String,
}
impl WithSerializer for AllowanceChargeReasonCodeType {
    type Serializer<'x> = quick_xml_serialize::AllowanceChargeReasonCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::AllowanceChargeReasonCodeTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::AllowanceChargeReasonCodeTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("qdt:AllowanceChargeReasonCodeType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for AllowanceChargeReasonCodeType {
    type Deserializer = quick_xml_deserialize::AllowanceChargeReasonCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct CountryIdType {
    pub content: String,
}
impl WithSerializer for CountryIdType {
    type Serializer<'x> = quick_xml_serialize::CountryIdTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CountryIdTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CountryIdTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:CountryIDType"),
            is_root,
        })
    }
}
impl WithDeserializer for CountryIdType {
    type Deserializer = quick_xml_deserialize::CountryIdTypeDeserializer;
}
#[derive(Debug)]
pub struct FormattedDateTimeTypeDateTimeStringType {
    pub format: String,
    pub content: String,
}
impl WithSerializer for FormattedDateTimeTypeDateTimeStringType {
    type Serializer<'x> =
        quick_xml_serialize::FormattedDateTimeTypeDateTimeStringTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok (quick_xml_serialize :: FormattedDateTimeTypeDateTimeStringTypeSerializer { value : self , state : Box :: new (quick_xml_serialize :: FormattedDateTimeTypeDateTimeStringTypeSerializerState :: Init__) , name : name . unwrap_or ("qdt:FormattedDateTimeTypeDateTimeString") , is_root , })
    }
}
impl WithDeserializer for FormattedDateTimeTypeDateTimeStringType {
    type Deserializer = quick_xml_deserialize::FormattedDateTimeTypeDateTimeStringTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct CrossIndustryInvoiceTypeDeserializer {
        exchanged_document_context: Option<super::ExchangedDocumentContextType>,
        exchanged_document: Option<super::ExchangedDocumentType>,
        supply_chain_trade_transaction: Option<super::SupplyChainTradeTransactionType>,
        state__: Box<CrossIndustryInvoiceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CrossIndustryInvoiceTypeDeserializerState {
        Init__,
        ExchangedDocumentContext(
            Option<<super::ExchangedDocumentContextType as WithDeserializer>::Deserializer>,
        ),
        ExchangedDocument(Option<<super::ExchangedDocumentType as WithDeserializer>::Deserializer>),
        SupplyChainTradeTransaction(
            Option<<super::SupplyChainTradeTransactionType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl CrossIndustryInvoiceTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                exchanged_document_context: None,
                exchanged_document: None,
                supply_chain_trade_transaction: None,
                state__: Box::new(CrossIndustryInvoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CrossIndustryInvoiceTypeDeserializerState,
        ) -> Result<(), Error> {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            match state {
                S::ExchangedDocumentContext(Some(deserializer)) => {
                    self.store_exchanged_document_context(deserializer.finish(helper)?)?
                }
                S::ExchangedDocument(Some(deserializer)) => {
                    self.store_exchanged_document(deserializer.finish(helper)?)?
                }
                S::SupplyChainTradeTransaction(Some(deserializer)) => {
                    self.store_supply_chain_trade_transaction(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_exchanged_document_context(
            &mut self,
            value: super::ExchangedDocumentContextType,
        ) -> Result<(), Error> {
            if self.exchanged_document_context.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExchangedDocumentContext",
                )))?;
            }
            self.exchanged_document_context = Some(value);
            Ok(())
        }
        fn store_exchanged_document(
            &mut self,
            value: super::ExchangedDocumentType,
        ) -> Result<(), Error> {
            if self.exchanged_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExchangedDocument",
                )))?;
            }
            self.exchanged_document = Some(value);
            Ok(())
        }
        fn store_supply_chain_trade_transaction(
            &mut self,
            value: super::SupplyChainTradeTransactionType,
        ) -> Result<(), Error> {
            if self.supply_chain_trade_transaction.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SupplyChainTradeTransaction",
                )))?;
            }
            self.supply_chain_trade_transaction = Some(value);
            Ok(())
        }
        fn handle_exchanged_document_context<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ExchangedDocumentContextType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ExchangedDocumentContext(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exchanged_document_context(data)?;
                    *self.state__ = S::ExchangedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ExchangedDocumentContext(Some(deserializer)));
                    *self.state__ = S::ExchangedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_exchanged_document<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ExchangedDocumentType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ExchangedDocument(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exchanged_document(data)?;
                    *self.state__ = S::SupplyChainTradeTransaction(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ExchangedDocument(Some(deserializer)));
                    *self.state__ = S::SupplyChainTradeTransaction(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_supply_chain_trade_transaction<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SupplyChainTradeTransactionType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SupplyChainTradeTransaction(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_supply_chain_trade_transaction(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SupplyChainTradeTransaction(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CrossIndustryInvoiceType>
        for CrossIndustryInvoiceTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CrossIndustryInvoiceType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CrossIndustryInvoiceType> {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::ExchangedDocumentContext(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_exchanged_document_context(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExchangedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_exchanged_document(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SupplyChainTradeTransaction(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_supply_chain_trade_transaction(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::ExchangedDocumentContext(None);
                        event
                    }
                    (
                        S::ExchangedDocumentContext(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RSM),
                            b"ExchangedDocumentContext",
                            false,
                        )?;
                        match self.handle_exchanged_document_context(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExchangedDocument(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RSM),
                            b"ExchangedDocument",
                            false,
                        )?;
                        match self.handle_exchanged_document(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SupplyChainTradeTransaction(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RSM),
                            b"SupplyChainTradeTransaction",
                            false,
                        )?;
                        match self.handle_supply_chain_trade_transaction(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::CrossIndustryInvoiceType, Error> {
            let state = replace(
                &mut *self.state__,
                CrossIndustryInvoiceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CrossIndustryInvoiceType {
                exchanged_document_context: helper
                    .finish_element("ExchangedDocumentContext", self.exchanged_document_context)?,
                exchanged_document: helper
                    .finish_element("ExchangedDocument", self.exchanged_document)?,
                supply_chain_trade_transaction: helper.finish_element(
                    "SupplyChainTradeTransaction",
                    self.supply_chain_trade_transaction,
                )?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentContextTypeDeserializer {
        business_process_specified_document_context_parameter:
            Option<super::DocumentContextParameterType>,
        guideline_specified_document_context_parameter: Option<super::DocumentContextParameterType>,
        state__: Box<ExchangedDocumentContextTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExchangedDocumentContextTypeDeserializerState {
        Init__,
        BusinessProcessSpecifiedDocumentContextParameter(
            Option<<super::DocumentContextParameterType as WithDeserializer>::Deserializer>,
        ),
        GuidelineSpecifiedDocumentContextParameter(
            Option<<super::DocumentContextParameterType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl ExchangedDocumentContextTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                business_process_specified_document_context_parameter: None,
                guideline_specified_document_context_parameter: None,
                state__: Box::new(ExchangedDocumentContextTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ExchangedDocumentContextTypeDeserializerState,
        ) -> Result<(), Error> {
            use ExchangedDocumentContextTypeDeserializerState as S;
            match state {
                S::BusinessProcessSpecifiedDocumentContextParameter(Some(deserializer)) => self
                    .store_business_process_specified_document_context_parameter(
                        deserializer.finish(helper)?,
                    )?,
                S::GuidelineSpecifiedDocumentContextParameter(Some(deserializer)) => self
                    .store_guideline_specified_document_context_parameter(
                        deserializer.finish(helper)?,
                    )?,
                _ => (),
            }
            Ok(())
        }
        fn store_business_process_specified_document_context_parameter(
            &mut self,
            value: super::DocumentContextParameterType,
        ) -> Result<(), Error> {
            if self
                .business_process_specified_document_context_parameter
                .is_some()
            {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BusinessProcessSpecifiedDocumentContextParameter",
                )))?;
            }
            self.business_process_specified_document_context_parameter = Some(value);
            Ok(())
        }
        fn store_guideline_specified_document_context_parameter(
            &mut self,
            value: super::DocumentContextParameterType,
        ) -> Result<(), Error> {
            if self
                .guideline_specified_document_context_parameter
                .is_some()
            {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"GuidelineSpecifiedDocumentContextParameter",
                )))?;
            }
            self.guideline_specified_document_context_parameter = Some(value);
            Ok(())
        }
        fn handle_business_process_specified_document_context_parameter<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DocumentContextParameterType>,
            fallback: &mut Option<ExchangedDocumentContextTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExchangedDocumentContextTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BusinessProcessSpecifiedDocumentContextParameter(None));
                *self.state__ = S::GuidelineSpecifiedDocumentContextParameter(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_business_process_specified_document_context_parameter(data)?;
                    *self.state__ = S::GuidelineSpecifiedDocumentContextParameter(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BusinessProcessSpecifiedDocumentContextParameter(
                        Some(deserializer),
                    ));
                    *self.state__ = S::GuidelineSpecifiedDocumentContextParameter(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_guideline_specified_document_context_parameter<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DocumentContextParameterType>,
            fallback: &mut Option<ExchangedDocumentContextTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExchangedDocumentContextTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::GuidelineSpecifiedDocumentContextParameter(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_guideline_specified_document_context_parameter(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::GuidelineSpecifiedDocumentContextParameter(Some(
                        deserializer,
                    )));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ExchangedDocumentContextType>
        for ExchangedDocumentContextTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentContextType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentContextType> {
            use ExchangedDocumentContextTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (
                        S::BusinessProcessSpecifiedDocumentContextParameter(Some(deserializer)),
                        event,
                    ) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_business_process_specified_document_context_parameter(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GuidelineSpecifiedDocumentContextParameter(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_guideline_specified_document_context_parameter(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::BusinessProcessSpecifiedDocumentContextParameter(None);
                        event
                    }
                    (
                        S::BusinessProcessSpecifiedDocumentContextParameter(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BusinessProcessSpecifiedDocumentContextParameter",
                            false,
                        )?;
                        match self.handle_business_process_specified_document_context_parameter(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::GuidelineSpecifiedDocumentContextParameter(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"GuidelineSpecifiedDocumentContextParameter",
                            false,
                        )?;
                        match self.handle_guideline_specified_document_context_parameter(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ExchangedDocumentContextType, Error> {
            let state = replace(
                &mut *self.state__,
                ExchangedDocumentContextTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ExchangedDocumentContextType {
                business_process_specified_document_context_parameter: self
                    .business_process_specified_document_context_parameter,
                guideline_specified_document_context_parameter: helper.finish_element(
                    "GuidelineSpecifiedDocumentContextParameter",
                    self.guideline_specified_document_context_parameter,
                )?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentTypeDeserializer {
        id: Option<super::IdType>,
        type_code: Option<super::DocumentCodeType>,
        issue_date_time: Option<super::DateTimeType>,
        included_note: Vec<super::NoteType>,
        state__: Box<ExchangedDocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExchangedDocumentTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        TypeCode(Option<<super::DocumentCodeType as WithDeserializer>::Deserializer>),
        IssueDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        IncludedNote(Option<<super::NoteType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ExchangedDocumentTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: None,
                type_code: None,
                issue_date_time: None,
                included_note: Vec::new(),
                state__: Box::new(ExchangedDocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ExchangedDocumentTypeDeserializerState,
        ) -> Result<(), Error> {
            use ExchangedDocumentTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(helper)?)?
                }
                S::IssueDateTime(Some(deserializer)) => {
                    self.store_issue_date_time(deserializer.finish(helper)?)?
                }
                S::IncludedNote(Some(deserializer)) => {
                    self.store_included_note(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn store_type_code(&mut self, value: super::DocumentCodeType) -> Result<(), Error> {
            if self.type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TypeCode",
                )))?;
            }
            self.type_code = Some(value);
            Ok(())
        }
        fn store_issue_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.issue_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IssueDateTime",
                )))?;
            }
            self.issue_date_time = Some(value);
            Ok(())
        }
        fn store_included_note(&mut self, value: super::NoteType) -> Result<(), Error> {
            self.included_note.push(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExchangedDocumentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state__ = S::TypeCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::TypeCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_type_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DocumentCodeType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExchangedDocumentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TypeCode(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    *self.state__ = S::IssueDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TypeCode(Some(deserializer)));
                    *self.state__ = S::IssueDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_issue_date_time<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExchangedDocumentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::IssueDateTime(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_issue_date_time(data)?;
                    *self.state__ = S::IncludedNote(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::IssueDateTime(Some(deserializer)));
                    *self.state__ = S::IncludedNote(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_included_note<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NoteType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExchangedDocumentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::IncludedNote(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_included_note(data)?;
                    *self.state__ = S::IncludedNote(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::IncludedNote(Some(deserializer)));
                    *self.state__ = S::IncludedNote(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ExchangedDocumentType> for ExchangedDocumentTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentType> {
            use ExchangedDocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IssueDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_issue_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IncludedNote(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_included_note(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TypeCode",
                            false,
                        )?;
                        match self.handle_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IssueDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"IssueDateTime",
                            false,
                        )?;
                        match self.handle_issue_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IncludedNote(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"IncludedNote",
                            false,
                        )?;
                        match self.handle_included_note(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ExchangedDocumentType, Error> {
            let state = replace(
                &mut *self.state__,
                ExchangedDocumentTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ExchangedDocumentType {
                id: helper.finish_element("ID", self.id)?,
                type_code: helper.finish_element("TypeCode", self.type_code)?,
                issue_date_time: helper.finish_element("IssueDateTime", self.issue_date_time)?,
                included_note: self.included_note,
            })
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainTradeTransactionTypeDeserializer {
        applicable_header_trade_agreement: Option<super::HeaderTradeAgreementType>,
        applicable_header_trade_delivery: Option<super::HeaderTradeDeliveryType>,
        applicable_header_trade_settlement: Option<super::HeaderTradeSettlementType>,
        state__: Box<SupplyChainTradeTransactionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SupplyChainTradeTransactionTypeDeserializerState {
        Init__,
        ApplicableHeaderTradeAgreement(
            Option<<super::HeaderTradeAgreementType as WithDeserializer>::Deserializer>,
        ),
        ApplicableHeaderTradeDelivery(
            Option<<super::HeaderTradeDeliveryType as WithDeserializer>::Deserializer>,
        ),
        ApplicableHeaderTradeSettlement(
            Option<<super::HeaderTradeSettlementType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl SupplyChainTradeTransactionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                applicable_header_trade_agreement: None,
                applicable_header_trade_delivery: None,
                applicable_header_trade_settlement: None,
                state__: Box::new(SupplyChainTradeTransactionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SupplyChainTradeTransactionTypeDeserializerState,
        ) -> Result<(), Error> {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            match state {
                S::ApplicableHeaderTradeAgreement(Some(deserializer)) => {
                    self.store_applicable_header_trade_agreement(deserializer.finish(helper)?)?
                }
                S::ApplicableHeaderTradeDelivery(Some(deserializer)) => {
                    self.store_applicable_header_trade_delivery(deserializer.finish(helper)?)?
                }
                S::ApplicableHeaderTradeSettlement(Some(deserializer)) => {
                    self.store_applicable_header_trade_settlement(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_applicable_header_trade_agreement(
            &mut self,
            value: super::HeaderTradeAgreementType,
        ) -> Result<(), Error> {
            if self.applicable_header_trade_agreement.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ApplicableHeaderTradeAgreement",
                )))?;
            }
            self.applicable_header_trade_agreement = Some(value);
            Ok(())
        }
        fn store_applicable_header_trade_delivery(
            &mut self,
            value: super::HeaderTradeDeliveryType,
        ) -> Result<(), Error> {
            if self.applicable_header_trade_delivery.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ApplicableHeaderTradeDelivery",
                )))?;
            }
            self.applicable_header_trade_delivery = Some(value);
            Ok(())
        }
        fn store_applicable_header_trade_settlement(
            &mut self,
            value: super::HeaderTradeSettlementType,
        ) -> Result<(), Error> {
            if self.applicable_header_trade_settlement.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ApplicableHeaderTradeSettlement",
                )))?;
            }
            self.applicable_header_trade_settlement = Some(value);
            Ok(())
        }
        fn handle_applicable_header_trade_agreement<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::HeaderTradeAgreementType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ApplicableHeaderTradeAgreement(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_agreement(data)?;
                    *self.state__ = S::ApplicableHeaderTradeDelivery(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ApplicableHeaderTradeAgreement(Some(deserializer)));
                    *self.state__ = S::ApplicableHeaderTradeDelivery(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_applicable_header_trade_delivery<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::HeaderTradeDeliveryType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ApplicableHeaderTradeDelivery(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_delivery(data)?;
                    *self.state__ = S::ApplicableHeaderTradeSettlement(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ApplicableHeaderTradeDelivery(Some(deserializer)));
                    *self.state__ = S::ApplicableHeaderTradeSettlement(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_applicable_header_trade_settlement<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::HeaderTradeSettlementType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ApplicableHeaderTradeSettlement(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_settlement(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ApplicableHeaderTradeSettlement(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SupplyChainTradeTransactionType>
        for SupplyChainTradeTransactionTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainTradeTransactionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainTradeTransactionType> {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::ApplicableHeaderTradeAgreement(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_applicable_header_trade_agreement(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableHeaderTradeDelivery(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_applicable_header_trade_delivery(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableHeaderTradeSettlement(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_applicable_header_trade_settlement(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::ApplicableHeaderTradeAgreement(None);
                        event
                    }
                    (
                        S::ApplicableHeaderTradeAgreement(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeAgreement",
                            false,
                        )?;
                        match self.handle_applicable_header_trade_agreement(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::ApplicableHeaderTradeDelivery(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeDelivery",
                            false,
                        )?;
                        match self.handle_applicable_header_trade_delivery(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::ApplicableHeaderTradeSettlement(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeSettlement",
                            false,
                        )?;
                        match self.handle_applicable_header_trade_settlement(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SupplyChainTradeTransactionType, Error> {
            let state = replace(
                &mut *self.state__,
                SupplyChainTradeTransactionTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SupplyChainTradeTransactionType {
                applicable_header_trade_agreement: helper.finish_element(
                    "ApplicableHeaderTradeAgreement",
                    self.applicable_header_trade_agreement,
                )?,
                applicable_header_trade_delivery: helper.finish_element(
                    "ApplicableHeaderTradeDelivery",
                    self.applicable_header_trade_delivery,
                )?,
                applicable_header_trade_settlement: helper.finish_element(
                    "ApplicableHeaderTradeSettlement",
                    self.applicable_header_trade_settlement,
                )?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentContextParameterTypeDeserializer {
        id: Option<super::IdType>,
        state__: Box<DocumentContextParameterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentContextParameterTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentContextParameterTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: None,
                state__: Box::new(DocumentContextParameterTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DocumentContextParameterTypeDeserializerState,
        ) -> Result<(), Error> {
            use DocumentContextParameterTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<DocumentContextParameterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocumentContextParameterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocumentContextParameterType>
        for DocumentContextParameterTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentContextParameterType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentContextParameterType> {
            use DocumentContextParameterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DocumentContextParameterType, Error> {
            let state = replace(
                &mut *self.state__,
                DocumentContextParameterTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DocumentContextParameterType {
                id: helper.finish_element("ID", self.id)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IdTypeDeserializer {
        scheme_id: Option<String>,
        content: Option<String>,
        state__: Box<IdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IdTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IdTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut scheme_id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"schemeID")
                ) {
                    helper.read_attrib(&mut scheme_id, b"schemeID", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                scheme_id: scheme_id,
                content: None,
                state__: Box::new(IdTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: IdTypeDeserializerState,
        ) -> Result<(), Error> {
            if let IdTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::IdType> {
            use IdTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IdType> for IdTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IdType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IdType> {
            use IdTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::IdType, Error> {
            let state = replace(&mut *self.state__, IdTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::IdType {
                scheme_id: self.scheme_id,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<DocumentCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DocumentCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(DocumentCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DocumentCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let DocumentCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::DocumentCodeType> {
            use DocumentCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocumentCodeType> for DocumentCodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentCodeType> {
            use DocumentCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DocumentCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                DocumentCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DocumentCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDeserializer {
        content: Option<super::DateTimeTypeContent>,
        state__: Box<DateTimeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateTimeTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::DateTimeTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DateTimeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(DateTimeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DateTimeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let DateTimeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::DateTimeTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DateTimeTypeContent>,
            fallback: &mut Option<DateTimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DateTimeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeType> for DateTimeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeType> {
            use DateTimeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::DateTimeTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DateTimeType, Error> {
            let state = replace(&mut *self.state__, DateTimeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::DateTimeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeContentDeserializer {
        state__: Box<DateTimeTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DateTimeTypeContentDeserializerState {
        Init__,
        DateTimeString(
            Option<super::DateTimeTypeDateTimeStringType>,
            Option<<super::DateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer>,
            Option<<super::DateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DateTimeTypeContent),
        Unknown__,
    }
    impl DateTimeTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UDT),
                    Some(b"DateTimeString")
                ) {
                    let output = <super::DateTimeTypeDateTimeStringType as WithDeserializer>::init(
                        helper, event,
                    )?;
                    return self.handle_date_time_string(helper, Default::default(), None, output);
                }
            }
            *self.state__ = DateTimeTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: DateTimeTypeContentDeserializerState,
        ) -> Result<super::DateTimeTypeContent, Error> {
            use DateTimeTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::DateTimeString(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_date_time_string(&mut values, value)?;
                    }
                    Ok(super::DateTimeTypeContent::DateTimeString(
                        helper.finish_element("DateTimeString", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_date_time_string(
            values: &mut Option<super::DateTimeTypeDateTimeStringType>,
            value: super::DateTimeTypeDateTimeStringType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DateTimeString",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_date_time_string<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DateTimeTypeDateTimeStringType>,
            fallback: Option<
                <super::DateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer,
            >,
            output: DeserializerOutput<'de, super::DateTimeTypeDateTimeStringType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DateTimeTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_date_time_string(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_date_time_string(&mut values, data)?;
                    let data = Self::finish_state(helper, S::DateTimeString(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::DateTimeString(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeTypeContent> for DateTimeTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeContent> {
            let deserializer = Self {
                state__: Box::new(DateTimeTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, DateTimeTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeContent> {
            use DateTimeTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::DateTimeString(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_date_time_string(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::DateTimeString(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UDT),
                            b"DateTimeString",
                            false,
                        )?;
                        match self.handle_date_time_string(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DateTimeTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct NoteTypeDeserializer {
        content: Option<super::TextType>,
        subject_code: Option<super::CodeType>,
        state__: Box<NoteTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NoteTypeDeserializerState {
        Init__,
        Content(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SubjectCode(Option<<super::CodeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NoteTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                subject_code: None,
                state__: Box::new(NoteTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NoteTypeDeserializerState,
        ) -> Result<(), Error> {
            use NoteTypeDeserializerState as S;
            match state {
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(helper)?)?
                }
                S::SubjectCode(Some(deserializer)) => {
                    self.store_subject_code(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content",
                )))?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn store_subject_code(&mut self, value: super::CodeType) -> Result<(), Error> {
            if self.subject_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SubjectCode",
                )))?;
            }
            self.subject_code = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<NoteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NoteTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::SubjectCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content(Some(deserializer)));
                    *self.state__ = S::SubjectCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_subject_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CodeType>,
            fallback: &mut Option<NoteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NoteTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SubjectCode(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_subject_code(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SubjectCode(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::NoteType> for NoteTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NoteType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NoteType> {
            use NoteTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SubjectCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_subject_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Content(None);
                        event
                    }
                    (S::Content(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"Content",
                            false,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SubjectCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SubjectCode",
                            false,
                        )?;
                        match self.handle_subject_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NoteType, Error> {
            let state = replace(&mut *self.state__, NoteTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::NoteType {
                content: helper.finish_element("Content", self.content)?,
                subject_code: self.subject_code,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeAgreementTypeDeserializer {
        buyer_reference: Option<super::TextType>,
        seller_trade_party: Option<super::TradePartyType>,
        buyer_trade_party: Option<super::TradePartyType>,
        seller_tax_representative_trade_party: Option<super::TradePartyType>,
        buyer_order_referenced_document: Option<super::ReferencedDocumentType>,
        contract_referenced_document: Option<super::ReferencedDocumentType>,
        state__: Box<HeaderTradeAgreementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeAgreementTypeDeserializerState {
        Init__,
        BuyerReference(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SellerTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        BuyerTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        SellerTaxRepresentativeTradeParty(
            Option<<super::TradePartyType as WithDeserializer>::Deserializer>,
        ),
        BuyerOrderReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        ContractReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl HeaderTradeAgreementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                buyer_reference: None,
                seller_trade_party: None,
                buyer_trade_party: None,
                seller_tax_representative_trade_party: None,
                buyer_order_referenced_document: None,
                contract_referenced_document: None,
                state__: Box::new(HeaderTradeAgreementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: HeaderTradeAgreementTypeDeserializerState,
        ) -> Result<(), Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            match state {
                S::BuyerReference(Some(deserializer)) => {
                    self.store_buyer_reference(deserializer.finish(helper)?)?
                }
                S::SellerTradeParty(Some(deserializer)) => {
                    self.store_seller_trade_party(deserializer.finish(helper)?)?
                }
                S::BuyerTradeParty(Some(deserializer)) => {
                    self.store_buyer_trade_party(deserializer.finish(helper)?)?
                }
                S::SellerTaxRepresentativeTradeParty(Some(deserializer)) => {
                    self.store_seller_tax_representative_trade_party(deserializer.finish(helper)?)?
                }
                S::BuyerOrderReferencedDocument(Some(deserializer)) => {
                    self.store_buyer_order_referenced_document(deserializer.finish(helper)?)?
                }
                S::ContractReferencedDocument(Some(deserializer)) => {
                    self.store_contract_referenced_document(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_buyer_reference(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.buyer_reference.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BuyerReference",
                )))?;
            }
            self.buyer_reference = Some(value);
            Ok(())
        }
        fn store_seller_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.seller_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SellerTradeParty",
                )))?;
            }
            self.seller_trade_party = Some(value);
            Ok(())
        }
        fn store_buyer_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.buyer_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BuyerTradeParty",
                )))?;
            }
            self.buyer_trade_party = Some(value);
            Ok(())
        }
        fn store_seller_tax_representative_trade_party(
            &mut self,
            value: super::TradePartyType,
        ) -> Result<(), Error> {
            if self.seller_tax_representative_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SellerTaxRepresentativeTradeParty",
                )))?;
            }
            self.seller_tax_representative_trade_party = Some(value);
            Ok(())
        }
        fn store_buyer_order_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            if self.buyer_order_referenced_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BuyerOrderReferencedDocument",
                )))?;
            }
            self.buyer_order_referenced_document = Some(value);
            Ok(())
        }
        fn store_contract_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            if self.contract_referenced_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ContractReferencedDocument",
                )))?;
            }
            self.contract_referenced_document = Some(value);
            Ok(())
        }
        fn handle_buyer_reference<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BuyerReference(None));
                *self.state__ = S::SellerTradeParty(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_reference(data)?;
                    *self.state__ = S::SellerTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BuyerReference(Some(deserializer)));
                    *self.state__ = S::SellerTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_seller_trade_party<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SellerTradeParty(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_seller_trade_party(data)?;
                    *self.state__ = S::BuyerTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SellerTradeParty(Some(deserializer)));
                    *self.state__ = S::BuyerTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_buyer_trade_party<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BuyerTradeParty(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_trade_party(data)?;
                    *self.state__ = S::SellerTaxRepresentativeTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BuyerTradeParty(Some(deserializer)));
                    *self.state__ = S::SellerTaxRepresentativeTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_seller_tax_representative_trade_party<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SellerTaxRepresentativeTradeParty(None));
                *self.state__ = S::BuyerOrderReferencedDocument(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_seller_tax_representative_trade_party(data)?;
                    *self.state__ = S::BuyerOrderReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback
                        .get_or_insert(S::SellerTaxRepresentativeTradeParty(Some(deserializer)));
                    *self.state__ = S::BuyerOrderReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_buyer_order_referenced_document<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BuyerOrderReferencedDocument(None));
                *self.state__ = S::ContractReferencedDocument(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_order_referenced_document(data)?;
                    *self.state__ = S::ContractReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BuyerOrderReferencedDocument(Some(deserializer)));
                    *self.state__ = S::ContractReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_contract_referenced_document<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ContractReferencedDocument(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_contract_referenced_document(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ContractReferencedDocument(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeAgreementType>
        for HeaderTradeAgreementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeAgreementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeAgreementType> {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::BuyerReference(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_buyer_reference(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SellerTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_seller_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BuyerTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_buyer_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SellerTaxRepresentativeTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_seller_tax_representative_trade_party(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BuyerOrderReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_buyer_order_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ContractReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_contract_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::BuyerReference(None);
                        event
                    }
                    (S::BuyerReference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BuyerReference",
                            false,
                        )?;
                        match self.handle_buyer_reference(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SellerTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SellerTradeParty",
                            false,
                        )?;
                        match self.handle_seller_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BuyerTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BuyerTradeParty",
                            false,
                        )?;
                        match self.handle_buyer_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SellerTaxRepresentativeTradeParty(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SellerTaxRepresentativeTradeParty",
                            false,
                        )?;
                        match self.handle_seller_tax_representative_trade_party(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::BuyerOrderReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BuyerOrderReferencedDocument",
                            false,
                        )?;
                        match self.handle_buyer_order_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::ContractReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ContractReferencedDocument",
                            false,
                        )?;
                        match self.handle_contract_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::HeaderTradeAgreementType, Error> {
            let state = replace(
                &mut *self.state__,
                HeaderTradeAgreementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::HeaderTradeAgreementType {
                buyer_reference: self.buyer_reference,
                seller_trade_party: helper
                    .finish_element("SellerTradeParty", self.seller_trade_party)?,
                buyer_trade_party: helper
                    .finish_element("BuyerTradeParty", self.buyer_trade_party)?,
                seller_tax_representative_trade_party: self.seller_tax_representative_trade_party,
                buyer_order_referenced_document: self.buyer_order_referenced_document,
                contract_referenced_document: self.contract_referenced_document,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeDeliveryTypeDeserializer {
        ship_to_trade_party: Option<super::TradePartyType>,
        actual_delivery_supply_chain_event: Option<super::SupplyChainEventType>,
        despatch_advice_referenced_document: Option<super::ReferencedDocumentType>,
        state__: Box<HeaderTradeDeliveryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeDeliveryTypeDeserializerState {
        Init__,
        ShipToTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        ActualDeliverySupplyChainEvent(
            Option<<super::SupplyChainEventType as WithDeserializer>::Deserializer>,
        ),
        DespatchAdviceReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl HeaderTradeDeliveryTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                ship_to_trade_party: None,
                actual_delivery_supply_chain_event: None,
                despatch_advice_referenced_document: None,
                state__: Box::new(HeaderTradeDeliveryTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: HeaderTradeDeliveryTypeDeserializerState,
        ) -> Result<(), Error> {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            match state {
                S::ShipToTradeParty(Some(deserializer)) => {
                    self.store_ship_to_trade_party(deserializer.finish(helper)?)?
                }
                S::ActualDeliverySupplyChainEvent(Some(deserializer)) => {
                    self.store_actual_delivery_supply_chain_event(deserializer.finish(helper)?)?
                }
                S::DespatchAdviceReferencedDocument(Some(deserializer)) => {
                    self.store_despatch_advice_referenced_document(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_ship_to_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.ship_to_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ShipToTradeParty",
                )))?;
            }
            self.ship_to_trade_party = Some(value);
            Ok(())
        }
        fn store_actual_delivery_supply_chain_event(
            &mut self,
            value: super::SupplyChainEventType,
        ) -> Result<(), Error> {
            if self.actual_delivery_supply_chain_event.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ActualDeliverySupplyChainEvent",
                )))?;
            }
            self.actual_delivery_supply_chain_event = Some(value);
            Ok(())
        }
        fn store_despatch_advice_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            if self.despatch_advice_referenced_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DespatchAdviceReferencedDocument",
                )))?;
            }
            self.despatch_advice_referenced_document = Some(value);
            Ok(())
        }
        fn handle_ship_to_trade_party<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeDeliveryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ShipToTradeParty(None));
                *self.state__ = S::ActualDeliverySupplyChainEvent(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ship_to_trade_party(data)?;
                    *self.state__ = S::ActualDeliverySupplyChainEvent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ShipToTradeParty(Some(deserializer)));
                    *self.state__ = S::ActualDeliverySupplyChainEvent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_actual_delivery_supply_chain_event<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SupplyChainEventType>,
            fallback: &mut Option<HeaderTradeDeliveryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ActualDeliverySupplyChainEvent(None));
                *self.state__ = S::DespatchAdviceReferencedDocument(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_actual_delivery_supply_chain_event(data)?;
                    *self.state__ = S::DespatchAdviceReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ActualDeliverySupplyChainEvent(Some(deserializer)));
                    *self.state__ = S::DespatchAdviceReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_despatch_advice_referenced_document<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeDeliveryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DespatchAdviceReferencedDocument(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_despatch_advice_referenced_document(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DespatchAdviceReferencedDocument(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeDeliveryType>
        for HeaderTradeDeliveryTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeDeliveryType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeDeliveryType> {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::ShipToTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_ship_to_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ActualDeliverySupplyChainEvent(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_actual_delivery_supply_chain_event(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DespatchAdviceReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_despatch_advice_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::ShipToTradeParty(None);
                        event
                    }
                    (S::ShipToTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ShipToTradeParty",
                            false,
                        )?;
                        match self.handle_ship_to_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::ActualDeliverySupplyChainEvent(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ActualDeliverySupplyChainEvent",
                            false,
                        )?;
                        match self.handle_actual_delivery_supply_chain_event(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::DespatchAdviceReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"DespatchAdviceReferencedDocument",
                            false,
                        )?;
                        match self.handle_despatch_advice_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::HeaderTradeDeliveryType, Error> {
            let state = replace(
                &mut *self.state__,
                HeaderTradeDeliveryTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::HeaderTradeDeliveryType {
                ship_to_trade_party: self.ship_to_trade_party,
                actual_delivery_supply_chain_event: self.actual_delivery_supply_chain_event,
                despatch_advice_referenced_document: self.despatch_advice_referenced_document,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeSettlementTypeDeserializer {
        creditor_reference_id: Option<super::IdType>,
        payment_reference: Option<super::TextType>,
        tax_currency_code: Option<super::CurrencyCodeType>,
        invoice_currency_code: Option<super::CurrencyCodeType>,
        payee_trade_party: Option<super::TradePartyType>,
        specified_trade_settlement_payment_means: Vec<super::TradeSettlementPaymentMeansType>,
        applicable_trade_tax: Vec<super::TradeTaxType>,
        billing_specified_period: Option<super::SpecifiedPeriodType>,
        specified_trade_allowance_charge: Vec<super::TradeAllowanceChargeType>,
        specified_trade_payment_terms: Option<super::TradePaymentTermsType>,
        specified_trade_settlement_header_monetary_summation:
            Option<super::TradeSettlementHeaderMonetarySummationType>,
        invoice_referenced_document: Vec<super::ReferencedDocumentType>,
        receivable_specified_trade_accounting_account: Option<super::TradeAccountingAccountType>,
        state__: Box<HeaderTradeSettlementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeSettlementTypeDeserializerState {
        Init__ , CreditorReferenceId (Option << super :: IdType as WithDeserializer > :: Deserializer >) , PaymentReference (Option << super :: TextType as WithDeserializer > :: Deserializer >) , TaxCurrencyCode (Option << super :: CurrencyCodeType as WithDeserializer > :: Deserializer >) , InvoiceCurrencyCode (Option << super :: CurrencyCodeType as WithDeserializer > :: Deserializer >) , PayeeTradeParty (Option << super :: TradePartyType as WithDeserializer > :: Deserializer >) , SpecifiedTradeSettlementPaymentMeans (Option << super :: TradeSettlementPaymentMeansType as WithDeserializer > :: Deserializer >) , ApplicableTradeTax (Option << super :: TradeTaxType as WithDeserializer > :: Deserializer >) , BillingSpecifiedPeriod (Option << super :: SpecifiedPeriodType as WithDeserializer > :: Deserializer >) , SpecifiedTradeAllowanceCharge (Option << super :: TradeAllowanceChargeType as WithDeserializer > :: Deserializer >) , SpecifiedTradePaymentTerms (Option << super :: TradePaymentTermsType as WithDeserializer > :: Deserializer >) , SpecifiedTradeSettlementHeaderMonetarySummation (Option << super :: TradeSettlementHeaderMonetarySummationType as WithDeserializer > :: Deserializer >) , InvoiceReferencedDocument (Option << super :: ReferencedDocumentType as WithDeserializer > :: Deserializer >) , ReceivableSpecifiedTradeAccountingAccount (Option << super :: TradeAccountingAccountType as WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl HeaderTradeSettlementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                creditor_reference_id: None,
                payment_reference: None,
                tax_currency_code: None,
                invoice_currency_code: None,
                payee_trade_party: None,
                specified_trade_settlement_payment_means: Vec::new(),
                applicable_trade_tax: Vec::new(),
                billing_specified_period: None,
                specified_trade_allowance_charge: Vec::new(),
                specified_trade_payment_terms: None,
                specified_trade_settlement_header_monetary_summation: None,
                invoice_referenced_document: Vec::new(),
                receivable_specified_trade_accounting_account: None,
                state__: Box::new(HeaderTradeSettlementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: HeaderTradeSettlementTypeDeserializerState,
        ) -> Result<(), Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            match state {
                S::CreditorReferenceId(Some(deserializer)) => {
                    self.store_creditor_reference_id(deserializer.finish(helper)?)?
                }
                S::PaymentReference(Some(deserializer)) => {
                    self.store_payment_reference(deserializer.finish(helper)?)?
                }
                S::TaxCurrencyCode(Some(deserializer)) => {
                    self.store_tax_currency_code(deserializer.finish(helper)?)?
                }
                S::InvoiceCurrencyCode(Some(deserializer)) => {
                    self.store_invoice_currency_code(deserializer.finish(helper)?)?
                }
                S::PayeeTradeParty(Some(deserializer)) => {
                    self.store_payee_trade_party(deserializer.finish(helper)?)?
                }
                S::SpecifiedTradeSettlementPaymentMeans(Some(deserializer)) => self
                    .store_specified_trade_settlement_payment_means(deserializer.finish(helper)?)?,
                S::ApplicableTradeTax(Some(deserializer)) => {
                    self.store_applicable_trade_tax(deserializer.finish(helper)?)?
                }
                S::BillingSpecifiedPeriod(Some(deserializer)) => {
                    self.store_billing_specified_period(deserializer.finish(helper)?)?
                }
                S::SpecifiedTradeAllowanceCharge(Some(deserializer)) => {
                    self.store_specified_trade_allowance_charge(deserializer.finish(helper)?)?
                }
                S::SpecifiedTradePaymentTerms(Some(deserializer)) => {
                    self.store_specified_trade_payment_terms(deserializer.finish(helper)?)?
                }
                S::SpecifiedTradeSettlementHeaderMonetarySummation(Some(deserializer)) => self
                    .store_specified_trade_settlement_header_monetary_summation(
                        deserializer.finish(helper)?,
                    )?,
                S::InvoiceReferencedDocument(Some(deserializer)) => {
                    self.store_invoice_referenced_document(deserializer.finish(helper)?)?
                }
                S::ReceivableSpecifiedTradeAccountingAccount(Some(deserializer)) => self
                    .store_receivable_specified_trade_accounting_account(
                        deserializer.finish(helper)?,
                    )?,
                _ => (),
            }
            Ok(())
        }
        fn store_creditor_reference_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.creditor_reference_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CreditorReferenceID",
                )))?;
            }
            self.creditor_reference_id = Some(value);
            Ok(())
        }
        fn store_payment_reference(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.payment_reference.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PaymentReference",
                )))?;
            }
            self.payment_reference = Some(value);
            Ok(())
        }
        fn store_tax_currency_code(&mut self, value: super::CurrencyCodeType) -> Result<(), Error> {
            if self.tax_currency_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TaxCurrencyCode",
                )))?;
            }
            self.tax_currency_code = Some(value);
            Ok(())
        }
        fn store_invoice_currency_code(
            &mut self,
            value: super::CurrencyCodeType,
        ) -> Result<(), Error> {
            if self.invoice_currency_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"InvoiceCurrencyCode",
                )))?;
            }
            self.invoice_currency_code = Some(value);
            Ok(())
        }
        fn store_payee_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.payee_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PayeeTradeParty",
                )))?;
            }
            self.payee_trade_party = Some(value);
            Ok(())
        }
        fn store_specified_trade_settlement_payment_means(
            &mut self,
            value: super::TradeSettlementPaymentMeansType,
        ) -> Result<(), Error> {
            self.specified_trade_settlement_payment_means.push(value);
            Ok(())
        }
        fn store_applicable_trade_tax(&mut self, value: super::TradeTaxType) -> Result<(), Error> {
            self.applicable_trade_tax.push(value);
            Ok(())
        }
        fn store_billing_specified_period(
            &mut self,
            value: super::SpecifiedPeriodType,
        ) -> Result<(), Error> {
            if self.billing_specified_period.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BillingSpecifiedPeriod",
                )))?;
            }
            self.billing_specified_period = Some(value);
            Ok(())
        }
        fn store_specified_trade_allowance_charge(
            &mut self,
            value: super::TradeAllowanceChargeType,
        ) -> Result<(), Error> {
            self.specified_trade_allowance_charge.push(value);
            Ok(())
        }
        fn store_specified_trade_payment_terms(
            &mut self,
            value: super::TradePaymentTermsType,
        ) -> Result<(), Error> {
            if self.specified_trade_payment_terms.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SpecifiedTradePaymentTerms",
                )))?;
            }
            self.specified_trade_payment_terms = Some(value);
            Ok(())
        }
        fn store_specified_trade_settlement_header_monetary_summation(
            &mut self,
            value: super::TradeSettlementHeaderMonetarySummationType,
        ) -> Result<(), Error> {
            if self
                .specified_trade_settlement_header_monetary_summation
                .is_some()
            {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SpecifiedTradeSettlementHeaderMonetarySummation",
                )))?;
            }
            self.specified_trade_settlement_header_monetary_summation = Some(value);
            Ok(())
        }
        fn store_invoice_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            self.invoice_referenced_document.push(value);
            Ok(())
        }
        fn store_receivable_specified_trade_accounting_account(
            &mut self,
            value: super::TradeAccountingAccountType,
        ) -> Result<(), Error> {
            if self.receivable_specified_trade_accounting_account.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ReceivableSpecifiedTradeAccountingAccount",
                )))?;
            }
            self.receivable_specified_trade_accounting_account = Some(value);
            Ok(())
        }
        fn handle_creditor_reference_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CreditorReferenceId(None));
                *self.state__ = S::PaymentReference(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_creditor_reference_id(data)?;
                    *self.state__ = S::PaymentReference(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CreditorReferenceId(Some(deserializer)));
                    *self.state__ = S::PaymentReference(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_payment_reference<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PaymentReference(None));
                *self.state__ = S::TaxCurrencyCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payment_reference(data)?;
                    *self.state__ = S::TaxCurrencyCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PaymentReference(Some(deserializer)));
                    *self.state__ = S::TaxCurrencyCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tax_currency_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CurrencyCodeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TaxCurrencyCode(None));
                *self.state__ = S::InvoiceCurrencyCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_currency_code(data)?;
                    *self.state__ = S::InvoiceCurrencyCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TaxCurrencyCode(Some(deserializer)));
                    *self.state__ = S::InvoiceCurrencyCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_invoice_currency_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CurrencyCodeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::InvoiceCurrencyCode(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_invoice_currency_code(data)?;
                    *self.state__ = S::PayeeTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::InvoiceCurrencyCode(Some(deserializer)));
                    *self.state__ = S::PayeeTradeParty(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_payee_trade_party<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PayeeTradeParty(None));
                *self.state__ = S::SpecifiedTradeSettlementPaymentMeans(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payee_trade_party(data)?;
                    *self.state__ = S::SpecifiedTradeSettlementPaymentMeans(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PayeeTradeParty(Some(deserializer)));
                    *self.state__ = S::SpecifiedTradeSettlementPaymentMeans(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_specified_trade_settlement_payment_means<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeSettlementPaymentMeansType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpecifiedTradeSettlementPaymentMeans(None));
                *self.state__ = S::ApplicableTradeTax(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_settlement_payment_means(data)?;
                    *self.state__ = S::SpecifiedTradeSettlementPaymentMeans(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback
                        .get_or_insert(S::SpecifiedTradeSettlementPaymentMeans(Some(deserializer)));
                    *self.state__ = S::SpecifiedTradeSettlementPaymentMeans(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_applicable_trade_tax<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeTaxType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.applicable_trade_tax.len() < 1usize {
                    fallback.get_or_insert(S::ApplicableTradeTax(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::ApplicableTradeTax(None));
                    *self.state__ = S::BillingSpecifiedPeriod(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_trade_tax(data)?;
                    *self.state__ = S::ApplicableTradeTax(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ApplicableTradeTax(Some(deserializer)));
                    *self.state__ = S::ApplicableTradeTax(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_billing_specified_period<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SpecifiedPeriodType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BillingSpecifiedPeriod(None));
                *self.state__ = S::SpecifiedTradeAllowanceCharge(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_billing_specified_period(data)?;
                    *self.state__ = S::SpecifiedTradeAllowanceCharge(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BillingSpecifiedPeriod(Some(deserializer)));
                    *self.state__ = S::SpecifiedTradeAllowanceCharge(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_specified_trade_allowance_charge<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeAllowanceChargeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpecifiedTradeAllowanceCharge(None));
                *self.state__ = S::SpecifiedTradePaymentTerms(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_allowance_charge(data)?;
                    *self.state__ = S::SpecifiedTradeAllowanceCharge(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SpecifiedTradeAllowanceCharge(Some(deserializer)));
                    *self.state__ = S::SpecifiedTradeAllowanceCharge(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_specified_trade_payment_terms<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradePaymentTermsType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpecifiedTradePaymentTerms(None));
                *self.state__ = S::SpecifiedTradeSettlementHeaderMonetarySummation(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_payment_terms(data)?;
                    *self.state__ = S::SpecifiedTradeSettlementHeaderMonetarySummation(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SpecifiedTradePaymentTerms(Some(deserializer)));
                    *self.state__ = S::SpecifiedTradeSettlementHeaderMonetarySummation(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_specified_trade_settlement_header_monetary_summation<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeSettlementHeaderMonetarySummationType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpecifiedTradeSettlementHeaderMonetarySummation(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_settlement_header_monetary_summation(data)?;
                    *self.state__ = S::InvoiceReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SpecifiedTradeSettlementHeaderMonetarySummation(
                        Some(deserializer),
                    ));
                    *self.state__ = S::InvoiceReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_invoice_referenced_document<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::InvoiceReferencedDocument(None));
                *self.state__ = S::ReceivableSpecifiedTradeAccountingAccount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_invoice_referenced_document(data)?;
                    *self.state__ = S::InvoiceReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::InvoiceReferencedDocument(Some(deserializer)));
                    *self.state__ = S::InvoiceReferencedDocument(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_receivable_specified_trade_accounting_account<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeAccountingAccountType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ReceivableSpecifiedTradeAccountingAccount(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_receivable_specified_trade_accounting_account(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ReceivableSpecifiedTradeAccountingAccount(Some(
                        deserializer,
                    )));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeSettlementType>
        for HeaderTradeSettlementTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeSettlementType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeSettlementType> {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CreditorReferenceId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_creditor_reference_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PaymentReference(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_payment_reference(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxCurrencyCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tax_currency_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::InvoiceCurrencyCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_invoice_currency_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayeeTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_payee_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTradeSettlementPaymentMeans(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_specified_trade_settlement_payment_means(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableTradeTax(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_applicable_trade_tax(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BillingSpecifiedPeriod(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_billing_specified_period(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTradeAllowanceCharge(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_specified_trade_allowance_charge(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTradePaymentTerms(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_specified_trade_payment_terms(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementHeaderMonetarySummation(Some(deserializer)),
                        event,
                    ) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_specified_trade_settlement_header_monetary_summation(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::InvoiceReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_invoice_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ReceivableSpecifiedTradeAccountingAccount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_receivable_specified_trade_accounting_account(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::CreditorReferenceId(None);
                        event
                    }
                    (S::CreditorReferenceId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CreditorReferenceID",
                            false,
                        )?;
                        match self.handle_creditor_reference_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PaymentReference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"PaymentReference",
                            false,
                        )?;
                        match self.handle_payment_reference(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxCurrencyCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TaxCurrencyCode",
                            false,
                        )?;
                        match self.handle_tax_currency_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::InvoiceCurrencyCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"InvoiceCurrencyCode",
                            false,
                        )?;
                        match self.handle_invoice_currency_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayeeTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"PayeeTradeParty",
                            false,
                        )?;
                        match self.handle_payee_trade_party(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementPaymentMeans(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeSettlementPaymentMeans",
                            false,
                        )?;
                        match self.handle_specified_trade_settlement_payment_means(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableTradeTax(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ApplicableTradeTax",
                            false,
                        )?;
                        match self.handle_applicable_trade_tax(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::BillingSpecifiedPeriod(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BillingSpecifiedPeriod",
                            false,
                        )?;
                        match self.handle_billing_specified_period(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTradeAllowanceCharge(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeAllowanceCharge",
                            false,
                        )?;
                        match self.handle_specified_trade_allowance_charge(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTradePaymentTerms(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradePaymentTerms",
                            false,
                        )?;
                        match self.handle_specified_trade_payment_terms(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementHeaderMonetarySummation(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeSettlementHeaderMonetarySummation",
                            false,
                        )?;
                        match self.handle_specified_trade_settlement_header_monetary_summation(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::InvoiceReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"InvoiceReferencedDocument",
                            false,
                        )?;
                        match self.handle_invoice_referenced_document(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::ReceivableSpecifiedTradeAccountingAccount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ReceivableSpecifiedTradeAccountingAccount",
                            false,
                        )?;
                        match self.handle_receivable_specified_trade_accounting_account(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::HeaderTradeSettlementType, Error> {
            let state = replace(
                &mut *self.state__,
                HeaderTradeSettlementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::HeaderTradeSettlementType {
                creditor_reference_id: self.creditor_reference_id,
                payment_reference: self.payment_reference,
                tax_currency_code: self.tax_currency_code,
                invoice_currency_code: helper
                    .finish_element("InvoiceCurrencyCode", self.invoice_currency_code)?,
                payee_trade_party: self.payee_trade_party,
                specified_trade_settlement_payment_means: self
                    .specified_trade_settlement_payment_means,
                applicable_trade_tax: helper.finish_vec(1usize, None, self.applicable_trade_tax)?,
                billing_specified_period: self.billing_specified_period,
                specified_trade_allowance_charge: self.specified_trade_allowance_charge,
                specified_trade_payment_terms: self.specified_trade_payment_terms,
                specified_trade_settlement_header_monetary_summation: helper.finish_element(
                    "SpecifiedTradeSettlementHeaderMonetarySummation",
                    self.specified_trade_settlement_header_monetary_summation,
                )?,
                invoice_referenced_document: self.invoice_referenced_document,
                receivable_specified_trade_accounting_account: self
                    .receivable_specified_trade_accounting_account,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDateTimeStringTypeDeserializer {
        format: String,
        content: Option<String>,
        state__: Box<DateTimeTypeDateTimeStringTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateTimeTypeDateTimeStringTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DateTimeTypeDateTimeStringTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut format: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"format")
                ) {
                    helper.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                format: format.ok_or_else(|| ErrorKind::MissingAttribute("format".into()))?,
                content: None,
                state__: Box::new(DateTimeTypeDateTimeStringTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DateTimeTypeDateTimeStringTypeDeserializerState,
        ) -> Result<(), Error> {
            if let DateTimeTypeDateTimeStringTypeDeserializerState::Content__(deserializer) = state
            {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType> {
            use DateTimeTypeDateTimeStringTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeTypeDateTimeStringType>
        for DateTimeTypeDateTimeStringTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType> {
            use DateTimeTypeDateTimeStringTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DateTimeTypeDateTimeStringType, Error> {
            let state = replace(
                &mut *self.state__,
                DateTimeTypeDateTimeStringTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DateTimeTypeDateTimeStringType {
                format: self.format,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TextTypeDeserializer {
        content: Option<String>,
        state__: Box<TextTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TextTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TextTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(TextTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TextTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TextTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TextType> {
            use TextTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TextType> for TextTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextType> {
            use TextTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TextType, Error> {
            let state = replace(&mut *self.state__, TextTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::TextType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CodeTypeDeserializer {
        content: Option<String>,
        state__: Box<CodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(CodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let CodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CodeType> {
            use CodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CodeType> for CodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CodeType> {
            use CodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CodeType, Error> {
            let state = replace(&mut *self.state__, CodeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::CodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradePartyTypeDeserializer {
        id: Vec<super::IdType>,
        global_id: Vec<super::IdType>,
        name: Option<super::TextType>,
        specified_legal_organization: Option<super::LegalOrganizationType>,
        postal_trade_address: Option<super::TradeAddressType>,
        uri_universal_communication: Option<super::UniversalCommunicationType>,
        specified_tax_registration: Vec<super::TaxRegistrationType>,
        state__: Box<TradePartyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradePartyTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        GlobalId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Name(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SpecifiedLegalOrganization(
            Option<<super::LegalOrganizationType as WithDeserializer>::Deserializer>,
        ),
        PostalTradeAddress(Option<<super::TradeAddressType as WithDeserializer>::Deserializer>),
        UriUniversalCommunication(
            Option<<super::UniversalCommunicationType as WithDeserializer>::Deserializer>,
        ),
        SpecifiedTaxRegistration(
            Option<<super::TaxRegistrationType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl TradePartyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: Vec::new(),
                global_id: Vec::new(),
                name: None,
                specified_legal_organization: None,
                postal_trade_address: None,
                uri_universal_communication: None,
                specified_tax_registration: Vec::new(),
                state__: Box::new(TradePartyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradePartyTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradePartyTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                S::GlobalId(Some(deserializer)) => {
                    self.store_global_id(deserializer.finish(helper)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                S::SpecifiedLegalOrganization(Some(deserializer)) => {
                    self.store_specified_legal_organization(deserializer.finish(helper)?)?
                }
                S::PostalTradeAddress(Some(deserializer)) => {
                    self.store_postal_trade_address(deserializer.finish(helper)?)?
                }
                S::UriUniversalCommunication(Some(deserializer)) => {
                    self.store_uri_universal_communication(deserializer.finish(helper)?)?
                }
                S::SpecifiedTaxRegistration(Some(deserializer)) => {
                    self.store_specified_tax_registration(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            self.id.push(value);
            Ok(())
        }
        fn store_global_id(&mut self, value: super::IdType) -> Result<(), Error> {
            self.global_id.push(value);
            Ok(())
        }
        fn store_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_specified_legal_organization(
            &mut self,
            value: super::LegalOrganizationType,
        ) -> Result<(), Error> {
            if self.specified_legal_organization.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SpecifiedLegalOrganization",
                )))?;
            }
            self.specified_legal_organization = Some(value);
            Ok(())
        }
        fn store_postal_trade_address(
            &mut self,
            value: super::TradeAddressType,
        ) -> Result<(), Error> {
            if self.postal_trade_address.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PostalTradeAddress",
                )))?;
            }
            self.postal_trade_address = Some(value);
            Ok(())
        }
        fn store_uri_universal_communication(
            &mut self,
            value: super::UniversalCommunicationType,
        ) -> Result<(), Error> {
            if self.uri_universal_communication.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"URIUniversalCommunication",
                )))?;
            }
            self.uri_universal_communication = Some(value);
            Ok(())
        }
        fn store_specified_tax_registration(
            &mut self,
            value: super::TaxRegistrationType,
        ) -> Result<(), Error> {
            self.specified_tax_registration.push(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
                *self.state__ = S::GlobalId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state__ = S::Id(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::Id(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_global_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::GlobalId(None));
                *self.state__ = S::Name(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_global_id(data)?;
                    *self.state__ = S::GlobalId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::GlobalId(Some(deserializer)));
                    *self.state__ = S::GlobalId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Name(None));
                *self.state__ = S::SpecifiedLegalOrganization(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state__ = S::SpecifiedLegalOrganization(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::SpecifiedLegalOrganization(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_specified_legal_organization<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::LegalOrganizationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpecifiedLegalOrganization(None));
                *self.state__ = S::PostalTradeAddress(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_legal_organization(data)?;
                    *self.state__ = S::PostalTradeAddress(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SpecifiedLegalOrganization(Some(deserializer)));
                    *self.state__ = S::PostalTradeAddress(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_postal_trade_address<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeAddressType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PostalTradeAddress(None));
                *self.state__ = S::UriUniversalCommunication(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_postal_trade_address(data)?;
                    *self.state__ = S::UriUniversalCommunication(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PostalTradeAddress(Some(deserializer)));
                    *self.state__ = S::UriUniversalCommunication(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_uri_universal_communication<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::UniversalCommunicationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::UriUniversalCommunication(None));
                *self.state__ = S::SpecifiedTaxRegistration(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_uri_universal_communication(data)?;
                    *self.state__ = S::SpecifiedTaxRegistration(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::UriUniversalCommunication(Some(deserializer)));
                    *self.state__ = S::SpecifiedTaxRegistration(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_specified_tax_registration<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TaxRegistrationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePartyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpecifiedTaxRegistration(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_tax_registration(data)?;
                    if self.specified_tax_registration.len() < 2usize {
                        *self.state__ = S::SpecifiedTaxRegistration(None);
                    } else {
                        *self.state__ = S::Done__;
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SpecifiedTaxRegistration(Some(deserializer)));
                    if self.specified_tax_registration.len() < 1usize {
                        *self.state__ = S::SpecifiedTaxRegistration(None);
                    } else {
                        *self.state__ = S::Done__;
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradePartyType> for TradePartyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePartyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePartyType> {
            use TradePartyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GlobalId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_global_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedLegalOrganization(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_specified_legal_organization(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PostalTradeAddress(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_postal_trade_address(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::UriUniversalCommunication(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_uri_universal_communication(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTaxRegistration(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_specified_tax_registration(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GlobalId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"GlobalID",
                            false,
                        )?;
                        match self.handle_global_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedLegalOrganization(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SpecifiedLegalOrganization",
                            false,
                        )?;
                        match self.handle_specified_legal_organization(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PostalTradeAddress(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"PostalTradeAddress",
                            false,
                        )?;
                        match self.handle_postal_trade_address(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::UriUniversalCommunication(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"URIUniversalCommunication",
                            false,
                        )?;
                        match self.handle_uri_universal_communication(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTaxRegistration(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTaxRegistration",
                            false,
                        )?;
                        match self.handle_specified_tax_registration(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradePartyType, Error> {
            let state = replace(
                &mut *self.state__,
                TradePartyTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradePartyType {
                id: self.id,
                global_id: self.global_id,
                name: self.name,
                specified_legal_organization: self.specified_legal_organization,
                postal_trade_address: self.postal_trade_address,
                uri_universal_communication: self.uri_universal_communication,
                specified_tax_registration: self.specified_tax_registration,
            })
        }
    }
    #[derive(Debug)]
    pub struct ReferencedDocumentTypeDeserializer {
        issuer_assigned_id: Option<super::IdType>,
        formatted_issue_date_time: Option<super::FormattedDateTimeType>,
        state__: Box<ReferencedDocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ReferencedDocumentTypeDeserializerState {
        Init__,
        IssuerAssignedId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        FormattedIssueDateTime(
            Option<<super::FormattedDateTimeType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl ReferencedDocumentTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                issuer_assigned_id: None,
                formatted_issue_date_time: None,
                state__: Box::new(ReferencedDocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ReferencedDocumentTypeDeserializerState,
        ) -> Result<(), Error> {
            use ReferencedDocumentTypeDeserializerState as S;
            match state {
                S::IssuerAssignedId(Some(deserializer)) => {
                    self.store_issuer_assigned_id(deserializer.finish(helper)?)?
                }
                S::FormattedIssueDateTime(Some(deserializer)) => {
                    self.store_formatted_issue_date_time(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_issuer_assigned_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.issuer_assigned_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IssuerAssignedID",
                )))?;
            }
            self.issuer_assigned_id = Some(value);
            Ok(())
        }
        fn store_formatted_issue_date_time(
            &mut self,
            value: super::FormattedDateTimeType,
        ) -> Result<(), Error> {
            if self.formatted_issue_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"FormattedIssueDateTime",
                )))?;
            }
            self.formatted_issue_date_time = Some(value);
            Ok(())
        }
        fn handle_issuer_assigned_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<ReferencedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferencedDocumentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::IssuerAssignedId(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_issuer_assigned_id(data)?;
                    *self.state__ = S::FormattedIssueDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::IssuerAssignedId(Some(deserializer)));
                    *self.state__ = S::FormattedIssueDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_formatted_issue_date_time<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FormattedDateTimeType>,
            fallback: &mut Option<ReferencedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferencedDocumentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::FormattedIssueDateTime(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_formatted_issue_date_time(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::FormattedIssueDateTime(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ReferencedDocumentType> for ReferencedDocumentTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferencedDocumentType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferencedDocumentType> {
            use ReferencedDocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::IssuerAssignedId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_issuer_assigned_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::FormattedIssueDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_formatted_issue_date_time(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::IssuerAssignedId(None);
                        event
                    }
                    (S::IssuerAssignedId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"IssuerAssignedID",
                            false,
                        )?;
                        match self.handle_issuer_assigned_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::FormattedIssueDateTime(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"FormattedIssueDateTime",
                            false,
                        )?;
                        match self.handle_formatted_issue_date_time(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ReferencedDocumentType, Error> {
            let state = replace(
                &mut *self.state__,
                ReferencedDocumentTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ReferencedDocumentType {
                issuer_assigned_id: helper
                    .finish_element("IssuerAssignedID", self.issuer_assigned_id)?,
                formatted_issue_date_time: self.formatted_issue_date_time,
            })
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainEventTypeDeserializer {
        occurrence_date_time: Option<super::DateTimeType>,
        state__: Box<SupplyChainEventTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SupplyChainEventTypeDeserializerState {
        Init__,
        OccurrenceDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SupplyChainEventTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                occurrence_date_time: None,
                state__: Box::new(SupplyChainEventTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SupplyChainEventTypeDeserializerState,
        ) -> Result<(), Error> {
            use SupplyChainEventTypeDeserializerState as S;
            match state {
                S::OccurrenceDateTime(Some(deserializer)) => {
                    self.store_occurrence_date_time(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_occurrence_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.occurrence_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"OccurrenceDateTime",
                )))?;
            }
            self.occurrence_date_time = Some(value);
            Ok(())
        }
        fn handle_occurrence_date_time<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<SupplyChainEventTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SupplyChainEventTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::OccurrenceDateTime(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_occurrence_date_time(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::OccurrenceDateTime(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SupplyChainEventType> for SupplyChainEventTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainEventType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainEventType> {
            use SupplyChainEventTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::OccurrenceDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_occurrence_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::OccurrenceDateTime(None);
                        event
                    }
                    (S::OccurrenceDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"OccurrenceDateTime",
                            false,
                        )?;
                        match self.handle_occurrence_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SupplyChainEventType, Error> {
            let state = replace(
                &mut *self.state__,
                SupplyChainEventTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SupplyChainEventType {
                occurrence_date_time: helper
                    .finish_element("OccurrenceDateTime", self.occurrence_date_time)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CurrencyCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<CurrencyCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CurrencyCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CurrencyCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(CurrencyCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CurrencyCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let CurrencyCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType> {
            use CurrencyCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CurrencyCodeType> for CurrencyCodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType> {
            use CurrencyCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::CurrencyCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                CurrencyCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CurrencyCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementPaymentMeansTypeDeserializer {
        type_code: Option<super::PaymentMeansCodeType>,
        payer_party_debtor_financial_account: Option<super::DebtorFinancialAccountType>,
        payee_party_creditor_financial_account: Option<super::CreditorFinancialAccountType>,
        state__: Box<TradeSettlementPaymentMeansTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeSettlementPaymentMeansTypeDeserializerState {
        Init__,
        TypeCode(Option<<super::PaymentMeansCodeType as WithDeserializer>::Deserializer>),
        PayerPartyDebtorFinancialAccount(
            Option<<super::DebtorFinancialAccountType as WithDeserializer>::Deserializer>,
        ),
        PayeePartyCreditorFinancialAccount(
            Option<<super::CreditorFinancialAccountType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl TradeSettlementPaymentMeansTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                type_code: None,
                payer_party_debtor_financial_account: None,
                payee_party_creditor_financial_account: None,
                state__: Box::new(TradeSettlementPaymentMeansTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradeSettlementPaymentMeansTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            match state {
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(helper)?)?
                }
                S::PayerPartyDebtorFinancialAccount(Some(deserializer)) => {
                    self.store_payer_party_debtor_financial_account(deserializer.finish(helper)?)?
                }
                S::PayeePartyCreditorFinancialAccount(Some(deserializer)) => {
                    self.store_payee_party_creditor_financial_account(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_type_code(&mut self, value: super::PaymentMeansCodeType) -> Result<(), Error> {
            if self.type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TypeCode",
                )))?;
            }
            self.type_code = Some(value);
            Ok(())
        }
        fn store_payer_party_debtor_financial_account(
            &mut self,
            value: super::DebtorFinancialAccountType,
        ) -> Result<(), Error> {
            if self.payer_party_debtor_financial_account.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PayerPartyDebtorFinancialAccount",
                )))?;
            }
            self.payer_party_debtor_financial_account = Some(value);
            Ok(())
        }
        fn store_payee_party_creditor_financial_account(
            &mut self,
            value: super::CreditorFinancialAccountType,
        ) -> Result<(), Error> {
            if self.payee_party_creditor_financial_account.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PayeePartyCreditorFinancialAccount",
                )))?;
            }
            self.payee_party_creditor_financial_account = Some(value);
            Ok(())
        }
        fn handle_type_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PaymentMeansCodeType>,
            fallback: &mut Option<TradeSettlementPaymentMeansTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TypeCode(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    *self.state__ = S::PayerPartyDebtorFinancialAccount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TypeCode(Some(deserializer)));
                    *self.state__ = S::PayerPartyDebtorFinancialAccount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_payer_party_debtor_financial_account<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DebtorFinancialAccountType>,
            fallback: &mut Option<TradeSettlementPaymentMeansTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PayerPartyDebtorFinancialAccount(None));
                *self.state__ = S::PayeePartyCreditorFinancialAccount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payer_party_debtor_financial_account(data)?;
                    *self.state__ = S::PayeePartyCreditorFinancialAccount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PayerPartyDebtorFinancialAccount(Some(deserializer)));
                    *self.state__ = S::PayeePartyCreditorFinancialAccount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_payee_party_creditor_financial_account<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CreditorFinancialAccountType>,
            fallback: &mut Option<TradeSettlementPaymentMeansTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PayeePartyCreditorFinancialAccount(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payee_party_creditor_financial_account(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback
                        .get_or_insert(S::PayeePartyCreditorFinancialAccount(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradeSettlementPaymentMeansType>
        for TradeSettlementPaymentMeansTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementPaymentMeansType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementPaymentMeansType> {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayerPartyDebtorFinancialAccount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_payer_party_debtor_financial_account(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayeePartyCreditorFinancialAccount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_payee_party_creditor_financial_account(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::TypeCode(None);
                        event
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TypeCode",
                            false,
                        )?;
                        match self.handle_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::PayerPartyDebtorFinancialAccount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"PayerPartyDebtorFinancialAccount",
                            false,
                        )?;
                        match self.handle_payer_party_debtor_financial_account(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::PayeePartyCreditorFinancialAccount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"PayeePartyCreditorFinancialAccount",
                            false,
                        )?;
                        match self.handle_payee_party_creditor_financial_account(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradeSettlementPaymentMeansType, Error> {
            let state = replace(
                &mut *self.state__,
                TradeSettlementPaymentMeansTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradeSettlementPaymentMeansType {
                type_code: helper.finish_element("TypeCode", self.type_code)?,
                payer_party_debtor_financial_account: self.payer_party_debtor_financial_account,
                payee_party_creditor_financial_account: self.payee_party_creditor_financial_account,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeTaxTypeDeserializer {
        calculated_amount: Option<super::AmountType>,
        type_code: Option<super::TaxTypeCodeType>,
        exemption_reason: Option<super::TextType>,
        basis_amount: Option<super::AmountType>,
        category_code: Option<super::TaxCategoryCodeType>,
        exemption_reason_code: Option<super::CodeType>,
        due_date_type_code: Option<super::TimeReferenceCodeType>,
        rate_applicable_percent: Option<super::PercentType>,
        state__: Box<TradeTaxTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeTaxTypeDeserializerState {
        Init__,
        CalculatedAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TypeCode(Option<<super::TaxTypeCodeType as WithDeserializer>::Deserializer>),
        ExemptionReason(Option<<super::TextType as WithDeserializer>::Deserializer>),
        BasisAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        CategoryCode(Option<<super::TaxCategoryCodeType as WithDeserializer>::Deserializer>),
        ExemptionReasonCode(Option<<super::CodeType as WithDeserializer>::Deserializer>),
        DueDateTypeCode(Option<<super::TimeReferenceCodeType as WithDeserializer>::Deserializer>),
        RateApplicablePercent(Option<<super::PercentType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeTaxTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                calculated_amount: None,
                type_code: None,
                exemption_reason: None,
                basis_amount: None,
                category_code: None,
                exemption_reason_code: None,
                due_date_type_code: None,
                rate_applicable_percent: None,
                state__: Box::new(TradeTaxTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradeTaxTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradeTaxTypeDeserializerState as S;
            match state {
                S::CalculatedAmount(Some(deserializer)) => {
                    self.store_calculated_amount(deserializer.finish(helper)?)?
                }
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(helper)?)?
                }
                S::ExemptionReason(Some(deserializer)) => {
                    self.store_exemption_reason(deserializer.finish(helper)?)?
                }
                S::BasisAmount(Some(deserializer)) => {
                    self.store_basis_amount(deserializer.finish(helper)?)?
                }
                S::CategoryCode(Some(deserializer)) => {
                    self.store_category_code(deserializer.finish(helper)?)?
                }
                S::ExemptionReasonCode(Some(deserializer)) => {
                    self.store_exemption_reason_code(deserializer.finish(helper)?)?
                }
                S::DueDateTypeCode(Some(deserializer)) => {
                    self.store_due_date_type_code(deserializer.finish(helper)?)?
                }
                S::RateApplicablePercent(Some(deserializer)) => {
                    self.store_rate_applicable_percent(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_calculated_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.calculated_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CalculatedAmount",
                )))?;
            }
            self.calculated_amount = Some(value);
            Ok(())
        }
        fn store_type_code(&mut self, value: super::TaxTypeCodeType) -> Result<(), Error> {
            if self.type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TypeCode",
                )))?;
            }
            self.type_code = Some(value);
            Ok(())
        }
        fn store_exemption_reason(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.exemption_reason.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExemptionReason",
                )))?;
            }
            self.exemption_reason = Some(value);
            Ok(())
        }
        fn store_basis_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.basis_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BasisAmount",
                )))?;
            }
            self.basis_amount = Some(value);
            Ok(())
        }
        fn store_category_code(&mut self, value: super::TaxCategoryCodeType) -> Result<(), Error> {
            if self.category_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CategoryCode",
                )))?;
            }
            self.category_code = Some(value);
            Ok(())
        }
        fn store_exemption_reason_code(&mut self, value: super::CodeType) -> Result<(), Error> {
            if self.exemption_reason_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExemptionReasonCode",
                )))?;
            }
            self.exemption_reason_code = Some(value);
            Ok(())
        }
        fn store_due_date_type_code(
            &mut self,
            value: super::TimeReferenceCodeType,
        ) -> Result<(), Error> {
            if self.due_date_type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DueDateTypeCode",
                )))?;
            }
            self.due_date_type_code = Some(value);
            Ok(())
        }
        fn store_rate_applicable_percent(
            &mut self,
            value: super::PercentType,
        ) -> Result<(), Error> {
            if self.rate_applicable_percent.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RateApplicablePercent",
                )))?;
            }
            self.rate_applicable_percent = Some(value);
            Ok(())
        }
        fn handle_calculated_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CalculatedAmount(None));
                *self.state__ = S::TypeCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_calculated_amount(data)?;
                    *self.state__ = S::TypeCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CalculatedAmount(Some(deserializer)));
                    *self.state__ = S::TypeCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_type_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TaxTypeCodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TypeCode(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    *self.state__ = S::ExemptionReason(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TypeCode(Some(deserializer)));
                    *self.state__ = S::ExemptionReason(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_exemption_reason<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ExemptionReason(None));
                *self.state__ = S::BasisAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exemption_reason(data)?;
                    *self.state__ = S::BasisAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ExemptionReason(Some(deserializer)));
                    *self.state__ = S::BasisAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_basis_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BasisAmount(None));
                *self.state__ = S::CategoryCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_basis_amount(data)?;
                    *self.state__ = S::CategoryCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BasisAmount(Some(deserializer)));
                    *self.state__ = S::CategoryCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_category_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TaxCategoryCodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CategoryCode(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_category_code(data)?;
                    *self.state__ = S::ExemptionReasonCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CategoryCode(Some(deserializer)));
                    *self.state__ = S::ExemptionReasonCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_exemption_reason_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ExemptionReasonCode(None));
                *self.state__ = S::DueDateTypeCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exemption_reason_code(data)?;
                    *self.state__ = S::DueDateTypeCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ExemptionReasonCode(Some(deserializer)));
                    *self.state__ = S::DueDateTypeCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_due_date_type_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TimeReferenceCodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DueDateTypeCode(None));
                *self.state__ = S::RateApplicablePercent(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_date_type_code(data)?;
                    *self.state__ = S::RateApplicablePercent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DueDateTypeCode(Some(deserializer)));
                    *self.state__ = S::RateApplicablePercent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_rate_applicable_percent<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PercentType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeTaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::RateApplicablePercent(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_rate_applicable_percent(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::RateApplicablePercent(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradeTaxType> for TradeTaxTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeTaxType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeTaxType> {
            use TradeTaxTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CalculatedAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_calculated_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExemptionReason(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_exemption_reason(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BasisAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_basis_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CategoryCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_category_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExemptionReasonCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_exemption_reason_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DueDateTypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_due_date_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RateApplicablePercent(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_rate_applicable_percent(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::CalculatedAmount(None);
                        event
                    }
                    (S::CalculatedAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CalculatedAmount",
                            false,
                        )?;
                        match self.handle_calculated_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TypeCode",
                            false,
                        )?;
                        match self.handle_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExemptionReason(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ExemptionReason",
                            false,
                        )?;
                        match self.handle_exemption_reason(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BasisAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BasisAmount",
                            false,
                        )?;
                        match self.handle_basis_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CategoryCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CategoryCode",
                            false,
                        )?;
                        match self.handle_category_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExemptionReasonCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ExemptionReasonCode",
                            false,
                        )?;
                        match self.handle_exemption_reason_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DueDateTypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"DueDateTypeCode",
                            false,
                        )?;
                        match self.handle_due_date_type_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::RateApplicablePercent(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"RateApplicablePercent",
                            false,
                        )?;
                        match self.handle_rate_applicable_percent(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TradeTaxType, Error> {
            let state = replace(&mut *self.state__, TradeTaxTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::TradeTaxType {
                calculated_amount: self.calculated_amount,
                type_code: helper.finish_element("TypeCode", self.type_code)?,
                exemption_reason: self.exemption_reason,
                basis_amount: self.basis_amount,
                category_code: helper.finish_element("CategoryCode", self.category_code)?,
                exemption_reason_code: self.exemption_reason_code,
                due_date_type_code: self.due_date_type_code,
                rate_applicable_percent: self.rate_applicable_percent,
            })
        }
    }
    #[derive(Debug)]
    pub struct SpecifiedPeriodTypeDeserializer {
        start_date_time: Option<super::DateTimeType>,
        end_date_time: Option<super::DateTimeType>,
        state__: Box<SpecifiedPeriodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SpecifiedPeriodTypeDeserializerState {
        Init__,
        StartDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        EndDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SpecifiedPeriodTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                start_date_time: None,
                end_date_time: None,
                state__: Box::new(SpecifiedPeriodTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SpecifiedPeriodTypeDeserializerState,
        ) -> Result<(), Error> {
            use SpecifiedPeriodTypeDeserializerState as S;
            match state {
                S::StartDateTime(Some(deserializer)) => {
                    self.store_start_date_time(deserializer.finish(helper)?)?
                }
                S::EndDateTime(Some(deserializer)) => {
                    self.store_end_date_time(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_start_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.start_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"StartDateTime",
                )))?;
            }
            self.start_date_time = Some(value);
            Ok(())
        }
        fn store_end_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.end_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"EndDateTime",
                )))?;
            }
            self.end_date_time = Some(value);
            Ok(())
        }
        fn handle_start_date_time<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<SpecifiedPeriodTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SpecifiedPeriodTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::StartDateTime(None));
                *self.state__ = S::EndDateTime(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_start_date_time(data)?;
                    *self.state__ = S::EndDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::StartDateTime(Some(deserializer)));
                    *self.state__ = S::EndDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_end_date_time<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<SpecifiedPeriodTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SpecifiedPeriodTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::EndDateTime(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_end_date_time(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::EndDateTime(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SpecifiedPeriodType> for SpecifiedPeriodTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpecifiedPeriodType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpecifiedPeriodType> {
            use SpecifiedPeriodTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::StartDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_start_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::EndDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_end_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::StartDateTime(None);
                        event
                    }
                    (S::StartDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"StartDateTime",
                            false,
                        )?;
                        match self.handle_start_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::EndDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"EndDateTime",
                            false,
                        )?;
                        match self.handle_end_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SpecifiedPeriodType, Error> {
            let state = replace(
                &mut *self.state__,
                SpecifiedPeriodTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SpecifiedPeriodType {
                start_date_time: self.start_date_time,
                end_date_time: self.end_date_time,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeAllowanceChargeTypeDeserializer {
        charge_indicator: Option<super::IndicatorType>,
        calculation_percent: Option<super::PercentType>,
        basis_amount: Option<super::AmountType>,
        actual_amount: Option<super::AmountType>,
        reason_code: Option<super::AllowanceChargeReasonCodeType>,
        reason: Option<super::TextType>,
        category_trade_tax: Option<super::TradeTaxType>,
        state__: Box<TradeAllowanceChargeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAllowanceChargeTypeDeserializerState {
        Init__,
        ChargeIndicator(Option<<super::IndicatorType as WithDeserializer>::Deserializer>),
        CalculationPercent(Option<<super::PercentType as WithDeserializer>::Deserializer>),
        BasisAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        ActualAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        ReasonCode(
            Option<<super::AllowanceChargeReasonCodeType as WithDeserializer>::Deserializer>,
        ),
        Reason(Option<<super::TextType as WithDeserializer>::Deserializer>),
        CategoryTradeTax(Option<<super::TradeTaxType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAllowanceChargeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                charge_indicator: None,
                calculation_percent: None,
                basis_amount: None,
                actual_amount: None,
                reason_code: None,
                reason: None,
                category_trade_tax: None,
                state__: Box::new(TradeAllowanceChargeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradeAllowanceChargeTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            match state {
                S::ChargeIndicator(Some(deserializer)) => {
                    self.store_charge_indicator(deserializer.finish(helper)?)?
                }
                S::CalculationPercent(Some(deserializer)) => {
                    self.store_calculation_percent(deserializer.finish(helper)?)?
                }
                S::BasisAmount(Some(deserializer)) => {
                    self.store_basis_amount(deserializer.finish(helper)?)?
                }
                S::ActualAmount(Some(deserializer)) => {
                    self.store_actual_amount(deserializer.finish(helper)?)?
                }
                S::ReasonCode(Some(deserializer)) => {
                    self.store_reason_code(deserializer.finish(helper)?)?
                }
                S::Reason(Some(deserializer)) => self.store_reason(deserializer.finish(helper)?)?,
                S::CategoryTradeTax(Some(deserializer)) => {
                    self.store_category_trade_tax(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_charge_indicator(&mut self, value: super::IndicatorType) -> Result<(), Error> {
            if self.charge_indicator.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ChargeIndicator",
                )))?;
            }
            self.charge_indicator = Some(value);
            Ok(())
        }
        fn store_calculation_percent(&mut self, value: super::PercentType) -> Result<(), Error> {
            if self.calculation_percent.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CalculationPercent",
                )))?;
            }
            self.calculation_percent = Some(value);
            Ok(())
        }
        fn store_basis_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.basis_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BasisAmount",
                )))?;
            }
            self.basis_amount = Some(value);
            Ok(())
        }
        fn store_actual_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.actual_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ActualAmount",
                )))?;
            }
            self.actual_amount = Some(value);
            Ok(())
        }
        fn store_reason_code(
            &mut self,
            value: super::AllowanceChargeReasonCodeType,
        ) -> Result<(), Error> {
            if self.reason_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ReasonCode",
                )))?;
            }
            self.reason_code = Some(value);
            Ok(())
        }
        fn store_reason(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.reason.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Reason",
                )))?;
            }
            self.reason = Some(value);
            Ok(())
        }
        fn store_category_trade_tax(&mut self, value: super::TradeTaxType) -> Result<(), Error> {
            if self.category_trade_tax.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CategoryTradeTax",
                )))?;
            }
            self.category_trade_tax = Some(value);
            Ok(())
        }
        fn handle_charge_indicator<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IndicatorType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ChargeIndicator(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_charge_indicator(data)?;
                    *self.state__ = S::CalculationPercent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ChargeIndicator(Some(deserializer)));
                    *self.state__ = S::CalculationPercent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_calculation_percent<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PercentType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CalculationPercent(None));
                *self.state__ = S::BasisAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_calculation_percent(data)?;
                    *self.state__ = S::BasisAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CalculationPercent(Some(deserializer)));
                    *self.state__ = S::BasisAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_basis_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BasisAmount(None));
                *self.state__ = S::ActualAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_basis_amount(data)?;
                    *self.state__ = S::ActualAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BasisAmount(Some(deserializer)));
                    *self.state__ = S::ActualAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_actual_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ActualAmount(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_actual_amount(data)?;
                    *self.state__ = S::ReasonCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ActualAmount(Some(deserializer)));
                    *self.state__ = S::ReasonCode(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_reason_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AllowanceChargeReasonCodeType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ReasonCode(None));
                *self.state__ = S::Reason(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_reason_code(data)?;
                    *self.state__ = S::Reason(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ReasonCode(Some(deserializer)));
                    *self.state__ = S::Reason(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_reason<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Reason(None));
                *self.state__ = S::CategoryTradeTax(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_reason(data)?;
                    *self.state__ = S::CategoryTradeTax(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Reason(Some(deserializer)));
                    *self.state__ = S::CategoryTradeTax(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_category_trade_tax<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TradeTaxType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CategoryTradeTax(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_category_trade_tax(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CategoryTradeTax(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradeAllowanceChargeType>
        for TradeAllowanceChargeTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAllowanceChargeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAllowanceChargeType> {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::ChargeIndicator(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_charge_indicator(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CalculationPercent(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_calculation_percent(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BasisAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_basis_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ActualAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_actual_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ReasonCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_reason_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Reason(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_reason(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CategoryTradeTax(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_category_trade_tax(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::ChargeIndicator(None);
                        event
                    }
                    (S::ChargeIndicator(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ChargeIndicator",
                            false,
                        )?;
                        match self.handle_charge_indicator(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CalculationPercent(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CalculationPercent",
                            false,
                        )?;
                        match self.handle_calculation_percent(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BasisAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"BasisAmount",
                            false,
                        )?;
                        match self.handle_basis_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ActualAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ActualAmount",
                            false,
                        )?;
                        match self.handle_actual_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ReasonCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ReasonCode",
                            false,
                        )?;
                        match self.handle_reason_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Reason(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"Reason",
                            false,
                        )?;
                        match self.handle_reason(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CategoryTradeTax(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CategoryTradeTax",
                            false,
                        )?;
                        match self.handle_category_trade_tax(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradeAllowanceChargeType, Error> {
            let state = replace(
                &mut *self.state__,
                TradeAllowanceChargeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradeAllowanceChargeType {
                charge_indicator: helper
                    .finish_element("ChargeIndicator", self.charge_indicator)?,
                calculation_percent: self.calculation_percent,
                basis_amount: self.basis_amount,
                actual_amount: helper.finish_element("ActualAmount", self.actual_amount)?,
                reason_code: self.reason_code,
                reason: self.reason,
                category_trade_tax: helper
                    .finish_element("CategoryTradeTax", self.category_trade_tax)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradePaymentTermsTypeDeserializer {
        description: Option<super::TextType>,
        due_date_date_time: Option<super::DateTimeType>,
        direct_debit_mandate_id: Option<super::IdType>,
        state__: Box<TradePaymentTermsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradePaymentTermsTypeDeserializerState {
        Init__,
        Description(Option<<super::TextType as WithDeserializer>::Deserializer>),
        DueDateDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        DirectDebitMandateId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradePaymentTermsTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                description: None,
                due_date_date_time: None,
                direct_debit_mandate_id: None,
                state__: Box::new(TradePaymentTermsTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradePaymentTermsTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradePaymentTermsTypeDeserializerState as S;
            match state {
                S::Description(Some(deserializer)) => {
                    self.store_description(deserializer.finish(helper)?)?
                }
                S::DueDateDateTime(Some(deserializer)) => {
                    self.store_due_date_date_time(deserializer.finish(helper)?)?
                }
                S::DirectDebitMandateId(Some(deserializer)) => {
                    self.store_direct_debit_mandate_id(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_description(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.description.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Description",
                )))?;
            }
            self.description = Some(value);
            Ok(())
        }
        fn store_due_date_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.due_date_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DueDateDateTime",
                )))?;
            }
            self.due_date_date_time = Some(value);
            Ok(())
        }
        fn store_direct_debit_mandate_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.direct_debit_mandate_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DirectDebitMandateID",
                )))?;
            }
            self.direct_debit_mandate_id = Some(value);
            Ok(())
        }
        fn handle_description<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradePaymentTermsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePaymentTermsTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Description(None));
                *self.state__ = S::DueDateDateTime(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_description(data)?;
                    *self.state__ = S::DueDateDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Description(Some(deserializer)));
                    *self.state__ = S::DueDateDateTime(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_due_date_date_time<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<TradePaymentTermsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePaymentTermsTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DueDateDateTime(None));
                *self.state__ = S::DirectDebitMandateId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_date_date_time(data)?;
                    *self.state__ = S::DirectDebitMandateId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DueDateDateTime(Some(deserializer)));
                    *self.state__ = S::DirectDebitMandateId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_direct_debit_mandate_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradePaymentTermsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradePaymentTermsTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DirectDebitMandateId(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_direct_debit_mandate_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DirectDebitMandateId(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradePaymentTermsType> for TradePaymentTermsTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePaymentTermsType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePaymentTermsType> {
            use TradePaymentTermsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Description(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_description(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DueDateDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_due_date_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DirectDebitMandateId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_direct_debit_mandate_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Description(None);
                        event
                    }
                    (S::Description(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"Description",
                            false,
                        )?;
                        match self.handle_description(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DueDateDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"DueDateDateTime",
                            false,
                        )?;
                        match self.handle_due_date_date_time(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::DirectDebitMandateId(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"DirectDebitMandateID",
                            false,
                        )?;
                        match self.handle_direct_debit_mandate_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradePaymentTermsType, Error> {
            let state = replace(
                &mut *self.state__,
                TradePaymentTermsTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradePaymentTermsType {
                description: self.description,
                due_date_date_time: self.due_date_date_time,
                direct_debit_mandate_id: self.direct_debit_mandate_id,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementHeaderMonetarySummationTypeDeserializer {
        line_total_amount: Option<super::AmountType>,
        charge_total_amount: Option<super::AmountType>,
        allowance_total_amount: Option<super::AmountType>,
        tax_basis_total_amount: Option<super::AmountType>,
        tax_total_amount: Vec<super::AmountType>,
        grand_total_amount: Option<super::AmountType>,
        total_prepaid_amount: Option<super::AmountType>,
        due_payable_amount: Option<super::AmountType>,
        state__: Box<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeSettlementHeaderMonetarySummationTypeDeserializerState {
        Init__,
        LineTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        ChargeTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        AllowanceTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TaxBasisTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TaxTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        GrandTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TotalPrepaidAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        DuePayableAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeSettlementHeaderMonetarySummationTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                line_total_amount: None,
                charge_total_amount: None,
                allowance_total_amount: None,
                tax_basis_total_amount: None,
                tax_total_amount: Vec::new(),
                grand_total_amount: None,
                total_prepaid_amount: None,
                due_payable_amount: None,
                state__: Box::new(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradeSettlementHeaderMonetarySummationTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            match state {
                S::LineTotalAmount(Some(deserializer)) => {
                    self.store_line_total_amount(deserializer.finish(helper)?)?
                }
                S::ChargeTotalAmount(Some(deserializer)) => {
                    self.store_charge_total_amount(deserializer.finish(helper)?)?
                }
                S::AllowanceTotalAmount(Some(deserializer)) => {
                    self.store_allowance_total_amount(deserializer.finish(helper)?)?
                }
                S::TaxBasisTotalAmount(Some(deserializer)) => {
                    self.store_tax_basis_total_amount(deserializer.finish(helper)?)?
                }
                S::TaxTotalAmount(Some(deserializer)) => {
                    self.store_tax_total_amount(deserializer.finish(helper)?)?
                }
                S::GrandTotalAmount(Some(deserializer)) => {
                    self.store_grand_total_amount(deserializer.finish(helper)?)?
                }
                S::TotalPrepaidAmount(Some(deserializer)) => {
                    self.store_total_prepaid_amount(deserializer.finish(helper)?)?
                }
                S::DuePayableAmount(Some(deserializer)) => {
                    self.store_due_payable_amount(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_line_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.line_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineTotalAmount",
                )))?;
            }
            self.line_total_amount = Some(value);
            Ok(())
        }
        fn store_charge_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.charge_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ChargeTotalAmount",
                )))?;
            }
            self.charge_total_amount = Some(value);
            Ok(())
        }
        fn store_allowance_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.allowance_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"AllowanceTotalAmount",
                )))?;
            }
            self.allowance_total_amount = Some(value);
            Ok(())
        }
        fn store_tax_basis_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.tax_basis_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TaxBasisTotalAmount",
                )))?;
            }
            self.tax_basis_total_amount = Some(value);
            Ok(())
        }
        fn store_tax_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            self.tax_total_amount.push(value);
            Ok(())
        }
        fn store_grand_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.grand_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"GrandTotalAmount",
                )))?;
            }
            self.grand_total_amount = Some(value);
            Ok(())
        }
        fn store_total_prepaid_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.total_prepaid_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TotalPrepaidAmount",
                )))?;
            }
            self.total_prepaid_amount = Some(value);
            Ok(())
        }
        fn store_due_payable_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.due_payable_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DuePayableAmount",
                )))?;
            }
            self.due_payable_amount = Some(value);
            Ok(())
        }
        fn handle_line_total_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::LineTotalAmount(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_total_amount(data)?;
                    *self.state__ = S::ChargeTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::LineTotalAmount(Some(deserializer)));
                    *self.state__ = S::ChargeTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_charge_total_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ChargeTotalAmount(None));
                *self.state__ = S::AllowanceTotalAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_charge_total_amount(data)?;
                    *self.state__ = S::AllowanceTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ChargeTotalAmount(Some(deserializer)));
                    *self.state__ = S::AllowanceTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_allowance_total_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::AllowanceTotalAmount(None));
                *self.state__ = S::TaxBasisTotalAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_allowance_total_amount(data)?;
                    *self.state__ = S::TaxBasisTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::AllowanceTotalAmount(Some(deserializer)));
                    *self.state__ = S::TaxBasisTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tax_basis_total_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TaxBasisTotalAmount(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_basis_total_amount(data)?;
                    *self.state__ = S::TaxTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TaxBasisTotalAmount(Some(deserializer)));
                    *self.state__ = S::TaxTotalAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tax_total_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TaxTotalAmount(None));
                *self.state__ = S::GrandTotalAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_total_amount(data)?;
                    if self.tax_total_amount.len() < 2usize {
                        *self.state__ = S::TaxTotalAmount(None);
                    } else {
                        *self.state__ = S::GrandTotalAmount(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TaxTotalAmount(Some(deserializer)));
                    if self.tax_total_amount.len() < 1usize {
                        *self.state__ = S::TaxTotalAmount(None);
                    } else {
                        *self.state__ = S::GrandTotalAmount(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_grand_total_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::GrandTotalAmount(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_grand_total_amount(data)?;
                    *self.state__ = S::TotalPrepaidAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::GrandTotalAmount(Some(deserializer)));
                    *self.state__ = S::TotalPrepaidAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_total_prepaid_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TotalPrepaidAmount(None));
                *self.state__ = S::DuePayableAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_total_prepaid_amount(data)?;
                    *self.state__ = S::DuePayableAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TotalPrepaidAmount(Some(deserializer)));
                    *self.state__ = S::DuePayableAmount(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_due_payable_amount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DuePayableAmount(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_payable_amount(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DuePayableAmount(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradeSettlementHeaderMonetarySummationType>
        for TradeSettlementHeaderMonetarySummationTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementHeaderMonetarySummationType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementHeaderMonetarySummationType> {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::LineTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_line_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ChargeTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_charge_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::AllowanceTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_allowance_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxBasisTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tax_basis_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tax_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GrandTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_grand_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TotalPrepaidAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_total_prepaid_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DuePayableAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_due_payable_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::LineTotalAmount(None);
                        event
                    }
                    (S::LineTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"LineTotalAmount",
                            false,
                        )?;
                        match self.handle_line_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ChargeTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ChargeTotalAmount",
                            false,
                        )?;
                        match self.handle_charge_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::AllowanceTotalAmount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"AllowanceTotalAmount",
                            false,
                        )?;
                        match self.handle_allowance_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxBasisTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TaxBasisTotalAmount",
                            false,
                        )?;
                        match self.handle_tax_basis_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TaxTotalAmount",
                            false,
                        )?;
                        match self.handle_tax_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GrandTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"GrandTotalAmount",
                            false,
                        )?;
                        match self.handle_grand_total_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TotalPrepaidAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TotalPrepaidAmount",
                            false,
                        )?;
                        match self.handle_total_prepaid_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DuePayableAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"DuePayableAmount",
                            false,
                        )?;
                        match self.handle_due_payable_amount(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradeSettlementHeaderMonetarySummationType, Error> {
            let state = replace(
                &mut *self.state__,
                TradeSettlementHeaderMonetarySummationTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradeSettlementHeaderMonetarySummationType {
                line_total_amount: helper
                    .finish_element("LineTotalAmount", self.line_total_amount)?,
                charge_total_amount: self.charge_total_amount,
                allowance_total_amount: self.allowance_total_amount,
                tax_basis_total_amount: helper
                    .finish_element("TaxBasisTotalAmount", self.tax_basis_total_amount)?,
                tax_total_amount: self.tax_total_amount,
                grand_total_amount: helper
                    .finish_element("GrandTotalAmount", self.grand_total_amount)?,
                total_prepaid_amount: self.total_prepaid_amount,
                due_payable_amount: helper
                    .finish_element("DuePayableAmount", self.due_payable_amount)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeAccountingAccountTypeDeserializer {
        id: Option<super::IdType>,
        state__: Box<TradeAccountingAccountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAccountingAccountTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAccountingAccountTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: None,
                state__: Box::new(TradeAccountingAccountTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradeAccountingAccountTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradeAccountingAccountTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradeAccountingAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAccountingAccountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradeAccountingAccountType>
        for TradeAccountingAccountTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAccountingAccountType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAccountingAccountType> {
            use TradeAccountingAccountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradeAccountingAccountType, Error> {
            let state = replace(
                &mut *self.state__,
                TradeAccountingAccountTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradeAccountingAccountType {
                id: helper.finish_element("ID", self.id)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct LegalOrganizationTypeDeserializer {
        id: Option<super::IdType>,
        trading_business_name: Option<super::TextType>,
        state__: Box<LegalOrganizationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LegalOrganizationTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        TradingBusinessName(Option<<super::TextType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl LegalOrganizationTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: None,
                trading_business_name: None,
                state__: Box::new(LegalOrganizationTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: LegalOrganizationTypeDeserializerState,
        ) -> Result<(), Error> {
            use LegalOrganizationTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                S::TradingBusinessName(Some(deserializer)) => {
                    self.store_trading_business_name(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn store_trading_business_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.trading_business_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TradingBusinessName",
                )))?;
            }
            self.trading_business_name = Some(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<LegalOrganizationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use LegalOrganizationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
                *self.state__ = S::TradingBusinessName(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state__ = S::TradingBusinessName(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::TradingBusinessName(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_trading_business_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<LegalOrganizationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use LegalOrganizationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TradingBusinessName(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_trading_business_name(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TradingBusinessName(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::LegalOrganizationType> for LegalOrganizationTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LegalOrganizationType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LegalOrganizationType> {
            use LegalOrganizationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TradingBusinessName(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_trading_business_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TradingBusinessName(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"TradingBusinessName",
                            false,
                        )?;
                        match self.handle_trading_business_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::LegalOrganizationType, Error> {
            let state = replace(
                &mut *self.state__,
                LegalOrganizationTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::LegalOrganizationType {
                id: self.id,
                trading_business_name: self.trading_business_name,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeAddressTypeDeserializer {
        postcode_code: Option<super::CodeType>,
        line_one: Option<super::TextType>,
        line_two: Option<super::TextType>,
        line_three: Option<super::TextType>,
        city_name: Option<super::TextType>,
        country_id: Option<super::CountryIdType>,
        country_sub_division_name: Option<super::TextType>,
        state__: Box<TradeAddressTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAddressTypeDeserializerState {
        Init__,
        PostcodeCode(Option<<super::CodeType as WithDeserializer>::Deserializer>),
        LineOne(Option<<super::TextType as WithDeserializer>::Deserializer>),
        LineTwo(Option<<super::TextType as WithDeserializer>::Deserializer>),
        LineThree(Option<<super::TextType as WithDeserializer>::Deserializer>),
        CityName(Option<<super::TextType as WithDeserializer>::Deserializer>),
        CountryId(Option<<super::CountryIdType as WithDeserializer>::Deserializer>),
        CountrySubDivisionName(Option<<super::TextType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAddressTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                postcode_code: None,
                line_one: None,
                line_two: None,
                line_three: None,
                city_name: None,
                country_id: None,
                country_sub_division_name: None,
                state__: Box::new(TradeAddressTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TradeAddressTypeDeserializerState,
        ) -> Result<(), Error> {
            use TradeAddressTypeDeserializerState as S;
            match state {
                S::PostcodeCode(Some(deserializer)) => {
                    self.store_postcode_code(deserializer.finish(helper)?)?
                }
                S::LineOne(Some(deserializer)) => {
                    self.store_line_one(deserializer.finish(helper)?)?
                }
                S::LineTwo(Some(deserializer)) => {
                    self.store_line_two(deserializer.finish(helper)?)?
                }
                S::LineThree(Some(deserializer)) => {
                    self.store_line_three(deserializer.finish(helper)?)?
                }
                S::CityName(Some(deserializer)) => {
                    self.store_city_name(deserializer.finish(helper)?)?
                }
                S::CountryId(Some(deserializer)) => {
                    self.store_country_id(deserializer.finish(helper)?)?
                }
                S::CountrySubDivisionName(Some(deserializer)) => {
                    self.store_country_sub_division_name(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_postcode_code(&mut self, value: super::CodeType) -> Result<(), Error> {
            if self.postcode_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PostcodeCode",
                )))?;
            }
            self.postcode_code = Some(value);
            Ok(())
        }
        fn store_line_one(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.line_one.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineOne",
                )))?;
            }
            self.line_one = Some(value);
            Ok(())
        }
        fn store_line_two(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.line_two.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineTwo",
                )))?;
            }
            self.line_two = Some(value);
            Ok(())
        }
        fn store_line_three(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.line_three.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineThree",
                )))?;
            }
            self.line_three = Some(value);
            Ok(())
        }
        fn store_city_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.city_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CityName",
                )))?;
            }
            self.city_name = Some(value);
            Ok(())
        }
        fn store_country_id(&mut self, value: super::CountryIdType) -> Result<(), Error> {
            if self.country_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CountryID",
                )))?;
            }
            self.country_id = Some(value);
            Ok(())
        }
        fn store_country_sub_division_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.country_sub_division_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CountrySubDivisionName",
                )))?;
            }
            self.country_sub_division_name = Some(value);
            Ok(())
        }
        fn handle_postcode_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CodeType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PostcodeCode(None));
                *self.state__ = S::LineOne(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_postcode_code(data)?;
                    *self.state__ = S::LineOne(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PostcodeCode(Some(deserializer)));
                    *self.state__ = S::LineOne(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_line_one<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::LineOne(None));
                *self.state__ = S::LineTwo(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_one(data)?;
                    *self.state__ = S::LineTwo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::LineOne(Some(deserializer)));
                    *self.state__ = S::LineTwo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_line_two<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::LineTwo(None));
                *self.state__ = S::LineThree(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_two(data)?;
                    *self.state__ = S::LineThree(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::LineTwo(Some(deserializer)));
                    *self.state__ = S::LineThree(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_line_three<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::LineThree(None));
                *self.state__ = S::CityName(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_three(data)?;
                    *self.state__ = S::CityName(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::LineThree(Some(deserializer)));
                    *self.state__ = S::CityName(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_city_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CityName(None));
                *self.state__ = S::CountryId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_city_name(data)?;
                    *self.state__ = S::CountryId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CityName(Some(deserializer)));
                    *self.state__ = S::CountryId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_country_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CountryIdType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CountryId(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country_id(data)?;
                    *self.state__ = S::CountrySubDivisionName(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CountryId(Some(deserializer)));
                    *self.state__ = S::CountrySubDivisionName(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_country_sub_division_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TradeAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CountrySubDivisionName(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country_sub_division_name(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CountrySubDivisionName(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TradeAddressType> for TradeAddressTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAddressType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAddressType> {
            use TradeAddressTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::PostcodeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_postcode_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineOne(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_line_one(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineTwo(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_line_two(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineThree(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_line_three(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CityName(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_city_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CountryId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_country_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CountrySubDivisionName(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_country_sub_division_name(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::PostcodeCode(None);
                        event
                    }
                    (S::PostcodeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"PostcodeCode",
                            false,
                        )?;
                        match self.handle_postcode_code(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineOne(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"LineOne",
                            false,
                        )?;
                        match self.handle_line_one(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineTwo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"LineTwo",
                            false,
                        )?;
                        match self.handle_line_two(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineThree(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"LineThree",
                            false,
                        )?;
                        match self.handle_line_three(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CityName(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CityName",
                            false,
                        )?;
                        match self.handle_city_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CountryId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CountryID",
                            false,
                        )?;
                        match self.handle_country_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::CountrySubDivisionName(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"CountrySubDivisionName",
                            false,
                        )?;
                        match self.handle_country_sub_division_name(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TradeAddressType, Error> {
            let state = replace(
                &mut *self.state__,
                TradeAddressTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TradeAddressType {
                postcode_code: self.postcode_code,
                line_one: self.line_one,
                line_two: self.line_two,
                line_three: self.line_three,
                city_name: self.city_name,
                country_id: helper.finish_element("CountryID", self.country_id)?,
                country_sub_division_name: self.country_sub_division_name,
            })
        }
    }
    #[derive(Debug)]
    pub struct UniversalCommunicationTypeDeserializer {
        uriid: Option<super::IdType>,
        state__: Box<UniversalCommunicationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UniversalCommunicationTypeDeserializerState {
        Init__,
        Uriid(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UniversalCommunicationTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                uriid: None,
                state__: Box::new(UniversalCommunicationTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: UniversalCommunicationTypeDeserializerState,
        ) -> Result<(), Error> {
            use UniversalCommunicationTypeDeserializerState as S;
            match state {
                S::Uriid(Some(deserializer)) => self.store_uriid(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_uriid(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.uriid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"URIID",
                )))?;
            }
            self.uriid = Some(value);
            Ok(())
        }
        fn handle_uriid<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<UniversalCommunicationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UniversalCommunicationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Uriid(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_uriid(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Uriid(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::UniversalCommunicationType>
        for UniversalCommunicationTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UniversalCommunicationType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UniversalCommunicationType> {
            use UniversalCommunicationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Uriid(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_uriid(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Uriid(None);
                        event
                    }
                    (S::Uriid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"URIID",
                            false,
                        )?;
                        match self.handle_uriid(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::UniversalCommunicationType, Error> {
            let state = replace(
                &mut *self.state__,
                UniversalCommunicationTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::UniversalCommunicationType {
                uriid: helper.finish_element("URIID", self.uriid)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxRegistrationTypeDeserializer {
        id: Option<super::IdType>,
        state__: Box<TaxRegistrationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxRegistrationTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TaxRegistrationTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: None,
                state__: Box::new(TaxRegistrationTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TaxRegistrationTypeDeserializerState,
        ) -> Result<(), Error> {
            use TaxRegistrationTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TaxRegistrationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TaxRegistrationTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TaxRegistrationType> for TaxRegistrationTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxRegistrationType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxRegistrationType> {
            use TaxRegistrationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TaxRegistrationType, Error> {
            let state = replace(
                &mut *self.state__,
                TaxRegistrationTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TaxRegistrationType {
                id: helper.finish_element("ID", self.id)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeDeserializer {
        date_time_string: Option<super::FormattedDateTimeTypeDateTimeStringType>,
        state__: Box<FormattedDateTimeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FormattedDateTimeTypeDeserializerState {
        Init__,
        DateTimeString(
            Option<
                <super::FormattedDateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer,
            >,
        ),
        Done__,
        Unknown__,
    }
    impl FormattedDateTimeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                date_time_string: None,
                state__: Box::new(FormattedDateTimeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FormattedDateTimeTypeDeserializerState,
        ) -> Result<(), Error> {
            use FormattedDateTimeTypeDeserializerState as S;
            match state {
                S::DateTimeString(Some(deserializer)) => {
                    self.store_date_time_string(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_date_time_string(
            &mut self,
            value: super::FormattedDateTimeTypeDateTimeStringType,
        ) -> Result<(), Error> {
            if self.date_time_string.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DateTimeString",
                )))?;
            }
            self.date_time_string = Some(value);
            Ok(())
        }
        fn handle_date_time_string<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FormattedDateTimeTypeDateTimeStringType>,
            fallback: &mut Option<FormattedDateTimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FormattedDateTimeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DateTimeString(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time_string(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DateTimeString(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FormattedDateTimeType> for FormattedDateTimeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeType> {
            use FormattedDateTimeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::DateTimeString(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_date_time_string(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::DateTimeString(None);
                        event
                    }
                    (S::DateTimeString(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_QDT),
                            b"DateTimeString",
                            false,
                        )?;
                        match self.handle_date_time_string(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::FormattedDateTimeType, Error> {
            let state = replace(
                &mut *self.state__,
                FormattedDateTimeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::FormattedDateTimeType {
                date_time_string: helper.finish_element("DateTimeString", self.date_time_string)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PaymentMeansCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<PaymentMeansCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PaymentMeansCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PaymentMeansCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(PaymentMeansCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PaymentMeansCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let PaymentMeansCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::PaymentMeansCodeType> {
            use PaymentMeansCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PaymentMeansCodeType> for PaymentMeansCodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PaymentMeansCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PaymentMeansCodeType> {
            use PaymentMeansCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::PaymentMeansCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                PaymentMeansCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PaymentMeansCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DebtorFinancialAccountTypeDeserializer {
        ibanid: Option<super::IdType>,
        state__: Box<DebtorFinancialAccountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DebtorFinancialAccountTypeDeserializerState {
        Init__,
        Ibanid(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DebtorFinancialAccountTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                ibanid: None,
                state__: Box::new(DebtorFinancialAccountTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DebtorFinancialAccountTypeDeserializerState,
        ) -> Result<(), Error> {
            use DebtorFinancialAccountTypeDeserializerState as S;
            match state {
                S::Ibanid(Some(deserializer)) => self.store_ibanid(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_ibanid(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.ibanid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IBANID",
                )))?;
            }
            self.ibanid = Some(value);
            Ok(())
        }
        fn handle_ibanid<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<DebtorFinancialAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DebtorFinancialAccountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Ibanid(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ibanid(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Ibanid(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DebtorFinancialAccountType>
        for DebtorFinancialAccountTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DebtorFinancialAccountType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DebtorFinancialAccountType> {
            use DebtorFinancialAccountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Ibanid(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_ibanid(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Ibanid(None);
                        event
                    }
                    (S::Ibanid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"IBANID",
                            false,
                        )?;
                        match self.handle_ibanid(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DebtorFinancialAccountType, Error> {
            let state = replace(
                &mut *self.state__,
                DebtorFinancialAccountTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DebtorFinancialAccountType {
                ibanid: helper.finish_element("IBANID", self.ibanid)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CreditorFinancialAccountTypeDeserializer {
        ibanid: Option<super::IdType>,
        proprietary_id: Option<super::IdType>,
        state__: Box<CreditorFinancialAccountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CreditorFinancialAccountTypeDeserializerState {
        Init__,
        Ibanid(Option<<super::IdType as WithDeserializer>::Deserializer>),
        ProprietaryId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CreditorFinancialAccountTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                ibanid: None,
                proprietary_id: None,
                state__: Box::new(CreditorFinancialAccountTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CreditorFinancialAccountTypeDeserializerState,
        ) -> Result<(), Error> {
            use CreditorFinancialAccountTypeDeserializerState as S;
            match state {
                S::Ibanid(Some(deserializer)) => self.store_ibanid(deserializer.finish(helper)?)?,
                S::ProprietaryId(Some(deserializer)) => {
                    self.store_proprietary_id(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_ibanid(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.ibanid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IBANID",
                )))?;
            }
            self.ibanid = Some(value);
            Ok(())
        }
        fn store_proprietary_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.proprietary_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ProprietaryID",
                )))?;
            }
            self.proprietary_id = Some(value);
            Ok(())
        }
        fn handle_ibanid<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<CreditorFinancialAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CreditorFinancialAccountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Ibanid(None));
                *self.state__ = S::ProprietaryId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ibanid(data)?;
                    *self.state__ = S::ProprietaryId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Ibanid(Some(deserializer)));
                    *self.state__ = S::ProprietaryId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_proprietary_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<CreditorFinancialAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CreditorFinancialAccountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::ProprietaryId(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_proprietary_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ProprietaryId(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CreditorFinancialAccountType>
        for CreditorFinancialAccountTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CreditorFinancialAccountType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CreditorFinancialAccountType> {
            use CreditorFinancialAccountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Ibanid(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_ibanid(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProprietaryId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_proprietary_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Ibanid(None);
                        event
                    }
                    (S::Ibanid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"IBANID",
                            false,
                        )?;
                        match self.handle_ibanid(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProprietaryId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_RAM),
                            b"ProprietaryID",
                            false,
                        )?;
                        match self.handle_proprietary_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::CreditorFinancialAccountType, Error> {
            let state = replace(
                &mut *self.state__,
                CreditorFinancialAccountTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CreditorFinancialAccountType {
                ibanid: self.ibanid,
                proprietary_id: self.proprietary_id,
            })
        }
    }
    #[derive(Debug)]
    pub struct AmountTypeDeserializer {
        currency_id: Option<String>,
        content: Option<f64>,
        state__: Box<AmountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AmountTypeDeserializerState {
        Init__,
        Content__(<f64 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AmountTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut currency_id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"currencyID")
                ) {
                    helper.read_attrib(&mut currency_id, b"currencyID", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                currency_id: currency_id,
                content: None,
                state__: Box::new(AmountTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AmountTypeDeserializerState,
        ) -> Result<(), Error> {
            if let AmountTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: f64) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, f64>,
        ) -> DeserializerResult<'de, super::AmountType> {
            use AmountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AmountType> for AmountTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AmountType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AmountType> {
            use AmountTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AmountType, Error> {
            let state = replace(&mut *self.state__, AmountTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AmountType {
                currency_id: self.currency_id,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxTypeCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<TaxTypeCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxTypeCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TaxTypeCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(TaxTypeCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TaxTypeCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TaxTypeCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TaxTypeCodeType> {
            use TaxTypeCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TaxTypeCodeType> for TaxTypeCodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxTypeCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxTypeCodeType> {
            use TaxTypeCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TaxTypeCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                TaxTypeCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TaxTypeCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxCategoryCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<TaxCategoryCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxCategoryCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TaxCategoryCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(TaxCategoryCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TaxCategoryCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TaxCategoryCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TaxCategoryCodeType> {
            use TaxCategoryCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TaxCategoryCodeType> for TaxCategoryCodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxCategoryCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxCategoryCodeType> {
            use TaxCategoryCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TaxCategoryCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                TaxCategoryCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TaxCategoryCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TimeReferenceCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<TimeReferenceCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TimeReferenceCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TimeReferenceCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(TimeReferenceCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TimeReferenceCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TimeReferenceCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TimeReferenceCodeType> {
            use TimeReferenceCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TimeReferenceCodeType> for TimeReferenceCodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TimeReferenceCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TimeReferenceCodeType> {
            use TimeReferenceCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TimeReferenceCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                TimeReferenceCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TimeReferenceCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PercentTypeDeserializer {
        content: Option<f64>,
        state__: Box<PercentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PercentTypeDeserializerState {
        Init__,
        Content__(<f64 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PercentTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(PercentTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PercentTypeDeserializerState,
        ) -> Result<(), Error> {
            if let PercentTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: f64) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, f64>,
        ) -> DeserializerResult<'de, super::PercentType> {
            use PercentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PercentType> for PercentTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PercentType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PercentType> {
            use PercentTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::PercentType, Error> {
            let state = replace(&mut *self.state__, PercentTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::PercentType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeDeserializer {
        content: Option<super::IndicatorTypeContent>,
        state__: Box<IndicatorTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IndicatorTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::IndicatorTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IndicatorTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(IndicatorTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: IndicatorTypeDeserializerState,
        ) -> Result<(), Error> {
            if let IndicatorTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::IndicatorTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::IndicatorTypeContent>,
            fallback: &mut Option<IndicatorTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use IndicatorTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IndicatorType> for IndicatorTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorType> {
            use IndicatorTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::IndicatorTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::IndicatorType, Error> {
            let state = replace(
                &mut *self.state__,
                IndicatorTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::IndicatorType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeContentDeserializer {
        state__: Box<IndicatorTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum IndicatorTypeContentDeserializerState {
        Init__,
        Indicator(
            Option<bool>,
            Option<<bool as WithDeserializer>::Deserializer>,
            Option<<bool as WithDeserializer>::Deserializer>,
        ),
        Done__(super::IndicatorTypeContent),
        Unknown__,
    }
    impl IndicatorTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UDT),
                    Some(b"Indicator")
                ) {
                    let output = <bool as WithDeserializer>::init(helper, event)?;
                    return self.handle_indicator(helper, Default::default(), None, output);
                }
            }
            *self.state__ = IndicatorTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: IndicatorTypeContentDeserializerState,
        ) -> Result<super::IndicatorTypeContent, Error> {
            use IndicatorTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Indicator(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_indicator(&mut values, value)?;
                    }
                    Ok(super::IndicatorTypeContent::Indicator(
                        helper.finish_element("Indicator", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_indicator(values: &mut Option<bool>, value: bool) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Indicator",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_indicator<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<bool>,
            fallback: Option<<bool as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, bool>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use IndicatorTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_indicator(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_indicator(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Indicator(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Indicator(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IndicatorTypeContent> for IndicatorTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorTypeContent> {
            let deserializer = Self {
                state__: Box::new(IndicatorTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, IndicatorTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorTypeContent> {
            use IndicatorTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Indicator(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_indicator(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Indicator(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UDT),
                            b"Indicator",
                            false,
                        )?;
                        match self.handle_indicator(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::IndicatorTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AllowanceChargeReasonCodeTypeDeserializer {
        content: Option<String>,
        state__: Box<AllowanceChargeReasonCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AllowanceChargeReasonCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AllowanceChargeReasonCodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(AllowanceChargeReasonCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AllowanceChargeReasonCodeTypeDeserializerState,
        ) -> Result<(), Error> {
            if let AllowanceChargeReasonCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::AllowanceChargeReasonCodeType> {
            use AllowanceChargeReasonCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AllowanceChargeReasonCodeType>
        for AllowanceChargeReasonCodeTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AllowanceChargeReasonCodeType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AllowanceChargeReasonCodeType> {
            use AllowanceChargeReasonCodeTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AllowanceChargeReasonCodeType, Error> {
            let state = replace(
                &mut *self.state__,
                AllowanceChargeReasonCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AllowanceChargeReasonCodeType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CountryIdTypeDeserializer {
        content: Option<String>,
        state__: Box<CountryIdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CountryIdTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CountryIdTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(CountryIdTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CountryIdTypeDeserializerState,
        ) -> Result<(), Error> {
            if let CountryIdTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CountryIdType> {
            use CountryIdTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CountryIdType> for CountryIdTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CountryIdType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CountryIdType> {
            use CountryIdTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CountryIdType, Error> {
            let state = replace(
                &mut *self.state__,
                CountryIdTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CountryIdType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeDateTimeStringTypeDeserializer {
        format: String,
        content: Option<String>,
        state__: Box<FormattedDateTimeTypeDateTimeStringTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FormattedDateTimeTypeDateTimeStringTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl FormattedDateTimeTypeDateTimeStringTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut format: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_QDT),
                    Some(b"format")
                ) {
                    helper.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                format: format.ok_or_else(|| ErrorKind::MissingAttribute("format".into()))?,
                content: None,
                state__: Box::new(FormattedDateTimeTypeDateTimeStringTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FormattedDateTimeTypeDateTimeStringTypeDeserializerState,
        ) -> Result<(), Error> {
            if let FormattedDateTimeTypeDateTimeStringTypeDeserializerState::Content__(
                deserializer,
            ) = state
            {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeTypeDateTimeStringType> {
            use FormattedDateTimeTypeDateTimeStringTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FormattedDateTimeTypeDateTimeStringType>
        for FormattedDateTimeTypeDateTimeStringTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeTypeDateTimeStringType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeTypeDateTimeStringType> {
            use FormattedDateTimeTypeDateTimeStringTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::FormattedDateTimeTypeDateTimeStringType, Error> {
            let state = replace(
                &mut *self.state__,
                FormattedDateTimeTypeDateTimeStringTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::FormattedDateTimeTypeDateTimeStringType {
                format: self.format,
                content: helper.finish_content(self.content)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct CrossIndustryInvoiceTypeSerializer<'ser> {
        pub(super) value: &'ser super::CrossIndustryInvoiceType,
        pub(super) state: Box<CrossIndustryInvoiceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CrossIndustryInvoiceTypeSerializerState<'ser> {
        Init__,
        ExchangedDocumentContext(
            <super::ExchangedDocumentContextType as WithSerializer>::Serializer<'ser>,
        ),
        ExchangedDocument(<super::ExchangedDocumentType as WithSerializer>::Serializer<'ser>),
        SupplyChainTradeTransaction(
            <super::SupplyChainTradeTransactionType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CrossIndustryInvoiceTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CrossIndustryInvoiceTypeSerializerState::Init__ => {
                        *self.state =
                            CrossIndustryInvoiceTypeSerializerState::ExchangedDocumentContext(
                                WithSerializer::serializer(
                                    &self.value.exchanged_document_context,
                                    Some("rsm:ExchangedDocumentContext"),
                                    false,
                                )?,
                            );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RSM),
                                &super::NS_RSM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocumentContext(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocument(
                                        WithSerializer::serializer(
                                            &self.value.exchanged_document,
                                            Some("rsm:ExchangedDocument"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocument(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                CrossIndustryInvoiceTypeSerializerState::SupplyChainTradeTransaction(
                                    WithSerializer::serializer(
                                        &self.value.supply_chain_trade_transaction,
                                        Some("rsm:SupplyChainTradeTransaction"),
                                        false,
                                    )?,
                                )
                        }
                    },
                    CrossIndustryInvoiceTypeSerializerState::SupplyChainTradeTransaction(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CrossIndustryInvoiceTypeSerializerState::End__,
                        }
                    }
                    CrossIndustryInvoiceTypeSerializerState::End__ => {
                        *self.state = CrossIndustryInvoiceTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CrossIndustryInvoiceTypeSerializerState::Done__ => return Ok(None),
                    CrossIndustryInvoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CrossIndustryInvoiceTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CrossIndustryInvoiceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentContextTypeSerializer<'ser> {
        pub(super) value: &'ser super::ExchangedDocumentContextType,
        pub(super) state: Box<ExchangedDocumentContextTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ExchangedDocumentContextTypeSerializerState<'ser> {
        Init__,
        BusinessProcessSpecifiedDocumentContextParameter(
            IterSerializer<
                'ser,
                Option<&'ser super::DocumentContextParameterType>,
                super::DocumentContextParameterType,
            >,
        ),
        GuidelineSpecifiedDocumentContextParameter(
            <super::DocumentContextParameterType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExchangedDocumentContextTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { ExchangedDocumentContextTypeSerializerState :: Init__ => { * self . state = ExchangedDocumentContextTypeSerializerState :: BusinessProcessSpecifiedDocumentContextParameter (IterSerializer :: new (self . value . business_process_specified_document_context_parameter . as_ref () , Some ("ram:BusinessProcessSpecifiedDocumentContextParameter") , false)) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } ExchangedDocumentContextTypeSerializerState :: BusinessProcessSpecifiedDocumentContextParameter (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = ExchangedDocumentContextTypeSerializerState :: GuidelineSpecifiedDocumentContextParameter (WithSerializer :: serializer (& self . value . guideline_specified_document_context_parameter , Some ("ram:GuidelineSpecifiedDocumentContextParameter") , false) ?) , } ExchangedDocumentContextTypeSerializerState :: GuidelineSpecifiedDocumentContextParameter (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = ExchangedDocumentContextTypeSerializerState :: End__ , } ExchangedDocumentContextTypeSerializerState :: End__ => { * self . state = ExchangedDocumentContextTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } ExchangedDocumentContextTypeSerializerState :: Done__ => return Ok (None) , ExchangedDocumentContextTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ExchangedDocumentContextTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExchangedDocumentContextTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentTypeSerializer<'ser> {
        pub(super) value: &'ser super::ExchangedDocumentType,
        pub(super) state: Box<ExchangedDocumentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ExchangedDocumentTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        TypeCode(<super::DocumentCodeType as WithSerializer>::Serializer<'ser>),
        IssueDateTime(<super::DateTimeType as WithSerializer>::Serializer<'ser>),
        IncludedNote(IterSerializer<'ser, &'ser [super::NoteType], super::NoteType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExchangedDocumentTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExchangedDocumentTypeSerializerState::Init__ => {
                        *self.state = ExchangedDocumentTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ExchangedDocumentTypeSerializerState::Id(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ExchangedDocumentTypeSerializerState::TypeCode(
                                    WithSerializer::serializer(
                                        &self.value.type_code,
                                        Some("ram:TypeCode"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    ExchangedDocumentTypeSerializerState::TypeCode(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ExchangedDocumentTypeSerializerState::IssueDateTime(
                                    WithSerializer::serializer(
                                        &self.value.issue_date_time,
                                        Some("ram:IssueDateTime"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    ExchangedDocumentTypeSerializerState::IssueDateTime(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ExchangedDocumentTypeSerializerState::IncludedNote(
                                    IterSerializer::new(
                                        &self.value.included_note[..],
                                        Some("ram:IncludedNote"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    ExchangedDocumentTypeSerializerState::IncludedNote(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExchangedDocumentTypeSerializerState::End__,
                        }
                    }
                    ExchangedDocumentTypeSerializerState::End__ => {
                        *self.state = ExchangedDocumentTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ExchangedDocumentTypeSerializerState::Done__ => return Ok(None),
                    ExchangedDocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ExchangedDocumentTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExchangedDocumentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainTradeTransactionTypeSerializer<'ser> {
        pub(super) value: &'ser super::SupplyChainTradeTransactionType,
        pub(super) state: Box<SupplyChainTradeTransactionTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SupplyChainTradeTransactionTypeSerializerState<'ser> {
        Init__,
        ApplicableHeaderTradeAgreement(
            <super::HeaderTradeAgreementType as WithSerializer>::Serializer<'ser>,
        ),
        ApplicableHeaderTradeDelivery(
            <super::HeaderTradeDeliveryType as WithSerializer>::Serializer<'ser>,
        ),
        ApplicableHeaderTradeSettlement(
            <super::HeaderTradeSettlementType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SupplyChainTradeTransactionTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { SupplyChainTradeTransactionTypeSerializerState :: Init__ => { * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeAgreement (WithSerializer :: serializer (& self . value . applicable_header_trade_agreement , Some ("ram:ApplicableHeaderTradeAgreement") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_QDT) , & super :: NS_QDT) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeAgreement (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeDelivery (WithSerializer :: serializer (& self . value . applicable_header_trade_delivery , Some ("ram:ApplicableHeaderTradeDelivery") , false) ?) , } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeDelivery (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeSettlement (WithSerializer :: serializer (& self . value . applicable_header_trade_settlement , Some ("ram:ApplicableHeaderTradeSettlement") , false) ?) , } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeSettlement (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: End__ , } SupplyChainTradeTransactionTypeSerializerState :: End__ => { * self . state = SupplyChainTradeTransactionTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } SupplyChainTradeTransactionTypeSerializerState :: Done__ => return Ok (None) , SupplyChainTradeTransactionTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SupplyChainTradeTransactionTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SupplyChainTradeTransactionTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocumentContextParameterTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocumentContextParameterType,
        pub(super) state: Box<DocumentContextParameterTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocumentContextParameterTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentContextParameterTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentContextParameterTypeSerializerState::Init__ => {
                        *self.state = DocumentContextParameterTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentContextParameterTypeSerializerState::Id(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentContextParameterTypeSerializerState::End__,
                    },
                    DocumentContextParameterTypeSerializerState::End__ => {
                        *self.state = DocumentContextParameterTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentContextParameterTypeSerializerState::Done__ => return Ok(None),
                    DocumentContextParameterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DocumentContextParameterTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentContextParameterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IdTypeSerializer<'ser> {
        pub(super) value: &'ser super::IdType,
        pub(super) state: Box<IdTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum IdTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IdTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IdTypeSerializerState::Init__ => {
                        *self.state = IdTypeSerializerState::Content__(WithSerializer::serializer(
                            &self.value.content,
                            None,
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "schemeID", &self.value.scheme_id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    IdTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = IdTypeSerializerState::End__,
                    },
                    IdTypeSerializerState::End__ => {
                        *self.state = IdTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    IdTypeSerializerState::Done__ => return Ok(None),
                    IdTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for IdTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IdTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocumentCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocumentCodeType,
        pub(super) state: Box<DocumentCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocumentCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentCodeTypeSerializerState::Init__ => {
                        *self.state = DocumentCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentCodeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DocumentCodeTypeSerializerState::End__,
                        }
                    }
                    DocumentCodeTypeSerializerState::End__ => {
                        *self.state = DocumentCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentCodeTypeSerializerState::Done__ => return Ok(None),
                    DocumentCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DocumentCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeSerializer<'ser> {
        pub(super) value: &'ser super::DateTimeType,
        pub(super) state: Box<DateTimeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DateTimeTypeSerializerState<'ser> {
        Init__,
        Content__(<super::DateTimeTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DateTimeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeSerializerState::Init__ => {
                        *self.state = DateTimeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DateTimeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DateTimeTypeSerializerState::End__,
                        }
                    }
                    DateTimeTypeSerializerState::End__ => {
                        *self.state = DateTimeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DateTimeTypeSerializerState::Done__ => return Ok(None),
                    DateTimeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DateTimeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DateTimeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::DateTimeTypeContent,
        pub(super) state: Box<DateTimeTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum DateTimeTypeContentSerializerState<'ser> {
        Init__,
        DateTimeString(<super::DateTimeTypeDateTimeStringType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DateTimeTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeContentSerializerState::Init__ => match self.value {
                        super::DateTimeTypeContent::DateTimeString(x) => {
                            *self.state = DateTimeTypeContentSerializerState::DateTimeString(
                                WithSerializer::serializer(x, Some("udt:DateTimeString"), false)?,
                            )
                        }
                    },
                    DateTimeTypeContentSerializerState::DateTimeString(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DateTimeTypeContentSerializerState::Done__,
                        }
                    }
                    DateTimeTypeContentSerializerState::Done__ => return Ok(None),
                    DateTimeTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DateTimeTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DateTimeTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NoteTypeSerializer<'ser> {
        pub(super) value: &'ser super::NoteType,
        pub(super) state: Box<NoteTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NoteTypeSerializerState<'ser> {
        Init__,
        Content(<super::TextType as WithSerializer>::Serializer<'ser>),
        SubjectCode(IterSerializer<'ser, Option<&'ser super::CodeType>, super::CodeType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NoteTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NoteTypeSerializerState::Init__ => {
                        *self.state = NoteTypeSerializerState::Content(WithSerializer::serializer(
                            &self.value.content,
                            Some("ram:Content"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NoteTypeSerializerState::Content(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = NoteTypeSerializerState::SubjectCode(IterSerializer::new(
                                self.value.subject_code.as_ref(),
                                Some("ram:SubjectCode"),
                                false,
                            ))
                        }
                    },
                    NoteTypeSerializerState::SubjectCode(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NoteTypeSerializerState::End__,
                    },
                    NoteTypeSerializerState::End__ => {
                        *self.state = NoteTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NoteTypeSerializerState::Done__ => return Ok(None),
                    NoteTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NoteTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NoteTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeAgreementTypeSerializer<'ser> {
        pub(super) value: &'ser super::HeaderTradeAgreementType,
        pub(super) state: Box<HeaderTradeAgreementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeaderTradeAgreementTypeSerializerState<'ser> {
        Init__,
        BuyerReference(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        SellerTradeParty(<super::TradePartyType as WithSerializer>::Serializer<'ser>),
        BuyerTradeParty(<super::TradePartyType as WithSerializer>::Serializer<'ser>),
        SellerTaxRepresentativeTradeParty(
            IterSerializer<'ser, Option<&'ser super::TradePartyType>, super::TradePartyType>,
        ),
        BuyerOrderReferencedDocument(
            IterSerializer<
                'ser,
                Option<&'ser super::ReferencedDocumentType>,
                super::ReferencedDocumentType,
            >,
        ),
        ContractReferencedDocument(
            IterSerializer<
                'ser,
                Option<&'ser super::ReferencedDocumentType>,
                super::ReferencedDocumentType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeAgreementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeAgreementTypeSerializerState :: Init__ => { * self . state = HeaderTradeAgreementTypeSerializerState :: BuyerReference (IterSerializer :: new (self . value . buyer_reference . as_ref () , Some ("ram:BuyerReference") , false)) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_QDT) , & super :: NS_QDT) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeAgreementTypeSerializerState :: BuyerReference (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: SellerTradeParty (WithSerializer :: serializer (& self . value . seller_trade_party , Some ("ram:SellerTradeParty") , false) ?) , } HeaderTradeAgreementTypeSerializerState :: SellerTradeParty (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: BuyerTradeParty (WithSerializer :: serializer (& self . value . buyer_trade_party , Some ("ram:BuyerTradeParty") , false) ?) , } HeaderTradeAgreementTypeSerializerState :: BuyerTradeParty (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: SellerTaxRepresentativeTradeParty (IterSerializer :: new (self . value . seller_tax_representative_trade_party . as_ref () , Some ("ram:SellerTaxRepresentativeTradeParty") , false)) , } HeaderTradeAgreementTypeSerializerState :: SellerTaxRepresentativeTradeParty (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: BuyerOrderReferencedDocument (IterSerializer :: new (self . value . buyer_order_referenced_document . as_ref () , Some ("ram:BuyerOrderReferencedDocument") , false)) , } HeaderTradeAgreementTypeSerializerState :: BuyerOrderReferencedDocument (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: ContractReferencedDocument (IterSerializer :: new (self . value . contract_referenced_document . as_ref () , Some ("ram:ContractReferencedDocument") , false)) , } HeaderTradeAgreementTypeSerializerState :: ContractReferencedDocument (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: End__ , } HeaderTradeAgreementTypeSerializerState :: End__ => { * self . state = HeaderTradeAgreementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeAgreementTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeAgreementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for HeaderTradeAgreementTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeaderTradeAgreementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeDeliveryTypeSerializer<'ser> {
        pub(super) value: &'ser super::HeaderTradeDeliveryType,
        pub(super) state: Box<HeaderTradeDeliveryTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeaderTradeDeliveryTypeSerializerState<'ser> {
        Init__,
        ShipToTradeParty(
            IterSerializer<'ser, Option<&'ser super::TradePartyType>, super::TradePartyType>,
        ),
        ActualDeliverySupplyChainEvent(
            IterSerializer<
                'ser,
                Option<&'ser super::SupplyChainEventType>,
                super::SupplyChainEventType,
            >,
        ),
        DespatchAdviceReferencedDocument(
            IterSerializer<
                'ser,
                Option<&'ser super::ReferencedDocumentType>,
                super::ReferencedDocumentType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeDeliveryTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeDeliveryTypeSerializerState :: Init__ => { * self . state = HeaderTradeDeliveryTypeSerializerState :: ShipToTradeParty (IterSerializer :: new (self . value . ship_to_trade_party . as_ref () , Some ("ram:ShipToTradeParty") , false)) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_QDT) , & super :: NS_QDT) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeDeliveryTypeSerializerState :: ShipToTradeParty (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeDeliveryTypeSerializerState :: ActualDeliverySupplyChainEvent (IterSerializer :: new (self . value . actual_delivery_supply_chain_event . as_ref () , Some ("ram:ActualDeliverySupplyChainEvent") , false)) , } HeaderTradeDeliveryTypeSerializerState :: ActualDeliverySupplyChainEvent (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeDeliveryTypeSerializerState :: DespatchAdviceReferencedDocument (IterSerializer :: new (self . value . despatch_advice_referenced_document . as_ref () , Some ("ram:DespatchAdviceReferencedDocument") , false)) , } HeaderTradeDeliveryTypeSerializerState :: DespatchAdviceReferencedDocument (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeDeliveryTypeSerializerState :: End__ , } HeaderTradeDeliveryTypeSerializerState :: End__ => { * self . state = HeaderTradeDeliveryTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeDeliveryTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeDeliveryTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for HeaderTradeDeliveryTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeaderTradeDeliveryTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeSettlementTypeSerializer<'ser> {
        pub(super) value: &'ser super::HeaderTradeSettlementType,
        pub(super) state: Box<HeaderTradeSettlementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeaderTradeSettlementTypeSerializerState<'ser> {
        Init__,
        CreditorReferenceId(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        PaymentReference(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        TaxCurrencyCode(
            IterSerializer<'ser, Option<&'ser super::CurrencyCodeType>, super::CurrencyCodeType>,
        ),
        InvoiceCurrencyCode(<super::CurrencyCodeType as WithSerializer>::Serializer<'ser>),
        PayeeTradeParty(
            IterSerializer<'ser, Option<&'ser super::TradePartyType>, super::TradePartyType>,
        ),
        SpecifiedTradeSettlementPaymentMeans(
            IterSerializer<
                'ser,
                &'ser [super::TradeSettlementPaymentMeansType],
                super::TradeSettlementPaymentMeansType,
            >,
        ),
        ApplicableTradeTax(IterSerializer<'ser, &'ser [super::TradeTaxType], super::TradeTaxType>),
        BillingSpecifiedPeriod(
            IterSerializer<
                'ser,
                Option<&'ser super::SpecifiedPeriodType>,
                super::SpecifiedPeriodType,
            >,
        ),
        SpecifiedTradeAllowanceCharge(
            IterSerializer<
                'ser,
                &'ser [super::TradeAllowanceChargeType],
                super::TradeAllowanceChargeType,
            >,
        ),
        SpecifiedTradePaymentTerms(
            IterSerializer<
                'ser,
                Option<&'ser super::TradePaymentTermsType>,
                super::TradePaymentTermsType,
            >,
        ),
        SpecifiedTradeSettlementHeaderMonetarySummation(
            <super::TradeSettlementHeaderMonetarySummationType as WithSerializer>::Serializer<'ser>,
        ),
        InvoiceReferencedDocument(
            IterSerializer<
                'ser,
                &'ser [super::ReferencedDocumentType],
                super::ReferencedDocumentType,
            >,
        ),
        ReceivableSpecifiedTradeAccountingAccount(
            IterSerializer<
                'ser,
                Option<&'ser super::TradeAccountingAccountType>,
                super::TradeAccountingAccountType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeSettlementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeSettlementTypeSerializerState :: Init__ => { * self . state = HeaderTradeSettlementTypeSerializerState :: CreditorReferenceId (IterSerializer :: new (self . value . creditor_reference_id . as_ref () , Some ("ram:CreditorReferenceID") , false)) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_QDT) , & super :: NS_QDT) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeSettlementTypeSerializerState :: CreditorReferenceId (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: PaymentReference (IterSerializer :: new (self . value . payment_reference . as_ref () , Some ("ram:PaymentReference") , false)) , } HeaderTradeSettlementTypeSerializerState :: PaymentReference (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: TaxCurrencyCode (IterSerializer :: new (self . value . tax_currency_code . as_ref () , Some ("ram:TaxCurrencyCode") , false)) , } HeaderTradeSettlementTypeSerializerState :: TaxCurrencyCode (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: InvoiceCurrencyCode (WithSerializer :: serializer (& self . value . invoice_currency_code , Some ("ram:InvoiceCurrencyCode") , false) ?) , } HeaderTradeSettlementTypeSerializerState :: InvoiceCurrencyCode (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: PayeeTradeParty (IterSerializer :: new (self . value . payee_trade_party . as_ref () , Some ("ram:PayeeTradeParty") , false)) , } HeaderTradeSettlementTypeSerializerState :: PayeeTradeParty (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementPaymentMeans (IterSerializer :: new (& self . value . specified_trade_settlement_payment_means [..] , Some ("ram:SpecifiedTradeSettlementPaymentMeans") , false)) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementPaymentMeans (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: ApplicableTradeTax (IterSerializer :: new (& self . value . applicable_trade_tax [..] , Some ("ram:ApplicableTradeTax") , false)) , } HeaderTradeSettlementTypeSerializerState :: ApplicableTradeTax (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: BillingSpecifiedPeriod (IterSerializer :: new (self . value . billing_specified_period . as_ref () , Some ("ram:BillingSpecifiedPeriod") , false)) , } HeaderTradeSettlementTypeSerializerState :: BillingSpecifiedPeriod (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeAllowanceCharge (IterSerializer :: new (& self . value . specified_trade_allowance_charge [..] , Some ("ram:SpecifiedTradeAllowanceCharge") , false)) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeAllowanceCharge (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradePaymentTerms (IterSerializer :: new (self . value . specified_trade_payment_terms . as_ref () , Some ("ram:SpecifiedTradePaymentTerms") , false)) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradePaymentTerms (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (WithSerializer :: serializer (& self . value . specified_trade_settlement_header_monetary_summation , Some ("ram:SpecifiedTradeSettlementHeaderMonetarySummation") , false) ?) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: InvoiceReferencedDocument (IterSerializer :: new (& self . value . invoice_referenced_document [..] , Some ("ram:InvoiceReferencedDocument") , false)) , } HeaderTradeSettlementTypeSerializerState :: InvoiceReferencedDocument (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: ReceivableSpecifiedTradeAccountingAccount (IterSerializer :: new (self . value . receivable_specified_trade_accounting_account . as_ref () , Some ("ram:ReceivableSpecifiedTradeAccountingAccount") , false)) , } HeaderTradeSettlementTypeSerializerState :: ReceivableSpecifiedTradeAccountingAccount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: End__ , } HeaderTradeSettlementTypeSerializerState :: End__ => { * self . state = HeaderTradeSettlementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeSettlementTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeSettlementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for HeaderTradeSettlementTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeaderTradeSettlementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        pub(super) value: &'ser super::DateTimeTypeDateTimeStringType,
        pub(super) state: Box<DateTimeTypeDateTimeStringTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DateTimeTypeDateTimeStringTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeDateTimeStringTypeSerializerState::Init__ => {
                        *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        helper.write_attrib(&mut bytes, "format", &self.value.format)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DateTimeTypeDateTimeStringTypeSerializerState::Content__(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DateTimeTypeDateTimeStringTypeSerializerState::End__,
                    },
                    DateTimeTypeDateTimeStringTypeSerializerState::End__ => {
                        *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DateTimeTypeDateTimeStringTypeSerializerState::Done__ => return Ok(None),
                    DateTimeTypeDateTimeStringTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TextTypeSerializer<'ser> {
        pub(super) value: &'ser super::TextType,
        pub(super) state: Box<TextTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TextTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TextTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TextTypeSerializerState::Init__ => {
                        *self.state = TextTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TextTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TextTypeSerializerState::End__,
                    },
                    TextTypeSerializerState::End__ => {
                        *self.state = TextTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TextTypeSerializerState::Done__ => return Ok(None),
                    TextTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TextTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TextTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::CodeType,
        pub(super) state: Box<CodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CodeTypeSerializerState::Init__ => {
                        *self.state = CodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CodeTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CodeTypeSerializerState::End__,
                    },
                    CodeTypeSerializerState::End__ => {
                        *self.state = CodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CodeTypeSerializerState::Done__ => return Ok(None),
                    CodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradePartyTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradePartyType,
        pub(super) state: Box<TradePartyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradePartyTypeSerializerState<'ser> {
        Init__,
        Id(IterSerializer<'ser, &'ser [super::IdType], super::IdType>),
        GlobalId(IterSerializer<'ser, &'ser [super::IdType], super::IdType>),
        Name(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        SpecifiedLegalOrganization(
            IterSerializer<
                'ser,
                Option<&'ser super::LegalOrganizationType>,
                super::LegalOrganizationType,
            >,
        ),
        PostalTradeAddress(
            IterSerializer<'ser, Option<&'ser super::TradeAddressType>, super::TradeAddressType>,
        ),
        UriUniversalCommunication(
            IterSerializer<
                'ser,
                Option<&'ser super::UniversalCommunicationType>,
                super::UniversalCommunicationType,
            >,
        ),
        SpecifiedTaxRegistration(
            IterSerializer<'ser, &'ser [super::TaxRegistrationType], super::TaxRegistrationType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradePartyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradePartyTypeSerializerState::Init__ => {
                        *self.state = TradePartyTypeSerializerState::Id(IterSerializer::new(
                            &self.value.id[..],
                            Some("ram:ID"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradePartyTypeSerializerState::Id(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradePartyTypeSerializerState::GlobalId(IterSerializer::new(
                                    &self.value.global_id[..],
                                    Some("ram:GlobalID"),
                                    false,
                                ))
                        }
                    },
                    TradePartyTypeSerializerState::GlobalId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePartyTypeSerializerState::Name(IterSerializer::new(
                                        self.value.name.as_ref(),
                                        Some("ram:Name"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradePartyTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradePartyTypeSerializerState::SpecifiedLegalOrganization(
                                IterSerializer::new(
                                    self.value.specified_legal_organization.as_ref(),
                                    Some("ram:SpecifiedLegalOrganization"),
                                    false,
                                ),
                            )
                        }
                    },
                    TradePartyTypeSerializerState::SpecifiedLegalOrganization(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradePartyTypeSerializerState::PostalTradeAddress(
                                    IterSerializer::new(
                                        self.value.postal_trade_address.as_ref(),
                                        Some("ram:PostalTradeAddress"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradePartyTypeSerializerState::PostalTradeAddress(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePartyTypeSerializerState::UriUniversalCommunication(
                                        IterSerializer::new(
                                            self.value.uri_universal_communication.as_ref(),
                                            Some("ram:URIUniversalCommunication"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradePartyTypeSerializerState::UriUniversalCommunication(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePartyTypeSerializerState::SpecifiedTaxRegistration(
                                        IterSerializer::new(
                                            &self.value.specified_tax_registration[..],
                                            Some("ram:SpecifiedTaxRegistration"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradePartyTypeSerializerState::SpecifiedTaxRegistration(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradePartyTypeSerializerState::End__,
                        }
                    }
                    TradePartyTypeSerializerState::End__ => {
                        *self.state = TradePartyTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradePartyTypeSerializerState::Done__ => return Ok(None),
                    TradePartyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradePartyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradePartyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ReferencedDocumentTypeSerializer<'ser> {
        pub(super) value: &'ser super::ReferencedDocumentType,
        pub(super) state: Box<ReferencedDocumentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ReferencedDocumentTypeSerializerState<'ser> {
        Init__,
        IssuerAssignedId(<super::IdType as WithSerializer>::Serializer<'ser>),
        FormattedIssueDateTime(
            IterSerializer<
                'ser,
                Option<&'ser super::FormattedDateTimeType>,
                super::FormattedDateTimeType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ReferencedDocumentTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ReferencedDocumentTypeSerializerState::Init__ => {
                        *self.state = ReferencedDocumentTypeSerializerState::IssuerAssignedId(
                            WithSerializer::serializer(
                                &self.value.issuer_assigned_id,
                                Some("ram:IssuerAssignedID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ReferencedDocumentTypeSerializerState::IssuerAssignedId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    ReferencedDocumentTypeSerializerState::FormattedIssueDateTime(
                                        IterSerializer::new(
                                            self.value.formatted_issue_date_time.as_ref(),
                                            Some("ram:FormattedIssueDateTime"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    ReferencedDocumentTypeSerializerState::FormattedIssueDateTime(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ReferencedDocumentTypeSerializerState::End__,
                        }
                    }
                    ReferencedDocumentTypeSerializerState::End__ => {
                        *self.state = ReferencedDocumentTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ReferencedDocumentTypeSerializerState::Done__ => return Ok(None),
                    ReferencedDocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ReferencedDocumentTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ReferencedDocumentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainEventTypeSerializer<'ser> {
        pub(super) value: &'ser super::SupplyChainEventType,
        pub(super) state: Box<SupplyChainEventTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SupplyChainEventTypeSerializerState<'ser> {
        Init__,
        OccurrenceDateTime(<super::DateTimeType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SupplyChainEventTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SupplyChainEventTypeSerializerState::Init__ => {
                        *self.state = SupplyChainEventTypeSerializerState::OccurrenceDateTime(
                            WithSerializer::serializer(
                                &self.value.occurrence_date_time,
                                Some("ram:OccurrenceDateTime"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SupplyChainEventTypeSerializerState::OccurrenceDateTime(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SupplyChainEventTypeSerializerState::End__,
                        }
                    }
                    SupplyChainEventTypeSerializerState::End__ => {
                        *self.state = SupplyChainEventTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SupplyChainEventTypeSerializerState::Done__ => return Ok(None),
                    SupplyChainEventTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SupplyChainEventTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SupplyChainEventTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CurrencyCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::CurrencyCodeType,
        pub(super) state: Box<CurrencyCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CurrencyCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CurrencyCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CurrencyCodeTypeSerializerState::Init__ => {
                        *self.state = CurrencyCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CurrencyCodeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CurrencyCodeTypeSerializerState::End__,
                        }
                    }
                    CurrencyCodeTypeSerializerState::End__ => {
                        *self.state = CurrencyCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CurrencyCodeTypeSerializerState::Done__ => return Ok(None),
                    CurrencyCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CurrencyCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CurrencyCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementPaymentMeansTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeSettlementPaymentMeansType,
        pub(super) state: Box<TradeSettlementPaymentMeansTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeSettlementPaymentMeansTypeSerializerState<'ser> {
        Init__,
        TypeCode(<super::PaymentMeansCodeType as WithSerializer>::Serializer<'ser>),
        PayerPartyDebtorFinancialAccount(
            IterSerializer<
                'ser,
                Option<&'ser super::DebtorFinancialAccountType>,
                super::DebtorFinancialAccountType,
            >,
        ),
        PayeePartyCreditorFinancialAccount(
            IterSerializer<
                'ser,
                Option<&'ser super::CreditorFinancialAccountType>,
                super::CreditorFinancialAccountType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeSettlementPaymentMeansTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { TradeSettlementPaymentMeansTypeSerializerState :: Init__ => { * self . state = TradeSettlementPaymentMeansTypeSerializerState :: TypeCode (WithSerializer :: serializer (& self . value . type_code , Some ("ram:TypeCode") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_QDT) , & super :: NS_QDT) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } TradeSettlementPaymentMeansTypeSerializerState :: TypeCode (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementPaymentMeansTypeSerializerState :: PayerPartyDebtorFinancialAccount (IterSerializer :: new (self . value . payer_party_debtor_financial_account . as_ref () , Some ("ram:PayerPartyDebtorFinancialAccount") , false)) , } TradeSettlementPaymentMeansTypeSerializerState :: PayerPartyDebtorFinancialAccount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementPaymentMeansTypeSerializerState :: PayeePartyCreditorFinancialAccount (IterSerializer :: new (self . value . payee_party_creditor_financial_account . as_ref () , Some ("ram:PayeePartyCreditorFinancialAccount") , false)) , } TradeSettlementPaymentMeansTypeSerializerState :: PayeePartyCreditorFinancialAccount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementPaymentMeansTypeSerializerState :: End__ , } TradeSettlementPaymentMeansTypeSerializerState :: End__ => { * self . state = TradeSettlementPaymentMeansTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } TradeSettlementPaymentMeansTypeSerializerState :: Done__ => return Ok (None) , TradeSettlementPaymentMeansTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradeSettlementPaymentMeansTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeSettlementPaymentMeansTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeTaxTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeTaxType,
        pub(super) state: Box<TradeTaxTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeTaxTypeSerializerState<'ser> {
        Init__,
        CalculatedAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        TypeCode(<super::TaxTypeCodeType as WithSerializer>::Serializer<'ser>),
        ExemptionReason(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        BasisAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        CategoryCode(<super::TaxCategoryCodeType as WithSerializer>::Serializer<'ser>),
        ExemptionReasonCode(IterSerializer<'ser, Option<&'ser super::CodeType>, super::CodeType>),
        DueDateTypeCode(
            IterSerializer<
                'ser,
                Option<&'ser super::TimeReferenceCodeType>,
                super::TimeReferenceCodeType,
            >,
        ),
        RateApplicablePercent(
            IterSerializer<'ser, Option<&'ser super::PercentType>, super::PercentType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeTaxTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeTaxTypeSerializerState::Init__ => {
                        *self.state =
                            TradeTaxTypeSerializerState::CalculatedAmount(IterSerializer::new(
                                self.value.calculated_amount.as_ref(),
                                Some("ram:CalculatedAmount"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeTaxTypeSerializerState::CalculatedAmount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::TypeCode(
                                    WithSerializer::serializer(
                                        &self.value.type_code,
                                        Some("ram:TypeCode"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::TypeCode(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradeTaxTypeSerializerState::ExemptionReason(IterSerializer::new(
                                    self.value.exemption_reason.as_ref(),
                                    Some("ram:ExemptionReason"),
                                    false,
                                ))
                        }
                    },
                    TradeTaxTypeSerializerState::ExemptionReason(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeTaxTypeSerializerState::BasisAmount(IterSerializer::new(
                                        self.value.basis_amount.as_ref(),
                                        Some("ram:BasisAmount"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::BasisAmount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::CategoryCode(
                                    WithSerializer::serializer(
                                        &self.value.category_code,
                                        Some("ram:CategoryCode"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::CategoryCode(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::ExemptionReasonCode(
                                    IterSerializer::new(
                                        self.value.exemption_reason_code.as_ref(),
                                        Some("ram:ExemptionReasonCode"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::ExemptionReasonCode(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::DueDateTypeCode(
                                    IterSerializer::new(
                                        self.value.due_date_type_code.as_ref(),
                                        Some("ram:DueDateTypeCode"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::DueDateTypeCode(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::RateApplicablePercent(
                                    IterSerializer::new(
                                        self.value.rate_applicable_percent.as_ref(),
                                        Some("ram:RateApplicablePercent"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::RateApplicablePercent(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeTaxTypeSerializerState::End__,
                        }
                    }
                    TradeTaxTypeSerializerState::End__ => {
                        *self.state = TradeTaxTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeTaxTypeSerializerState::Done__ => return Ok(None),
                    TradeTaxTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradeTaxTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeTaxTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpecifiedPeriodTypeSerializer<'ser> {
        pub(super) value: &'ser super::SpecifiedPeriodType,
        pub(super) state: Box<SpecifiedPeriodTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SpecifiedPeriodTypeSerializerState<'ser> {
        Init__,
        StartDateTime(IterSerializer<'ser, Option<&'ser super::DateTimeType>, super::DateTimeType>),
        EndDateTime(IterSerializer<'ser, Option<&'ser super::DateTimeType>, super::DateTimeType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpecifiedPeriodTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SpecifiedPeriodTypeSerializerState::Init__ => {
                        *self.state =
                            SpecifiedPeriodTypeSerializerState::StartDateTime(IterSerializer::new(
                                self.value.start_date_time.as_ref(),
                                Some("ram:StartDateTime"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SpecifiedPeriodTypeSerializerState::StartDateTime(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = SpecifiedPeriodTypeSerializerState::EndDateTime(
                                    IterSerializer::new(
                                        self.value.end_date_time.as_ref(),
                                        Some("ram:EndDateTime"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    SpecifiedPeriodTypeSerializerState::EndDateTime(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SpecifiedPeriodTypeSerializerState::End__,
                        }
                    }
                    SpecifiedPeriodTypeSerializerState::End__ => {
                        *self.state = SpecifiedPeriodTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SpecifiedPeriodTypeSerializerState::Done__ => return Ok(None),
                    SpecifiedPeriodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SpecifiedPeriodTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SpecifiedPeriodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeAllowanceChargeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeAllowanceChargeType,
        pub(super) state: Box<TradeAllowanceChargeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeAllowanceChargeTypeSerializerState<'ser> {
        Init__,
        ChargeIndicator(<super::IndicatorType as WithSerializer>::Serializer<'ser>),
        CalculationPercent(
            IterSerializer<'ser, Option<&'ser super::PercentType>, super::PercentType>,
        ),
        BasisAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        ActualAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        ReasonCode(
            IterSerializer<
                'ser,
                Option<&'ser super::AllowanceChargeReasonCodeType>,
                super::AllowanceChargeReasonCodeType,
            >,
        ),
        Reason(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        CategoryTradeTax(<super::TradeTaxType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAllowanceChargeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAllowanceChargeTypeSerializerState::Init__ => {
                        *self.state = TradeAllowanceChargeTypeSerializerState::ChargeIndicator(
                            WithSerializer::serializer(
                                &self.value.charge_indicator,
                                Some("ram:ChargeIndicator"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAllowanceChargeTypeSerializerState::ChargeIndicator(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAllowanceChargeTypeSerializerState::CalculationPercent(
                                        IterSerializer::new(
                                            self.value.calculation_percent.as_ref(),
                                            Some("ram:CalculationPercent"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::CalculationPercent(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::BasisAmount(
                                    IterSerializer::new(
                                        self.value.basis_amount.as_ref(),
                                        Some("ram:BasisAmount"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::BasisAmount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::ActualAmount(
                                    WithSerializer::serializer(
                                        &self.value.actual_amount,
                                        Some("ram:ActualAmount"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::ActualAmount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::ReasonCode(
                                    IterSerializer::new(
                                        self.value.reason_code.as_ref(),
                                        Some("ram:ReasonCode"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::ReasonCode(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::Reason(
                                    IterSerializer::new(
                                        self.value.reason.as_ref(),
                                        Some("ram:Reason"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::Reason(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAllowanceChargeTypeSerializerState::CategoryTradeTax(
                                        WithSerializer::serializer(
                                            &self.value.category_trade_tax,
                                            Some("ram:CategoryTradeTax"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::CategoryTradeTax(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeAllowanceChargeTypeSerializerState::End__,
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::End__ => {
                        *self.state = TradeAllowanceChargeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAllowanceChargeTypeSerializerState::Done__ => return Ok(None),
                    TradeAllowanceChargeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradeAllowanceChargeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeAllowanceChargeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradePaymentTermsTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradePaymentTermsType,
        pub(super) state: Box<TradePaymentTermsTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradePaymentTermsTypeSerializerState<'ser> {
        Init__,
        Description(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        DueDateDateTime(
            IterSerializer<'ser, Option<&'ser super::DateTimeType>, super::DateTimeType>,
        ),
        DirectDebitMandateId(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradePaymentTermsTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradePaymentTermsTypeSerializerState::Init__ => {
                        *self.state =
                            TradePaymentTermsTypeSerializerState::Description(IterSerializer::new(
                                self.value.description.as_ref(),
                                Some("ram:Description"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradePaymentTermsTypeSerializerState::Description(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradePaymentTermsTypeSerializerState::DueDateDateTime(
                                    IterSerializer::new(
                                        self.value.due_date_date_time.as_ref(),
                                        Some("ram:DueDateDateTime"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradePaymentTermsTypeSerializerState::DueDateDateTime(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePaymentTermsTypeSerializerState::DirectDebitMandateId(
                                        IterSerializer::new(
                                            self.value.direct_debit_mandate_id.as_ref(),
                                            Some("ram:DirectDebitMandateID"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradePaymentTermsTypeSerializerState::DirectDebitMandateId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradePaymentTermsTypeSerializerState::End__,
                        }
                    }
                    TradePaymentTermsTypeSerializerState::End__ => {
                        *self.state = TradePaymentTermsTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradePaymentTermsTypeSerializerState::Done__ => return Ok(None),
                    TradePaymentTermsTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradePaymentTermsTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradePaymentTermsTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeSettlementHeaderMonetarySummationType,
        pub(super) state: Box<TradeSettlementHeaderMonetarySummationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeSettlementHeaderMonetarySummationTypeSerializerState<'ser> {
        Init__,
        LineTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        ChargeTotalAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        AllowanceTotalAmount(
            IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>,
        ),
        TaxBasisTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        TaxTotalAmount(IterSerializer<'ser, &'ser [super::AmountType], super::AmountType>),
        GrandTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        TotalPrepaidAmount(
            IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>,
        ),
        DuePayableAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { TradeSettlementHeaderMonetarySummationTypeSerializerState :: Init__ => { * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: LineTotalAmount (WithSerializer :: serializer (& self . value . line_total_amount , Some ("ram:LineTotalAmount") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; helper . begin_ns_scope () ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_RAM) , & super :: NS_RAM) ; helper . write_xmlns (& mut bytes , Some (& super :: PREFIX_UDT) , & super :: NS_UDT) ; } return Ok (Some (Event :: Start (bytes))) } TradeSettlementHeaderMonetarySummationTypeSerializerState :: LineTotalAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: ChargeTotalAmount (IterSerializer :: new (self . value . charge_total_amount . as_ref () , Some ("ram:ChargeTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: ChargeTotalAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: AllowanceTotalAmount (IterSerializer :: new (self . value . allowance_total_amount . as_ref () , Some ("ram:AllowanceTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: AllowanceTotalAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxBasisTotalAmount (WithSerializer :: serializer (& self . value . tax_basis_total_amount , Some ("ram:TaxBasisTotalAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxBasisTotalAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxTotalAmount (IterSerializer :: new (& self . value . tax_total_amount [..] , Some ("ram:TaxTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxTotalAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: GrandTotalAmount (WithSerializer :: serializer (& self . value . grand_total_amount , Some ("ram:GrandTotalAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: GrandTotalAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TotalPrepaidAmount (IterSerializer :: new (self . value . total_prepaid_amount . as_ref () , Some ("ram:TotalPrepaidAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TotalPrepaidAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: DuePayableAmount (WithSerializer :: serializer (& self . value . due_payable_amount , Some ("ram:DuePayableAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: DuePayableAmount (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: End__ , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: End__ => { * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } TradeSettlementHeaderMonetarySummationTypeSerializerState :: Done__ => return Ok (None) , TradeSettlementHeaderMonetarySummationTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeSettlementHeaderMonetarySummationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeAccountingAccountTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeAccountingAccountType,
        pub(super) state: Box<TradeAccountingAccountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeAccountingAccountTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAccountingAccountTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAccountingAccountTypeSerializerState::Init__ => {
                        *self.state = TradeAccountingAccountTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAccountingAccountTypeSerializerState::Id(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeAccountingAccountTypeSerializerState::End__,
                        }
                    }
                    TradeAccountingAccountTypeSerializerState::End__ => {
                        *self.state = TradeAccountingAccountTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAccountingAccountTypeSerializerState::Done__ => return Ok(None),
                    TradeAccountingAccountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradeAccountingAccountTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeAccountingAccountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct LegalOrganizationTypeSerializer<'ser> {
        pub(super) value: &'ser super::LegalOrganizationType,
        pub(super) state: Box<LegalOrganizationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum LegalOrganizationTypeSerializerState<'ser> {
        Init__,
        Id(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        TradingBusinessName(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> LegalOrganizationTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    LegalOrganizationTypeSerializerState::Init__ => {
                        *self.state = LegalOrganizationTypeSerializerState::Id(
                            IterSerializer::new(self.value.id.as_ref(), Some("ram:ID"), false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    LegalOrganizationTypeSerializerState::Id(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    LegalOrganizationTypeSerializerState::TradingBusinessName(
                                        IterSerializer::new(
                                            self.value.trading_business_name.as_ref(),
                                            Some("ram:TradingBusinessName"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    LegalOrganizationTypeSerializerState::TradingBusinessName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = LegalOrganizationTypeSerializerState::End__,
                        }
                    }
                    LegalOrganizationTypeSerializerState::End__ => {
                        *self.state = LegalOrganizationTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    LegalOrganizationTypeSerializerState::Done__ => return Ok(None),
                    LegalOrganizationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for LegalOrganizationTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = LegalOrganizationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeAddressTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeAddressType,
        pub(super) state: Box<TradeAddressTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeAddressTypeSerializerState<'ser> {
        Init__,
        PostcodeCode(IterSerializer<'ser, Option<&'ser super::CodeType>, super::CodeType>),
        LineOne(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        LineTwo(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        LineThree(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        CityName(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        CountryId(<super::CountryIdType as WithSerializer>::Serializer<'ser>),
        CountrySubDivisionName(
            IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAddressTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAddressTypeSerializerState::Init__ => {
                        *self.state =
                            TradeAddressTypeSerializerState::PostcodeCode(IterSerializer::new(
                                self.value.postcode_code.as_ref(),
                                Some("ram:PostcodeCode"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAddressTypeSerializerState::PostcodeCode(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAddressTypeSerializerState::LineOne(IterSerializer::new(
                                        self.value.line_one.as_ref(),
                                        Some("ram:LineOne"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::LineOne(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAddressTypeSerializerState::LineTwo(IterSerializer::new(
                                        self.value.line_two.as_ref(),
                                        Some("ram:LineTwo"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::LineTwo(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAddressTypeSerializerState::LineThree(IterSerializer::new(
                                        self.value.line_three.as_ref(),
                                        Some("ram:LineThree"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::LineThree(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAddressTypeSerializerState::CityName(IterSerializer::new(
                                        self.value.city_name.as_ref(),
                                        Some("ram:CityName"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::CityName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAddressTypeSerializerState::CountryId(
                                    WithSerializer::serializer(
                                        &self.value.country_id,
                                        Some("ram:CountryID"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::CountryId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAddressTypeSerializerState::CountrySubDivisionName(
                                        IterSerializer::new(
                                            self.value.country_sub_division_name.as_ref(),
                                            Some("ram:CountrySubDivisionName"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::CountrySubDivisionName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeAddressTypeSerializerState::End__,
                        }
                    }
                    TradeAddressTypeSerializerState::End__ => {
                        *self.state = TradeAddressTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAddressTypeSerializerState::Done__ => return Ok(None),
                    TradeAddressTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TradeAddressTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeAddressTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct UniversalCommunicationTypeSerializer<'ser> {
        pub(super) value: &'ser super::UniversalCommunicationType,
        pub(super) state: Box<UniversalCommunicationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum UniversalCommunicationTypeSerializerState<'ser> {
        Init__,
        Uriid(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UniversalCommunicationTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UniversalCommunicationTypeSerializerState::Init__ => {
                        *self.state = UniversalCommunicationTypeSerializerState::Uriid(
                            WithSerializer::serializer(
                                &self.value.uriid,
                                Some("ram:URIID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UniversalCommunicationTypeSerializerState::Uriid(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UniversalCommunicationTypeSerializerState::End__,
                        }
                    }
                    UniversalCommunicationTypeSerializerState::End__ => {
                        *self.state = UniversalCommunicationTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    UniversalCommunicationTypeSerializerState::Done__ => return Ok(None),
                    UniversalCommunicationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for UniversalCommunicationTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = UniversalCommunicationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TaxRegistrationTypeSerializer<'ser> {
        pub(super) value: &'ser super::TaxRegistrationType,
        pub(super) state: Box<TaxRegistrationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TaxRegistrationTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TaxRegistrationTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxRegistrationTypeSerializerState::Init__ => {
                        *self.state = TaxRegistrationTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxRegistrationTypeSerializerState::Id(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TaxRegistrationTypeSerializerState::End__,
                        }
                    }
                    TaxRegistrationTypeSerializerState::End__ => {
                        *self.state = TaxRegistrationTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxRegistrationTypeSerializerState::Done__ => return Ok(None),
                    TaxRegistrationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TaxRegistrationTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TaxRegistrationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeSerializer<'ser> {
        pub(super) value: &'ser super::FormattedDateTimeType,
        pub(super) state: Box<FormattedDateTimeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FormattedDateTimeTypeSerializerState<'ser> {
        Init__,
        DateTimeString(
            <super::FormattedDateTimeTypeDateTimeStringType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FormattedDateTimeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FormattedDateTimeTypeSerializerState::Init__ => {
                        *self.state = FormattedDateTimeTypeSerializerState::DateTimeString(
                            WithSerializer::serializer(
                                &self.value.date_time_string,
                                Some("qdt:DateTimeString"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FormattedDateTimeTypeSerializerState::DateTimeString(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = FormattedDateTimeTypeSerializerState::End__,
                        }
                    }
                    FormattedDateTimeTypeSerializerState::End__ => {
                        *self.state = FormattedDateTimeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FormattedDateTimeTypeSerializerState::Done__ => return Ok(None),
                    FormattedDateTimeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FormattedDateTimeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FormattedDateTimeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PaymentMeansCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::PaymentMeansCodeType,
        pub(super) state: Box<PaymentMeansCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PaymentMeansCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PaymentMeansCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PaymentMeansCodeTypeSerializerState::Init__ => {
                        *self.state = PaymentMeansCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PaymentMeansCodeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PaymentMeansCodeTypeSerializerState::End__,
                        }
                    }
                    PaymentMeansCodeTypeSerializerState::End__ => {
                        *self.state = PaymentMeansCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PaymentMeansCodeTypeSerializerState::Done__ => return Ok(None),
                    PaymentMeansCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PaymentMeansCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PaymentMeansCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DebtorFinancialAccountTypeSerializer<'ser> {
        pub(super) value: &'ser super::DebtorFinancialAccountType,
        pub(super) state: Box<DebtorFinancialAccountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DebtorFinancialAccountTypeSerializerState<'ser> {
        Init__,
        Ibanid(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DebtorFinancialAccountTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DebtorFinancialAccountTypeSerializerState::Init__ => {
                        *self.state = DebtorFinancialAccountTypeSerializerState::Ibanid(
                            WithSerializer::serializer(
                                &self.value.ibanid,
                                Some("ram:IBANID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DebtorFinancialAccountTypeSerializerState::Ibanid(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DebtorFinancialAccountTypeSerializerState::End__,
                        }
                    }
                    DebtorFinancialAccountTypeSerializerState::End__ => {
                        *self.state = DebtorFinancialAccountTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DebtorFinancialAccountTypeSerializerState::Done__ => return Ok(None),
                    DebtorFinancialAccountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DebtorFinancialAccountTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DebtorFinancialAccountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CreditorFinancialAccountTypeSerializer<'ser> {
        pub(super) value: &'ser super::CreditorFinancialAccountType,
        pub(super) state: Box<CreditorFinancialAccountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CreditorFinancialAccountTypeSerializerState<'ser> {
        Init__,
        Ibanid(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        ProprietaryId(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CreditorFinancialAccountTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CreditorFinancialAccountTypeSerializerState::Init__ => {
                        *self.state = CreditorFinancialAccountTypeSerializerState::Ibanid(
                            IterSerializer::new(
                                self.value.ibanid.as_ref(),
                                Some("ram:IBANID"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_RAM),
                                &super::NS_RAM,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CreditorFinancialAccountTypeSerializerState::Ibanid(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    CreditorFinancialAccountTypeSerializerState::ProprietaryId(
                                        IterSerializer::new(
                                            self.value.proprietary_id.as_ref(),
                                            Some("ram:ProprietaryID"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    CreditorFinancialAccountTypeSerializerState::ProprietaryId(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CreditorFinancialAccountTypeSerializerState::End__,
                    },
                    CreditorFinancialAccountTypeSerializerState::End__ => {
                        *self.state = CreditorFinancialAccountTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CreditorFinancialAccountTypeSerializerState::Done__ => return Ok(None),
                    CreditorFinancialAccountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CreditorFinancialAccountTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CreditorFinancialAccountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AmountTypeSerializer<'ser> {
        pub(super) value: &'ser super::AmountType,
        pub(super) state: Box<AmountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AmountTypeSerializerState<'ser> {
        Init__,
        Content__(<f64 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AmountTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AmountTypeSerializerState::Init__ => {
                        *self.state = AmountTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        helper.write_attrib_opt(
                            &mut bytes,
                            "currencyID",
                            &self.value.currency_id,
                        )?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AmountTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AmountTypeSerializerState::End__,
                    },
                    AmountTypeSerializerState::End__ => {
                        *self.state = AmountTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AmountTypeSerializerState::Done__ => return Ok(None),
                    AmountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AmountTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AmountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TaxTypeCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TaxTypeCodeType,
        pub(super) state: Box<TaxTypeCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TaxTypeCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TaxTypeCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxTypeCodeTypeSerializerState::Init__ => {
                        *self.state = TaxTypeCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxTypeCodeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TaxTypeCodeTypeSerializerState::End__,
                        }
                    }
                    TaxTypeCodeTypeSerializerState::End__ => {
                        *self.state = TaxTypeCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxTypeCodeTypeSerializerState::Done__ => return Ok(None),
                    TaxTypeCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TaxTypeCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TaxTypeCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TaxCategoryCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TaxCategoryCodeType,
        pub(super) state: Box<TaxCategoryCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TaxCategoryCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TaxCategoryCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxCategoryCodeTypeSerializerState::Init__ => {
                        *self.state = TaxCategoryCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxCategoryCodeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TaxCategoryCodeTypeSerializerState::End__,
                        }
                    }
                    TaxCategoryCodeTypeSerializerState::End__ => {
                        *self.state = TaxCategoryCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxCategoryCodeTypeSerializerState::Done__ => return Ok(None),
                    TaxCategoryCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TaxCategoryCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TaxCategoryCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TimeReferenceCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TimeReferenceCodeType,
        pub(super) state: Box<TimeReferenceCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TimeReferenceCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TimeReferenceCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TimeReferenceCodeTypeSerializerState::Init__ => {
                        *self.state = TimeReferenceCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TimeReferenceCodeTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TimeReferenceCodeTypeSerializerState::End__,
                        }
                    }
                    TimeReferenceCodeTypeSerializerState::End__ => {
                        *self.state = TimeReferenceCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TimeReferenceCodeTypeSerializerState::Done__ => return Ok(None),
                    TimeReferenceCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TimeReferenceCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TimeReferenceCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PercentTypeSerializer<'ser> {
        pub(super) value: &'ser super::PercentType,
        pub(super) state: Box<PercentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PercentTypeSerializerState<'ser> {
        Init__,
        Content__(<f64 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PercentTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PercentTypeSerializerState::Init__ => {
                        *self.state = PercentTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PercentTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PercentTypeSerializerState::End__,
                    },
                    PercentTypeSerializerState::End__ => {
                        *self.state = PercentTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PercentTypeSerializerState::Done__ => return Ok(None),
                    PercentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PercentTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PercentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeSerializer<'ser> {
        pub(super) value: &'ser super::IndicatorType,
        pub(super) state: Box<IndicatorTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum IndicatorTypeSerializerState<'ser> {
        Init__,
        Content__(<super::IndicatorTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IndicatorTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IndicatorTypeSerializerState::Init__ => {
                        *self.state = IndicatorTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_UDT),
                                &super::NS_UDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    IndicatorTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = IndicatorTypeSerializerState::End__,
                        }
                    }
                    IndicatorTypeSerializerState::End__ => {
                        *self.state = IndicatorTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    IndicatorTypeSerializerState::Done__ => return Ok(None),
                    IndicatorTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for IndicatorTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IndicatorTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::IndicatorTypeContent,
        pub(super) state: Box<IndicatorTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum IndicatorTypeContentSerializerState<'ser> {
        Init__,
        Indicator(<bool as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IndicatorTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IndicatorTypeContentSerializerState::Init__ => match self.value {
                        super::IndicatorTypeContent::Indicator(x) => {
                            *self.state = IndicatorTypeContentSerializerState::Indicator(
                                WithSerializer::serializer(x, Some("udt:Indicator"), false)?,
                            )
                        }
                    },
                    IndicatorTypeContentSerializerState::Indicator(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = IndicatorTypeContentSerializerState::Done__,
                        }
                    }
                    IndicatorTypeContentSerializerState::Done__ => return Ok(None),
                    IndicatorTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for IndicatorTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IndicatorTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AllowanceChargeReasonCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::AllowanceChargeReasonCodeType,
        pub(super) state: Box<AllowanceChargeReasonCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AllowanceChargeReasonCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AllowanceChargeReasonCodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AllowanceChargeReasonCodeTypeSerializerState::Init__ => {
                        *self.state = AllowanceChargeReasonCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AllowanceChargeReasonCodeTypeSerializerState::Content__(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AllowanceChargeReasonCodeTypeSerializerState::End__,
                    },
                    AllowanceChargeReasonCodeTypeSerializerState::End__ => {
                        *self.state = AllowanceChargeReasonCodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AllowanceChargeReasonCodeTypeSerializerState::Done__ => return Ok(None),
                    AllowanceChargeReasonCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AllowanceChargeReasonCodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AllowanceChargeReasonCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CountryIdTypeSerializer<'ser> {
        pub(super) value: &'ser super::CountryIdType,
        pub(super) state: Box<CountryIdTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CountryIdTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CountryIdTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CountryIdTypeSerializerState::Init__ => {
                        *self.state = CountryIdTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CountryIdTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CountryIdTypeSerializerState::End__,
                        }
                    }
                    CountryIdTypeSerializerState::End__ => {
                        *self.state = CountryIdTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CountryIdTypeSerializerState::Done__ => return Ok(None),
                    CountryIdTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CountryIdTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CountryIdTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeDateTimeStringTypeSerializer<'ser> {
        pub(super) value: &'ser super::FormattedDateTimeTypeDateTimeStringType,
        pub(super) state: Box<FormattedDateTimeTypeDateTimeStringTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FormattedDateTimeTypeDateTimeStringTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FormattedDateTimeTypeDateTimeStringTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Init__ => {
                        *self.state =
                            FormattedDateTimeTypeDateTimeStringTypeSerializerState::Content__(
                                WithSerializer::serializer(&self.value.content, None, false)?,
                            );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_QDT),
                                &super::NS_QDT,
                            );
                        }
                        helper.write_attrib(&mut bytes, "format", &self.value.format)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::End__
                            }
                        }
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::End__ => {
                        *self.state =
                            FormattedDateTimeTypeDateTimeStringTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Done__ => {
                        return Ok(None)
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Phantom__(_) => {
                        unreachable!()
                    }
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FormattedDateTimeTypeDateTimeStringTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FormattedDateTimeTypeDateTimeStringTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
