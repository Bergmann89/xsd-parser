use std::borrow::{BorrowMut, Cow};
use std::fmt::Debug;
use std::io::Write;
use std::marker::PhantomData;
use std::mem::replace;
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::ops::{Deref, DerefMut};

use quick_xml::{
    escape::escape,
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    name::QName as XmlQName,
    Writer,
};

use crate::misc::{Namespace, NamespacePrefix};
use crate::xml::QName;

use super::{Error, ErrorKind, RawByteStr};

/// Namespace of an element as determined by the parent of the element.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum ElementNamespace {
    /// Not known (root element, or a caller that uses `serializer`).
    #[default]
    Unknown,

    /// The element is in no namespace (unqualified local element).
    None,

    /// The element is in the passed namespace.
    Namespace(Namespace),
}

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

    /// Like [`serializer`](Self::serializer), but also passes the `namespace`
    /// of the element, as determined by the parent.
    ///
    /// The default implementation ignores the namespace.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] is the serializer could not be initialized.
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _namespace = namespace;

        self.serializer(name, is_root)
    }
}

/// Trait that defines a serializer that can be used to destruct a type to
/// suitable XML [`Event`]s.
pub trait Serializer<'ser>: Debug {
    /// Get the next XML event from the serializer.
    ///
    /// # Errors
    ///
    /// Returns an error if the serializer encountered any issue.
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>>;

    /// Turn this serializer into an iterator using a newly created [`SerializeHelper`].
    fn into_iter(self) -> SerializerIter<'ser, Self, SerializeHelper>
    where
        Self: Sized,
    {
        self.into_iter_with(SerializeHelper::default())
    }

    /// Turn this serializer into an iterator using the passed [`SerializeHelper`].
    fn into_iter_with<H>(self, helper: H) -> SerializerIter<'ser, Self, H>
    where
        Self: Sized,
        H: BorrowMut<SerializeHelper>,
    {
        SerializerIter::new(self, helper)
    }
}

impl<'ser, X> Serializer<'ser> for X
where
    X: DerefMut + Debug,
    X::Target: Serializer<'ser>,
{
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
        self.deref_mut().next(helper)
    }
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

    /// Like [`serializer`](Self::serializer), but also passes the `namespace`
    /// of the element, as determined by the parent.
    ///
    /// The default implementation ignores the namespace.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] is the serializer could not be initialized.
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<BoxedSerializer<'ser>, Error> {
        let _namespace = namespace;

        WithBoxedSerializer::serializer(self, name, is_root)
    }
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

    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<BoxedSerializer<'ser>, Error> {
        Ok(Box::new(WithSerializer::serializer_with_namespace(
            self, name, namespace, is_root,
        )?))
    }
}

/// Boxed version of a [`Serializer`].
pub type BoxedSerializer<'ser> = Box<dyn Serializer<'ser> + 'ser>;

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
        SerializeImpl::new(self, Some(root), writer)?.serialize_sync()
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
            SerializeImpl::new(self, Some(root), writer)?
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
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error>;
}

/// Marker trait used to automatically implement [`SerializeBytes`] for any type
/// that implements [`ToString`].
pub trait SerializeBytesToString: ToString {}

impl<X> SerializeBytes for X
where
    X: SerializeBytesToString,
{
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        let _helper = helper;

        Ok(Some(Cow::Owned(self.to_string())))
    }
}

impl SerializeBytesToString for bool {}
impl SerializeBytesToString for String {}

impl SerializeBytesToString for u8 {}
impl SerializeBytesToString for u16 {}
impl SerializeBytesToString for u32 {}
impl SerializeBytesToString for u64 {}
impl SerializeBytesToString for u128 {}
impl SerializeBytesToString for usize {}

impl SerializeBytesToString for i8 {}
impl SerializeBytesToString for i16 {}
impl SerializeBytesToString for i32 {}
impl SerializeBytesToString for i64 {}
impl SerializeBytesToString for i128 {}
impl SerializeBytesToString for isize {}

impl SerializeBytesToString for f32 {}
impl SerializeBytesToString for f64 {}

impl SerializeBytesToString for NonZeroU8 {}
impl SerializeBytesToString for NonZeroU16 {}
impl SerializeBytesToString for NonZeroU32 {}
impl SerializeBytesToString for NonZeroU64 {}
impl SerializeBytesToString for NonZeroU128 {}
impl SerializeBytesToString for NonZeroUsize {}

impl SerializeBytesToString for NonZeroI8 {}
impl SerializeBytesToString for NonZeroI16 {}
impl SerializeBytesToString for NonZeroI32 {}
impl SerializeBytesToString for NonZeroI64 {}
impl SerializeBytesToString for NonZeroI128 {}
impl SerializeBytesToString for NonZeroIsize {}

#[cfg(feature = "num")]
impl SerializeBytesToString for num::BigInt {}

#[cfg(feature = "num")]
impl SerializeBytesToString for num::BigUint {}

/// Marker trait used to automatically implement [`WithSerializer`] for any type
/// that implements [`SerializeBytes`].
pub trait WithSerializeToBytes: SerializeBytes {}

impl<X> WithSerializeToBytes for X where X: SerializeBytesToString {}

impl<X> WithSerializer for X
where
    X: WithSerializeToBytes + Debug,
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

    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(ContentSerializer::new(self, name, is_root).with_namespace(namespace))
    }
}

/// Implements a [`Serializer`] for any type that implements [`SerializeBytes`].

#[derive(Debug)]
#[allow(missing_docs)]
pub enum ContentSerializer<'ser, T> {
    Begin {
        name: Option<&'ser str>,
        namespace: ElementNamespace,
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

        Self::Begin {
            name,
            namespace: ElementNamespace::Unknown,
            value,
        }
    }

    /// Set the `namespace` of the element that is emitted by this serializer.
    #[must_use]
    pub fn with_namespace(mut self, namespace: ElementNamespace) -> Self {
        if let Self::Begin { namespace: ns, .. } = &mut self {
            *ns = namespace;
        }

        self
    }
}

