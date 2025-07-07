use std::borrow::Cow;
use std::fmt::Debug;
use std::io::Write;
use std::mem::replace;

use quick_xml::{
    escape::escape,
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use super::{Error, ErrorKind, RawByteStr};

/// Trait that defines the [`Serializer`] for a type.
pub trait WithSerializer {
    /// The serializer to use for this type.
    type Serializer<'x>: Serializer<'x>
    where
        Self: 'x;

    /// Initializes a new serializer from the passed `value`.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] is the serializer could not be initialized.
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error>;
}

impl<X> WithSerializer for X
where
    X: SerializeBytes + Debug,
{
    type Serializer<'x>
        = ContentSerializer<'x, X>
    where
        Self: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(ContentSerializer::new(self, name, is_root))
    }
}

/// Trait that defines a serializer that can be used to destruct a type to
/// suitable XML [`Event`]s.
pub trait Serializer<'ser>: Iterator<Item = Result<Event<'ser>, Error>> + Debug {}

impl<'ser, X> Serializer<'ser> for X where
    X: Iterator<Item = Result<Event<'ser>, Error>> + Debug + Sized
{
}

/// Trait that returns a boxed version of a [`Serializer`] for any type that
/// implements [`WithSerializer`].
pub trait WithBoxedSerializer {
    /// Initializes a new serializer from the passed `value`.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] is the serializer could not be initialized.
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<BoxedSerializer<'ser>, Error>;
}

impl<X> WithBoxedSerializer for X
where
    X: WithSerializer,
{
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<BoxedSerializer<'ser>, Error> {
        Ok(Box::new(WithSerializer::serializer(self, name, is_root)?))
    }
}

/// Boxed version of a [`Serializer`].
pub type BoxedSerializer<'ser> =
    Box<dyn Serializer<'ser, Item = Result<Event<'ser>, Error>> + 'ser>;

/// Trait that could be implemented by types to support serialization to XML
/// using the [`quick_xml`] crate.
pub trait SerializeSync: Sized {
    /// Error returned by the `serialize` method.
    type Error;

    /// Serializes the type to XML using the provided `writer`.
    ///
    /// # Errors
    ///
    /// Returns a suitable error if the operation was not successful.
    fn serialize<W: Write>(&self, root: &str, writer: &mut Writer<W>) -> Result<(), Self::Error>;
}

impl<X> SerializeSync for X
where
    X: WithSerializer,
{
    type Error = Error;

    fn serialize<W: Write>(&self, root: &str, writer: &mut Writer<W>) -> Result<(), Self::Error> {
        SerializeHelper::new(self, Some(root), writer)?.serialize_sync()
    }
}

/// Trait that could be implemented by types to support asynchronous serialization
/// to XML using the [`quick_xml`] crate.
#[cfg(feature = "async")]
pub trait SerializeAsync: Sized {
    /// Future that is returned by the `serialize_async` method.
    type Future<'x>: std::future::Future<Output = Result<(), Self::Error>> + 'x
    where
        Self: 'x;

    /// Error returned by the `serialize_async` method.
    type Error;

    /// Asynchronously serializes the type to XML using the provided `writer`.
    fn serialize_async<'a, W: tokio::io::AsyncWrite + Unpin>(
        &'a self,
        root: &'a str,
        writer: &'a mut Writer<W>,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "async")]
impl<X> SerializeAsync for X
where
    X: WithSerializer,
{
    type Future<'x>
        = std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Self::Error>> + 'x>>
    where
        X: 'x;

    type Error = Error;

    fn serialize_async<'a, W: tokio::io::AsyncWrite + Unpin>(
        &'a self,
        root: &'a str,
        writer: &'a mut Writer<W>,
    ) -> Self::Future<'a> {
        Box::pin(async move {
            SerializeHelper::new(self, Some(root), writer)?
                .serialize_async()
                .await
        })
    }
}

/// Trait that could be implemented by types to support serialization to
/// XML byte streams.
///
/// This is usually implemented for simple types like numbers, strings or enums.
pub trait SerializeBytes: Sized {
    /// Try to serialize the type to bytes.
    ///
    /// This is used to serialize the type to attributes or raw element
    /// content.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the serialization was not successful.
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error>;
}

/// Marker trait used to automatically implement [`SerializeBytes`] for any type
/// that implements [`ToString`].
pub trait SerializeBytesToString: ToString {}

impl<X> SerializeBytes for X
where
    X: SerializeBytesToString,
{
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        Ok(Some(Cow::Owned(self.to_string())))
    }
}

impl SerializeBytesToString for bool {}
impl SerializeBytesToString for String {}

impl SerializeBytesToString for u8 {}
impl SerializeBytesToString for u16 {}
impl SerializeBytesToString for u32 {}
impl SerializeBytesToString for u64 {}
impl SerializeBytesToString for usize {}

impl SerializeBytesToString for i8 {}
impl SerializeBytesToString for i16 {}
impl SerializeBytesToString for i32 {}
impl SerializeBytesToString for i64 {}
impl SerializeBytesToString for isize {}

impl SerializeBytesToString for f32 {}
impl SerializeBytesToString for f64 {}

#[cfg(feature = "num")]
impl SerializeBytesToString for num::BigInt {}

#[cfg(feature = "num")]
impl SerializeBytesToString for num::BigUint {}

/// Implements a [`Serializer`] for any type that implements [`SerializeBytes`].

