pub const NS_XS: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub type AsMut = TestType;
pub type AsRef = TestType;
pub type Box = TestType;
pub type Clone = TestType;
pub type Copy = TestType;
pub type Debug = TestType;
pub type Default = TestType;
pub type Drop = TestType;
pub type Eq = TestType;
pub type Fn = TestType;
pub type FnMut = TestType;
pub type FnOnce = TestType;
pub type From = TestType;
pub type FromIterator = TestType;
pub type Into = TestType;
pub type IntoIterator = TestType;
pub type Iterator = TestType;
pub type Option = TestType;
pub type Ord = TestType;
pub type PartialEq = TestType;
pub type PartialOrd = TestType;
pub type Result = TestType;
pub type Send = TestType;
pub type Sized = TestType;
pub type String = TestType;
pub type Sync = TestType;
#[derive(Debug)]
pub struct TestType(pub ::std::string::String);
impl TestType {
    pub fn new(
        inner: ::std::string::String,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::std::string::String {
        self.0
    }
    pub fn validate_value(
        value: &::std::string::String,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if !value.is_empty() {
            return Err(::xsd_parser_types::quick_xml::ValidateError::MaxLength(
                0usize,
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<TestType> for ::std::string::String {
    fn from(value: TestType) -> ::std::string::String {
        value.0
    }
}
impl ::core::convert::TryFrom<::std::string::String> for TestType {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for TestType {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TestType {
    fn serialize_bytes(
        &self,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        self.0.serialize_bytes()
    }
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TestType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::std::string::String::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
pub type ToString = TestType;
pub type TryFrom = TestType;
pub type TryInto = TestType;
pub type Unpin = TestType;
pub type Vec = TestType;
#[derive(Debug, Default)]
pub struct EntitiesType(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for EntitiesType {
    fn serialize_bytes(
        &self,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EntitiesType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| ::std::string::String::deserialize_bytes(helper, bytes))
                .collect::<::core::result::Result<::std::vec::Vec<_>, _>>()?,
        ))
    }
}
#[derive(Debug, Default)]
pub struct EntityType(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for EntityType {
    fn serialize_bytes(
        &self,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EntityType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| ::std::string::String::deserialize_bytes(helper, bytes))
                .collect::<::core::result::Result<::std::vec::Vec<_>, _>>()?,
        ))
    }
}
pub type IdType = ::std::string::String;
pub type IdrefType = ::std::string::String;
#[derive(Debug, Default)]
pub struct IdrefsType(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for IdrefsType {
    fn serialize_bytes(
        &self,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for IdrefsType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| ::std::string::String::deserialize_bytes(helper, bytes))
                .collect::<::core::result::Result<::std::vec::Vec<_>, _>>()?,
        ))
    }
}
pub type NcNameType = ::std::string::String;
pub type NmtokenType = ::std::string::String;
#[derive(Debug, Default)]
pub struct NmtokensType(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for NmtokensType {
    fn serialize_bytes(
        &self,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for NmtokensType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| ::std::string::String::deserialize_bytes(helper, bytes))
                .collect::<::core::result::Result<::std::vec::Vec<_>, _>>()?,
        ))
    }
}
pub type NotationType = ::std::string::String;
pub type NameType = ::std::string::String;
pub type QNameType = ::std::string::String;
pub type AnySimpleType = ::std::string::String;
#[derive(Debug)]
pub struct AnyType;
impl ::xsd_parser_types::quick_xml::WithSerializer for AnyType {
    type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser str>,
        is_root: bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::AnyTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(quick_xml_serialize::AnyTypeSerializerState::Init__),
            name: name.unwrap_or("anyType"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for AnyType {
    type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
}
pub type AnyUriType = ::std::string::String;
pub type Base64BinaryType = ::std::string::String;
pub type BooleanType = ::core::primitive::bool;
pub type ByteType = ::core::primitive::i8;
pub type DateType = ::std::string::String;
pub type DateTimeType = ::std::string::String;
pub type DecimalType = ::core::primitive::f64;
pub type DoubleType = ::core::primitive::f64;
pub type DurationType = ::std::string::String;
pub type FloatType = ::core::primitive::f32;
pub type GDayType = ::std::string::String;
pub type GMonthType = ::std::string::String;
pub type GMonthDayType = ::std::string::String;
pub type GYearType = ::std::string::String;
pub type GYearMonthType = ::std::string::String;
pub type HexBinaryType = ::std::string::String;
pub type IntType = ::core::primitive::i32;
pub type IntegerType = ::core::primitive::i32;
pub type LanguageType = ::std::string::String;
pub type LongType = ::core::primitive::i64;
pub type NegativeIntegerType = ::core::primitive::isize;
pub type NonNegativeIntegerType = ::core::primitive::usize;
pub type NonPositiveIntegerType = ::core::primitive::isize;
pub type NormalizedStringType = ::std::string::String;
pub type PositiveIntegerType = ::core::primitive::usize;
pub type ShortType = ::core::primitive::i16;
pub type StringType = ::std::string::String;
pub type TimeType = ::std::string::String;
pub type TokenType = ::std::string::String;
pub type UnsignedByteType = ::core::primitive::u8;
pub type UnsignedIntType = ::core::primitive::u32;
pub type UnsignedLongType = ::core::primitive::u64;
pub type UnsignedShortType = ::core::primitive::u16;
pub mod quick_xml_deserialize {
    use xsd_parser_types::quick_xml::Deserializer as _;
    #[derive(Debug)]
    pub struct AnyTypeDeserializer {
        state__: ::std::boxed::Box<AnyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            Ok(Self {
                state__: ::std::boxed::Box::new(AnyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: AnyTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            Ok(())
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnyType> {
            if let ::xsd_parser_types::quick_xml::Event::End(_) = &event {
                Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                        self.finish(helper)?,
                    ),
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                    allow_any: false,
                })
            } else if matches!(
                &event,
                ::xsd_parser_types::quick_xml::Event::Text(_)
                    | ::xsd_parser_types::quick_xml::Event::CData(_)
            ) {
                Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                        self,
                    ),
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                    allow_any: true,
                })
            } else {
                Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                        self,
                    ),
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                    allow_any: true,
                })
            }
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::AnyType, ::xsd_parser_types::quick_xml::Error> {
            let state =
                ::core::mem::replace(&mut *self.state__, AnyTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnyType {})
        }
    }
}
pub mod quick_xml_serialize {
    #[derive(Debug)]
    pub struct AnyTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnyType,
        pub(super) state: ::std::boxed::Box<AnyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnyTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    AnyTypeSerializerState::Init__ => {
                        *self.state = AnyTypeSerializerState::Done__;
                        let bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Empty(bytes)));
                    }
                    AnyTypeSerializerState::Done__ => return Ok(None),
                    AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::core::iter::Iterator for AnyTypeSerializer<'ser> {
        type Item = ::core::result::Result<
            ::xsd_parser_types::quick_xml::Event<'ser>,
            ::xsd_parser_types::quick_xml::Error,
        >;
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