impl<'ser, T> Serializer<'ser> for ContentSerializer<'ser, T>
where
    T: SerializeBytes + Debug,
{
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
        loop {
            match replace(self, Self::Done) {
                Self::Begin {
                    name,
                    namespace,
                    value,
                } => match value.serialize_bytes(helper) {
                    Ok(None) => {
                        return name.map(|name| {
                            let mut bytes = BytesStart::new(name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, name, &namespace);
                            helper.end_ns_scope();

                            Ok(Event::Empty(bytes))
                        })
                    }
                    Ok(Some(data)) => {
                        if data.contains("]]>") {
                            return Some(Err(ErrorKind::InvalidData(RawByteStr::from_slice(
                                data.as_bytes(),
                            ))
                            .into()));
                        }

                        *self = Self::Data { name, data };

                        if let Some(name) = name {
                            let mut bytes = BytesStart::new(name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, name, &namespace);

                            return Some(Ok(Event::Start(bytes)));
                        }
                    }
                    Err(error) => return Some(Err(error)),
                },
                Self::Data { name, data } => {
                    if let Some(name) = name {
                        *self = Self::End { name };
                    }

                    if data.is_empty() {
                        continue;
                    }

                    return Some(Ok(Event::Text(BytesText::from_escaped(escape(data)))));
                }
                Self::End { name } => {
                    helper.end_ns_scope();

                    return Some(Ok(Event::End(BytesEnd::new(name))));
                }
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
        namespace: ElementNamespace,
        iter: <T as IntoIterator>::IntoIter,
    },
    Emitting {
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        iter: <T as IntoIterator>::IntoIter,
        serializer: TItem::Serializer<'ser>,
    },
    Done,
}

impl<'ser, T, TItem> Serializer<'ser> for IterSerializer<'ser, T, TItem>
where
    T: IntoIterator<Item = &'ser TItem> + Debug + 'ser,
    <T as IntoIterator>::IntoIter: Debug,
    TItem: WithSerializer + Debug + 'ser,
{
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
        loop {
            match replace(self, Self::Done) {
                Self::Pending {
                    name,
                    namespace,
                    mut iter,
                } => {
                    let item = iter.next()?;

                    match item.serializer_with_namespace(name, namespace.clone(), false) {
                        Ok(serializer) => {
                            *self = Self::Emitting {
                                name,
                                namespace,
                                iter,
                                serializer,
                            }
                        }
                        Err(error) => return Some(Err(error)),
                    }
                }
                Self::Emitting {
                    name,
                    namespace,
                    iter,
                    mut serializer,
                } => {
                    if let Some(ret) = serializer.next(helper) {
                        *self = Self::Emitting {
                            name,
                            namespace,
                            iter,
                            serializer,
                        };

                        return Some(ret);
                    }

                    *self = Self::Pending {
                        name,
                        namespace,
                        iter,
                    };
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
            namespace: ElementNamespace::Unknown,
            iter: value.into_iter(),
        }
    }

    /// Set the `namespace` that is passed to the serializer of every item.
    #[must_use]
    pub fn with_namespace(mut self, namespace: ElementNamespace) -> Self {
        if let Self::Pending { namespace: ns, .. } = &mut self {
            *ns = namespace;
        }

        self
    }
}

/* DerefIter */

/// An iterator that emits the dereferenced item of `T`.
///
/// This implements [`Option::as_deref`] for iterators.
#[derive(Debug)]
pub struct DerefIter<T>(pub T::IntoIter)
where
    T: IntoIterator;

impl<T> DerefIter<T>
where
    T: IntoIterator,
{
    /// Create a new [`DerefIter`] from the passed `inner` value.
    pub fn new(inner: T) -> Self {
        Self(inner.into_iter())
    }
}

impl<'x, T, TItem> Iterator for DerefIter<T>
where
    T: IntoIterator,
    T::IntoIter: Iterator<Item = &'x TItem>,
    TItem: Deref + 'x,
{
    type Item = &'x TItem::Target;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Deref::deref)
    }
}

/// Implements [`Iterator`] for any [`Serializer`] `S` using the [`BorrowMut<SerializeHelper>`] `H`.
#[derive(Debug)]
pub struct SerializerIter<'ser, S, H>
where
    S: Serializer<'ser>,
    H: BorrowMut<SerializeHelper>,
{
    serializer: S,
    helper: H,
    lt: PhantomData<&'ser ()>,
}

impl<'ser, S, H> SerializerIter<'ser, S, H>
where
    S: Serializer<'ser>,
    H: BorrowMut<SerializeHelper>,
{
    pub fn new(serializer: S, helper: H) -> Self {
        Self {
            serializer,
            helper,
            lt: PhantomData,
        }
    }
}

impl<'ser, S, H> Iterator for SerializerIter<'ser, S, H>
where
    S: Serializer<'ser>,
    H: BorrowMut<SerializeHelper>,
{
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.serializer.next(self.helper.borrow_mut())
    }
}

/* XsiTypeSerializer */

/// A [`Serializer`] that wraps another (boxed) serializer to serialize a value
/// of a dynamic type that is dispatched via the `xsi:type` attribute.
///
/// The wrapped serializer emits the value using the element name of its actual
/// (concrete) type. This serializer rewrites the resulting element so that it
/// uses the dynamic type's base element `name` and moves the original element
/// name into a `xsi:type` attribute instead. If the value is already using the
/// base element name (i.e. it is the base type itself) no `xsi:type` attribute
/// is added.
#[derive(Debug)]
pub struct XsiTypeSerializer<'ser, T> {
    inner: T,
    name: &'ser str,
    namespace: ElementNamespace,
    is_root: bool,
    state: XsiTypeState,
}

#[derive(Debug)]
enum XsiTypeState {
    /// The serializer was just created, no event was emitted yet.
    Init,

