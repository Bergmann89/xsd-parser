//! The `quick_xml` module contains helper types for serializing and deserializing
//! generated code using the [`quick_xml`] crate.

pub mod reader;

mod attributes;
mod deserialize;
mod error;
mod serialize;

pub use std::io::Write as XmlWrite;

pub use quick_xml::{
    events::{BytesCData, BytesDecl, BytesEnd, BytesPI, BytesStart, BytesText, Event},
    name::{LocalName, Namespace, QName, ResolveResult},
    Writer,
};

pub use crate::misc::RawByteStr;

pub use self::attributes::{filter_xmlns_attributes, write_attrib, write_attrib_opt};
pub use self::deserialize::{
    ContentDeserializer, DeserializeBytes, DeserializeReader, DeserializeStrError, DeserializeSync,
    Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
    ElementHandlerOutput, WithDeserializer,
};
pub use self::error::{Error, Kind as ErrorKind, UnionError};
pub use self::reader::{ErrorReader, IoReader, SliceReader, XmlReader, XmlReaderSync};
pub use self::serialize::{
    BoxedSerializer, ContentSerializer, IterSerializer, SerializeBytes, SerializeSync, Serializer,
    WithBoxedSerializer, WithSerializer,
};

#[cfg(feature = "async")]
pub use tokio::io::AsyncWrite as XmlWriteAsync;

#[cfg(feature = "async")]
pub use self::serialize::SerializeAsync;

#[cfg(feature = "async")]
pub use self::deserialize::DeserializeAsync;

#[cfg(feature = "async")]
pub use self::reader::XmlReaderAsync;
