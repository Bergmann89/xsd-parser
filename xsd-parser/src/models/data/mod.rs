//! Rust-oriented data type representations for code generation.
//!
//! This module defines the full set of intermediate structures used to represent
//! resolved and Rust-specific data types, derived from interpreted and optimized
//! XML schemas meta types.
//!
//! These types abstract the XML schema constructs into idiomatic Rust equivalents
//! like structs, enums, options, and vectors, and form the backbone of the code
//! generation phase.

mod build_in;
mod complex;
mod constrains;
mod custom;
mod dynamic;
mod enumeration;
mod occurs;
mod path_data;
mod reference;
mod simple;
mod tag_name;
mod type_;
mod types;
mod union;

use std::ops::Bound;

use crate::models::meta::Constrains;

pub use self::build_in::BuildInData;
pub use self::complex::{
    ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement,
    ComplexDataElementOrigin, ComplexDataEnum, ComplexDataStruct, ComplexFlags, StructMode,
};
pub use self::constrains::ConstrainsData;
pub use self::custom::CustomData;
pub use self::dynamic::{DerivedType, DynamicData};
pub use self::enumeration::{EnumerationData, EnumerationDataVariant};
pub use self::occurs::Occurs;
pub use self::path_data::PathData;
pub use self::reference::ReferenceData;
pub use self::simple::SimpleData;
pub use self::tag_name::TagName;
pub use self::type_::{DataType, DataTypeVariant};
pub use self::types::DataTypes;
pub use self::union::{UnionData, UnionTypeVariant};

/// A generic configuration value wrapper that supports different merging strategies.
///
/// This enum is used to represent configuration fields that may either use default values,
/// extend existing ones, or completely overwrite them. It provides a flexible mechanism
/// for combining configurations from multiple sources.
#[derive(Default, Debug)]
pub enum ConfigValue<T> {
    /// Uses the default behavior or value.
    #[default]
    Default,

    /// Appends or merges the provided value with existing data.
    Extend(T),

    /// Replaces any existing data with the provided value.
    Overwrite(T),
}

impl Constrains {
    /// Returns `true` if this simple type needs value validation, `false` otherwise.
    #[must_use]
    pub fn need_value_validation(&self) -> bool {
        self.range.start != Bound::Unbounded
            || self.range.end != Bound::Unbounded
            || self.min_length.is_some()
            || self.max_length.is_some()
    }

    /// Returns `true` if this simple type needs string validation, `false` otherwise.
    #[must_use]
    pub fn need_string_validation(&self) -> bool {
        !self.patterns.is_empty() || self.total_digits.is_some() || self.fraction_digits.is_some()
    }
}
