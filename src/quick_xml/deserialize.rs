use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::str::{from_utf8, FromStr};

use quick_xml::{
    escape::unescape,
    events::{attributes::Attribute, BytesStart, BytesText, Event},
    name::{Namespace, QName, ResolveResult},
};
use thiserror::Error;

use super::{Error, ErrorKind, RawByteStr, XmlReader, XmlReaderSync};

/// Trait that defines the [`Deserializer`] for a type.
pub trait WithDeserializer: Sized {
    /// The deserializer to use for this type.
    type Deserializer: for<'de> Deserializer<'de, Self>;
}

impl<X> WithDeserializer for X
where
    X: DeserializeBytes + Debug,
{
    type Deserializer = ContentDeserializer<X>;
}

/// Trait that defines a deserializer that can be used to construct a type from a
/// XML [`Event`]s.
pub trait Deserializer<'de, T>: Debug + Sized
where
    T: WithDeserializer<Deserializer = Self>,
{
    /// Initializes a new deserializer from the passed `reader` and the initial `event`.
    ///
    /// # Errors
    ///
    /// Returns an [`struct@Error`] if the initialization of the deserializer failed.
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, T>
    where
        R: XmlReader;

    /// Processes the next XML [`Event`].
    ///
    /// # Errors
    ///
    /// Returns an [`struct@Error`] if processing the event failed.
    fn next<R>(self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, T>
    where
        R: XmlReader;

    /// Force the deserializer to finish.
    ///
    /// # Errors
    ///
    /// Returns an [`struct@Error`] if the deserializer could not finish.
    fn finish<R>(self, reader: &R) -> Result<T, Error>
    where
        R: XmlReader;
}

/// Result type returned by the [`Deserializer`] trait.
pub type DeserializerResult<'a, T> = Result<DeserializerOutput<'a, T>, Error>;

/// Controls the flow of the deserializer
#[derive(Debug)]
pub enum ElementHandlerOutput<'a> {
    /// Continue with the deserialization
    Continue {
        /// Event to continue the deserialization process with.
        event: Event<'a>,

        /// Wether if any element is allowed for the current deserializer.
        allow_any: bool,
    },

    /// Break the deserialization
    Break {
        /// Instructions how to deal with a maybe unhandled event
        /// returned by the child deserializer .
        event: DeserializerEvent<'a>,

        /// Wether if any element is allowed for the current deserializer.
        allow_any: bool,
    },
}

impl<'a> ElementHandlerOutput<'a> {
    /// Create a [`Continue`](Self::Continue) instance.
    #[must_use]
    pub fn continue_(event: Event<'a>, allow_any: bool) -> Self {
        Self::Continue { event, allow_any }
    }

    /// Create a [`Break`](Self::Break) instance.
    #[must_use]
    pub fn break_(event: DeserializerEvent<'a>, allow_any: bool) -> Self {
        Self::Break { event, allow_any }
    }

    /// Create a [`Break`](Self::Break) instance that will return the passed
    /// `event` to the parent deserializers for further processing.
    #[must_use]
    pub fn return_to_parent(event: Event<'a>, allow_any: bool) -> Self {
        Self::break_(DeserializerEvent::Continue(event), allow_any)
    }

    /// Create a [`Break`](Self::Break) instance that will return the passed
    /// `event` to root of the deserialization process.
    #[must_use]
    pub fn return_to_root(event: Event<'a>, allow_any: bool) -> Self {
        Self::break_(DeserializerEvent::Break(event), allow_any)
    }

    /// Create a [`Continue`](Self::Continue) instance if the passed `event` is
    /// a `Continue(Start)`, `Continue(Empty)`, or `Continue(End)`,
    /// a [`Break`](Self::Break) instance otherwise.
    #[must_use]
    pub fn from_event(event: DeserializerEvent<'a>, allow_any: bool) -> Self {
        match event {
            DeserializerEvent::Continue(
                event @ (Event::Start(_) | Event::Empty(_) | Event::End(_)),
            ) => Self::continue_(event, allow_any),
            event => Self::break_(event, allow_any),
        }
    }

