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
mod ident_cache;
mod name;
mod naming;

/// Convenient type to not break the public interface.
///
/// The `Ident` was renamed to [`TypeIdent`].
#[deprecated(note = "Use TypeIdent instead")]
pub type Ident = TypeIdent;

pub use self::ident::{
    AttributeIdent, ElementIdent, EnumerationIdent, IdentType, NodeIdent, NodeIdentType,
    PropertyIdent, TypeIdent, TypeIdentType,
};
pub use self::ident_cache::IdentCache;
pub use self::name::Name;
pub use self::naming::{
    format_ident, format_unknown_variant, unify_string, ExplicitNameBuilder, ExplicitNaming,
    NameBuilder, Naming,
};
