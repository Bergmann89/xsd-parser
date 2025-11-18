//! This module defines different miscellaneous types and functions.

mod namespace;
mod namespace_prefix;
mod raw_byte_str;

pub use self::namespace::Namespace;
pub use self::namespace_prefix::NamespacePrefix;
pub use self::raw_byte_str::{format_utf8_slice, RawByteStr};