    /// Create a [`Continue`](Self::Continue) instance if the passed `event` is
    /// a `Continue(End)`, a [`Break`](Self::Break) instance otherwise.
    #[must_use]
    pub fn from_event_end(event: DeserializerEvent<'a>, allow_any: bool) -> Self {
        match event {
            DeserializerEvent::Continue(event @ Event::End(_)) => Self::continue_(event, allow_any),
            DeserializerEvent::Continue(event) => {
                Self::break_(DeserializerEvent::Break(event), allow_any)
            }
            event => Self::break_(event, allow_any),
        }
    }
}

/// Type that is used to bundle the output of a [`Deserializer`] operation.
#[derive(Debug)]
pub struct DeserializerOutput<'a, T>
where
    T: WithDeserializer,
{
    /// Artifact produced by the deserializer.
    pub artifact: DeserializerArtifact<T>,

    /// Contains the processed event if it was not consumed by the deserializer.
    pub event: DeserializerEvent<'a>,

    /// Whether the deserializer allows other XML elements in the current state or not.
    /// If this is set to `true` and the `event` is not consumed, the event should
    /// be skipped. For [`Event::Start`] this would mean to skip the whole element
    /// until the corresponding [`Event::End`] is received.
    pub allow_any: bool,
}

/// Artifact that is returned by a [`Deserializer`].
///
/// This contains either the deserialized data or the deserializer itself.
#[derive(Debug)]
pub enum DeserializerArtifact<T>
where
    T: WithDeserializer,
{
    /// Is returned if the deserialization process is finished and not data was produced.
    None,

    /// Contains the actual type constructed by the deserializer, once the deserializer has
    /// finished it's construction.
    Data(T),

    /// Contains the deserializer after an operation on the deserializer has been executed.
    /// This will be returned if the deserialization of the type is not finished yet.
    Deserializer(T::Deserializer),
}

impl<T> DeserializerArtifact<T>
where
    T: WithDeserializer,
{
    /// Check if this is a [`DeserializerArtifact::None`].
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Create a new [`DeserializerArtifact`] instance from the passed `data`.
    ///
    /// If `data` is `Some` a [`DeserializerArtifact::Data`] is created. If it
    /// is a `None` a [`DeserializerArtifact::None`] is crated.
    pub fn from_data(data: Option<T>) -> Self {
        if let Some(data) = data {
            Self::Data(data)
        } else {
            Self::None
        }
    }

    /// Create a new [`DeserializerArtifact`] instance from the passed `deserializer`.
    ///
    /// If `data` is `Some` a [`DeserializerArtifact::Deserializer`] is created.
    /// If it is a `None` a [`DeserializerArtifact::None`] is crated.
    pub fn from_deserializer(deserializer: Option<T::Deserializer>) -> Self {
        if let Some(deserializer) = deserializer {
            Self::Deserializer(deserializer)
        } else {
            Self::None
        }
    }

    /// Split the deserializer artifact into two options.
    /// One for the data and one for the deserializer.
    #[inline]
    pub fn into_parts(self) -> (Option<T>, Option<T::Deserializer>) {
        match self {
            Self::None => (None, None),
            Self::Data(data) => (Some(data), None),
            Self::Deserializer(deserializer) => (None, Some(deserializer)),
        }
    }

    /// Maps the data or the deserializer to new types using the passed mappers.
    #[inline]
    pub fn map<F, G, X>(self, data_mapper: F, deserializer_mapper: G) -> DeserializerArtifact<X>
    where
        X: WithDeserializer,
        F: FnOnce(T) -> X,
        G: FnOnce(T::Deserializer) -> X::Deserializer,
    {
        match self {
            Self::None => DeserializerArtifact::None,
            Self::Data(data) => DeserializerArtifact::Data(data_mapper(data)),
            Self::Deserializer(deserializer) => {
                DeserializerArtifact::Deserializer(deserializer_mapper(deserializer))
            }
        }
    }
}

/// Indicates what to do with a event returned by a deserializer
#[derive(Debug)]
pub enum DeserializerEvent<'a> {
    /// The event was consumed by the deserializer, nothing to handle here.
    None,

    /// The event is handled and should be returned to the deserialization root
    /// for additional evaluation.
    Break(Event<'a>),

    /// The event was not consumed by the deserializer an may be processed again
    /// by it's any of it's parents.
    Continue(Event<'a>),
}

impl<'a> DeserializerEvent<'a> {
    /// Extract the event as `Option`.
    #[must_use]
    pub fn into_event(self) -> Option<Event<'a>> {
        match self {
            Self::None => None,
            Self::Break(event) | Self::Continue(event) => Some(event),
        }
    }
}

