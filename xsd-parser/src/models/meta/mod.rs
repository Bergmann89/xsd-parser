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
mod reference;
mod simple;
mod type_;
mod type_eq;
mod types;
mod union;

use std::ops::{Bound, Range};

pub use self::attribute::{AnyAttributeMeta, AttributeMeta, AttributeMetaVariant, AttributesMeta};
pub use self::base::Base;
pub use self::complex::{ComplexMeta, GroupMeta};
pub use self::custom::{CustomDefaultImpl, CustomMeta, CustomMetaNamespace};
pub use self::dynamic::{DerivedTypeMeta, DynamicMeta};
pub use self::element::{AnyMeta, ElementMeta, ElementMetaVariant, ElementMode, ElementsMeta};
pub use self::enumeration::{EnumerationMeta, EnumerationMetaVariant, EnumerationMetaVariants};
pub use self::reference::ReferenceMeta;
pub use self::simple::{SimpleMeta, WhiteSpace};
pub use self::type_::{BuildInMeta, MetaType, MetaTypeVariant};
pub use self::type_eq::TypeEq;
pub use self::types::{MetaTypes, ModuleMeta, SchemaMeta};
pub use self::union::{UnionMeta, UnionMetaType, UnionMetaTypes};

/// Constrains defined by the different facets of a type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Constrains {
    /// Range the value should be in.
    pub range: Range<Bound<String>>,

    /// Number of total digits the value maximal should have.
    pub total_digits: Option<usize>,

    /// Number of fraction digits the value maximal should have.
    pub fraction_digits: Option<usize>,

    /// Regex pattern the value should fulfill.
    pub patterns: Vec<String>,

    /// The minimum length the value should have.
    pub min_length: Option<usize>,

    /// The maximum length the value should have.
    pub max_length: Option<usize>,

    /// Defines the whitespace handling.
    pub whitespace: WhiteSpace,
}

impl Default for Constrains {
    fn default() -> Self {
        Self {
            range: Range {
                start: Bound::Unbounded,
                end: Bound::Unbounded,
            },
            total_digits: None,
            fraction_digits: None,
            patterns: Vec::new(),
            min_length: None,
            max_length: None,
            whitespace: WhiteSpace::default(),
        }
    }
}
