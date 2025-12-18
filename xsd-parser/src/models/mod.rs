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
mod ident_map;
mod name;
mod naming;

pub use self::ident::{Ident, IdentType};
pub use self::ident_map::IdentMap;
pub use self::name::Name;
pub use self::naming::{NameBuilder, Naming};
