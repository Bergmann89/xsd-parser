use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: Namespace = Namespace::new_const(b"urn:example:minimal");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub type Amount = CurrencyAmountType;
#[derive(Debug)]
pub struct CurrencyAmountType {
    pub ccy: String,
    pub content: Decimal,
}
impl WithSerializer for CurrencyAmountType {
    type Serializer<'x> = quick_xml_serialize::CurrencyAmountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CurrencyAmountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CurrencyAmountTypeSerializerState::Init__),
            name: name.unwrap_or("CurrencyAmount"),
            is_root,
        })
    }
}
impl WithDeserializer for CurrencyAmountType {
    type Deserializer = quick_xml_deserialize::CurrencyAmountTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
        WithDeserializer,
    };
    #[derive(Debug)]
    pub struct CurrencyAmountTypeDeserializer {
        ccy: String,
        content: Option<super::Decimal>,
        state__: Box<CurrencyAmountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CurrencyAmountTypeDeserializerState {
        Init__,
        Content__(<super::Decimal as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CurrencyAmountTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut ccy: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"Ccy")
                ) {
                    helper.read_attrib(&mut ccy, b"Ccy", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                ccy: ccy.ok_or_else(|| ErrorKind::MissingAttribute("Ccy".into()))?,
                content: None,
                state__: Box::new(CurrencyAmountTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CurrencyAmountTypeDeserializerState,
        ) -> Result<(), Error> {
            if let CurrencyAmountTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::Decimal) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Decimal>,
        ) -> DeserializerResult<'de, super::CurrencyAmountType> {
            use CurrencyAmountTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::CurrencyAmountType> for CurrencyAmountTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyAmountType> {
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
        ) -> DeserializerResult<'de, super::CurrencyAmountType> {
            use CurrencyAmountTypeDeserializerState as S;
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
        ) -> Result<super::CurrencyAmountType, Error> {
            let state = replace(
                &mut *self.state__,
                CurrencyAmountTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CurrencyAmountType {
                ccy: self.ccy,
                content: helper.finish_content(self.content)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct CurrencyAmountTypeSerializer<'ser> {
        pub(super) value: &'ser super::CurrencyAmountType,
        pub(super) state: Box<CurrencyAmountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CurrencyAmountTypeSerializerState<'ser> {
        Init__,
        Content__(<super::Decimal as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CurrencyAmountTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CurrencyAmountTypeSerializerState::Init__ => {
                        *self.state = CurrencyAmountTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        helper.write_attrib(&mut bytes, "Ccy", &self.value.ccy)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CurrencyAmountTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CurrencyAmountTypeSerializerState::End__,
                        }
                    }
                    CurrencyAmountTypeSerializerState::End__ => {
                        *self.state = CurrencyAmountTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CurrencyAmountTypeSerializerState::Done__ => return Ok(None),
                    CurrencyAmountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CurrencyAmountTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CurrencyAmountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
