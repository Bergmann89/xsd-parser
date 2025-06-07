//! Rust-oriented data type representations for code generation.
//!
//! This module defines the complete set of data structures used to represent
//! resolved, Rust-specific type information in preparation for rendering.
//! These types are derived from interpreted and optimized schema definitions,
//! and form the core of the code generation stage.
//!
//! These models are used directly by the generator and renderer stages
//! to produce idiomatic, type-safe Rust code.

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
