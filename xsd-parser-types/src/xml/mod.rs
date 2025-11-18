//! The [`xml`](self) module contains different types to store unstructured XML
//! data. This is useful to represents `xs:any` and `xs:anyAttribute` information
//! from the XML schema.

mod attributes;
mod element;
mod mixed;
mod namespaces;
mod nillable;
mod text;
mod value;

pub use self::attributes::{
    AnyAttributes, Attributes, Key as AttributeKey, Value as AttributeValue,
};
pub use self::element::{AnyElement, AnyElements, Element, Elements};
pub use self::mixed::{Mixed, MixedDeserializer, MixedSerializer};
pub use self::namespaces::{
    Key as NamespaceKey, Namespaces, NamespacesShared, Value as NamespaceValue,
};
pub use self::nillable::{Nillable, NillableDeserializer, NillableSerializer};
pub use self::text::{Text, TextDeserializer, TextSerializer};
pub use self::value::Value;
