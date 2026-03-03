#![allow(missing_docs)]

use std::borrow::Cow;
use std::fmt::Display;
use std::str::from_utf8;

#[cfg(feature = "num")]
use num::{BigInt, BigRational, BigUint};

#[cfg(feature = "quick-xml")]
use quick_xml::events::Event;

#[cfg(feature = "quick-xml")]
use crate::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind,
        SerializeHelper, Serializer, WithDeserializer, WithSerializer,
    },
};
use crate::{
    quick_xml::{DeserializeBytes, SerializeBytes},
    xml::NamespacesShared,
};

use super::QName;

#[cfg(feature = "num")]
pub type Decimal = BigRational;

#[cfg(not(feature = "num"))]
pub type Decimal = f64;

#[cfg(feature = "num")]
pub type Integer = BigInt;

#[cfg(not(feature = "num"))]
pub type Integer = isize;

#[cfg(feature = "num")]
pub type Unsigned = BigUint;

#[cfg(not(feature = "num"))]
pub type Unsigned = usize;

#[cfg(feature = "base64")]
pub type Base64Binary = Vec<u8>;

#[cfg(not(feature = "base64"))]
pub type Base64Binary = String;

/// Type that represents an `xs:anySimpleType` value.
#[derive(Debug, Clone)]
pub enum AnySimpleType {
    String(String),
    Boolean(bool),

    Float(f32),
    Double(f64),
    Decimal(Decimal),

    Duration(String),
    DateTime(String),
    Time(String),
    Date(String),
    GYearMonth(String),
    GYear(String),
    GMonthDay(String),
    GMonth(String),
    GDay(String),

    HexBinary(Vec<u8>),
    Base64Binary(Base64Binary),

    AnyURI(String),
    QName(QName),
    Notation(String),

    Integer(Integer),
    PositiveInteger(Unsigned),
    NegativeInteger(Integer),
    NonNegativeInteger(Unsigned),
    NonPositiveInteger(Integer),

    Long(i64),
    Int(i32),
    Short(i16),
    Byte(i8),

    UnsignedLong(u64),
    UnsignedInt(u32),
    UnsignedShort(u16),
    UnsignedByte(u8),

    NormalizedString(String),
    Token(String),
    Language(String),
    NmToken(String),
    Name(String),
    NcName(String),
    Id(String),
    IdRef(String),
    Entity(String),

    NmTokens(Vec<String>),
    IdRefs(Vec<String>),
    Entities(Vec<String>),

    Unknown {
        /// Type of the value stored in `content`, if it was specified in the XML.
        type_: Option<QName>,

        /// Actual content of the value, as a string.
        content: String,

        /// Namespaces that were in scope when the value was parsed, which may be
        /// needed to resolve the `type_` if it is present.
        namespaces: NamespacesShared<'static>,
    },
}

impl AnySimpleType {
    /// Converts the value to a string, which can be used for serialization.
    ///
    /// If the value is of a type that has a specific string representation
    /// (e.g. `boolean`, `integer`, etc.), it will be converted to that representation.
    /// Otherwise, the original string content will be returned.
    #[must_use]
    pub fn to_str(&self) -> Cow<'_, str> {
        use AnySimpleType as S;

