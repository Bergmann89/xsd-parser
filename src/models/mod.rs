//! Data model definitions used throughout the transformation pipeline.
//!
//! This module contains structured representations of intermediate and final
//! data forms used in parsing, interpretation, code generation, and rendering.
//! It serves as the central type layer for schema handling and Rust code generation.

pub mod code;
pub mod data;
pub mod meta;
pub mod schema;

mod ident;
mod name;
mod raw_byte_str;

pub use self::ident::{Ident, IdentType};
pub use self::name::Name;
pub use self::raw_byte_str::RawByteStr;

pub(crate) use self::raw_byte_str::format_utf8_slice;