    /// The opening element was not emitted yet.
    Pending,

    /// The opening element was emitted, waiting for the matching closing
    /// element. The contained value is the current nesting depth.
    Open(usize),

    /// The opening (and closing) element was emitted.
    Done,
}

impl<'ser, T> XsiTypeSerializer<'ser, T> {
    /// Create a new [`XsiTypeSerializer`] that wraps the passed `inner`
    /// serializer and rewrites the emitted element to use the passed base
    /// element `name`.
    #[must_use]
    pub fn new(inner: T, name: &'ser str, is_root: bool) -> Self {
        Self {
            inner,
            name,
            namespace: ElementNamespace::Unknown,
            is_root,
            state: XsiTypeState::Init,
        }
    }

    /// Set the `namespace` of the rewritten element.
    #[must_use]
    pub fn with_namespace(mut self, namespace: ElementNamespace) -> Self {
        self.namespace = namespace;

        self
    }

    /// Rewrite the opening element so that it uses the base element name and
    /// references its original (concrete) name via a `xsi:type` attribute.
    fn rewrite(&self, bytes: BytesStart<'ser>, helper: &mut SerializeHelper) -> BytesStart<'ser> {
        // The value uses the base element name already, so it is the base type
        // and does not need a `xsi:type` attribute.
        if bytes.name().0 == self.name {
            return bytes;
        }

        let type_name = bytes.name().0.to_owned();

        let mut new = BytesStart::new(self.name);
        helper.write_xmlns_for_element(&mut new, self.name, &self.namespace);

        // An empty inner element has already ended its scope, so the helper does
        // not know about its `xmlns:xsi` anymore.
        let has_xmlns_xsi = bytes
            .attributes()
            .any(|attr| matches!(attr, Ok(attr) if attr.key.0 == "xmlns:xsi"));
        if self.is_root && !has_xmlns_xsi {
            helper.write_xmlns(&mut new, Some(&NamespacePrefix::XSI), &Namespace::XSI);
        }

        new.push_attribute(("xsi:type", &type_name[..]));
        new.extend_attributes(bytes.attributes().filter_map(|attr| {
            let attr = attr.ok()?;

            if attr.key.0 == "xsi:type" {
                None
            } else {
                Some(attr)
            }
        }));

        new
    }
}

impl<'ser, T> Serializer<'ser> for XsiTypeSerializer<'ser, T>
where
    T: Serializer<'ser>,
{
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
        // The scope is opened before the inner serializer emits its element,
        // so the declarations of `rewrite` are on the level of the element.
        if matches!(self.state, XsiTypeState::Init) {
            self.state = XsiTypeState::Pending;
            helper.begin_ns_scope();
        }

        let event = match self.inner.next(helper) {
            Some(Ok(event)) => event,
            ret => {
                if matches!(self.state, XsiTypeState::Pending) {
                    self.state = XsiTypeState::Done;
                    helper.end_ns_scope();
                }

                return ret;
            }
        };

        if matches!(self.state, XsiTypeState::Pending) {
            return Some(Ok(match event {
                Event::Empty(bytes) => {
                    let bytes = self.rewrite(bytes, helper);
                    self.state = XsiTypeState::Done;
                    helper.end_ns_scope();

                    Event::Empty(bytes)
                }
                Event::Start(bytes) => {
                    let bytes = self.rewrite(bytes, helper);
                    self.state = XsiTypeState::Open(1);

                    Event::Start(bytes)
                }
                other => {
                    self.state = XsiTypeState::Done;
                    helper.end_ns_scope();

                    other
                }
            }));
        }

        if let XsiTypeState::Open(depth) = &mut self.state {
            match &event {
                Event::Start(_) => *depth += 1,
                Event::End(_) => {
                    *depth -= 1;

                    if *depth == 0 {
                        self.state = XsiTypeState::Done;

                        helper.end_ns_scope();

                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                }
                _ => (),
            }
        }

        Some(Ok(event))
    }
}

/// Trait for collecting namespaces of a type and all its sub-types.
///
/// Implement this trait to allow the dynamic serialization mode to discover which
/// XML namespaces a value actually requires at runtime.
pub trait CollectNamespaces {
    /// Collect all XML namespaces that `self` (and its children) need into `helper`.
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>);
}

macro_rules! impl_collect_namespaces {
    ($ident:path) => {
        impl CollectNamespaces for $ident {
            fn collect_namespaces(&self, _: &mut SerializeHelper, _: &mut BytesStart<'_>) {}
        }
    };
}

// Implementations for primitive types (all no-ops).
impl_collect_namespaces!(bool);
impl_collect_namespaces!(char);
impl_collect_namespaces!(str);
impl_collect_namespaces!(String);

impl_collect_namespaces!(u8);
impl_collect_namespaces!(u16);
impl_collect_namespaces!(u32);
impl_collect_namespaces!(u64);
impl_collect_namespaces!(u128);
impl_collect_namespaces!(usize);

impl_collect_namespaces!(i8);
impl_collect_namespaces!(i16);
impl_collect_namespaces!(i32);
impl_collect_namespaces!(i64);
impl_collect_namespaces!(i128);
impl_collect_namespaces!(isize);

impl_collect_namespaces!(f32);
impl_collect_namespaces!(f64);

impl_collect_namespaces!(std::num::NonZeroU8);
impl_collect_namespaces!(std::num::NonZeroU16);
impl_collect_namespaces!(std::num::NonZeroU32);
impl_collect_namespaces!(std::num::NonZeroU64);
impl_collect_namespaces!(std::num::NonZeroU128);
impl_collect_namespaces!(std::num::NonZeroUsize);

impl_collect_namespaces!(std::num::NonZeroI8);
impl_collect_namespaces!(std::num::NonZeroI16);
impl_collect_namespaces!(std::num::NonZeroI32);
impl_collect_namespaces!(std::num::NonZeroI64);
impl_collect_namespaces!(std::num::NonZeroI128);
impl_collect_namespaces!(std::num::NonZeroIsize);

#[cfg(feature = "num")]
impl_collect_namespaces!(num::BigInt);

#[cfg(feature = "num")]
impl_collect_namespaces!(num::BigUint);

impl<T: CollectNamespaces> CollectNamespaces for Option<T> {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        if let Some(inner) = self {
            inner.collect_namespaces(helper, bytes);
        }
    }
}