        match self {
            S::Boolean(x) => Cow::Owned(x.to_string()),

            S::Float(x) => Cow::Owned(x.to_string()),
            S::Double(x) => Cow::Owned(x.to_string()),
            S::Decimal(x) => Cow::Owned(x.to_string()),

            S::String(x)
            | S::Duration(x)
            | S::DateTime(x)
            | S::Time(x)
            | S::Date(x)
            | S::GYearMonth(x)
            | S::GYear(x)
            | S::GMonthDay(x)
            | S::GMonth(x)
            | S::GDay(x)
            | S::AnyURI(x)
            | S::Notation(x)
            | S::NormalizedString(x)
            | S::Token(x)
            | S::Language(x)
            | S::NmToken(x)
            | S::Name(x)
            | S::NcName(x)
            | S::Id(x)
            | S::IdRef(x)
            | S::Entity(x)
            | S::Unknown { content: x, .. } => Cow::Borrowed(x),

            S::HexBinary(x) => Cow::Owned(x.iter().fold(String::new(), |mut s, b| {
                use std::fmt::Write;
                let _ = write!(s, "{b:02X}");
                s
            })),
            S::Base64Binary(x) => Cow::Owned(format_base64_binary(x)),
            S::QName(x) => Cow::Owned(x.to_string()),

            S::Integer(x) => Cow::Owned(x.to_string()),
            S::PositiveInteger(x) => Cow::Owned(x.to_string()),
            S::NegativeInteger(x) => Cow::Owned(x.to_string()),
            S::NonNegativeInteger(x) => Cow::Owned(x.to_string()),
            S::NonPositiveInteger(x) => Cow::Owned(x.to_string()),

            S::Long(x) => Cow::Owned(x.to_string()),
            S::Int(x) => Cow::Owned(x.to_string()),
            S::Short(x) => Cow::Owned(x.to_string()),
            S::Byte(x) => Cow::Owned(x.to_string()),

            S::UnsignedLong(x) => Cow::Owned(x.to_string()),
            S::UnsignedInt(x) => Cow::Owned(x.to_string()),
            S::UnsignedShort(x) => Cow::Owned(x.to_string()),
            S::UnsignedByte(x) => Cow::Owned(x.to_string()),

            S::NmTokens(x) => Cow::Owned(format_vec(x)),
            S::IdRefs(x) => Cow::Owned(format_vec(x)),
            S::Entities(x) => Cow::Owned(format_vec(x)),
        }
    }
}

#[cfg(feature = "quick-xml")]
impl WithSerializer for AnySimpleType {
    type Serializer<'x>
        = AnySimpleTypeSerializer<'x>
    where
        Self: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, crate::quick_xml::Error> {
        let _is_root = is_root;

        Ok(AnySimpleTypeSerializer::Begin {
            name: name.unwrap_or("Value"),
            value: self,
        })
    }
}

#[cfg(feature = "quick-xml")]
impl WithDeserializer for AnySimpleType {
    type Deserializer = AnySimpleTypeDeserializer;
}

impl SerializeBytes for AnySimpleType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        let _helper = helper;

        Ok(Some(self.to_str()))
    }
}

impl DeserializeBytes for AnySimpleType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Self::deserialize_str(helper, from_utf8(bytes)?)
    }

    fn deserialize_str(helper: &mut DeserializeHelper, s: &str) -> Result<Self, Error> {
        Ok(Self::Unknown {
            type_: None,
            content: s.to_string(),
            namespaces: helper.namespaces().clone(),
        })
    }
}

#[derive(Debug)]
#[cfg(feature = "quick-xml")]
pub enum AnySimpleTypeSerializer<'ser> {
    Begin {
        name: &'ser str,
        value: &'ser AnySimpleType,
    },
    Content {
        name: &'ser str,
        content: String,
    },
    End {
        name: &'ser str,
    },
    Done,
}

