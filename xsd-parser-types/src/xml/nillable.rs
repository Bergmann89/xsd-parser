use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "quick-xml")]
use quick_xml::events::{BytesStart, Event};

use crate::misc::Namespace;
use crate::traits::WithNamespace;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    CollectNamespaces, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
    DeserializerOutput, DeserializerResult, ElementNamespace, Error, ErrorKind, SerializeHelper,
    Serializer, WithDeserializer, WithSerializer,
};

/// Implements a nillable XML element.
///
/// ```xml
/// <element xsi:nil="true" />
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Nillable<T>(pub Option<T>);

impl<T> Nillable<T> {
    /// Create a new [`Nillable`] instance that contains the passed `value`.
    #[inline]
    #[must_use]
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }

    /// Create a new [`Nillable`] instance that is `nil`.
    #[inline]
    #[must_use]
    pub fn nil() -> Self {
        Self(None)
    }

    /// Extracts the inner `Option<T>`.
    #[inline]
    pub fn into_inner(self) -> Option<T> {
        self.0
    }

    /// Returns `true` if this instance is `nil`, `false` otherwise.
    #[inline]
    pub fn is_nil(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// Panics if the nillable does not contain a value.
    #[inline]
    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }
}

impl<T> From<Nillable<T>> for Option<T> {
    #[inline]
    fn from(value: Nillable<T>) -> Self {
        value.0
    }
}

impl<T> From<T> for Nillable<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> From<Option<T>> for Nillable<T> {
    #[inline]
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> Deref for Nillable<T> {
    type Target = Option<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Nillable<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> WithNamespace for Nillable<T>
where
    T: WithNamespace,
{
    fn prefix() -> Option<&'static str> {
        T::prefix()
    }

    fn namespace() -> Option<&'static str> {
        T::namespace()
    }
}

