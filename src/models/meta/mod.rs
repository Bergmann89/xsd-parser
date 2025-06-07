//! The `meta` module contains all [`MetaType`] related definitions and structures.
//!
//! This module represents the internal type system used after schema interpretation,
//! serving as an intermediate form between raw XML schema and Rust code generation.
//! It is produced by the [`Interpreter`](crate::Interpreter) and optionally
//! optimized by the [`Optimizer`](crate::Optimizer).

mod attribute;
mod base;
mod complex;
mod custom;
mod dynamic;
mod element;
mod enumeration;
mod name_builder;
mod reference;
mod type_;
mod type_eq;
mod types;
mod union;

use std::hash::Hasher;

pub use self::attribute::{AnyAttributeMeta, AttributeMeta, AttributeMetaVariant, AttributesMeta};
pub use self::base::Base;
pub use self::complex::{ComplexMeta, GroupMeta};
pub use self::custom::{CustomDefaultImpl, CustomMeta};
pub use self::dynamic::DynamicMeta;
pub use self::element::{AnyMeta, ElementMeta, ElementMetaVariant, ElementMode, ElementsMeta};
pub use self::enumeration::{EnumerationMeta, EnumerationMetaVariant, EnumerationMetaVariants};
pub use self::name_builder::{NameBuilder, NameFallback};
pub use self::reference::ReferenceMeta;
pub use self::type_::{BuildInMeta, MetaType, MetaTypeVariant};
pub use self::type_eq::TypeEq;
pub use self::types::{MetaTypes, ModuleMeta};
pub use self::union::{UnionMeta, UnionMetaType, UnionMetaTypes};

use crate::models::schema::xs::Use;

fn use_hash<H: Hasher>(use_: &Use, hasher: &mut H) {
    match use_ {
        Use::Prohibited => hasher.write_u8(0),
        Use::Optional => hasher.write_u8(1),
        Use::Required => hasher.write_u8(2),
    }
}