/// Trait that could be implemented by types to support deserialization from XML
/// using the [`quick_xml`] crate.
pub trait DeserializeSync<'de, R>: Sized
where
    R: XmlReaderSync<'de>,
{
    /// Error that is returned by the `deserialize` method.
    type Error;

    /// Deserialize the type from the passed `reader`.
    ///
    /// # Errors
    ///
    /// Will return a suitable error if the operation failed.
    fn deserialize(reader: &mut R) -> Result<Self, Self::Error>;
}

impl<'de, R, X> DeserializeSync<'de, R> for X
where
    R: XmlReaderSync<'de>,
    X: WithDeserializer,
{
    type Error = Error;

    fn deserialize(reader: &mut R) -> Result<Self, Self::Error> {
        DeserializeHelper::new(reader).deserialize_sync()
    }
}

/// Trait that could be implemented by types to support asynchronous
/// deserialization from XML using the [`quick_xml`] crate.
#[cfg(feature = "async")]
pub trait DeserializeAsync<'de, R>: Sized
where
    R: super::XmlReaderAsync<'de>,
{
    /// Future that is returned by the [`deserialize_async`] method.
    type Future<'x>: std::future::Future<Output = Result<Self, Self::Error>>
    where
        R: 'x,
        'de: 'x;

    /// Error that is returned by the future generated by the [`deserialize_async`] method.
    type Error;

    /// Asynchronously deserializes the type from the passed `reader`.
    fn deserialize_async<'x>(reader: &'x mut R) -> Self::Future<'x>
    where
        'de: 'x;
}

#[cfg(feature = "async")]
impl<'de, R, X> DeserializeAsync<'de, R> for X
where
    R: super::XmlReaderAsync<'de>,
    X: WithDeserializer,
{
    type Future<'x>
        = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + 'x>>
    where
        R: 'x,
        'de: 'x;

    type Error = Error;

    fn deserialize_async<'x>(reader: &'x mut R) -> Self::Future<'x>
    where
        'de: 'x,
    {
        Box::pin(async move { DeserializeHelper::new(reader).deserialize_async().await })
    }
}

/// Trait that could be implemented by types to support deserialization from
/// XML byte streams using the [`quick_xml`] crate.
///
/// This is usually implemented for simple types like numbers, strings or enums.
pub trait DeserializeBytes: Sized {
    /// Try to deserialize the type from bytes.
    ///
    /// This is used to deserialize the type from attributes or raw element
    /// content.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`struct@Error`] if the deserialization was not successful.
    fn deserialize_bytes<R: XmlReader>(reader: &R, bytes: &[u8]) -> Result<Self, Error>;
}

/// Error that is raised by the [`DeserializeBytes`] trait if the type implements
/// [`FromStr`], but the conversion from the string has failed.
#[derive(Debug, Error)]
#[error("Unable to deserialize value from string (value = {value}, error = {error})")]
pub struct DeserializeStrError<E> {
    /// Value that could not be parsed.
    pub value: String,

    /// Error forwarded from [`FromStr`].
    pub error: E,
}

impl DeserializeBytes for bool {
    fn deserialize_bytes<R: XmlReader>(reader: &R, bytes: &[u8]) -> Result<Self, Error> {
        let _reader = reader;

        match bytes {
            b"TRUE" | b"True" | b"true" | b"YES" | b"Yes" | b"yes" | b"1" => Ok(true),
            b"FALSE" | b"False" | b"false" | b"NO" | b"No" | b"no" | b"0" => Ok(false),
            _ => Err(ErrorKind::UnknownOrInvalidValue(bytes.to_owned().into()).into()),
        }
    }
}

/// Marker trait used to automatically implement [`DeserializeBytes`] for any
/// type that implements [`FromStr`].
pub trait DeserializeBytesFromStr: FromStr {}

