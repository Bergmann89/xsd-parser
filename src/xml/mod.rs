//! The [`xml`](self) module contains different types to store unstructured XML
//! data. This is useful to represents `xs:any` and `xs:anyAttribute` information
//! from the XML schema.

mod attributes;
mod element;
mod namespaces;
mod value;

pub use attributes::{AnyAttributes, Attributes};
pub use element::{AnyElement, AnyElements, Element, Elements};
pub use namespaces::{Namespaces, NamespacesShared};
pub use value::Value;