#[derive(Debug)]
#[allow(missing_docs)]
pub enum ContentSerializer<'ser, T> {
    Begin {
        name: Option<&'ser str>,
        value: &'ser T,
    },
    Data {
        name: Option<&'ser str>,
        data: Cow<'ser, str>,
    },
    End {
        name: &'ser str,
    },
    Done,
}

impl<'ser, T> ContentSerializer<'ser, T>
where
    T: SerializeBytes + Debug,
{
    /// Create a new [`ContentSerializer`] instance from the passed arguments.
    ///
    /// If a `name` was passed it will emit suitable [`Start`](Event::Start)
    /// and [`End`](Event::End) events. If no `name` was passed only the content
    /// is rendered.
    ///
    /// # Errors
    ///
    /// Will throw a [`ErrorKind::MissingName`] error, if `None` is passed as
    /// `name`.
    pub fn new(value: &'ser T, name: Option<&'ser str>, is_root: bool) -> Self {
        let _is_root = is_root;

        Self::Begin { name, value }
    }
}

impl<'ser, T> Iterator for ContentSerializer<'ser, T>
where
    T: SerializeBytes + Debug,
{
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match replace(self, Self::Done) {
                Self::Begin { name, value } => match value.serialize_bytes() {
                    Ok(None) => return name.map(|name| Ok(Event::Empty(BytesStart::new(name)))),
                    Ok(Some(data)) => {
                        if data.contains("]]>") {
                            return Some(Err(ErrorKind::InvalidData(RawByteStr::from_slice(
                                data.as_bytes(),
                            ))
                            .into()));
                        }

                        *self = Self::Data { name, data };

                        if let Some(name) = name {
                            return Some(Ok(Event::Start(BytesStart::new(name))));
                        }
                    }
                    Err(error) => return Some(Err(error)),
                },
                Self::Data { name, data } => {
                    if let Some(name) = name {
                        *self = Self::End { name };
                    }

                    return Some(Ok(Event::Text(BytesText::from_escaped(escape(data)))));
                }
                Self::End { name } => return Some(Ok(Event::End(BytesEnd::new(name)))),
                Self::Done => return None,
            }
        }
    }
}

/// Implements a [`Serializer`] for any type that implements an [`Iterator`]
/// that emits references to a type that implements [`WithSerializer`].
#[derive(Debug)]
#[allow(missing_docs)]
pub enum IterSerializer<'ser, T, TItem>
where
    T: IntoIterator<Item = &'ser TItem> + 'ser,
    <T as IntoIterator>::IntoIter: Debug,
    TItem: WithSerializer + 'ser,
{
    Pending {
        name: Option<&'ser str>,
        iter: <T as IntoIterator>::IntoIter,
    },
    Emitting {
        name: Option<&'ser str>,
        iter: <T as IntoIterator>::IntoIter,
        serializer: TItem::Serializer<'ser>,
    },
    Done,
}

impl<'ser, T, TItem> Iterator for IterSerializer<'ser, T, TItem>
where
    T: IntoIterator<Item = &'ser TItem> + 'ser,
    <T as IntoIterator>::IntoIter: Debug,
    TItem: WithSerializer + 'ser,
{
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match replace(self, Self::Done) {
                Self::Pending { name, mut iter } => {
                    let item = iter.next()?;

                    match item.serializer(name, false) {
                        Ok(serializer) => {
                            *self = Self::Emitting {
                                name,
                                iter,
                                serializer,
                            }
                        }
                        Err(error) => return Some(Err(error)),
                    }
                }
                Self::Emitting {
                    name,
                    iter,
                    mut serializer,
                } => {
                    if let Some(ret) = serializer.next() {
                        *self = Self::Emitting {
                            name,
                            iter,
                            serializer,
                        };

                        return Some(ret);
                    }

                    *self = Self::Pending { name, iter };
                }
                Self::Done => return None,
            }
        }
    }
}

impl<'ser, T, TItem> IterSerializer<'ser, T, TItem>
where
    T: IntoIterator<Item = &'ser TItem> + 'ser,
    <T as IntoIterator>::IntoIter: Debug,
    TItem: WithSerializer + 'ser,
{
    /// Create a new [`IterSerializer`] instance from the passed arguments.
    pub fn new(value: T, name: Option<&'ser str>, is_root: bool) -> Self {
        let _is_root = is_root;

        Self::Pending {
            name,
            iter: value.into_iter(),
        }
    }
}

/* SerializeHelper */

struct SerializeHelper<'a, T, W>
where
    T: WithSerializer + 'a,
{
    writer: &'a mut Writer<W>,
    serializer: T::Serializer<'a>,
}

impl<'a, T, W> SerializeHelper<'a, T, W>
where
    T: WithSerializer,
{
    fn new(value: &'a T, name: Option<&'a str>, writer: &'a mut Writer<W>) -> Result<Self, Error> {
        let serializer = value.serializer(name, true)?;

        Ok(Self { writer, serializer })
    }
}

impl<T, W> SerializeHelper<'_, T, W>
where
    T: WithSerializer,
    W: Write,
{
    fn serialize_sync(&mut self) -> Result<(), Error> {
        for event in self.serializer.by_ref() {
            self.writer
                .write_event(event?)
                .map_err(|error| ErrorKind::XmlError(error.into()))?;
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<T, W> SerializeHelper<'_, T, W>
where
    T: WithSerializer,
    W: tokio::io::AsyncWrite + Unpin,
{
    async fn serialize_async(&mut self) -> Result<(), Error> {
        for event in self.serializer.by_ref() {
            self.writer
                .write_event_async(event?)
                .await
                .map_err(ErrorKind::XmlError)?;
        }

        Ok(())
    }
}