#[cfg(feature = "quick-xml")]
impl<'ser> Serializer<'ser> for AnySimpleTypeSerializer<'ser> {
    #[allow(clippy::too_many_lines)]
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
        use core::mem::replace;
        use quick_xml::{
            escape::escape,
            events::{BytesEnd, BytesStart, BytesText},
        };

        let mut fixed_ns = false;

        match replace(self, Self::Done) {
            Self::Begin { name, value } => {
                use AnySimpleType as S;

                let (type_, content): (Option<QName>, String) = match value {
                    S::String(x) => (Some(QName::from_bytes(b"xs:string")), x.to_string()),
                    S::Boolean(x) => (Some(QName::from_bytes(b"xs:boolean")), x.to_string()),

                    S::Float(x) => (Some(QName::from_bytes(b"xs:float")), x.to_string()),
                    S::Double(x) => (Some(QName::from_bytes(b"xs:double")), x.to_string()),
                    S::Decimal(x) => (Some(QName::from_bytes(b"xs:decimal")), x.to_string()),

                    S::Duration(x) => (Some(QName::from_bytes(b"xs:duration")), x.to_string()),
                    S::DateTime(x) => (Some(QName::from_bytes(b"xs:dateTime")), x.to_string()),
                    S::Time(x) => (Some(QName::from_bytes(b"xs:time")), x.to_string()),
                    S::Date(x) => (Some(QName::from_bytes(b"xs:date")), x.to_string()),
                    S::GYearMonth(x) => (Some(QName::from_bytes(b"xs:gYearMonth")), x.to_string()),
                    S::GYear(x) => (Some(QName::from_bytes(b"xs:gYear")), x.to_string()),
                    S::GMonthDay(x) => (Some(QName::from_bytes(b"xs:gMonthDay")), x.to_string()),
                    S::GMonth(x) => (Some(QName::from_bytes(b"xs:gMonth")), x.to_string()),
                    S::GDay(x) => (Some(QName::from_bytes(b"xs:gDay")), x.to_string()),

                    S::HexBinary(x) => (
                        Some(QName::from_bytes(b"xs:hexBinary")),
                        x.iter().fold(String::new(), |mut s, b| {
                            use std::fmt::Write;
                            let _ = write!(s, "{b:02X}");
                            s
                        }),
                    ),
                    S::Base64Binary(x) => (
                        Some(QName::from_bytes(b"xs:base64Binary")),
                        format_base64_binary(x),
                    ),

                    S::AnyURI(x) => (Some(QName::from_bytes(b"xs:anyURI")), x.to_string()),
                    S::QName(x) => (Some(QName::from_bytes(b"xs:QName")), x.to_string()),
                    S::Notation(x) => (Some(QName::from_bytes(b"xs:NOTATION")), x.to_string()),

                    S::Integer(x) => (Some(QName::from_bytes(b"xs:integer")), x.to_string()),
                    S::PositiveInteger(x) => (
                        Some(QName::from_bytes(b"xs:positiveInteger")),
                        x.to_string(),
                    ),
                    S::NegativeInteger(x) => (
                        Some(QName::from_bytes(b"xs:negativeInteger")),
                        x.to_string(),
                    ),
                    S::NonNegativeInteger(x) => (
                        Some(QName::from_bytes(b"xs:nonNegativeInteger")),
                        x.to_string(),
                    ),
                    S::NonPositiveInteger(x) => (
                        Some(QName::from_bytes(b"xs:nonPositiveInteger")),
                        x.to_string(),
                    ),

                    S::Long(x) => (Some(QName::from_bytes(b"xs:long")), x.to_string()),
                    S::Int(x) => (Some(QName::from_bytes(b"xs:int")), x.to_string()),
                    S::Short(x) => (Some(QName::from_bytes(b"xs:short")), x.to_string()),
                    S::Byte(x) => (Some(QName::from_bytes(b"xs:byte")), x.to_string()),

                    S::UnsignedLong(x) => {
                        (Some(QName::from_bytes(b"xs:unsignedLong")), x.to_string())
                    }
                    S::UnsignedInt(x) => {
                        (Some(QName::from_bytes(b"xs:unsignedInt")), x.to_string())
                    }
                    S::UnsignedShort(x) => {
                        (Some(QName::from_bytes(b"xs:unsignedShort")), x.to_string())
                    }
                    S::UnsignedByte(x) => {
                        (Some(QName::from_bytes(b"xs:unsignedByte")), x.to_string())
                    }

                    S::NormalizedString(x) => (
                        Some(QName::from_bytes(b"xs:normalizedString")),
                        x.to_string(),
                    ),
                    S::Token(x) => (Some(QName::from_bytes(b"xs:token")), x.to_string()),
                    S::Language(x) => (Some(QName::from_bytes(b"xs:language")), x.to_string()),
                    S::NmToken(x) => (Some(QName::from_bytes(b"xs:NMTOKEN")), x.to_string()),
                    S::Name(x) => (Some(QName::from_bytes(b"xs:Name")), x.to_string()),
                    S::NcName(x) => (Some(QName::from_bytes(b"xs:NCName")), x.to_string()),
                    S::Id(x) => (Some(QName::from_bytes(b"xs:ID")), x.to_string()),
                    S::IdRef(x) => (Some(QName::from_bytes(b"xs:IDREF")), x.to_string()),
                    S::Entity(x) => (Some(QName::from_bytes(b"xs:ENTITY")), x.to_string()),

                    S::NmTokens(x) => (Some(QName::from_bytes(b"xs:NMTOKENS")), format_vec(x)),
                    S::IdRefs(x) => (Some(QName::from_bytes(b"xs:IDREFS")), format_vec(x)),
                    S::Entities(x) => (Some(QName::from_bytes(b"xs:ENTITIES")), format_vec(x)),

                    S::Unknown {
                        type_,
                        content,
                        namespaces: _,
                    } => {
                        fixed_ns = true;

                        (type_.clone(), content.to_string())
                    }
                };

                let mut bytes = BytesStart::new(name);

                helper.begin_ns_scope();

                if let Some(mut type_) = type_ {
                    if !fixed_ns && type_.namespace().is_none() {
                        type_ = type_.with_namespace(Namespace::XS);
                    }

                    let type_ = helper.add_qname_namespace(&mut bytes, type_);

                    helper.write_xmlns(&mut bytes, Some(&NamespacePrefix::XSI), &Namespace::XSI);
                    bytes.push_attribute((&b"xsi:type"[..], type_.as_ref()));
                }

                *self = Self::Content { name, content };

                Some(Ok(Event::Start(bytes)))
            }
            Self::Content { name, content } => {
                *self = Self::End { name };

                Some(Ok(Event::Text(BytesText::from_escaped(escape(content)))))
            }
            Self::End { name } => Some(Ok(Event::End(BytesEnd::new(name)))),
            Self::Done => None,
        }
    }
}