impl<X> DeserializeBytes for X
where
    X: DeserializeBytesFromStr,
    X::Err: std::error::Error + Send + Sync + 'static,
{
    fn deserialize_bytes<R: XmlReader>(reader: &R, bytes: &[u8]) -> Result<Self, Error> {
        let _reader = reader;
        let s = from_utf8(bytes).map_err(Error::from)?;

        X::from_str(s).map_err(|error| {
            Error::custom(DeserializeStrError {
                value: s.into(),
                error,
            })
        })
    }
}

impl DeserializeBytesFromStr for String {}

impl DeserializeBytesFromStr for u8 {}
impl DeserializeBytesFromStr for u16 {}
impl DeserializeBytesFromStr for u32 {}
impl DeserializeBytesFromStr for u64 {}
impl DeserializeBytesFromStr for usize {}

impl DeserializeBytesFromStr for i8 {}
impl DeserializeBytesFromStr for i16 {}
impl DeserializeBytesFromStr for i32 {}
impl DeserializeBytesFromStr for i64 {}
impl DeserializeBytesFromStr for isize {}

impl DeserializeBytesFromStr for f32 {}
impl DeserializeBytesFromStr for f64 {}

#[cfg(feature = "num")]
impl DeserializeBytesFromStr for num::BigInt {}

#[cfg(feature = "num")]
impl DeserializeBytesFromStr for num::BigUint {}

/// Implements a [`Deserializer`] for any type that implements [`DeserializeBytes`].
#[derive(Debug)]
pub struct ContentDeserializer<T> {
    data: Vec<u8>,
    marker: PhantomData<T>,
}

impl<'de, T> Deserializer<'de, T> for ContentDeserializer<T>
where
    T: DeserializeBytes + Debug,
{
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, T>
    where
        R: XmlReader,
    {
        match event {
            Event::Start(_) => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(Self {
                    data: Vec::new(),
                    marker: PhantomData,
                }),
                event: DeserializerEvent::None,
                allow_any: false,
            }),
            Event::Empty(_) => {
                let data = T::deserialize_bytes(reader, &[])?;

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(data),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            }
            event => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Continue(event),
                allow_any: false,
            }),
        }
    }

    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, T>
    where
        R: XmlReader,
    {
        match event {
            Event::Text(x) => {
                self.data.extend_from_slice(x.as_ref());

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            }
            Event::GeneralRef(x) => {
                self.data.push(b'&');
                self.data.extend_from_slice(x.as_ref());
                self.data.push(b';');

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            }
            Event::End(_) => {
                let data = self.finish(reader)?;

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(data),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            }
            event => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            }),
        }
    }

    fn finish<R>(self, reader: &R) -> Result<T, Error>
    where
        R: XmlReader,
    {
        let text = from_utf8(&self.data[..])?;
        let text = BytesText::from_escaped(text);
        let text = text.decode()?;
        let text = unescape(&text)?;

        T::deserialize_bytes(reader, text.as_bytes().trim_ascii())
    }
}

/* DeserializeReader */

/// Reader trait with additional helper methods for deserializing.
pub trait DeserializeReader: XmlReader {
    /// Helper function to convert and store an attribute from the XML event.
    ///
    /// # Errors
    ///
    /// Returns an [`struct@Error`] with [`ErrorKind::DuplicateAttribute`] if `store`
    /// already contained a value.
    fn read_attrib<T>(
        &self,
        store: &mut Option<T>,
        name: &'static [u8],
        value: &[u8],
    ) -> Result<(), Error>
    where
        T: DeserializeBytes,
    {
        if store.is_some() {
            self.err(ErrorKind::DuplicateAttribute(RawByteStr::from(name)))?;
        }

        let value = self.map_result(T::deserialize_bytes(self, value))?;
        *store = Some(value);

        Ok(())
    }

    /// Raise the [`UnexpectedAttribute`](ErrorKind::UnexpectedAttribute) error
    /// for the passed `attrib`.
    ///
    /// # Errors
    ///
    /// Will always return the [`UnexpectedAttribute`](ErrorKind::UnexpectedAttribute)
    /// error.
    fn raise_unexpected_attrib(&self, attrib: Attribute<'_>) -> Result<(), Error> {
        self.err(ErrorKind::UnexpectedAttribute(RawByteStr::from_slice(
            attrib.key.into_inner(),
        )))
    }

