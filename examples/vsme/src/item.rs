//! Runtime support type for the `xbrli:item` substitution group.
//!
//! XBRL declares hundreds of facts as members of the `xbrli:item` substitution
//! group. Most of them share the same type and only differ by their XML tag
//! name. Instead of generating one dedicated type (and enum variant) per
//! element, the `vsme` build script groups the elements by their type and
//! represents each group with a single [`ItemWrapper`].
//!
//! An [`ItemWrapper`] wraps the shared inner type `T` and remembers which of the
//! supported tags (provided by the [`ItemTags`] implementation `TTags`) the
//! value was deserialized from, so it can be serialized back using the correct
//! tag name.

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{
        DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, Error, Event, ResolveResult, WithDeserializer,
        WithSerializer,
    },
};

/// Wraps a value of type `T` that can be represented by one of multiple XML tags
/// at runtime.
///
/// The set of supported tags is provided by the [`ItemTags`] implementation
/// `TTags`. The [`index`](Self::index) stores which of those tags the value
/// actually uses, so it can be serialized back to the original tag name.
pub(crate) struct ItemWrapper<T, TTags>
where
    TTags: ItemTags,
{
    index: usize,
    pub inner: T,
    _tags: PhantomData<TTags>,
}

impl<T, TTags> ItemWrapper<T, TTags>
where
    TTags: ItemTags,
{
    /// Create a new [`ItemWrapper`] for the tag with the passed `index`.
    #[must_use]
    pub(crate) fn new(index: usize, inner: T) -> Self {
        Self {
            index,
            inner,
            _tags: PhantomData,
        }
    }

    /// Get the (qualified) XML tag name this value uses.
    #[must_use]
    pub(crate) fn tag(&self) -> &'static ItemTag {
        &TTags::tags()[self.index]
    }
}

impl<T, TTags> Deref for ItemWrapper<T, TTags>
where
    TTags: ItemTags,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, TTags> DerefMut for ItemWrapper<T, TTags>
where
    TTags: ItemTags,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, TTags> Debug for ItemWrapper<T, TTags>
where
    T: Debug,
    TTags: ItemTags,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ItemWrapper")
            .field("tag", self.tag())
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

/// A single XML tag that is supported by an [`ItemWrapper`].
#[derive(Debug)]
pub(crate) struct ItemTag {
    /// Tag name to be used for this element.
    pub tag: &'static str,

    /// Local name of the tag (without any namespace prefix).
    pub name: &'static str,

    /// Namespace of the tag (`None` if the tag has no namespace).
    ///
    /// This is usually initialized from one of the namespace constants
    /// generated next to the schema types (e.g. `NS_VSME`). It is used together
    /// with [`name`](Self::name) to match incoming elements in a namespace
    /// aware way during deserialization.
    pub namespace: Namespace,
}

/// Provides the list of XML tags that are supported by an [`ItemWrapper`].
pub(crate) trait ItemTags {
    /// The XML tags that are represented by the [`ItemWrapper`].
    fn tags() -> &'static [ItemTag];
}

/* Serialize */

impl<T, TTags> WithSerializer for ItemWrapper<T, TTags>
where
    T: WithSerializer,
    TTags: ItemTags,
{
    type Serializer<'x>
        = T::Serializer<'x>
    where
        Self: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        // The actual XML tag is determined by the wrapped value, not by the name
        // passed in by the parent (which would just be the name of the grouped
        // Rust type).
        let _name = name;

        self.inner.serializer(Some(self.tag().tag), is_root)
    }
}

/* Deserialize */

impl<T, TTags> WithDeserializer for ItemWrapper<T, TTags>
where
    T: WithDeserializer,
    TTags: ItemTags,
{
    type Deserializer = ItemWrapperDeserializer<T, TTags>;
}

/// Deserializer for the [`ItemWrapper`] type.
pub(crate) struct ItemWrapperDeserializer<T, TTags>
where
    T: WithDeserializer,
{
    index: usize,
    inner: T::Deserializer,
    _tags: PhantomData<TTags>,
}

impl<T, TTags> Debug for ItemWrapperDeserializer<T, TTags>
where
    T: WithDeserializer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ItemWrapperDeserializer")
            .field("index", &self.index)
            .field("inner", &self.inner)
            .finish()
    }
}

impl<'de, T, TTags> Deserializer<'de, ItemWrapper<T, TTags>> for ItemWrapperDeserializer<T, TTags>
where
    T: WithDeserializer,
    TTags: ItemTags,
{
    fn init(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, ItemWrapper<T, TTags>> {
        // Find the tag index of the current element. The lookup is namespace
        // aware (so a document is free to use a different prefix than the
        // schema): we resolve the element's namespace only once (resolving is
        // not free) and then search the tag list using the resolved namespace
        // and local name. If the element is not one of the supported tags, we
        // do not handle it and return the event back to the caller (so the next
        // variant of the choice can try it).
        let index = match &event {
            Event::Start(bytes) | Event::Empty(bytes) => {
                let (resolved, local) = helper.resolve(bytes.name(), true);
                let local = local.into_inner();
                let namespace = match resolved {
                    ResolveResult::Bound(namespace) => Some(namespace.0),
                    _ => None,
                };

                TTags::tags().iter().position(|tag| {
                    tag.name.as_bytes() == local
                        && namespace.is_none_or(|ns| ns == tag.namespace.as_ref())
                })
            }
            _ => None,
        };

        let Some(index) = index else {
            return Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Continue(event),
                allow_any: false,
            });
        };

        let output = <T as WithDeserializer>::Deserializer::init(helper, event)?;

        Ok(map_output(index, output))
    }

    fn next(
        self,
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, ItemWrapper<T, TTags>> {
        let index = self.index;
        let output = self.inner.next(helper, event)?;

        Ok(map_output(index, output))
    }

    fn finish(self, helper: &mut DeserializeHelper) -> Result<ItemWrapper<T, TTags>, Error> {
        let inner = self.inner.finish(helper)?;

        Ok(ItemWrapper::new(self.index, inner))
    }
}

/// Map the [`DeserializerOutput`] of the wrapped type `T` to a suitable output
/// for the [`ItemWrapper`], keeping track of the resolved tag `index`.
fn map_output<T, TTags>(
    index: usize,
    output: DeserializerOutput<'_, T>,
) -> DeserializerOutput<'_, ItemWrapper<T, TTags>>
where
    T: WithDeserializer,
    TTags: ItemTags,
{
    let DeserializerOutput {
        artifact,
        event,
        allow_any,
    } = output;

    let artifact = artifact.map(
        |inner| ItemWrapper::new(index, inner),
        |inner| ItemWrapperDeserializer {
            index,
            inner,
            _tags: PhantomData,
        },
    );

    DeserializerOutput {
        artifact,
        event,
        allow_any,
    }
}
