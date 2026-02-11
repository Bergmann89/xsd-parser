use core::{ops::Deref, str::from_utf8};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        fraction_digits, whitespace_collapse, DeserializeBytes, DeserializeHelper, Error,
        SerializeBytes, SerializeHelper, ValidateError, WithDeserializer, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub content: Vec<RootTypeContent>,
}
#[derive(Debug)]
pub enum RootTypeContent {
    NegativeDecimal(NegativeDecimalType),
    PositiveDecimal(PositiveDecimalType),
    RestrictedString(RestrictedStringType),
}
impl WithSerializer for RootType {
    type Serializer<'x> = quick_xml_serialize::RootTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
            name: name.unwrap_or("Root"),
            is_root,
        })
    }
}
impl WithSerializer for RootTypeContent {
    type Serializer<'x> = quick_xml_serialize::RootTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::RootTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for RootType {
    type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
}
impl WithDeserializer for RootTypeContent {
    type Deserializer = quick_xml_deserialize::RootTypeContentDeserializer;
}
#[derive(Debug)]
pub struct NegativeDecimalType(pub f64);
impl NegativeDecimalType {
    pub fn new(inner: f64) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> f64 {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        fraction_digits(s, 2usize)?;
        Ok(())
    }
    pub fn validate_value(value: &f64) -> Result<(), ValidateError> {
        if *value < -999999999.99f64 {
            return Err(ValidateError::LessThan("-999999999.99"));
        }
        if *value >= 0f64 {
            return Err(ValidateError::GraterEqualThan("0"));
        }
        Ok(())
    }
}
impl From<NegativeDecimalType> for f64 {
    fn from(value: NegativeDecimalType) -> f64 {
        value.0
    }
}
impl TryFrom<f64> for NegativeDecimalType {
    type Error = ValidateError;
    fn try_from(value: f64) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for NegativeDecimalType {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for NegativeDecimalType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        let Self(inner) = self;
        Ok(Some(Cow::Owned(format!("{inner:.02}"))))
    }
}
impl DeserializeBytes for NegativeDecimalType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = f64::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
#[derive(Debug)]
pub struct PositiveDecimalType(pub f64);
impl PositiveDecimalType {
    pub fn new(inner: f64) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> f64 {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        fraction_digits(s, 2usize)?;
        Ok(())
    }
    pub fn validate_value(value: &f64) -> Result<(), ValidateError> {
        if *value <= 0f64 {
            return Err(ValidateError::LessEqualThan("0"));
        }
        if *value > 999999999.99f64 {
            return Err(ValidateError::GraterThan("999999999.99"));
        }
        Ok(())
    }
}
impl From<PositiveDecimalType> for f64 {
    fn from(value: PositiveDecimalType) -> f64 {
        value.0
    }
}
impl TryFrom<f64> for PositiveDecimalType {
    type Error = ValidateError;
    fn try_from(value: f64) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for PositiveDecimalType {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for PositiveDecimalType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        let Self(inner) = self;
        Ok(Some(Cow::Owned(format!("{inner:.02}"))))
    }
}
impl DeserializeBytes for PositiveDecimalType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = f64::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
#[derive(Debug)]
pub struct RestrictedStringType(pub String);
impl RestrictedStringType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 1usize]> = LazyLock::new(|| {
            [(
                "[A-Z][a-z]{4,9}",
                Regex::new("^(?:[A-Z][a-z]{4,9})$").unwrap(),
            )]
        });
        for (pattern, regex) in PATTERNS.iter() {
            if !regex.is_match(s) {
                return Err(ValidateError::Pattern(pattern));
            }
        }
        Ok(())
    }
    pub fn validate_value(value: &String) -> Result<(), ValidateError> {
        if value.len() < 5usize {
            return Err(ValidateError::MinLength(5usize));
        }
        if value.len() > 10usize {
            return Err(ValidateError::MaxLength(10usize));
        }
        Ok(())
    }
}
impl From<RestrictedStringType> for String {
    fn from(value: RestrictedStringType) -> String {
        value.0
    }
}
impl TryFrom<String> for RestrictedStringType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for RestrictedStringType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for RestrictedStringType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl DeserializeBytes for RestrictedStringType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        let buffer = whitespace_collapse(s);
        let s = buffer.trim();
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct RootTypeDeserializer {
        content: Vec<super::RootTypeContent>,
        state__: Box<RootTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RootTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RootTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RootTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(RootTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RootTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RootTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RootTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RootTypeContent>,
            fallback: &mut Option<RootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeDeserializerState as S;
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
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RootType> for RootTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType> {
            use RootTypeDeserializerState as S;
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
                            <super::RootTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RootType, Error> {
            let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::RootType {
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RootTypeContentDeserializer {
        state__: Box<RootTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RootTypeContentDeserializerState {
        Init__,
        NegativeDecimal(
            Option<super::NegativeDecimalType>,
            Option<<super::NegativeDecimalType as WithDeserializer>::Deserializer>,
            Option<<super::NegativeDecimalType as WithDeserializer>::Deserializer>,
        ),
        PositiveDecimal(
            Option<super::PositiveDecimalType>,
            Option<<super::PositiveDecimalType as WithDeserializer>::Deserializer>,
            Option<<super::PositiveDecimalType as WithDeserializer>::Deserializer>,
        ),
        RestrictedString(
            Option<super::RestrictedStringType>,
            Option<<super::RestrictedStringType as WithDeserializer>::Deserializer>,
            Option<<super::RestrictedStringType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RootTypeContent),
        Unknown__,
    }
    impl RootTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"NegativeDecimal")
                ) {
                    let output =
                        <super::NegativeDecimalType as WithDeserializer>::init(helper, event)?;
                    return self.handle_negative_decimal(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"PositiveDecimal")
                ) {
                    let output =
                        <super::PositiveDecimalType as WithDeserializer>::init(helper, event)?;
                    return self.handle_positive_decimal(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"RestrictedString")
                ) {
                    let output =
                        <super::RestrictedStringType as WithDeserializer>::init(helper, event)?;
                    return self.handle_restricted_string(helper, Default::default(), None, output);
                }
            }
            *self.state__ = RootTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RootTypeContentDeserializerState,
        ) -> Result<super::RootTypeContent, Error> {
            use RootTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::NegativeDecimal(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_negative_decimal(&mut values, value)?;
                    }
                    Ok(super::RootTypeContent::NegativeDecimal(
                        helper.finish_element("NegativeDecimal", values)?,
                    ))
                }
                S::PositiveDecimal(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_positive_decimal(&mut values, value)?;
                    }
                    Ok(super::RootTypeContent::PositiveDecimal(
                        helper.finish_element("PositiveDecimal", values)?,
                    ))
                }
                S::RestrictedString(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_restricted_string(&mut values, value)?;
                    }
                    Ok(super::RootTypeContent::RestrictedString(
                        helper.finish_element("RestrictedString", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_negative_decimal(
            values: &mut Option<super::NegativeDecimalType>,
            value: super::NegativeDecimalType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"NegativeDecimal",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_positive_decimal(
            values: &mut Option<super::PositiveDecimalType>,
            value: super::PositiveDecimalType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PositiveDecimal",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_restricted_string(
            values: &mut Option<super::RestrictedStringType>,
            value: super::RestrictedStringType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RestrictedString",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_negative_decimal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::NegativeDecimalType>,
            fallback: Option<<super::NegativeDecimalType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::NegativeDecimalType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeContentDeserializerState as S;
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
                Self::store_negative_decimal(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_negative_decimal(&mut values, data)?;
                    let data = Self::finish_state(helper, S::NegativeDecimal(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::NegativeDecimal(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_positive_decimal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PositiveDecimalType>,
            fallback: Option<<super::PositiveDecimalType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PositiveDecimalType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeContentDeserializerState as S;
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
                Self::store_positive_decimal(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_positive_decimal(&mut values, data)?;
                    let data = Self::finish_state(helper, S::PositiveDecimal(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::PositiveDecimal(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_restricted_string<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RestrictedStringType>,
            fallback: Option<<super::RestrictedStringType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RestrictedStringType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeContentDeserializerState as S;
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
                Self::store_restricted_string(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restricted_string(&mut values, data)?;
                    let data = Self::finish_state(helper, S::RestrictedString(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::RestrictedString(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RootTypeContent> for RootTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootTypeContent> {
            let deserializer = Self {
                state__: Box::new(RootTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RootTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::RootTypeContent> {
            use RootTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::NegativeDecimal(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_negative_decimal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::PositiveDecimal(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_positive_decimal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RestrictedString(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_restricted_string(helper, values, fallback, output)? {
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
                        S::NegativeDecimal(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"NegativeDecimal",
                            false,
                        )?;
                        match self.handle_negative_decimal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::PositiveDecimal(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"PositiveDecimal",
                            false,
                        )?;
                        match self.handle_positive_decimal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::RestrictedString(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"RestrictedString",
                            false,
                        )?;
                        match self.handle_restricted_string(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::RootTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct RootTypeSerializer<'ser> {
        pub(super) value: &'ser super::RootType,
        pub(super) state: Box<RootTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RootTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::RootTypeContent], super::RootTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeSerializerState::Init__ => {
                        *self.state = RootTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RootTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RootTypeSerializerState::End__,
                    },
                    RootTypeSerializerState::End__ => {
                        *self.state = RootTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RootTypeSerializerState::Done__ => return Ok(None),
                    RootTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RootTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RootTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::RootTypeContent,
        pub(super) state: Box<RootTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum RootTypeContentSerializerState<'ser> {
        Init__,
        NegativeDecimal(<super::NegativeDecimalType as WithSerializer>::Serializer<'ser>),
        PositiveDecimal(<super::PositiveDecimalType as WithSerializer>::Serializer<'ser>),
        RestrictedString(<super::RestrictedStringType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeContentSerializerState::Init__ => match self.value {
                        super::RootTypeContent::NegativeDecimal(x) => {
                            *self.state = RootTypeContentSerializerState::NegativeDecimal(
                                WithSerializer::serializer(x, Some("NegativeDecimal"), false)?,
                            )
                        }
                        super::RootTypeContent::PositiveDecimal(x) => {
                            *self.state = RootTypeContentSerializerState::PositiveDecimal(
                                WithSerializer::serializer(x, Some("PositiveDecimal"), false)?,
                            )
                        }
                        super::RootTypeContent::RestrictedString(x) => {
                            *self.state = RootTypeContentSerializerState::RestrictedString(
                                WithSerializer::serializer(x, Some("RestrictedString"), false)?,
                            )
                        }
                    },
                    RootTypeContentSerializerState::NegativeDecimal(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeContentSerializerState::Done__,
                        }
                    }
                    RootTypeContentSerializerState::PositiveDecimal(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeContentSerializerState::Done__,
                        }
                    }
                    RootTypeContentSerializerState::RestrictedString(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeContentSerializerState::Done__,
                        }
                    }
                    RootTypeContentSerializerState::Done__ => return Ok(None),
                    RootTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RootTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
