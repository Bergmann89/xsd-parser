//! `xsd-parser-types` defines types that are used by the code that is generated
//! by [`xsd-parser`](https://docs.rs/xsd-parser).

pub mod misc;
pub mod traits;

#[cfg(feature = "xml")]
pub mod xml;

#[cfg(feature = "quick-xml")]
pub mod quick_xml;

pub use self::traits::{AsAny, WithNamespace};