impl<T: CollectNamespaces> CollectNamespaces for Vec<T> {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        for item in self {
            item.collect_namespaces(helper, bytes);
        }
    }
}

impl<T: CollectNamespaces + ?Sized> CollectNamespaces for Box<T> {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        self.as_ref().collect_namespaces(helper, bytes);
    }
}

impl<T: CollectNamespaces, const N: usize> CollectNamespaces for [T; N] {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        for item in self {
            item.collect_namespaces(helper, bytes);
        }
    }
}

impl<T: CollectNamespaces> CollectNamespaces for [T] {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        for item in self {
            item.collect_namespaces(helper, bytes);
        }
    }
}

/// Helper that defines some useful methods needed for `quick_xml` serialization.
///
/// Mainly used in [`Serializer`] and [`SerializeBytes`].
#[derive(Default, Debug)]
pub struct SerializeHelper {
    level: usize,
    namespaces: Vec<NamespaceEntry>,
}

#[derive(Debug)]
struct NamespaceEntry {
    level: usize,
    prefix: Option<NamespacePrefix>,
    namespace: Namespace,
}

impl SerializeHelper {
    /// Begin a new namespace scope inside the serialized XML.
    ///
    /// This is usually the case, if you create a [`Event::Start`] or [`Event::Empty`].
    pub fn begin_ns_scope(&mut self) {
        self.level += 1;
    }

    /// End the current namespace scope inside the serialized XML.
    ///
    /// This is usually the case, if you create a [`Event::End`].
    pub fn end_ns_scope(&mut self) {
        self.level = self.level.saturating_sub(1);

        loop {
            let is_outdated =
                matches!(self.namespaces.last(), Some(entry) if entry.level > self.level);
            if is_outdated {
                self.namespaces.pop();
            } else {
                break;
            }
        }
    }

    /// Add the xmlns needed by the passed `qname` to the current scope, if not
    /// already active.
    ///
    /// The prefix of the qname is changed if the namespace is already active
    /// with a different prefix, or if the prefix of the qname is bound to a
    /// different namespace (a new prefix `ns0`, `ns1`, ... is declared then).
    /// The updated qname is returned.
    pub fn add_qname_namespace(&mut self, bytes: &mut BytesStart<'_>, mut qname: QName) -> QName {
        let Some(ns) = qname.take_namespace() else {
            return qname;
        };

        let prefix = if let Some(entry) = self.find_entry(&ns) {
            entry.prefix.clone()
        } else {
            let prefix = qname
                .prefix()
                .filter(|prefix| !prefix.is_empty())
                .map(|prefix| NamespacePrefix::new(prefix.to_vec()))
                .filter(|prefix| !self.is_declared(Some(prefix)))
                .unwrap_or_else(|| self.fresh_prefix());

            self.add_namespace_entry(bytes, Some(&prefix.0), ns.clone());

            Some(prefix)
        };

        let qname = if let Some(prefix) = &prefix {
            QName::from_bytes(
                prefix
                    .0
                    .iter()
                    .chain(Some(&b':'))
                    .chain(qname.local_name())
                    .copied()
                    .collect::<Vec<u8>>(),
            )
        } else {
            QName::from_bytes(qname.local_name().to_vec())
        };

        qname.with_namespace(ns)
    }

    /// Write a suitable `xmlns` attribute to the passed `bytes` object, if the
    /// `prefix` is not already bound to the `namespace` in the current scope.
    ///
    /// Every element that declares namespaces must begin its own scope first.
    /// If `prefix` is already declared for another namespace on the current
    /// element, nothing is written; that is an error of the caller.
    pub fn write_xmlns(
        &mut self,
        bytes: &mut BytesStart<'_>,
        prefix: Option<&NamespacePrefix>,
        namespace: &Namespace,
    ) {
        let prefix = prefix.filter(|p| !p.0.is_empty());
        if self.is_bound(prefix, namespace) || self.is_declared_on_element(prefix) {
            return;
        }

        self.add_namespace_entry(bytes, prefix.map(|x| &x.0[..]), namespace.clone());
    }

    /// Write a suitable `xmlns` attribute for the given `namespace`, deriving
    /// the prefix from the element `tag`.
    ///
    /// If `tag` contains a colon (e.g. `"tns:Element"`), the part before the
    /// colon is used as the namespace prefix (`xmlns:tns="..."`). Otherwise, a
    /// default namespace declaration is emitted (`xmlns="..."`). Conflicting
    /// declarations are handled like in [`write_xmlns`](Self::write_xmlns).
    pub fn write_xmlns_for_tag(
        &mut self,
        bytes: &mut BytesStart<'_>,
        tag: &str,
        namespace: &Namespace,
    ) {
        match tag.split_once(':') {
            Some((prefix, _)) => {
                let prefix = NamespacePrefix::from(prefix.as_bytes().to_vec());

                self.write_xmlns(bytes, Some(&prefix), namespace);
            }
            None => {
                self.write_xmlns(bytes, None, namespace);
            }
        }
    }