    /// Raises an [`UnexpectedAttribute`](ErrorKind::UnexpectedAttribute) error
    /// for the given attribute if it is not globally allowed (e.g., an XSI attribute).
    ///
    /// This method checks if the attribute is not globally allowed using
    /// [`is_globally_allowed_attrib`](DeserializeReader::is_globally_allowed_attrib)
    /// and, if so, raises the error. Otherwise, it returns `Ok(())`.
    ///
    /// # Errors
    ///
    /// Returns [`UnexpectedAttribute`](ErrorKind::UnexpectedAttribute) if the
    /// attribute is not globally allowed.
    fn raise_unexpected_attrib_checked(&self, attrib: Attribute<'_>) -> Result<(), Error> {
        if !self.is_globally_allowed_attrib(&attrib) {
            self.raise_unexpected_attrib(attrib)?;
        }

        Ok(())
    }

    /// Returns `true` if the given attribute is a globally allowed XML Schema
    /// Instance (XSI) attribute, `false` otherwise.
    ///
    /// Specifically, this checks if the attribute is in the `xsi` namespace and
    /// has a local name of `schemaLocation`, `noNamespaceSchemaLocation`, `type`,
    /// or `nil`. These attributes are globally valid and do not need to be
    /// explicitly declared in the XML schema.
    fn is_globally_allowed_attrib(&self, attrib: &Attribute<'_>) -> bool {
        if let (ResolveResult::Bound(x), local) = self.resolve(attrib.key, true) {
            let local = local.as_ref();
            x.0 == &**crate::models::schema::Namespace::XSI
                && (local == b"schemaLocation"
                    || local == b"noNamespaceSchemaLocation"
                    || local == b"type"
                    || local == b"nil")
        } else {
            false
        }
    }

    /// Try to resolve the local name of the passed qname and the expected namespace.
    ///
    /// Checks if the passed [`QName`] `name` matches the expected namespace `ns`
    /// and returns the local name of it. If `name` does not have a namespace prefix
    /// to resolve, the local name is just returned as is.
    fn resolve_local_name<'a>(&self, name: QName<'a>, ns: &[u8]) -> Option<&'a [u8]> {
        match self.resolve(name, true) {
            (ResolveResult::Unbound, local) => Some(local.into_inner()),
            (ResolveResult::Bound(x), local) if x.0 == ns => Some(local.into_inner()),
            (_, _) => None,
        }
    }

    /// Try to extract the resolved tag name of either a [`Start`](Event::Start) or a
    /// [`Empty`](Event::Empty) event.
    fn check_start_tag_name(&self, event: &Event<'_>, ns: Option<&[u8]>, name: &[u8]) -> bool {
        let (Event::Start(x) | Event::Empty(x)) = event else {
            return false;
        };

        if let Some(ns) = ns {
            matches!(self.resolve_local_name(x.name(), ns), Some(x) if x == name)
        } else {
            x.name().local_name().as_ref() == name
        }
    }

    /// Try to extract the type name of a dynamic type from the passed event.
    ///
    /// This method will try to extract the name of a dynamic type from
    /// [`Event::Start`] or [`Event::Empty`] by either using the explicit set name
    /// in the `type` attribute or by using the name of the xml tag.
    ///
    /// # Errors
    ///
    /// Raise an error if the attributes of the tag could not be resolved.
    fn get_dynamic_type_name<'a>(
        &self,
        event: &'a Event<'_>,
    ) -> Result<Option<Cow<'a, [u8]>>, Error> {
        let (Event::Start(b) | Event::Empty(b)) = &event else {
            return Ok(None);
        };

        let attrib = b
            .attributes()
            .find(|attrib| {
                let Ok(attrib) = attrib else { return false };
                let (resolve, name) = self.resolve(attrib.key, true);
                matches!(
                    resolve,
                    ResolveResult::Unbound
                        | ResolveResult::Bound(Namespace(
                            b"http://www.w3.org/2001/XMLSchema-instance"
                        ))
                ) && name.as_ref() == b"type"
            })
            .transpose()?;

        let name = attrib.map_or_else(|| Cow::Borrowed(b.name().0), |attrib| attrib.value);

        Ok(Some(name))
    }

    /// Initializes a deserializer from the passed `event`.
    ///
    /// If the event is [`Start`](Event::Start) or [`Empty`](Event::Empty), the passed
    /// function `f` is called with the [`BytesStart`] from the event to initialize the actual
    /// deserializer.
    ///
    /// # Errors
    ///
    /// Forwards the errors from raised by `f`.
    fn init_deserializer_from_start_event<'a, T, F>(
        &self,
        event: Event<'a>,
        f: F,
    ) -> Result<DeserializerOutput<'a, T>, Error>
    where
        T: WithDeserializer,
        F: FnOnce(&Self, &BytesStart<'a>) -> Result<<T as WithDeserializer>::Deserializer, Error>,
    {
        match event {
            Event::Start(start) => {
                let deserializer = f(self, &start)?;

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(deserializer),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            }
            Event::Empty(start) => {
                let deserializer = f(self, &start)?;
                let data = deserializer.finish(self)?;

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(data),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            }
            event => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Continue(event),
                allow_any: false,
            }),
        }
    }
}