#[cfg(feature = "quick-xml")]
impl<T> CollectNamespaces for Nillable<T>
where
    T: CollectNamespaces,
{
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        use crate::misc::NamespacePrefix;

        if self.0.is_none() {
            helper.write_xmlns(bytes, Some(&NamespacePrefix::XSI), &Namespace::XSI);
        } else if let Some(inner) = &self.0 {
            inner.collect_namespaces(helper, bytes);
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<T> WithSerializer for Nillable<T>
where
    T: WithSerializer,
{
    type Serializer<'x>
        = NillableSerializer<'x, T>
    where
        Self: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        self.serializer_with_namespace(name, ElementNamespace::Unknown, is_root)
    }

    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        if let Some(inner) = &self.0 {
            Ok(NillableSerializer::Some {
                inner: inner.serializer_with_namespace(name, namespace, is_root)?,
            })
        } else {
            Ok(NillableSerializer::Nil {
                name: name.unwrap_or("Nillable"),
                namespace,
            })
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<T> WithDeserializer for Nillable<T>
where
    T: WithDeserializer,
{
    type Deserializer = NillableDeserializer<T>;
}

/// Implements the [`Serializer`](crate::quick_xml::Serializer) trait for the [`Nillable`] types.
#[cfg(feature = "quick-xml")]
pub enum NillableSerializer<'ser, T>
where
    T: WithSerializer + 'ser,
{
    /// Serialize the value using its serializer.
    Some {
        /// Inner serializer that produces the actual elements.
        inner: T::Serializer<'ser>,
    },

    /// Serialize the `nil` value.
    Nil {
        /// Name of the emitted XML tag.
        name: &'ser str,

        /// Namespace of the emitted XML tag.
        namespace: ElementNamespace,
    },

    /// Serialization is done.
    Done,
}

#[cfg(feature = "quick-xml")]
impl<'ser, T> Debug for NillableSerializer<'ser, T>
where
    T: WithSerializer + 'ser,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Some { inner } => f.debug_struct("Some").field("inner", inner).finish(),
            Self::Nil { name, namespace } => f
                .debug_struct("Nil")
                .field("name", name)
                .field("namespace", namespace)
                .finish(),
            Self::Done => write!(f, "Done"),
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<'ser, T> Serializer<'ser> for NillableSerializer<'ser, T>
where
    T: WithSerializer + 'ser,
{
    fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
        match self {
            Self::Some { inner } => {
                let item = inner.next(helper);

                if item.is_none() {
                    *self = Self::Done;
                }

                item
            }
            Self::Nil { name, namespace } => {
                use crate::misc::NamespacePrefix;

                let name = helper.element_name(name, namespace);
                let mut bytes = BytesStart::new(name);

                helper.begin_ns_scope();
                helper.write_xmlns_for_element(&mut bytes, name, namespace);
                helper.write_xmlns(&mut bytes, Some(&NamespacePrefix::XSI), &Namespace::XSI);
                bytes.push_attribute(("xsi:nil", "true"));
                helper.end_ns_scope();

                *self = Self::Done;

                Some(Ok(Event::Empty(bytes)))
            }
            Self::Done => None,
        }
    }
}

/// Implements the [`Deserializer`] trait for the [`Nillable`] types.
#[cfg(feature = "quick-xml")]
pub enum NillableDeserializer<T>
where
    T: WithDeserializer,
{
    /// Deserialize the value.
    Inner {
        /// Deserializer to use to deserialize the value.
        inner: T::Deserializer,
    },

    /// We received a `nil` value, just wait for the end tag.
    EndTag,
}

#[cfg(feature = "quick-xml")]
impl<T> Debug for NillableDeserializer<T>
where
    T: WithDeserializer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Inner { inner } => f.debug_struct("Inner").field("inner", inner).finish(),
            Self::EndTag => write!(f, "Closing"),
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<T> NillableDeserializer<T>
where
    T: WithDeserializer,
{
    /// Does the same as `init`, but hands the name of the element the parent
    /// expects (if any) to the inner deserializer.
    fn init_with_name_opt<'de>(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
        name: Option<(Option<&[u8]>, &[u8])>,
    ) -> DeserializerResult<'de, Nillable<T>> {
        let (Event::Start(bytes) | Event::Empty(bytes)) = &event else {
            return Ok(DeserializerOutput {
                event: DeserializerEvent::Continue(event),
                artifact: DeserializerArtifact::None,
                allow_any: false,
            });
        };

        if let Some((ns, name)) = name {
            if !helper.check_start_tag_name(&event, ns, name) {
                return Ok(DeserializerOutput {
                    event: DeserializerEvent::Continue(event),
                    artifact: DeserializerArtifact::None,
                    allow_any: false,
                });
            }
        }

        for attrib in bytes.attributes() {
            let attrib = attrib?;
            if matches!(
                helper.resolve_attrib_local_name(attrib.key, &Namespace::XSI),
                Some(b"nil")
            ) && matches!(
                &*attrib.value,
                "TRUE" | "True" | "true" | "YES" | "Yes" | "yes" | "1"
            ) {
                let artifact = if matches!(&event, Event::Empty(_)) {
                    DeserializerArtifact::Data(Nillable(None))
                } else {
                    DeserializerArtifact::Deserializer(Self::EndTag)
                };

                return Ok(DeserializerOutput {
                    artifact,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            }
        }

        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = match name {
            Some((ns, name)) => T::Deserializer::init_with_name(helper, event, ns, name)?,
            None => T::Deserializer::init(helper, event)?,
        };

        let artifact = match artifact {
            DeserializerArtifact::Deserializer(inner) => {
                DeserializerArtifact::Deserializer(Self::Inner { inner })
            }
            DeserializerArtifact::Data(value) => DeserializerArtifact::Data(Nillable(Some(value))),
            DeserializerArtifact::None => DeserializerArtifact::None,
        };

        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
}

#[cfg(feature = "quick-xml")]
impl<'de, T> Deserializer<'de, Nillable<T>> for NillableDeserializer<T>
where
    T: WithDeserializer,
{
    fn init(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, Nillable<T>> {
        Self::init_with_name_opt(helper, event, None)
    }

    fn init_with_name(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
        ns: Option<&[u8]>,
        name: &[u8],
    ) -> DeserializerResult<'de, Nillable<T>> {
        Self::init_with_name_opt(helper, event, Some((ns, name)))
    }

    fn next(
        self,
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, Nillable<T>> {
        match self {
            Self::Inner { inner } => {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = inner.next(helper, event)?;

                let artifact = match artifact {
                    DeserializerArtifact::Deserializer(inner) => {
                        DeserializerArtifact::Deserializer(Self::Inner { inner })
                    }
                    DeserializerArtifact::Data(value) => {
                        DeserializerArtifact::Data(Nillable(Some(value)))
                    }
                    DeserializerArtifact::None => DeserializerArtifact::None,
                };

                Ok(DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                })
            }
            Self::EndTag => Ok(if let Event::End(_) = &event {
                DeserializerOutput {
                    artifact: DeserializerArtifact::Data(Nillable(None)),
                    event: DeserializerEvent::None,
                    allow_any: false,
                }
            } else {
                DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(Self::EndTag),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                }
            }),
        }
    }

    fn finish(self, helper: &mut DeserializeHelper) -> Result<Nillable<T>, Error> {
        match self {
            Self::Inner { inner } => {
                let value = inner.finish(helper)?;

                Ok(Nillable(Some(value)))
            }
            Self::EndTag => Err(ErrorKind::UnexpectedEoL)?,
        }
    }
}
