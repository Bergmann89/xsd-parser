//! This module provides the [`NamespaceScope`] type.

use std::borrow::Cow;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "quick-xml")]
use quick_xml::events::Event;

use crate::xml::NamespacesShared;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerOutput,
    DeserializerResult, Error, SerializeBytes, SerializeHelper, WithDeserializer, WithSerializer,
};

/// A wrapper type to store the namespaces in scope for a given XML element or attribute.
///
/// This wrapper type can be used for any type that needs to have access to the namespaces
/// after the deserialization is finished.
#[derive(Debug, Clone)]
pub struct NamespaceScope<T> {
    /// The actual value that was deserialized.
    pub inner: T,

    /// The namespaces in scope for the deserialized value.
    pub namespaces: NamespacesShared<'static>,
}

impl<T> Deref for NamespaceScope<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NamespaceScope<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Eq for NamespaceScope<T> where T: Eq {}

impl<T> PartialEq for NamespaceScope<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> Ord for NamespaceScope<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> PartialOrd for NamespaceScope<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Hash for NamespaceScope<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> NamespaceScope<T> {
    /// Returns the inner value, consuming the wrapper.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Splits the wrapper into its inner value and the namespaces, consuming the wrapper.
    pub fn into_parts(self) -> (T, NamespacesShared<'static>) {
        (self.inner, self.namespaces)
    }
}

impl<T> SerializeBytes for NamespaceScope<T>
where
    T: SerializeBytes,
{
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.inner.serialize_bytes(helper)
    }
}

impl<T> DeserializeBytes for NamespaceScope<T>
where
    T: DeserializeBytes,
{
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let inner = T::deserialize_bytes(helper, bytes)?;

        Ok(Self {
            inner,
            namespaces: helper.namespaces(),
        })
    }

    fn deserialize_str(helper: &mut DeserializeHelper, s: &str) -> Result<Self, Error> {
        let inner = T::deserialize_str(helper, s)?;

        Ok(Self {
            inner,
            namespaces: helper.namespaces(),
        })
    }
}

#[cfg(feature = "quick-xml")]
impl<T> WithSerializer for NamespaceScope<T>
where
    T: WithSerializer,
{
    type Serializer<'x>
        = T::Serializer<'x>
    where
        Self: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, crate::quick_xml::Error> {
        self.inner.serializer(name, is_root)
    }
}

#[cfg(feature = "quick-xml")]
impl<T> WithDeserializer for NamespaceScope<T>
where
    T: WithDeserializer,
{
    type Deserializer = NamespaceScopeDeserializer<T>;
}

/* NamespaceScopeDeserializer */

#[cfg(feature = "quick-xml")]
pub struct NamespaceScopeDeserializer<T>
where
    T: WithDeserializer,
{
    inner: T::Deserializer,
}

#[cfg(feature = "quick-xml")]
impl<T> NamespaceScopeDeserializer<T>
where
    T: WithDeserializer,
{
    fn map_output<'de>(
        helper: &mut DeserializeHelper,
        output: DeserializerOutput<'de, T>,
    ) -> DeserializerOutput<'de, NamespaceScope<T>> {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;

        let artifact = match artifact {
            DeserializerArtifact::None => DeserializerArtifact::None,
            DeserializerArtifact::Data(inner) => DeserializerArtifact::Data(NamespaceScope {
                inner,
                namespaces: helper.namespaces(),
            }),
            DeserializerArtifact::Deserializer(inner) => {
                DeserializerArtifact::Deserializer(NamespaceScopeDeserializer { inner })
            }
        };

        DeserializerOutput {
            artifact,
            event,
            allow_any,
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<'de, T> Deserializer<'de, NamespaceScope<T>> for NamespaceScopeDeserializer<T>
where
    T: WithDeserializer,
{
    fn init(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, NamespaceScope<T>> {
        let output = T::Deserializer::init(helper, event)?;

        Ok(Self::map_output(helper, output))
    }

    fn next(
        self,
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, NamespaceScope<T>> {
        let output = self.inner.next(helper, event)?;

        Ok(Self::map_output(helper, output))
    }

    fn finish(self, helper: &mut DeserializeHelper) -> Result<NamespaceScope<T>, Error> {
        let inner = self.inner.finish(helper)?;

        Ok(NamespaceScope {
            inner,
            namespaces: helper.namespaces(),
        })
    }
}

#[cfg(feature = "quick-xml")]
impl<T> Debug for NamespaceScopeDeserializer<T>
where
    T: WithDeserializer,
    T::Deserializer: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("NamespaceScopeDeserializer")
            .field("inner", &self.inner)
            .finish()
    }
}
