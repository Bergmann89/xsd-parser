use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_RSM: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100");
pub const NS_QDT: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:QualifiedDataType:100");
pub const NS_RAM: Namespace = Namespace::new_const(
    b"urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100",
);
pub const NS_UDT: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100");
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
pub struct HeaderTradeAgreementType {
    pub buyer_reference: Option<TextType>,
    pub seller_trade_party: TradePartyType,
    pub buyer_trade_party: TradePartyType,
    pub buyer_order_referenced_document: Option<ReferencedDocumentType>,
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
pub struct HeaderTradeDeliveryType;
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
    pub invoice_currency_code: CurrencyCodeType,
    pub specified_trade_settlement_header_monetary_summation:
        TradeSettlementHeaderMonetarySummationType,
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
pub struct TradePartyType {
    pub name: TextType,
    pub specified_legal_organization: Option<LegalOrganizationType>,
    pub postal_trade_address: Option<TradeAddressType>,
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
pub struct TradeSettlementHeaderMonetarySummationType {
    pub tax_basis_total_amount: AmountType,
    pub tax_total_amount: Vec<AmountType>,
    pub grand_total_amount: AmountType,
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
pub struct LegalOrganizationType {
    pub id: Option<IdType>,
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
    pub country_id: CountryIdType,
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
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, ContentDeserializer, DeserializeReader, Deserializer,
        DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
        ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct CrossIndustryInvoiceTypeDeserializer {
        exchanged_document_context: Option<super::ExchangedDocumentContextType>,
        exchanged_document: Option<super::ExchangedDocumentType>,
        supply_chain_trade_transaction: Option<super::SupplyChainTradeTransactionType>,
        state: Box<CrossIndustryInvoiceTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                exchanged_document_context: None,
                exchanged_document: None,
                supply_chain_trade_transaction: None,
                state: Box::new(CrossIndustryInvoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CrossIndustryInvoiceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            match state {
                S::ExchangedDocumentContext(Some(deserializer)) => {
                    self.store_exchanged_document_context(deserializer.finish(reader)?)?
                }
                S::ExchangedDocument(Some(deserializer)) => {
                    self.store_exchanged_document(deserializer.finish(reader)?)?
                }
                S::SupplyChainTradeTransaction(Some(deserializer)) => {
                    self.store_supply_chain_trade_transaction(deserializer.finish(reader)?)?
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
        fn handle_exchanged_document_context<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ExchangedDocumentContextType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.exchanged_document_context.is_some() {
                    fallback.get_or_insert(
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(None),
                    );
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exchanged_document_context(data)?;
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_exchanged_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ExchangedDocumentType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.exchanged_document.is_some() {
                    fallback.get_or_insert(
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None),
                    );
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        );
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exchanged_document(data)?;
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = CrossIndustryInvoiceTypeDeserializerState :: SupplyChainTradeTransaction (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_supply_chain_trade_transaction<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SupplyChainTradeTransactionType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.supply_chain_trade_transaction.is_some() {
                    fallback.get_or_insert(
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        ),
                    );
                    *self.state = CrossIndustryInvoiceTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        );
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_supply_chain_trade_transaction(data)?;
                    *self.state = CrossIndustryInvoiceTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (CrossIndustryInvoiceTypeDeserializerState :: SupplyChainTradeTransaction (Some (deserializer))) ;
                            *self.state = CrossIndustryInvoiceTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = CrossIndustryInvoiceTypeDeserializerState :: SupplyChainTradeTransaction (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CrossIndustryInvoiceType>
        for CrossIndustryInvoiceTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CrossIndustryInvoiceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CrossIndustryInvoiceType>
        where
            R: DeserializeReader,
        {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ExchangedDocumentContext(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exchanged_document_context(
                            reader,
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exchanged_document(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_supply_chain_trade_transaction(
                            reader,
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(
                                None,
                            );
                        event
                    }
                    (
                        S::ExchangedDocumentContext(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RSM),
                            b"ExchangedDocumentContext",
                        ) {
                            let output = < super :: ExchangedDocumentContextType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_exchanged_document_context(
                                reader,
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
                        } else {
                            *self.state = S::ExchangedDocument(None);
                            event
                        }
                    }
                    (S::ExchangedDocument(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RSM),
                            b"ExchangedDocument",
                        ) {
                            let output = < super :: ExchangedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_exchanged_document(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SupplyChainTradeTransaction(None);
                            event
                        }
                    }
                    (
                        S::SupplyChainTradeTransaction(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RSM),
                            b"SupplyChainTradeTransaction",
                        ) {
                            let output = < super :: SupplyChainTradeTransactionType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_supply_chain_trade_transaction(
                                reader,
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
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CrossIndustryInvoiceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CrossIndustryInvoiceTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CrossIndustryInvoiceType {
                exchanged_document_context: self
                    .exchanged_document_context
                    .ok_or_else(|| ErrorKind::MissingElement("ExchangedDocumentContext".into()))?,
                exchanged_document: self
                    .exchanged_document
                    .ok_or_else(|| ErrorKind::MissingElement("ExchangedDocument".into()))?,
                supply_chain_trade_transaction: self.supply_chain_trade_transaction.ok_or_else(
                    || ErrorKind::MissingElement("SupplyChainTradeTransaction".into()),
                )?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentContextTypeDeserializer {
        business_process_specified_document_context_parameter:
            Option<super::DocumentContextParameterType>,
        guideline_specified_document_context_parameter: Option<super::DocumentContextParameterType>,
        state: Box<ExchangedDocumentContextTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                business_process_specified_document_context_parameter: None,
                guideline_specified_document_context_parameter: None,
                state: Box::new(ExchangedDocumentContextTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ExchangedDocumentContextTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentContextTypeDeserializerState as S;
            match state {
                S::BusinessProcessSpecifiedDocumentContextParameter(Some(deserializer)) => self
                    .store_business_process_specified_document_context_parameter(
                        deserializer.finish(reader)?,
                    )?,
                S::GuidelineSpecifiedDocumentContextParameter(Some(deserializer)) => self
                    .store_guideline_specified_document_context_parameter(
                        deserializer.finish(reader)?,
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
        fn handle_business_process_specified_document_context_parameter<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentContextParameterType>,
            fallback: &mut Option<ExchangedDocumentContextTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (None)) ;
                * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_business_process_specified_document_context_parameter(data)?;
                    * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (Some (deserializer))) ;
                            * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_guideline_specified_document_context_parameter<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentContextParameterType>,
            fallback: &mut Option<ExchangedDocumentContextTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self
                    .guideline_specified_document_context_parameter
                    .is_some()
                {
                    fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None)) ;
                    *self.state = ExchangedDocumentContextTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_guideline_specified_document_context_parameter(data)?;
                    *self.state = ExchangedDocumentContextTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (Some (deserializer))) ;
                            *self.state = ExchangedDocumentContextTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExchangedDocumentContextType>
        for ExchangedDocumentContextTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentContextType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentContextType>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentContextTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (
                        S::BusinessProcessSpecifiedDocumentContextParameter(Some(deserializer)),
                        event,
                    ) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_business_process_specified_document_context_parameter(
                            reader,
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_guideline_specified_document_context_parameter(
                            reader,
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        * self . state = ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (None) ;
                        event
                    }
                    (
                        S::BusinessProcessSpecifiedDocumentContextParameter(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BusinessProcessSpecifiedDocumentContextParameter",
                        ) {
                            let output = < super :: DocumentContextParameterType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self
                                .handle_business_process_specified_document_context_parameter(
                                    reader,
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
                        } else {
                            *self.state = S::GuidelineSpecifiedDocumentContextParameter(None);
                            event
                        }
                    }
                    (
                        S::GuidelineSpecifiedDocumentContextParameter(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"GuidelineSpecifiedDocumentContextParameter",
                        ) {
                            let output = < super :: DocumentContextParameterType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_guideline_specified_document_context_parameter(
                                reader,
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
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ExchangedDocumentContextType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ExchangedDocumentContextTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ExchangedDocumentContextType {
                business_process_specified_document_context_parameter: self
                    .business_process_specified_document_context_parameter,
                guideline_specified_document_context_parameter: self
                    .guideline_specified_document_context_parameter
                    .ok_or_else(|| {
                        ErrorKind::MissingElement(
                            "GuidelineSpecifiedDocumentContextParameter".into(),
                        )
                    })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentTypeDeserializer {
        id: Option<super::IdType>,
        type_code: Option<super::DocumentCodeType>,
        issue_date_time: Option<super::DateTimeType>,
        state: Box<ExchangedDocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExchangedDocumentTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        TypeCode(Option<<super::DocumentCodeType as WithDeserializer>::Deserializer>),
        IssueDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ExchangedDocumentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                id: None,
                type_code: None,
                issue_date_time: None,
                state: Box::new(ExchangedDocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ExchangedDocumentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(reader)?)?
                }
                S::IssueDateTime(Some(deserializer)) => {
                    self.store_issue_date_time(deserializer.finish(reader)?)?
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
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::Id(None));
                    *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ExchangedDocumentTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::Id(
                                Some(deserializer),
                            ));
                            *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ExchangedDocumentTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_type_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentCodeType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.type_code.is_some() {
                    fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::TypeCode(None));
                    *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ExchangedDocumentTypeDeserializerState::TypeCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_issue_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.issue_date_time.is_some() {
                    fallback
                        .get_or_insert(ExchangedDocumentTypeDeserializerState::IssueDateTime(None));
                    *self.state = ExchangedDocumentTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_issue_date_time(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ExchangedDocumentTypeDeserializerState::IssueDateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ExchangedDocumentTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExchangedDocumentType> for ExchangedDocumentTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentType>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_type_code(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_issue_date_time(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ExchangedDocumentTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TypeCode(None);
                            event
                        }
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"TypeCode") {
                            let output =
                                <super::DocumentCodeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_type_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::IssueDateTime(None);
                            event
                        }
                    }
                    (S::IssueDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"IssueDateTime",
                        ) {
                            let output =
                                <super::DateTimeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_issue_date_time(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ExchangedDocumentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ExchangedDocumentTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ExchangedDocumentType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
                type_code: self
                    .type_code
                    .ok_or_else(|| ErrorKind::MissingElement("TypeCode".into()))?,
                issue_date_time: self
                    .issue_date_time
                    .ok_or_else(|| ErrorKind::MissingElement("IssueDateTime".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainTradeTransactionTypeDeserializer {
        applicable_header_trade_agreement: Option<super::HeaderTradeAgreementType>,
        applicable_header_trade_delivery: Option<super::HeaderTradeDeliveryType>,
        applicable_header_trade_settlement: Option<super::HeaderTradeSettlementType>,
        state: Box<SupplyChainTradeTransactionTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                applicable_header_trade_agreement: None,
                applicable_header_trade_delivery: None,
                applicable_header_trade_settlement: None,
                state: Box::new(SupplyChainTradeTransactionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SupplyChainTradeTransactionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            match state {
                S::ApplicableHeaderTradeAgreement(Some(deserializer)) => {
                    self.store_applicable_header_trade_agreement(deserializer.finish(reader)?)?
                }
                S::ApplicableHeaderTradeDelivery(Some(deserializer)) => {
                    self.store_applicable_header_trade_delivery(deserializer.finish(reader)?)?
                }
                S::ApplicableHeaderTradeSettlement(Some(deserializer)) => {
                    self.store_applicable_header_trade_settlement(deserializer.finish(reader)?)?
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
        fn handle_applicable_header_trade_agreement<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HeaderTradeAgreementType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_header_trade_agreement.is_some() {
                    fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (None)) ;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_agreement(data)?;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (Some (deserializer))) ;
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_applicable_header_trade_delivery<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HeaderTradeDeliveryType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_header_trade_delivery.is_some() {
                    fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None)) ;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_delivery(data)?;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (Some (deserializer))) ;
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_applicable_header_trade_settlement<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HeaderTradeSettlementType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_header_trade_settlement.is_some() {
                    fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None)) ;
                    *self.state = SupplyChainTradeTransactionTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_settlement(data)?;
                    *self.state = SupplyChainTradeTransactionTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (Some (deserializer))) ;
                            *self.state = SupplyChainTradeTransactionTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SupplyChainTradeTransactionType>
        for SupplyChainTradeTransactionTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainTradeTransactionType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainTradeTransactionType>
        where
            R: DeserializeReader,
        {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ApplicableHeaderTradeAgreement(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_header_trade_agreement(
                            reader,
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_header_trade_delivery(
                            reader,
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_header_trade_settlement(
                            reader,
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (None) ;
                        event
                    }
                    (
                        S::ApplicableHeaderTradeAgreement(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeAgreement",
                        ) {
                            let output = < super :: HeaderTradeAgreementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_applicable_header_trade_agreement(
                                reader,
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
                        } else {
                            *self.state = S::ApplicableHeaderTradeDelivery(None);
                            event
                        }
                    }
                    (
                        S::ApplicableHeaderTradeDelivery(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeDelivery",
                        ) {
                            let output = < super :: HeaderTradeDeliveryType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_applicable_header_trade_delivery(
                                reader,
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
                        } else {
                            *self.state = S::ApplicableHeaderTradeSettlement(None);
                            event
                        }
                    }
                    (
                        S::ApplicableHeaderTradeSettlement(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeSettlement",
                        ) {
                            let output = < super :: HeaderTradeSettlementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_applicable_header_trade_settlement(
                                reader,
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
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SupplyChainTradeTransactionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SupplyChainTradeTransactionTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SupplyChainTradeTransactionType {
                applicable_header_trade_agreement: self
                    .applicable_header_trade_agreement
                    .ok_or_else(|| {
                        ErrorKind::MissingElement("ApplicableHeaderTradeAgreement".into())
                    })?,
                applicable_header_trade_delivery: self
                    .applicable_header_trade_delivery
                    .ok_or_else(|| {
                        ErrorKind::MissingElement("ApplicableHeaderTradeDelivery".into())
                    })?,
                applicable_header_trade_settlement: self
                    .applicable_header_trade_settlement
                    .ok_or_else(|| {
                        ErrorKind::MissingElement("ApplicableHeaderTradeSettlement".into())
                    })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentContextParameterTypeDeserializer {
        id: Option<super::IdType>,
        state: Box<DocumentContextParameterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentContextParameterTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentContextParameterTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                id: None,
                state: Box::new(DocumentContextParameterTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentContextParameterTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DocumentContextParameterTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
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
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<DocumentContextParameterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(DocumentContextParameterTypeDeserializerState::Id(None));
                    *self.state = DocumentContextParameterTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DocumentContextParameterTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = DocumentContextParameterTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DocumentContextParameterTypeDeserializerState::Id(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DocumentContextParameterTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DocumentContextParameterTypeDeserializerState::Id(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocumentContextParameterType>
        for DocumentContextParameterTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentContextParameterType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentContextParameterType>
        where
            R: DeserializeReader,
        {
            use DocumentContextParameterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = DocumentContextParameterTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentContextParameterType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DocumentContextParameterTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DocumentContextParameterType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IdTypeDeserializer {
        scheme_id: Option<String>,
        content: Option<String>,
        state: Box<IdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IdTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IdTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut scheme_id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"schemeID")
                ) {
                    reader.read_attrib(&mut scheme_id, b"schemeID", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                scheme_id: scheme_id,
                content: None,
                state: Box::new(IdTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IdTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let IdTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::IdType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::IdType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IdType>
        where
            R: DeserializeReader,
        {
            use IdTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::IdType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, IdTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::IdType {
                scheme_id: self.scheme_id,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentCodeTypeDeserializer {
        content: Option<String>,
        state: Box<DocumentCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DocumentCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(DocumentCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DocumentCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::DocumentCodeType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentCodeType>
        where
            R: DeserializeReader,
        {
            use DocumentCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DocumentCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DocumentCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDeserializer {
        content: Option<super::DateTimeTypeContent>,
        state: Box<DateTimeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateTimeTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::DateTimeTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DateTimeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(DateTimeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DateTimeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DateTimeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeTypeContent>,
            fallback: &mut Option<DateTimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(DateTimeTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = DateTimeTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = DateTimeTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeType> for DateTimeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DateTimeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeType>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::DateTimeTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DateTimeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, DateTimeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DateTimeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeContentDeserializer {
        state: Box<DateTimeTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DateTimeTypeContentDeserializerState {
        Init__,
        DateTimeString(
            Option<super::DateTimeTypeDateTimeStringType>,
            Option<<super::DateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DateTimeTypeContent),
        Unknown__,
    }
    impl DateTimeTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<DateTimeTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self.state = fallback
                    .take()
                    .unwrap_or(DateTimeTypeContentDeserializerState::Init__);
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_UDT),
                Some(b"DateTimeString")
            ) {
                let output = < super :: DateTimeTypeDateTimeStringType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return self.handle_date_time_string(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self.state = fallback
                .take()
                .unwrap_or(DateTimeTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: DateTimeTypeContentDeserializerState,
        ) -> Result<super::DateTimeTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::DateTimeString(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_date_time_string(&mut values, value)?;
                    }
                    Ok(super::DateTimeTypeContent::DateTimeString(
                        values.ok_or_else(|| ErrorKind::MissingElement("DateTimeString".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_date_time_string<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DateTimeTypeDateTimeStringType>,
            output: DeserializerOutput<'de, super::DateTimeTypeDateTimeStringType>,
            fallback: &mut Option<DateTimeTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DateTimeTypeContentDeserializerState::Init__,
                    Some(DateTimeTypeContentDeserializerState::DateTimeString(
                        _,
                        Some(deserializer),
                    )) => DateTimeTypeContentDeserializerState::DateTimeString(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DateTimeTypeContentDeserializerState::DateTimeString(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_date_time_string(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_date_time_string(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DateTimeTypeContentDeserializerState::DateTimeString(values, None),
                    )?;
                    *self.state = DateTimeTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = DateTimeTypeContentDeserializerState::DateTimeString(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeTypeContent> for DateTimeTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(DateTimeTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, DateTimeTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeContent>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DateTimeString(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time_string(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::DateTimeString(values, None), event) => {
                        let output = < super :: DateTimeTypeDateTimeStringType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_date_time_string(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::DateTimeTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeAgreementTypeDeserializer {
        buyer_reference: Option<super::TextType>,
        seller_trade_party: Option<super::TradePartyType>,
        buyer_trade_party: Option<super::TradePartyType>,
        buyer_order_referenced_document: Option<super::ReferencedDocumentType>,
        state: Box<HeaderTradeAgreementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeAgreementTypeDeserializerState {
        Init__,
        BuyerReference(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SellerTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        BuyerTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        BuyerOrderReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl HeaderTradeAgreementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                buyer_reference: None,
                seller_trade_party: None,
                buyer_trade_party: None,
                buyer_order_referenced_document: None,
                state: Box::new(HeaderTradeAgreementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HeaderTradeAgreementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HeaderTradeAgreementTypeDeserializerState as S;
            match state {
                S::BuyerReference(Some(deserializer)) => {
                    self.store_buyer_reference(deserializer.finish(reader)?)?
                }
                S::SellerTradeParty(Some(deserializer)) => {
                    self.store_seller_trade_party(deserializer.finish(reader)?)?
                }
                S::BuyerTradeParty(Some(deserializer)) => {
                    self.store_buyer_trade_party(deserializer.finish(reader)?)?
                }
                S::BuyerOrderReferencedDocument(Some(deserializer)) => {
                    self.store_buyer_order_referenced_document(deserializer.finish(reader)?)?
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
        fn handle_buyer_reference<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HeaderTradeAgreementTypeDeserializerState::BuyerReference(
                    None,
                ));
                *self.state = HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_reference(data)?;
                    *self.state = HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeAgreementTypeDeserializerState::BuyerReference(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerReference(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_seller_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.seller_trade_party.is_some() {
                    fallback.get_or_insert(
                        HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None),
                    );
                    *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_seller_trade_party(data)?;
                    *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_buyer_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.buyer_trade_party.is_some() {
                    fallback.get_or_insert(
                        HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None),
                    );
                    *self.state =
                        HeaderTradeAgreementTypeDeserializerState::BuyerOrderReferencedDocument(
                            None,
                        );
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_trade_party(data)?;
                    *self.state =
                        HeaderTradeAgreementTypeDeserializerState::BuyerOrderReferencedDocument(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: BuyerOrderReferencedDocument (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_buyer_order_referenced_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeAgreementTypeDeserializerState::BuyerOrderReferencedDocument(None),
                );
                *self.state = HeaderTradeAgreementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_order_referenced_document(data)?;
                    *self.state = HeaderTradeAgreementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeAgreementTypeDeserializerState :: BuyerOrderReferencedDocument (Some (deserializer))) ;
                            *self.state = HeaderTradeAgreementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: BuyerOrderReferencedDocument (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeAgreementType>
        for HeaderTradeAgreementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeAgreementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeAgreementType>
        where
            R: DeserializeReader,
        {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::BuyerReference(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buyer_reference(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_seller_trade_party(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buyer_trade_party(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buyer_order_referenced_document(
                            reader,
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            HeaderTradeAgreementTypeDeserializerState::BuyerReference(None);
                        event
                    }
                    (S::BuyerReference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BuyerReference",
                        ) {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_buyer_reference(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SellerTradeParty(None);
                            event
                        }
                    }
                    (S::SellerTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SellerTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_seller_trade_party(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BuyerTradeParty(None);
                            event
                        }
                    }
                    (S::BuyerTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BuyerTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_buyer_trade_party(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BuyerOrderReferencedDocument(None);
                            event
                        }
                    }
                    (
                        S::BuyerOrderReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BuyerOrderReferencedDocument",
                        ) {
                            let output = < super :: ReferencedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_buyer_order_referenced_document(
                                reader,
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
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HeaderTradeAgreementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HeaderTradeAgreementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HeaderTradeAgreementType {
                buyer_reference: self.buyer_reference,
                seller_trade_party: self
                    .seller_trade_party
                    .ok_or_else(|| ErrorKind::MissingElement("SellerTradeParty".into()))?,
                buyer_trade_party: self
                    .buyer_trade_party
                    .ok_or_else(|| ErrorKind::MissingElement("BuyerTradeParty".into()))?,
                buyer_order_referenced_document: self.buyer_order_referenced_document,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeDeliveryTypeDeserializer {
        state: Box<HeaderTradeDeliveryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeDeliveryTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl HeaderTradeDeliveryTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                state: Box::new(HeaderTradeDeliveryTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HeaderTradeDeliveryTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeDeliveryType>
        for HeaderTradeDeliveryTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeDeliveryType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeDeliveryType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HeaderTradeDeliveryType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HeaderTradeDeliveryTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HeaderTradeDeliveryType {})
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeSettlementTypeDeserializer {
        invoice_currency_code: Option<super::CurrencyCodeType>,
        specified_trade_settlement_header_monetary_summation:
            Option<super::TradeSettlementHeaderMonetarySummationType>,
        state: Box<HeaderTradeSettlementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeSettlementTypeDeserializerState {
        Init__ , InvoiceCurrencyCode (Option << super :: CurrencyCodeType as WithDeserializer > :: Deserializer >) , SpecifiedTradeSettlementHeaderMonetarySummation (Option << super :: TradeSettlementHeaderMonetarySummationType as WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl HeaderTradeSettlementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                invoice_currency_code: None,
                specified_trade_settlement_header_monetary_summation: None,
                state: Box::new(HeaderTradeSettlementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HeaderTradeSettlementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HeaderTradeSettlementTypeDeserializerState as S;
            match state {
                S::InvoiceCurrencyCode(Some(deserializer)) => {
                    self.store_invoice_currency_code(deserializer.finish(reader)?)?
                }
                S::SpecifiedTradeSettlementHeaderMonetarySummation(Some(deserializer)) => self
                    .store_specified_trade_settlement_header_monetary_summation(
                        deserializer.finish(reader)?,
                    )?,
                _ => (),
            }
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
        fn handle_invoice_currency_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CurrencyCodeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.invoice_currency_code.is_some() {
                    fallback.get_or_insert(
                        HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None),
                    );
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_invoice_currency_code(data)?;
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(
                                    Some(deserializer),
                                ),
                            );
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_trade_settlement_header_monetary_summation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeSettlementHeaderMonetarySummationType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self
                    .specified_trade_settlement_header_monetary_summation
                    .is_some()
                {
                    fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None)) ;
                    *self.state = HeaderTradeSettlementTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_settlement_header_monetary_summation(data)?;
                    *self.state = HeaderTradeSettlementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (Some (deserializer))) ;
                            *self.state = HeaderTradeSettlementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeSettlementType>
        for HeaderTradeSettlementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeSettlementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeSettlementType>
        where
            R: DeserializeReader,
        {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::InvoiceCurrencyCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_invoice_currency_code(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_trade_settlement_header_monetary_summation(
                            reader,
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None);
                        event
                    }
                    (S::InvoiceCurrencyCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"InvoiceCurrencyCode",
                        ) {
                            let output =
                                <super::CurrencyCodeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_invoice_currency_code(
                                reader,
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
                        } else {
                            *self.state = S::SpecifiedTradeSettlementHeaderMonetarySummation(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementHeaderMonetarySummation(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeSettlementHeaderMonetarySummation",
                        ) {
                            let output = < super :: TradeSettlementHeaderMonetarySummationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_trade_settlement_header_monetary_summation(
                                reader,
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
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HeaderTradeSettlementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HeaderTradeSettlementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HeaderTradeSettlementType {
                invoice_currency_code: self
                    .invoice_currency_code
                    .ok_or_else(|| ErrorKind::MissingElement("InvoiceCurrencyCode".into()))?,
                specified_trade_settlement_header_monetary_summation: self
                    .specified_trade_settlement_header_monetary_summation
                    .ok_or_else(|| {
                        ErrorKind::MissingElement(
                            "SpecifiedTradeSettlementHeaderMonetarySummation".into(),
                        )
                    })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDateTimeStringTypeDeserializer {
        format: String,
        content: Option<String>,
        state: Box<DateTimeTypeDateTimeStringTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateTimeTypeDateTimeStringTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DateTimeTypeDateTimeStringTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut format: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"format")
                ) {
                    reader.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                format: format.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("format".into()))
                })?,
                content: None,
                state: Box::new(DateTimeTypeDateTimeStringTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DateTimeTypeDateTimeStringTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DateTimeTypeDateTimeStringTypeDeserializerState::Content__(deserializer) = state
            {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeDateTimeStringTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DateTimeTypeDateTimeStringType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DateTimeTypeDateTimeStringTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DateTimeTypeDateTimeStringType {
                format: self.format,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TextTypeDeserializer {
        content: Option<String>,
        state: Box<TextTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TextTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TextTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(TextTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TextTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TextTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TextType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TextType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextType>
        where
            R: DeserializeReader,
        {
            use TextTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TextType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TextTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TextType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradePartyTypeDeserializer {
        name: Option<super::TextType>,
        specified_legal_organization: Option<super::LegalOrganizationType>,
        postal_trade_address: Option<super::TradeAddressType>,
        specified_tax_registration: Vec<super::TaxRegistrationType>,
        state: Box<TradePartyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradePartyTypeDeserializerState {
        Init__,
        Name(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SpecifiedLegalOrganization(
            Option<<super::LegalOrganizationType as WithDeserializer>::Deserializer>,
        ),
        PostalTradeAddress(Option<<super::TradeAddressType as WithDeserializer>::Deserializer>),
        SpecifiedTaxRegistration(
            Option<<super::TaxRegistrationType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl TradePartyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                name: None,
                specified_legal_organization: None,
                postal_trade_address: None,
                specified_tax_registration: Vec::new(),
                state: Box::new(TradePartyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradePartyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradePartyTypeDeserializerState as S;
            match state {
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(reader)?)?,
                S::SpecifiedLegalOrganization(Some(deserializer)) => {
                    self.store_specified_legal_organization(deserializer.finish(reader)?)?
                }
                S::PostalTradeAddress(Some(deserializer)) => {
                    self.store_postal_trade_address(deserializer.finish(reader)?)?
                }
                S::SpecifiedTaxRegistration(Some(deserializer)) => {
                    self.store_specified_tax_registration(deserializer.finish(reader)?)?
                }
                _ => (),
            }
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
        fn store_specified_tax_registration(
            &mut self,
            value: super::TaxRegistrationType,
        ) -> Result<(), Error> {
            self.specified_tax_registration.push(value);
            Ok(())
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.name.is_some() {
                    fallback.get_or_insert(TradePartyTypeDeserializerState::Name(None));
                    *self.state = TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradePartyTypeDeserializerState::Name(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state = TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradePartyTypeDeserializerState::Name(Some(
                                deserializer,
                            )));
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::Name(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_legal_organization<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::LegalOrganizationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None),
                );
                *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_legal_organization(data)?;
                    *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::SpecifiedLegalOrganization(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedLegalOrganization(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_postal_trade_address<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeAddressType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::PostalTradeAddress(None));
                *self.state = TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_postal_trade_address(data)?;
                    *self.state = TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::PostalTradeAddress(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_tax_registration<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TaxRegistrationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::SpecifiedTaxRegistration(
                    None,
                ));
                *self.state = TradePartyTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_tax_registration(data)?;
                    if self.specified_tax_registration.len() < 2usize {
                        *self.state =
                            TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                    } else {
                        *self.state = TradePartyTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::SpecifiedTaxRegistration(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::SpecifiedTaxRegistration(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradePartyType> for TradePartyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TradePartyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePartyType>
        where
            R: DeserializeReader,
        {
            use TradePartyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_legal_organization(
                            reader,
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_postal_trade_address(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_tax_registration(
                            reader,
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradePartyTypeDeserializerState::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"Name") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_name(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedLegalOrganization(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedLegalOrganization(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedLegalOrganization",
                        ) {
                            let output = < super :: LegalOrganizationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_legal_organization(
                                reader,
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
                        } else {
                            *self.state = S::PostalTradeAddress(None);
                            event
                        }
                    }
                    (S::PostalTradeAddress(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PostalTradeAddress",
                        ) {
                            let output =
                                <super::TradeAddressType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_postal_trade_address(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedTaxRegistration(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTaxRegistration(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTaxRegistration",
                        ) {
                            let output = < super :: TaxRegistrationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_tax_registration(
                                reader,
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
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradePartyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TradePartyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TradePartyType {
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("Name".into()))?,
                specified_legal_organization: self.specified_legal_organization,
                postal_trade_address: self.postal_trade_address,
                specified_tax_registration: self.specified_tax_registration,
            })
        }
    }
    #[derive(Debug)]
    pub struct ReferencedDocumentTypeDeserializer {
        issuer_assigned_id: Option<super::IdType>,
        state: Box<ReferencedDocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ReferencedDocumentTypeDeserializerState {
        Init__,
        IssuerAssignedId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ReferencedDocumentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                issuer_assigned_id: None,
                state: Box::new(ReferencedDocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ReferencedDocumentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ReferencedDocumentTypeDeserializerState as S;
            match state {
                S::IssuerAssignedId(Some(deserializer)) => {
                    self.store_issuer_assigned_id(deserializer.finish(reader)?)?
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
        fn handle_issuer_assigned_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<ReferencedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.issuer_assigned_id.is_some() {
                    fallback.get_or_insert(
                        ReferencedDocumentTypeDeserializerState::IssuerAssignedId(None),
                    );
                    *self.state = ReferencedDocumentTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ReferencedDocumentTypeDeserializerState::IssuerAssignedId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_issuer_assigned_id(data)?;
                    *self.state = ReferencedDocumentTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ReferencedDocumentTypeDeserializerState::IssuerAssignedId(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = ReferencedDocumentTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ReferencedDocumentTypeDeserializerState::IssuerAssignedId(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ReferencedDocumentType> for ReferencedDocumentTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferencedDocumentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferencedDocumentType>
        where
            R: DeserializeReader,
        {
            use ReferencedDocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::IssuerAssignedId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_issuer_assigned_id(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            ReferencedDocumentTypeDeserializerState::IssuerAssignedId(None);
                        event
                    }
                    (S::IssuerAssignedId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"IssuerAssignedID",
                        ) {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_issuer_assigned_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ReferencedDocumentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ReferencedDocumentTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ReferencedDocumentType {
                issuer_assigned_id: self
                    .issuer_assigned_id
                    .ok_or_else(|| ErrorKind::MissingElement("IssuerAssignedID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CurrencyCodeTypeDeserializer {
        content: Option<String>,
        state: Box<CurrencyCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CurrencyCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CurrencyCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(CurrencyCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CurrencyCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CurrencyCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType>
        where
            R: DeserializeReader,
        {
            use CurrencyCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CurrencyCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CurrencyCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CurrencyCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementHeaderMonetarySummationTypeDeserializer {
        tax_basis_total_amount: Option<super::AmountType>,
        tax_total_amount: Vec<super::AmountType>,
        grand_total_amount: Option<super::AmountType>,
        due_payable_amount: Option<super::AmountType>,
        state: Box<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeSettlementHeaderMonetarySummationTypeDeserializerState {
        Init__,
        TaxBasisTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TaxTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        GrandTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        DuePayableAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeSettlementHeaderMonetarySummationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                tax_basis_total_amount: None,
                tax_total_amount: Vec::new(),
                grand_total_amount: None,
                due_payable_amount: None,
                state: Box::new(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeSettlementHeaderMonetarySummationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            match state {
                S::TaxBasisTotalAmount(Some(deserializer)) => {
                    self.store_tax_basis_total_amount(deserializer.finish(reader)?)?
                }
                S::TaxTotalAmount(Some(deserializer)) => {
                    self.store_tax_total_amount(deserializer.finish(reader)?)?
                }
                S::GrandTotalAmount(Some(deserializer)) => {
                    self.store_grand_total_amount(deserializer.finish(reader)?)?
                }
                S::DuePayableAmount(Some(deserializer)) => {
                    self.store_due_payable_amount(deserializer.finish(reader)?)?
                }
                _ => (),
            }
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
        fn store_due_payable_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.due_payable_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DuePayableAmount",
                )))?;
            }
            self.due_payable_amount = Some(value);
            Ok(())
        }
        fn handle_tax_basis_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.tax_basis_total_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None)) ;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::TaxTotalAmount(
                            None,
                        );
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_basis_total_amount(data)?;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::TaxTotalAmount(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_tax_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::TaxTotalAmount(
                        None,
                    ),
                );
                *self.state =
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::GrandTotalAmount(
                        None,
                    );
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_total_amount(data)?;
                    if self.tax_total_amount.len() < 2usize {
                        * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (None) ;
                    } else {
                        * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (None) ;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_grand_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.grand_total_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (None)) ;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_grand_total_amount(data)?;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_due_payable_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.due_payable_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None)) ;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_payable_amount(data)?;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (Some (deserializer))) ;
                            *self.state =
                                TradeSettlementHeaderMonetarySummationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeSettlementHeaderMonetarySummationType>
        for TradeSettlementHeaderMonetarySummationTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementHeaderMonetarySummationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementHeaderMonetarySummationType>
        where
            R: DeserializeReader,
        {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TaxBasisTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_tax_basis_total_amount(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_tax_total_amount(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_grand_total_amount(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_due_payable_amount(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None) ;
                        event
                    }
                    (S::TaxBasisTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TaxBasisTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_tax_basis_total_amount(
                                reader,
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
                        } else {
                            *self.state = S::TaxTotalAmount(None);
                            event
                        }
                    }
                    (S::TaxTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TaxTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_tax_total_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::GrandTotalAmount(None);
                            event
                        }
                    }
                    (S::GrandTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"GrandTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_grand_total_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DuePayableAmount(None);
                            event
                        }
                    }
                    (S::DuePayableAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"DuePayableAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_due_payable_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::TradeSettlementHeaderMonetarySummationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeSettlementHeaderMonetarySummationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeSettlementHeaderMonetarySummationType {
                tax_basis_total_amount: self
                    .tax_basis_total_amount
                    .ok_or_else(|| ErrorKind::MissingElement("TaxBasisTotalAmount".into()))?,
                tax_total_amount: self.tax_total_amount,
                grand_total_amount: self
                    .grand_total_amount
                    .ok_or_else(|| ErrorKind::MissingElement("GrandTotalAmount".into()))?,
                due_payable_amount: self
                    .due_payable_amount
                    .ok_or_else(|| ErrorKind::MissingElement("DuePayableAmount".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct LegalOrganizationTypeDeserializer {
        id: Option<super::IdType>,
        state: Box<LegalOrganizationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LegalOrganizationTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl LegalOrganizationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                id: None,
                state: Box::new(LegalOrganizationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: LegalOrganizationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use LegalOrganizationTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
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
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<LegalOrganizationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(LegalOrganizationTypeDeserializerState::Id(None));
                *self.state = LegalOrganizationTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = LegalOrganizationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LegalOrganizationTypeDeserializerState::Id(
                                Some(deserializer),
                            ));
                            *self.state = LegalOrganizationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                LegalOrganizationTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::LegalOrganizationType> for LegalOrganizationTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LegalOrganizationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LegalOrganizationType>
        where
            R: DeserializeReader,
        {
            use LegalOrganizationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = LegalOrganizationTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::LegalOrganizationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                LegalOrganizationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::LegalOrganizationType { id: self.id })
        }
    }
    #[derive(Debug)]
    pub struct TradeAddressTypeDeserializer {
        country_id: Option<super::CountryIdType>,
        state: Box<TradeAddressTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAddressTypeDeserializerState {
        Init__,
        CountryId(Option<<super::CountryIdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAddressTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                country_id: None,
                state: Box::new(TradeAddressTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeAddressTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeAddressTypeDeserializerState as S;
            match state {
                S::CountryId(Some(deserializer)) => {
                    self.store_country_id(deserializer.finish(reader)?)?
                }
                _ => (),
            }
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
        fn handle_country_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CountryIdType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.country_id.is_some() {
                    fallback.get_or_insert(TradeAddressTypeDeserializerState::CountryId(None));
                    *self.state = TradeAddressTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeAddressTypeDeserializerState::CountryId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country_id(data)?;
                    *self.state = TradeAddressTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeAddressTypeDeserializerState::CountryId(
                                Some(deserializer),
                            ));
                            *self.state = TradeAddressTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::CountryId(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeAddressType> for TradeAddressTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAddressType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAddressType>
        where
            R: DeserializeReader,
        {
            use TradeAddressTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::CountryId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_country_id(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradeAddressTypeDeserializerState::CountryId(None);
                        event
                    }
                    (S::CountryId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"CountryID") {
                            let output =
                                <super::CountryIdType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_country_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradeAddressType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeAddressTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeAddressType {
                country_id: self
                    .country_id
                    .ok_or_else(|| ErrorKind::MissingElement("CountryID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxRegistrationTypeDeserializer {
        id: Option<super::IdType>,
        state: Box<TaxRegistrationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxRegistrationTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TaxRegistrationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                id: None,
                state: Box::new(TaxRegistrationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TaxRegistrationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TaxRegistrationTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
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
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TaxRegistrationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(TaxRegistrationTypeDeserializerState::Id(None));
                    *self.state = TaxRegistrationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TaxRegistrationTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = TaxRegistrationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TaxRegistrationTypeDeserializerState::Id(Some(
                                deserializer,
                            )));
                            *self.state = TaxRegistrationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TaxRegistrationTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TaxRegistrationType> for TaxRegistrationTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxRegistrationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxRegistrationType>
        where
            R: DeserializeReader,
        {
            use TaxRegistrationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TaxRegistrationTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TaxRegistrationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TaxRegistrationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TaxRegistrationType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AmountTypeDeserializer {
        currency_id: Option<String>,
        content: Option<f64>,
        state: Box<AmountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AmountTypeDeserializerState {
        Init__,
        Content__(<f64 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AmountTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut currency_id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"currencyID")
                ) {
                    reader.read_attrib(&mut currency_id, b"currencyID", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                currency_id: currency_id,
                content: None,
                state: Box::new(AmountTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AmountTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let AmountTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, f64>,
        ) -> DeserializerResult<'de, super::AmountType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AmountType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AmountType>
        where
            R: DeserializeReader,
        {
            use AmountTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AmountType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AmountTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::AmountType {
                currency_id: self.currency_id,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CountryIdTypeDeserializer {
        content: Option<String>,
        state: Box<CountryIdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CountryIdTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CountryIdTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(CountryIdTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CountryIdTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CountryIdTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CountryIdType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::CountryIdType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CountryIdType>
        where
            R: DeserializeReader,
        {
            use CountryIdTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CountryIdType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, CountryIdTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::CountryIdType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        write_attrib, write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer,
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocumentContext(x) => {
                        match x.next().transpose()? {
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
                        .next()
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CrossIndustryInvoiceTypeSerializerState::End__,
                        }
                    }
                    CrossIndustryInvoiceTypeSerializerState::End__ => {
                        *self.state = CrossIndustryInvoiceTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CrossIndustryInvoiceTypeSerializerState::Done__ => return Ok(None),
                    CrossIndustryInvoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CrossIndustryInvoiceTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { ExchangedDocumentContextTypeSerializerState :: Init__ => { * self . state = ExchangedDocumentContextTypeSerializerState :: BusinessProcessSpecifiedDocumentContextParameter (IterSerializer :: new (self . value . business_process_specified_document_context_parameter . as_ref () , Some ("ram:BusinessProcessSpecifiedDocumentContextParameter") , false)) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } ExchangedDocumentContextTypeSerializerState :: BusinessProcessSpecifiedDocumentContextParameter (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = ExchangedDocumentContextTypeSerializerState :: GuidelineSpecifiedDocumentContextParameter (WithSerializer :: serializer (& self . value . guideline_specified_document_context_parameter , Some ("ram:GuidelineSpecifiedDocumentContextParameter") , false) ?) , } ExchangedDocumentContextTypeSerializerState :: GuidelineSpecifiedDocumentContextParameter (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = ExchangedDocumentContextTypeSerializerState :: End__ , } ExchangedDocumentContextTypeSerializerState :: End__ => { * self . state = ExchangedDocumentContextTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } ExchangedDocumentContextTypeSerializerState :: Done__ => return Ok (None) , ExchangedDocumentContextTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for ExchangedDocumentContextTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExchangedDocumentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExchangedDocumentTypeSerializerState::Init__ => {
                        *self.state = ExchangedDocumentTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ExchangedDocumentTypeSerializerState::Id(x) => match x.next().transpose()? {
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
                    },
                    ExchangedDocumentTypeSerializerState::TypeCode(x) => {
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExchangedDocumentTypeSerializerState::End__,
                        }
                    }
                    ExchangedDocumentTypeSerializerState::End__ => {
                        *self.state = ExchangedDocumentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ExchangedDocumentTypeSerializerState::Done__ => return Ok(None),
                    ExchangedDocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ExchangedDocumentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { SupplyChainTradeTransactionTypeSerializerState :: Init__ => { * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeAgreement (WithSerializer :: serializer (& self . value . applicable_header_trade_agreement , Some ("ram:ApplicableHeaderTradeAgreement") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeAgreement (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeDelivery (WithSerializer :: serializer (& self . value . applicable_header_trade_delivery , Some ("ram:ApplicableHeaderTradeDelivery") , false) ?) , } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeDelivery (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeSettlement (WithSerializer :: serializer (& self . value . applicable_header_trade_settlement , Some ("ram:ApplicableHeaderTradeSettlement") , false) ?) , } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeSettlement (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: End__ , } SupplyChainTradeTransactionTypeSerializerState :: End__ => { * self . state = SupplyChainTradeTransactionTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } SupplyChainTradeTransactionTypeSerializerState :: Done__ => return Ok (None) , SupplyChainTradeTransactionTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for SupplyChainTradeTransactionTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentContextParameterTypeSerializerState::Init__ => {
                        *self.state = DocumentContextParameterTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentContextParameterTypeSerializerState::Id(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentContextParameterTypeSerializerState::End__,
                    },
                    DocumentContextParameterTypeSerializerState::End__ => {
                        *self.state = DocumentContextParameterTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentContextParameterTypeSerializerState::Done__ => return Ok(None),
                    DocumentContextParameterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentContextParameterTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IdTypeSerializerState::Init__ => {
                        *self.state = IdTypeSerializerState::Content__(WithSerializer::serializer(
                            &self.value.content,
                            None,
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib_opt(&mut bytes, "udt:schemeID", &self.value.scheme_id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    IdTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = IdTypeSerializerState::End__,
                    },
                    IdTypeSerializerState::End__ => {
                        *self.state = IdTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    IdTypeSerializerState::Done__ => return Ok(None),
                    IdTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for IdTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentCodeTypeSerializerState::Init__ => {
                        *self.state = DocumentCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentCodeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentCodeTypeSerializerState::End__,
                    },
                    DocumentCodeTypeSerializerState::End__ => {
                        *self.state = DocumentCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentCodeTypeSerializerState::Done__ => return Ok(None),
                    DocumentCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeSerializerState::Init__ => {
                        *self.state = DateTimeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DateTimeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DateTimeTypeSerializerState::End__,
                    },
                    DateTimeTypeSerializerState::End__ => {
                        *self.state = DateTimeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DateTimeTypeSerializerState::Done__ => return Ok(None),
                    DateTimeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DateTimeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for DateTimeTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        BuyerOrderReferencedDocument(
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    HeaderTradeAgreementTypeSerializerState::Init__ => {
                        *self.state = HeaderTradeAgreementTypeSerializerState::BuyerReference(
                            IterSerializer::new(
                                self.value.buyer_reference.as_ref(),
                                Some("ram:BuyerReference"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    HeaderTradeAgreementTypeSerializerState::BuyerReference(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    HeaderTradeAgreementTypeSerializerState::SellerTradeParty(
                                        WithSerializer::serializer(
                                            &self.value.seller_trade_party,
                                            Some("ram:SellerTradeParty"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    HeaderTradeAgreementTypeSerializerState::SellerTradeParty(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    HeaderTradeAgreementTypeSerializerState::BuyerTradeParty(
                                        WithSerializer::serializer(
                                            &self.value.buyer_trade_party,
                                            Some("ram:BuyerTradeParty"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    HeaderTradeAgreementTypeSerializerState::BuyerTradeParty(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state =
                            HeaderTradeAgreementTypeSerializerState::BuyerOrderReferencedDocument(
                                IterSerializer::new(
                                    self.value.buyer_order_referenced_document.as_ref(),
                                    Some("ram:BuyerOrderReferencedDocument"),
                                    false,
                                ),
                            ),
                    },
                    HeaderTradeAgreementTypeSerializerState::BuyerOrderReferencedDocument(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = HeaderTradeAgreementTypeSerializerState::End__,
                        }
                    }
                    HeaderTradeAgreementTypeSerializerState::End__ => {
                        *self.state = HeaderTradeAgreementTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    HeaderTradeAgreementTypeSerializerState::Done__ => return Ok(None),
                    HeaderTradeAgreementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for HeaderTradeAgreementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeDeliveryTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    HeaderTradeDeliveryTypeSerializerState::Init__ => {
                        *self.state = HeaderTradeDeliveryTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    HeaderTradeDeliveryTypeSerializerState::Done__ => return Ok(None),
                    HeaderTradeDeliveryTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for HeaderTradeDeliveryTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        InvoiceCurrencyCode(<super::CurrencyCodeType as WithSerializer>::Serializer<'ser>),
        SpecifiedTradeSettlementHeaderMonetarySummation(
            <super::TradeSettlementHeaderMonetarySummationType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeSettlementTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeSettlementTypeSerializerState :: Init__ => { * self . state = HeaderTradeSettlementTypeSerializerState :: InvoiceCurrencyCode (WithSerializer :: serializer (& self . value . invoice_currency_code , Some ("ram:InvoiceCurrencyCode") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeSettlementTypeSerializerState :: InvoiceCurrencyCode (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (WithSerializer :: serializer (& self . value . specified_trade_settlement_header_monetary_summation , Some ("ram:SpecifiedTradeSettlementHeaderMonetarySummation") , false) ?) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: End__ , } HeaderTradeSettlementTypeSerializerState :: End__ => { * self . state = HeaderTradeSettlementTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeSettlementTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeSettlementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for HeaderTradeSettlementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeDateTimeStringTypeSerializerState::Init__ => {
                        *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib(&mut bytes, "udt:format", &self.value.format)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DateTimeTypeDateTimeStringTypeSerializerState::Content__(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DateTimeTypeDateTimeStringTypeSerializerState::End__,
                    },
                    DateTimeTypeDateTimeStringTypeSerializerState::End__ => {
                        *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DateTimeTypeDateTimeStringTypeSerializerState::Done__ => return Ok(None),
                    DateTimeTypeDateTimeStringTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TextTypeSerializerState::Init__ => {
                        *self.state = TextTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TextTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TextTypeSerializerState::End__,
                    },
                    TextTypeSerializerState::End__ => {
                        *self.state = TextTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TextTypeSerializerState::Done__ => return Ok(None),
                    TextTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TextTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
    pub struct TradePartyTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradePartyType,
        pub(super) state: Box<TradePartyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradePartyTypeSerializerState<'ser> {
        Init__,
        Name(<super::TextType as WithSerializer>::Serializer<'ser>),
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
        SpecifiedTaxRegistration(
            IterSerializer<'ser, &'ser [super::TaxRegistrationType], super::TaxRegistrationType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradePartyTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradePartyTypeSerializerState::Init__ => {
                        *self.state = TradePartyTypeSerializerState::Name(
                            WithSerializer::serializer(&self.value.name, Some("ram:Name"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradePartyTypeSerializerState::Name(x) => match x.next().transpose()? {
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
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradePartyTypeSerializerState::End__,
                        }
                    }
                    TradePartyTypeSerializerState::End__ => {
                        *self.state = TradePartyTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradePartyTypeSerializerState::Done__ => return Ok(None),
                    TradePartyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradePartyTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ReferencedDocumentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ReferencedDocumentTypeSerializerState::IssuerAssignedId(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ReferencedDocumentTypeSerializerState::End__,
                        }
                    }
                    ReferencedDocumentTypeSerializerState::End__ => {
                        *self.state = ReferencedDocumentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ReferencedDocumentTypeSerializerState::Done__ => return Ok(None),
                    ReferencedDocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ReferencedDocumentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CurrencyCodeTypeSerializerState::Init__ => {
                        *self.state = CurrencyCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CurrencyCodeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CurrencyCodeTypeSerializerState::End__,
                    },
                    CurrencyCodeTypeSerializerState::End__ => {
                        *self.state = CurrencyCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CurrencyCodeTypeSerializerState::Done__ => return Ok(None),
                    CurrencyCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CurrencyCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
    pub struct TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeSettlementHeaderMonetarySummationType,
        pub(super) state: Box<TradeSettlementHeaderMonetarySummationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeSettlementHeaderMonetarySummationTypeSerializerState<'ser> {
        Init__,
        TaxBasisTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        TaxTotalAmount(IterSerializer<'ser, &'ser [super::AmountType], super::AmountType>),
        GrandTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        DuePayableAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { TradeSettlementHeaderMonetarySummationTypeSerializerState :: Init__ => { * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxBasisTotalAmount (WithSerializer :: serializer (& self . value . tax_basis_total_amount , Some ("ram:TaxBasisTotalAmount") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxBasisTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxTotalAmount (IterSerializer :: new (& self . value . tax_total_amount [..] , Some ("ram:TaxTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: GrandTotalAmount (WithSerializer :: serializer (& self . value . grand_total_amount , Some ("ram:GrandTotalAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: GrandTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: DuePayableAmount (WithSerializer :: serializer (& self . value . due_payable_amount , Some ("ram:DuePayableAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: DuePayableAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: End__ , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: End__ => { * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } TradeSettlementHeaderMonetarySummationTypeSerializerState :: Done__ => return Ok (None) , TradeSettlementHeaderMonetarySummationTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> LegalOrganizationTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    LegalOrganizationTypeSerializerState::Init__ => {
                        *self.state = LegalOrganizationTypeSerializerState::Id(
                            IterSerializer::new(self.value.id.as_ref(), Some("ram:ID"), false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    LegalOrganizationTypeSerializerState::Id(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = LegalOrganizationTypeSerializerState::End__,
                    },
                    LegalOrganizationTypeSerializerState::End__ => {
                        *self.state = LegalOrganizationTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    LegalOrganizationTypeSerializerState::Done__ => return Ok(None),
                    LegalOrganizationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for LegalOrganizationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        CountryId(<super::CountryIdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAddressTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAddressTypeSerializerState::Init__ => {
                        *self.state =
                            TradeAddressTypeSerializerState::CountryId(WithSerializer::serializer(
                                &self.value.country_id,
                                Some("ram:CountryID"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAddressTypeSerializerState::CountryId(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TradeAddressTypeSerializerState::End__,
                    },
                    TradeAddressTypeSerializerState::End__ => {
                        *self.state = TradeAddressTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAddressTypeSerializerState::Done__ => return Ok(None),
                    TradeAddressTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradeAddressTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxRegistrationTypeSerializerState::Init__ => {
                        *self.state = TaxRegistrationTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxRegistrationTypeSerializerState::Id(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TaxRegistrationTypeSerializerState::End__,
                    },
                    TaxRegistrationTypeSerializerState::End__ => {
                        *self.state = TaxRegistrationTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxRegistrationTypeSerializerState::Done__ => return Ok(None),
                    TaxRegistrationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TaxRegistrationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AmountTypeSerializerState::Init__ => {
                        *self.state = AmountTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib_opt(&mut bytes, "udt:currencyID", &self.value.currency_id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AmountTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AmountTypeSerializerState::End__,
                    },
                    AmountTypeSerializerState::End__ => {
                        *self.state = AmountTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AmountTypeSerializerState::Done__ => return Ok(None),
                    AmountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for AmountTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CountryIdTypeSerializerState::Init__ => {
                        *self.state = CountryIdTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CountryIdTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CountryIdTypeSerializerState::End__,
                    },
                    CountryIdTypeSerializerState::End__ => {
                        *self.state = CountryIdTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CountryIdTypeSerializerState::Done__ => return Ok(None),
                    CountryIdTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CountryIdTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CountryIdTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