#[derive(Debug)]
#[cfg(feature = "quick-xml")]
pub struct AnySimpleTypeDeserializer {
    type_: Option<String>,
    content: ContentDeserializer<String>,
}

#[cfg(feature = "quick-xml")]
impl AnySimpleTypeDeserializer {
    fn handle_content_output<'a>(
        helper: &mut DeserializeHelper,
        type_: Option<String>,
        output: DeserializerOutput<'a, String>,
    ) -> DeserializerResult<'a, AnySimpleType> {
        match output.artifact {
            DeserializerArtifact::Deserializer(content) => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(Self { type_, content }),
                event: output.event,
                allow_any: false,
            }),
            DeserializerArtifact::Data(content) => {
                let any_simple_type = Self::finish_content(helper, type_.as_deref(), content)?;

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(any_simple_type),
                    event: output.event,
                    allow_any: false,
                })
            }
            DeserializerArtifact::None => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: output.event,
                allow_any: false,
            }),
        }
    }

    fn finish_content(
        helper: &mut DeserializeHelper,
        type_: Option<&str>,
        content: String,
    ) -> Result<AnySimpleType, Error> {
        use AnySimpleType as S;

        let type_ = type_
            .as_ref()
            .map(|name| QName::from_helper(helper, name.as_bytes()));
        let x = type_.as_ref().map(QName::local_name);

        macro_rules! map_err {
            ($x:expr) => {
                $x.map_err(|_| ErrorKind::InvalidData(content.into_bytes().into()))?
            };
        }

        match x {
            Some(b"string") => Ok(S::String(content)),
            Some(b"boolean") => Ok(S::Boolean(map_err!(content.parse()))),

            Some(b"float") => Ok(S::Float(map_err!(content.parse()))),
            Some(b"double") => Ok(S::Double(map_err!(content.parse()))),
            Some(b"decimal") => Ok(S::Decimal(map_err!(content.parse()))),

            Some(b"duration") => Ok(S::Duration(content)),
            Some(b"dateTime") => Ok(S::DateTime(content)),
            Some(b"time") => Ok(S::Time(content)),
            Some(b"date") => Ok(S::Date(content)),
            Some(b"gYearMonth") => Ok(S::GYearMonth(content)),
            Some(b"gYear") => Ok(S::GYear(content)),
            Some(b"gMonthDay") => Ok(S::GMonthDay(content)),
            Some(b"gMonth") => Ok(S::GMonth(content)),
            Some(b"gDay") => Ok(S::GDay(content)),

            Some(b"hexBinary") => {
                let bytes = (0..content.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&content[i..i + 2], 16))
                    .collect::<Result<Vec<_>, _>>();

                Ok(S::HexBinary(map_err!(bytes)))
            }
            Some(b"base64Binary") => Ok(S::Base64Binary(parse_base64_binary(&content)?)),

            Some(b"anyURI") => Ok(S::AnyURI(content)),
            Some(b"QName") => Ok(S::QName(QName::from_helper(helper, content.as_bytes()))),
            Some(b"NOTATION") => Ok(S::Notation(content)),

            Some(b"integer") => Ok(S::Integer(map_err!(content.parse()))),
            Some(b"positiveInteger") => Ok(S::PositiveInteger(map_err!(content.parse()))),
            Some(b"negativeInteger") => Ok(S::NegativeInteger(map_err!(content.parse()))),
            Some(b"nonNegativeInteger") => Ok(S::NonNegativeInteger(map_err!(content.parse()))),
            Some(b"nonPositiveInteger") => Ok(S::NonPositiveInteger(map_err!(content.parse()))),

            Some(b"long") => Ok(S::Long(map_err!(content.parse()))),
            Some(b"int") => Ok(S::Int(map_err!(content.parse()))),
            Some(b"short") => Ok(S::Short(map_err!(content.parse()))),
            Some(b"byte") => Ok(S::Byte(map_err!(content.parse()))),

            Some(b"unsignedLong") => Ok(S::UnsignedLong(map_err!(content.parse()))),
            Some(b"unsignedInt") => Ok(S::UnsignedInt(map_err!(content.parse()))),
            Some(b"unsignedShort") => Ok(S::UnsignedShort(map_err!(content.parse()))),
            Some(b"unsignedByte") => Ok(S::UnsignedByte(map_err!(content.parse()))),

            Some(b"normalizedString") => Ok(S::NormalizedString(content)),
            Some(b"token") => Ok(S::Token(content)),
            Some(b"language") => Ok(S::Language(content)),
            Some(b"NMTOKEN") => Ok(S::NmToken(content)),
            Some(b"Name") => Ok(S::Name(content)),
            Some(b"NCName") => Ok(S::NcName(content)),
            Some(b"ID") => Ok(S::Id(content)),
            Some(b"IDREF") => Ok(S::IdRef(content)),
            Some(b"ENTITY") => Ok(S::Entity(content)),

            Some(b"NMTOKENS") => Ok(S::NmTokens(parse_vec(&content))),
            Some(b"IDREFS") => Ok(S::IdRefs(parse_vec(&content))),
            Some(b"ENTITIES") => Ok(S::Entities(parse_vec(&content))),

            _ => Ok(S::Unknown {
                type_,
                content,
                namespaces: helper.namespaces(),
            }),
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<'de> Deserializer<'de, AnySimpleType> for AnySimpleTypeDeserializer {
    fn init(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, AnySimpleType> {
        let bytes = match &event {
            Event::Start(bytes) => bytes,
            Event::Empty(bytes) => bytes,
            _ => {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Continue(event),
                    allow_any: false,
                })
            }
        };

        let mut type_ = None;
        for attrib in bytes.attributes() {
            let attrib = attrib?;

            if matches!(
                helper.resolve_local_name(attrib.key, &Namespace::XSI),
                Some(b"type")
            ) {
                helper.read_attrib(&mut type_, b"type", &attrib.value)?;
            }
        }

        let output = ContentDeserializer::init(helper, event)?;

        AnySimpleTypeDeserializer::handle_content_output(helper, type_, output)
    }

    fn next(
        self,
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, AnySimpleType> {
        let output = self.content.next(helper, event)?;
        AnySimpleTypeDeserializer::handle_content_output(helper, self.type_, output)
    }

    fn finish(self, helper: &mut DeserializeHelper) -> Result<AnySimpleType, Error> {
        let content = self.content.finish(helper)?;

        AnySimpleTypeDeserializer::finish_content(helper, self.type_.as_deref(), content)
    }
}

#[inline]
#[cfg(feature = "base64")]
fn format_base64_binary(bytes: &[u8]) -> String {
    use base64::{prelude::BASE64_STANDARD, Engine};

    BASE64_STANDARD.encode(bytes)
}

#[inline]
#[cfg(not(feature = "base64"))]
fn format_base64_binary(s: &str) -> String {
    s.to_string()
}

#[inline]
#[cfg(feature = "base64")]
fn parse_base64_binary(bytes: &str) -> Result<Base64Binary, Error> {
    use base64::{prelude::BASE64_STANDARD, Engine};

    Ok(BASE64_STANDARD
        .decode(bytes)
        .map_err(|_| ErrorKind::InvalidData(bytes.as_bytes().to_vec().into()))?)
}

#[inline]
#[cfg(not(feature = "base64"))]
fn parse_base64_binary(s: &str) -> Result<Base64Binary, Error> {
    Ok(s.to_string())
}

fn format_vec<X: Display>(items: &[X]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_vec(content: &str) -> Vec<String> {
    content
        .split_whitespace()
        .map(ToString::to_string)
        .collect()
}
