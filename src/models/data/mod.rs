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
mod custom;
mod dynamic;
mod enumeration;
mod occurs;
mod reference;
mod type_;
mod types;
mod union;

pub use self::build_in::BuildInData;
pub use self::complex::{
    ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement,
    ComplexDataEnum, ComplexDataStruct, StructMode,
};
pub use self::custom::CustomData;
pub use self::dynamic::{DerivedType, DynamicData};
pub use self::enumeration::{EnumerationData, EnumerationTypeVariant};
pub use self::occurs::Occurs;
pub use self::reference::ReferenceData;
pub use self::type_::{DataType, DataTypeVariant};
pub use self::types::DataTypes;
pub use self::union::{UnionData, UnionTypeVariant};