    /// Write the `xmlns` attribute needed by the start tag of an element with
    /// the passed `name` and `namespace`.
    ///
    /// - [`ElementNamespace::Unknown`]: nothing is written.
    /// - [`ElementNamespace::Namespace`]: like [`write_xmlns_for_tag`](Self::write_xmlns_for_tag).
    /// - [`ElementNamespace::None`]: an unprefixed `name` undeclares an active
    ///   default namespace (`xmlns=""`). A prefixed `name` can not be in no
    ///   namespace, nothing is written.
    pub fn write_xmlns_for_element(
        &mut self,
        bytes: &mut BytesStart<'_>,
        name: &str,
        namespace: &ElementNamespace,
    ) {
        match namespace {
            ElementNamespace::Unknown => (),
            ElementNamespace::Namespace(ns) => self.write_xmlns_for_tag(bytes, name, ns),
            ElementNamespace::None => {
                debug_assert!(!name.contains(':'), "prefixed element without namespace");

                if !name.contains(':') {
                    self.write_xmlns(bytes, None, &Namespace::EMPTY);
                }
            }
        }
    }

    /// Write the passed `attrib` to the passed `bytes` object.
    ///
    /// # Errors
    /// An error is returned of the attribute could not be serialized.
    pub fn write_attrib<T>(
        &mut self,
        bytes: &mut BytesStart<'_>,
        name: &str,
        attrib: &T,
    ) -> Result<(), Error>
    where
        T: SerializeBytes,
    {
        if let Some(attrib) = SerializeBytes::serialize_bytes(attrib, self)? {
            bytes.push_attribute((name, attrib));
        }

        Ok(())
    }

    /// Write the passed `attrib`  to the passed `bytes` object.
    ///
    /// # Errors
    /// An error is returned of the attribute could not be serialized.
    pub fn write_attrib_opt<T>(
        &mut self,
        bytes: &mut BytesStart<'_>,
        name: &str,
        attrib: &Option<T>,
    ) -> Result<(), Error>
    where
        T: SerializeBytes,
    {
        let Some(attrib) = attrib else {
            return Ok(());
        };

        self.write_attrib(bytes, name, attrib)
    }

    /// Get the namespace prefix for the passed `namespace`.
    ///
    /// If the namespace is not active in the current scope, an error is returned.
    pub fn get_namespace_prefix(
        &self,
        namespace: &Namespace,
    ) -> Result<Option<NamespacePrefix>, Error> {
        if let Some(entry) = self.find_entry(namespace) {
            return Ok(entry.prefix.clone());
        }

        Err(ErrorKind::UnknownNamespace(namespace.clone()))?
    }

    /// Returns the newest entry of `namespace` whose prefix is still bound to it.
    fn find_entry(&self, namespace: &Namespace) -> Option<&NamespaceEntry> {
        self.namespaces.iter().rev().find(|entry| {
            &entry.namespace == namespace && self.is_bound(entry.prefix.as_ref(), namespace)
        })
    }

    /// Returns `true` if `prefix` is currently bound to `namespace`.
    fn is_bound(&self, prefix: Option<&NamespacePrefix>, namespace: &Namespace) -> bool {
        let entry = self
            .namespaces
            .iter()
            .rev()
            .find(|entry| entry.prefix.as_ref() == prefix);

        match entry {
            Some(entry) => &entry.namespace == namespace,
            None => prefix.is_none() && namespace.is_empty(),
        }
    }

    /// Returns `true` if `prefix` is bound to any namespace in the current scope.
    fn is_declared(&self, prefix: Option<&NamespacePrefix>) -> bool {
        self.namespaces
            .iter()
            .any(|entry| entry.prefix.as_ref() == prefix)
    }

    /// Returns `true` if `prefix` was declared on the current element.
    fn is_declared_on_element(&self, prefix: Option<&NamespacePrefix>) -> bool {
        self.namespaces
            .iter()
            .rev()
            .take_while(|entry| entry.level == self.level)
            .any(|entry| entry.prefix.as_ref() == prefix)
    }

    /// Returns a new prefix (`ns0`, `ns1`, ...) that is not bound in the
    /// current scope.
    fn fresh_prefix(&self) -> NamespacePrefix {
        let mut index = 0_usize;
        loop {
            let prefix = NamespacePrefix::new(format!("ns{index}").into_bytes());
            if !self.is_declared(Some(&prefix)) {
                return prefix;
            }

            index += 1;
        }
    }

    fn add_namespace_entry(
        &mut self,
        bytes: &mut BytesStart<'_>,
        prefix: Option<&[u8]>,
        namespace: Namespace,
    ) {
        let prefix = prefix.filter(|x| !x.is_empty());
        let xmlns = prefix.map(|x| format!("xmlns:{}", String::from_utf8_lossy(x)));
        let xmlns = xmlns.as_deref().unwrap_or("xmlns");
        bytes.push_attribute(Attribute {
            key: XmlQName(xmlns),
            value: String::from_utf8_lossy(&namespace),
        });

        self.namespaces.push(NamespaceEntry {
            level: self.level,
            prefix: prefix.map(|x| NamespacePrefix::new(x.to_vec())),
            namespace,
        });
    }
}

/* SerializeImpl */

struct SerializeImpl<'a, T, W>
where
    T: WithSerializer + 'a,
{
    helper: SerializeHelper,
    writer: &'a mut Writer<W>,
    serializer: T::Serializer<'a>,
}

impl<'a, T, W> SerializeImpl<'a, T, W>
where
    T: WithSerializer,
{
    fn new(value: &'a T, name: Option<&'a str>, writer: &'a mut Writer<W>) -> Result<Self, Error> {
        let helper = SerializeHelper::default();
        let serializer = value.serializer(name, true)?;

        Ok(Self {
            helper,
            writer,
            serializer,
        })
    }
}

