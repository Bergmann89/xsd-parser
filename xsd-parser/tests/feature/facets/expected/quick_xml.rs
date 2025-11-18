use core::{ops::Deref, str::from_utf8};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};
use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{
        fraction_digits, whitespace_collapse, DeserializeBytes, DeserializeReader, Error,
        SerializeBytes, ValidateError, WithDeserializer, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
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
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        let Self(inner) = self;
        Ok(Some(Cow::Owned(format!("{inner:.02}"))))
    }
}
impl DeserializeBytes for NegativeDecimalType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = f64::deserialize_str(reader, s)?;
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
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        let Self(inner) = self;
        Ok(Some(Cow::Owned(format!("{inner:.02}"))))
    }
}
impl DeserializeBytes for PositiveDecimalType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = f64::deserialize_str(reader, s)?;
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
        static PATTERNS: LazyLock<[Regex; 1usize]> =
            LazyLock::new(|| [Regex::new("[A-Z][a-z]{4,9}").unwrap()]);
        for pattern in PATTERNS.iter() {
            if !pattern.is_match(s) {
                return Err(ValidateError::Pattern(pattern.as_str()));
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
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes()
    }
}
impl DeserializeBytes for RestrictedStringType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        let s = from_utf8(bytes).map_err(Error::from)?;
        let buffer = whitespace_collapse(s);
        let s = buffer.trim();
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(reader, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(RootTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RootTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RootTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RootTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RootTypeContent>,
            fallback: &mut Option<RootTypeDeserializerState>,
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
                *self.state__ = fallback.take().unwrap_or(RootTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = RootTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = RootTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(RootTypeDeserializerState::Content__(deserializer));
                            *self.state__ = RootTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RootType> for RootTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RootType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType>
        where
            R: DeserializeReader,
        {
            use RootTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                            <super::RootTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RootType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::RootType {
                content: self.content,
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
        ),
        PositiveDecimal(
            Option<super::PositiveDecimalType>,
            Option<<super::PositiveDecimalType as WithDeserializer>::Deserializer>,
        ),
        RestrictedString(
            Option<super::RestrictedStringType>,
            Option<<super::RestrictedStringType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RootTypeContent),
        Unknown__,
    }
    impl RootTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RootTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"NegativeDecimal")
                ) {
                    let output =
                        <super::NegativeDecimalType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_negative_decimal(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"PositiveDecimal")
                ) {
                    let output =
                        <super::PositiveDecimalType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_positive_decimal(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"RestrictedString")
                ) {
                    let output =
                        <super::RestrictedStringType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_restricted_string(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(RootTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: RootTypeContentDeserializerState,
        ) -> Result<super::RootTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use RootTypeContentDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::NegativeDecimal(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_negative_decimal(&mut values, value)?;
                    }
                    Ok(super::RootTypeContent::NegativeDecimal(values.ok_or_else(
                        || ErrorKind::MissingElement("NegativeDecimal".into()),
                    )?))
                }
                S::PositiveDecimal(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_positive_decimal(&mut values, value)?;
                    }
                    Ok(super::RootTypeContent::PositiveDecimal(values.ok_or_else(
                        || ErrorKind::MissingElement("PositiveDecimal".into()),
                    )?))
                }
                S::RestrictedString(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_restricted_string(&mut values, value)?;
                    }
                    Ok(super::RootTypeContent::RestrictedString(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("RestrictedString".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
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
        fn handle_negative_decimal<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::NegativeDecimalType>,
            output: DeserializerOutput<'de, super::NegativeDecimalType>,
            fallback: &mut Option<RootTypeContentDeserializerState>,
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
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = RootTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => RootTypeContentDeserializerState::NegativeDecimal(values, None),
                    Some(RootTypeContentDeserializerState::NegativeDecimal(
                        _,
                        Some(deserializer),
                    )) => RootTypeContentDeserializerState::NegativeDecimal(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RootTypeContentDeserializerState::NegativeDecimal(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_negative_decimal(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_negative_decimal(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RootTypeContentDeserializerState::NegativeDecimal(values, None),
                    )?;
                    *self.state__ = RootTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = RootTypeContentDeserializerState::NegativeDecimal(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_positive_decimal<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PositiveDecimalType>,
            output: DeserializerOutput<'de, super::PositiveDecimalType>,
            fallback: &mut Option<RootTypeContentDeserializerState>,
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
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = RootTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => RootTypeContentDeserializerState::PositiveDecimal(values, None),
                    Some(RootTypeContentDeserializerState::PositiveDecimal(
                        _,
                        Some(deserializer),
                    )) => RootTypeContentDeserializerState::PositiveDecimal(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RootTypeContentDeserializerState::PositiveDecimal(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_positive_decimal(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_positive_decimal(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RootTypeContentDeserializerState::PositiveDecimal(values, None),
                    )?;
                    *self.state__ = RootTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = RootTypeContentDeserializerState::PositiveDecimal(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_restricted_string<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RestrictedStringType>,
            output: DeserializerOutput<'de, super::RestrictedStringType>,
            fallback: &mut Option<RootTypeContentDeserializerState>,
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
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = RootTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => RootTypeContentDeserializerState::RestrictedString(values, None),
                    Some(RootTypeContentDeserializerState::RestrictedString(
                        _,
                        Some(deserializer),
                    )) => RootTypeContentDeserializerState::RestrictedString(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RootTypeContentDeserializerState::RestrictedString(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_restricted_string(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_restricted_string(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RootTypeContentDeserializerState::RestrictedString(values, None),
                    )?;
                    *self.state__ = RootTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = RootTypeContentDeserializerState::RestrictedString(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RootTypeContent> for RootTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RootTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state__: Box::new(RootTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
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
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootTypeContent>
        where
            R: DeserializeReader,
        {
            use RootTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::NegativeDecimal(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_negative_decimal(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::PositiveDecimal(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_positive_decimal(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RestrictedString(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_restricted_string(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
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
                    (
                        S::NegativeDecimal(values, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"NegativeDecimal",
                            false,
                        )?;
                        match self.handle_negative_decimal(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::PositiveDecimal(values, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"PositiveDecimal",
                            false,
                        )?;
                        match self.handle_positive_decimal(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::RestrictedString(values, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"RestrictedString",
                            false,
                        )?;
                        match self.handle_restricted_string(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state__ = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
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
        fn finish<R>(self, reader: &R) -> Result<super::RootTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeSerializerState::Init__ => {
                        *self.state = RootTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RootTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for RootTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeContentSerializerState::Done__,
                        }
                    }
                    RootTypeContentSerializerState::PositiveDecimal(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeContentSerializerState::Done__,
                        }
                    }
                    RootTypeContentSerializerState::RestrictedString(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for RootTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