impl<X> DeserializeReader for X where X: XmlReader {}

/* DeserializeHelper */

struct DeserializeHelper<'a, 'de, T, R>
where
    T: WithDeserializer,
{
    reader: &'a mut R,
    deserializer: Option<T::Deserializer>,
    skip_depth: Option<usize>,
    marker: PhantomData<&'de ()>,
}

impl<'a, 'de, T, R> DeserializeHelper<'a, 'de, T, R>
where
    T: WithDeserializer,
    R: XmlReader,
{
    fn new(reader: &'a mut R) -> Self {
        Self {
            reader,
            deserializer: None,
            skip_depth: None,
            marker: PhantomData,
        }
    }

    fn handle_event(&mut self, event: Event<'_>) -> Result<Option<T>, Error> {
        let ret = match self.deserializer.take() {
            None => T::Deserializer::init(self.reader, event),
            Some(b) => b.next(self.reader, event),
        };
        let ret = self.reader.map_result(ret);

        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = ret?;

        let (data, deserializer) = artifact.into_parts();

        self.deserializer = deserializer;

        match event.into_event() {
            None
            | Some(
                Event::Decl(_)
                | Event::Text(_)
                | Event::Comment(_)
                | Event::DocType(_)
                | Event::GeneralRef(_)
                | Event::PI(_),
            ) => (),
            Some(event) if allow_any => {
                if matches!(event, Event::Start(_)) {
                    self.skip_depth = Some(1);
                }
            }
            Some(event) => return Err(ErrorKind::UnexpectedEvent(event.into_owned()).into()),
        }

        Ok(data)
    }

    fn handle_skip(&mut self, event: Event<'de>) -> Option<Event<'de>> {
        let Some(skip_depth) = self.skip_depth.as_mut() else {
            return Some(event);
        };

        match event {
            Event::Start(_) => *skip_depth += 1,
            Event::End(_) if *skip_depth == 1 => {
                self.skip_depth = None;

                return None;
            }
            Event::End(_) => *skip_depth -= 1,
            Event::Eof => return Some(Event::Eof),
            _ => (),
        }

        None
    }
}

impl<'de, T, R> DeserializeHelper<'_, 'de, T, R>
where
    T: WithDeserializer,
    R: XmlReaderSync<'de>,
{
    fn deserialize_sync(&mut self) -> Result<T, Error> {
        loop {
            let event = self.reader.read_event()?;

            if let Some(event) = self.handle_skip(event) {
                if let Some(data) = self
                    .handle_event(event)
                    .map_err(|error| self.reader.extend_error(error))?
                {
                    return Ok(data);
                }
            }
        }
    }
}
#[cfg(feature = "async")]
impl<'de, T, R> DeserializeHelper<'_, 'de, T, R>
where
    T: WithDeserializer,
    R: super::XmlReaderAsync<'de>,
{
    async fn deserialize_async(&mut self) -> Result<T, Error> {
        loop {
            let event = self.reader.read_event_async().await?;

            if let Some(event) = self.handle_skip(event) {
                if let Some(data) = self.handle_event(event)? {
                    return Ok(data);
                }
            }
        }
    }
}