impl<T, W> SerializeImpl<'_, T, W>
where
    T: WithSerializer,
    W: Write,
{
    fn serialize_sync(&mut self) -> Result<(), Error> {
        while let Some(event) = self.serializer.next(&mut self.helper) {
            self.writer
                .write_event(event?)
                .map_err(|error| ErrorKind::XmlError(error.into()))?;
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<T, W> SerializeImpl<'_, T, W>
where
    T: WithSerializer,
    W: tokio::io::AsyncWrite + Unpin,
{
    async fn serialize_async(&mut self) -> Result<(), Error> {
        while let Some(event) = self.serializer.next(&mut self.helper) {
            self.writer
                .write_event_async(event?)
                .await
                .map_err(ErrorKind::XmlError)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use quick_xml::events::{BytesEnd, BytesStart, Event};

    use crate::misc::{Namespace, NamespacePrefix};
    use crate::xml::{AnySimpleType, Element, Nillable, QName};

    use super::{
        ElementNamespace, Error, SerializeHelper, Serializer, WithSerializer, XsiTypeSerializer,
    };

    const NS_X: Namespace = Namespace::new_const(b"X");
    const NS_Y: Namespace = Namespace::new_const(b"Y");
    const PREFIX_A: NamespacePrefix = NamespacePrefix::new_const(b"a");
    const PREFIX_B: NamespacePrefix = NamespacePrefix::new_const(b"b");
    const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";

    fn events<'ser, S: Serializer<'ser>>(
        mut serializer: S,
        helper: &mut SerializeHelper,
    ) -> Vec<Event<'ser>> {
        let mut events = Vec::new();
        while let Some(event) = serializer.next(helper) {
            events.push(event.unwrap());
        }

        events
    }

    fn first_tag<'a>(events: &'a [Event<'_>]) -> &'a BytesStart<'a> {
        match events.first() {
            Some(Event::Start(bytes) | Event::Empty(bytes)) => bytes,
            event => panic!("Expected start tag, got {event:?}"),
        }
    }

    fn attribs(bytes: &BytesStart<'_>, key: &str) -> Vec<String> {
        bytes
            .attributes()
            .map(Result::unwrap)
            .filter(|attr| attr.key.0 == key)
            .map(|attr| attr.value.into_owned())
            .collect()
    }

    #[derive(Debug, Clone, Copy)]
    enum StubKind {
        Nothing,
        Empty,
        StartEnd,
    }

    /// Emits one `tns:Dog` element like generated code does.
    #[derive(Debug)]
    struct Stub {
        kind: StubKind,
        scope: bool,
        xsi: bool,
        step: usize,
    }

    impl Stub {
        fn new(kind: StubKind, scope: bool, xsi: bool) -> Self {
            Self {
                kind,
                scope,
                xsi,
                step: 0,
            }
        }
    }

    impl<'ser> Serializer<'ser> for Stub {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            let step = self.step;
            self.step += 1;

            match (self.kind, step) {
                (StubKind::Empty | StubKind::StartEnd, 0) => {
                    let mut bytes = BytesStart::new("tns:Dog");
                    if self.scope {
                        helper.begin_ns_scope();
                    }
                    if self.xsi {
                        helper.write_xmlns(
                            &mut bytes,
                            Some(&NamespacePrefix::XSI),
                            &Namespace::XSI,
                        );
                    }

                    if matches!(self.kind, StubKind::Empty) {
                        if self.scope {
                            helper.end_ns_scope();
                        }

                        Some(Ok(Event::Empty(bytes)))
                    } else {
                        Some(Ok(Event::Start(bytes)))
                    }
                }
                (StubKind::StartEnd, 1) => {
                    if self.scope {
                        helper.end_ns_scope();
                    }

                    Some(Ok(Event::End(BytesEnd::new("tns:Dog"))))
                }
                _ => None,
            }
        }
    }

    fn xsi_type(kind: StubKind, scope: bool, xsi: bool) -> XsiTypeSerializer<'static, Stub> {
        XsiTypeSerializer::new(Stub::new(kind, scope, xsi), "tns:Animal", true)
    }

    #[test]
    fn any_simple_type_ends_its_scope() {
        let mut helper = SerializeHelper::default();
        let value = AnySimpleType::String("a".into());

        for _ in 0..2 {
            let serializer = value.serializer(Some("v"), false).unwrap();
            let events = events(serializer, &mut helper);
            let bytes = first_tag(&events);

            assert_eq!(attribs(bytes, "xmlns:xsi"), [XSI]);
            assert_eq!(
                attribs(bytes, "xmlns:xs"),
                ["http://www.w3.org/2001/XMLSchema"]
            );
        }
    }

    #[test]
    fn any_simple_type_keeps_xsi_prefix() {
        let mut helper = SerializeHelper::default();
        let value = AnySimpleType::Unknown {
            type_: Some(QName::from_bytes(b"xsi:t".to_vec()).with_namespace(NS_Y)),
            content: "a".into(),
            namespaces: Default::default(),
        };

        let serializer = value.serializer(Some("v"), false).unwrap();
        let events = events(serializer, &mut helper);
        let bytes = first_tag(&events);

        assert_eq!(attribs(bytes, "xmlns:xsi"), [XSI]);
        assert_eq!(attribs(bytes, "xmlns:ns0"), ["Y"]);
        assert_eq!(attribs(bytes, "xsi:type"), ["ns0:t"]);
    }

    #[test]
    fn xsi_type_serializer_ends_its_scope() {
        for scope in [true, false] {
            for kind in [StubKind::Empty, StubKind::StartEnd] {
                let mut helper = SerializeHelper::default();

                for _ in 0..2 {
                    let events = events(xsi_type(kind, scope, false), &mut helper);
                    let bytes = first_tag(&events);

                    assert_eq!(attribs(bytes, "xmlns:xsi"), [XSI], "{kind:?} {scope}");
                    assert_eq!(attribs(bytes, "xsi:type"), ["tns:Dog"], "{kind:?} {scope}");
                }
            }
        }
    }

    #[test]
    fn xsi_type_serializer_ends_its_scope_without_inner_element() {
        let mut helper = SerializeHelper::default();

        assert!(events(xsi_type(StubKind::Nothing, true, false), &mut helper).is_empty());

        let events = events(xsi_type(StubKind::Empty, true, false), &mut helper);
        assert_eq!(attribs(first_tag(&events), "xmlns:xsi"), [XSI]);
    }

    #[test]
    fn xsi_type_serializer_keeps_inner_xsi_declaration() {
        for kind in [StubKind::Empty, StubKind::StartEnd] {
            let mut helper = SerializeHelper::default();

            let events = events(xsi_type(kind, true, true), &mut helper);
            assert_eq!(attribs(first_tag(&events), "xmlns:xsi"), [XSI], "{kind:?}");
        }
    }

    #[test]
    fn nillable_nil_has_its_own_scope() {
        let mut helper = SerializeHelper::default();
        let value = Nillable::<String>::nil();

        for _ in 0..2 {
            let serializer = value.serializer(Some("v"), false).unwrap();
            let events = events(serializer, &mut helper);

            assert_eq!(attribs(first_tag(&events), "xmlns:xsi"), [XSI]);
        }
    }

    #[test]
    fn shadowed_prefix_is_not_bound() {
        for prefix in [Some(&PREFIX_A), None] {
            let mut helper = SerializeHelper::default();

            helper.begin_ns_scope();
            helper.write_xmlns(&mut BytesStart::new("e"), prefix, &NS_X);
            helper.begin_ns_scope();
            helper.write_xmlns(&mut BytesStart::new("e"), prefix, &NS_Y);

            assert!(helper.get_namespace_prefix(&NS_X).is_err());

            helper.begin_ns_scope();
            let mut bytes = BytesStart::new("e");
            helper.write_xmlns(&mut bytes, prefix, &NS_X);

            let key = if prefix.is_some() { "xmlns:a" } else { "xmlns" };
            assert_eq!(attribs(&bytes, key), ["X"]);
        }
    }

    #[test]
    fn qname_does_not_use_shadowed_prefix() {
        let mut helper = SerializeHelper::default();

        helper.begin_ns_scope();
        helper.write_xmlns(&mut BytesStart::new("e"), Some(&PREFIX_A), &NS_X);
        helper.begin_ns_scope();
        helper.write_xmlns(&mut BytesStart::new("e"), Some(&PREFIX_A), &NS_Y);

        helper.begin_ns_scope();
        let mut bytes = BytesStart::new("e");
        let qname = QName::from_bytes(b"a:n".to_vec()).with_namespace(NS_X);
        let qname = helper.add_qname_namespace(&mut bytes, qname);

        assert_eq!(qname.as_bytes(), b"ns0:n");
        assert_eq!(attribs(&bytes, "xmlns:ns0"), ["X"]);
        assert!(attribs(&bytes, "xmlns:a").is_empty());
    }

    #[test]
    fn older_binding_is_used_if_newer_one_is_shadowed() {
        let mut helper = SerializeHelper::default();

        helper.begin_ns_scope();
        helper.write_xmlns(&mut BytesStart::new("e"), Some(&PREFIX_A), &NS_X);
        helper.begin_ns_scope();
        helper.write_xmlns(&mut BytesStart::new("e"), Some(&PREFIX_B), &NS_X);
        helper.begin_ns_scope();
        helper.write_xmlns(&mut BytesStart::new("e"), Some(&PREFIX_B), &NS_Y);

        assert_eq!(helper.get_namespace_prefix(&NS_X).unwrap(), Some(PREFIX_A));

        let mut bytes = BytesStart::new("e");
        let qname = QName::from_bytes(b"b:n".to_vec()).with_namespace(NS_X);
        let qname = helper.add_qname_namespace(&mut bytes, qname);

        assert_eq!(qname.as_bytes(), b"a:n");
        assert_eq!(bytes.attributes().count(), 0);
    }

    #[test]
    fn qname_does_not_rebind_prefix() {
        let mut helper = SerializeHelper::default();

        // Same element.
        let mut bytes = BytesStart::new("e");
        helper.begin_ns_scope();
        helper.write_xmlns(&mut bytes, Some(&PREFIX_A), &NS_X);
        let qname = QName::from_bytes(b"a:t".to_vec()).with_namespace(NS_Y);
        let qname = helper.add_qname_namespace(&mut bytes, qname);

        assert_eq!(qname.as_bytes(), b"ns0:t");
        assert_eq!(attribs(&bytes, "xmlns:a"), ["X"]);
        assert_eq!(attribs(&bytes, "xmlns:ns0"), ["Y"]);

        // Child element.
        let mut bytes = BytesStart::new("e");
        helper.begin_ns_scope();
        let qname = QName::from_bytes(b"a:t".to_vec()).with_namespace(NS_Y);
        let qname = helper.add_qname_namespace(&mut bytes, qname);

        assert_eq!(qname.as_bytes(), b"ns0:t");
        assert!(attribs(&bytes, "xmlns:a").is_empty());
    }

    #[test]
    fn prefix_is_declared_once_per_element() {
        let mut helper = SerializeHelper::default();
        let mut bytes = BytesStart::new("e");

        helper.begin_ns_scope();
        helper.write_xmlns(&mut bytes, Some(&PREFIX_A), &NS_X);
        helper.write_xmlns(&mut bytes, Some(&PREFIX_A), &NS_Y);
        assert_eq!(attribs(&bytes, "xmlns:a"), ["X"]);

        let mut bytes = BytesStart::new("e");
        helper.begin_ns_scope();
        helper.write_xmlns(&mut bytes, None, &NS_X);
        helper.write_xmlns_for_tag(&mut bytes, "el", &NS_Y);
        assert_eq!(attribs(&bytes, "xmlns"), ["X"]);
    }

    type Case<'a> = (
        &'a str,
        &'a ElementNamespace,
        bool,
        &'a [&'a str],
        &'a [&'a str],
    );

    /// A helper, optionally with the default namespace `X` active on the
    /// (open) root element.
    fn helper(default_namespace: bool) -> SerializeHelper {
        let mut helper = SerializeHelper::default();
        if default_namespace {
            helper.begin_ns_scope();
            helper.write_xmlns(&mut BytesStart::new("root"), None, &NS_X);
        }

        helper
    }

    #[test]
    fn write_xmlns_for_element_writes_expected_declarations() {
        let ns_x = ElementNamespace::Namespace(NS_X);
        let ns_y = ElementNamespace::Namespace(NS_Y);

        // name, namespace, default namespace `X` active, expected `xmlns`, expected `xmlns:a`
        let cases: [Case<'_>; 14] = [
            ("e", &ElementNamespace::Unknown, false, &[], &[]),
            ("e", &ElementNamespace::Unknown, true, &[], &[]),
            ("a:e", &ElementNamespace::Unknown, false, &[], &[]),
            ("a:e", &ElementNamespace::Unknown, true, &[], &[]),
            ("e", &ns_y, false, &["Y"], &[]),
            ("e", &ns_y, true, &["Y"], &[]),
            ("e", &ns_x, false, &["X"], &[]),
            ("e", &ns_x, true, &[], &[]),
            ("a:e", &ns_y, false, &[], &["Y"]),
            ("a:e", &ns_y, true, &[], &["Y"]),
            ("e", &ElementNamespace::None, false, &[], &[]),
            ("e", &ElementNamespace::None, true, &[""], &[]),
            ("a:e", &ns_x, false, &[], &["X"]),
            ("a:e", &ns_x, true, &[], &["X"]),
        ];

        for (name, namespace, default_namespace, xmlns, xmlns_a) in cases {
            let mut helper = helper(default_namespace);
            let mut bytes = BytesStart::new(name);

            helper.begin_ns_scope();
            helper.write_xmlns_for_element(&mut bytes, name, namespace);

            let case = format!("{name} {namespace:?} {default_namespace}");
            assert_eq!(attribs(&bytes, "xmlns"), xmlns, "{case}");
            assert_eq!(attribs(&bytes, "xmlns:a"), xmlns_a, "{case}");
        }
    }

    #[test]
    fn content_serializer_undeclares_default_namespace() {
        for default_namespace in [false, true] {
            let mut helper = helper(default_namespace);
            let level = helper.level;
            let value = String::from("a");

            let serializer = value
                .serializer_with_namespace(Some("v"), ElementNamespace::None, false)
                .unwrap();
            let events = events(serializer, &mut helper);

            let expected: &[&str] = if default_namespace { &[""] } else { &[] };
            assert_eq!(attribs(first_tag(&events), "xmlns"), expected);
            assert_eq!(events.len(), 3);
            assert_eq!(helper.level, level);
        }
    }

    #[test]
    fn nillable_nil_undeclares_default_namespace() {
        let mut helper = helper(true);
        let value = Nillable::<String>::nil();

        let serializer = value
            .serializer_with_namespace(Some("v"), ElementNamespace::None, false)
            .unwrap();
        let events = events(serializer, &mut helper);

        assert_eq!(attribs(first_tag(&events), "xmlns"), [""]);
        assert_eq!(helper.level, 1);
    }

    #[test]
    fn any_simple_type_undeclares_default_namespace() {
        let mut helper = helper(true);
        let value = AnySimpleType::String("a".into());

        let serializer = value
            .serializer_with_namespace(Some("v"), ElementNamespace::None, false)
            .unwrap();
        let events = events(serializer, &mut helper);

        assert_eq!(attribs(first_tag(&events), "xmlns"), [""]);
        assert_eq!(helper.level, 1);
    }

    #[test]
    fn xsi_type_serializer_writes_namespace_of_element() {
        for kind in [StubKind::Empty, StubKind::StartEnd] {
            let mut helper = helper(true);
            let serializer = XsiTypeSerializer::new(Stub::new(kind, true, false), "Animal", true)
                .with_namespace(ElementNamespace::None);

            let events = events(serializer, &mut helper);

            assert_eq!(attribs(first_tag(&events), "xmlns"), [""], "{kind:?}");
            assert_eq!(helper.level, 1, "{kind:?}");
        }
    }

    #[test]
    fn element_replay_without_default_namespace_is_in_no_namespace() {
        let mut helper = helper(true);
        let element = Element::new().name(b"e".to_vec());

        let serializer = element.serializer(None, false).unwrap();
        let events = events(serializer, &mut helper);

        assert_eq!(attribs(first_tag(&events), "xmlns"), [""]);
        assert_eq!(helper.level, 1);
    }

    #[test]
    fn element_replay_keeps_inherited_default_namespace() {
        let mut helper = helper(true);
        let element = Element::new()
            .name(b"e".to_vec())
            .namespace(b"".to_vec(), b"X".to_vec());

        let serializer = element.serializer(None, false).unwrap();
        let events = events(serializer, &mut helper);

        assert!(attribs(first_tag(&events), "xmlns").is_empty());
        assert_eq!(helper.level, 1);
    }

    #[test]
    fn element_replay_with_empty_default_namespace_writes_one_declaration() {
        let mut helper = helper(true);
        let element = Element::new()
            .name(b"e".to_vec())
            .namespace(b"".to_vec(), b"".to_vec());

        let serializer = element.serializer(None, false).unwrap();
        let events = events(serializer, &mut helper);

        assert_eq!(attribs(first_tag(&events), "xmlns"), [""]);
        assert_eq!(helper.level, 1);
    }

    #[test]
    fn serializer_with_namespace_defaults_to_serializer() {
        #[derive(Debug, Default)]
        struct Plain {
            calls: Cell<usize>,
        }

        impl WithSerializer for Plain {
            type Serializer<'x> = Stub;

            fn serializer<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                is_root: bool,
            ) -> Result<Stub, Error> {
                let _args = (name, is_root);
                self.calls.set(self.calls.get() + 1);

                Ok(Stub::new(StubKind::Empty, false, false))
            }
        }

        let value = Plain::default();
        let serializer = value
            .serializer_with_namespace(Some("v"), ElementNamespace::None, false)
            .unwrap();
        let events = events(serializer, &mut SerializeHelper::default());

        assert_eq!(value.calls.get(), 1);
        assert!(attribs(first_tag(&events), "xmlns").is_empty());
    }
}
