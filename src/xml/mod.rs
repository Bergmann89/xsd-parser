//! The [`xml`](self) module contains different types to store unstructured XML
//! data. This is useful to represents `xs:any` and `xs:anyAttribute` information
//! from the XML schema.

mod attributes;
mod element;
mod namespaces;
mod value;

pub use attributes::{AnyAttributes, Attributes, Key as AttributeKey, Value as AttributeValue};
pub use element::{AnyElement, AnyElements, Element, Elements};
pub use namespaces::{Key as NamespaceKey, Namespaces, NamespacesShared, Value as NamespaceValue};
pub use value::Value;
