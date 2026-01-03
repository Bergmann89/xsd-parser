//! Type-level optimization utilities for reducing schema complexity.
//!
//! This module defines the [`Optimizer`] and supporting logic for transforming
//! a [`MetaTypes`] structure into a simpler, cleaner, and smaller form.
//!
//! Optimizations include:
//! - Flattening nested complex types
//! - Removing unused or duplicate unions/enums
//! - Resolving typedef chains
//! - Normalizing choice cardinalities
//! - Simplifying unrestricted base types
//!
//! These transformations are especially useful before code generation, to avoid
//! verbose or redundant Rust output and ensure efficient, idiomatic structures.

mod dynamic_to_choice;
mod empty_enums;
mod empty_unions;
mod flatten_complex_type;
mod flatten_unions;
mod merge_choice_cardinality;
mod merge_enum_unions;
mod misc;
mod remove_duplicates;
mod replace_xs_any_type_with_any_element;
mod resolve_typedefs;
mod simplify_mixed_types;
mod unrestricted_base;

use thiserror::Error;

use crate::models::{meta::MetaTypes, TypeIdent};

use self::misc::{BaseMap, TypedefMap};

pub use self::unrestricted_base::UnrestrictedBaseFlags;

/// Optimizes a [`MetaTypes`] structure by reducing redundant or verbose type definitions.
///
/// The [`Optimizer`] performs various semantic transformations on a type graph,
/// such as flattening unions, removing empty enums, simplifying typedef chains,
/// and reducing nested complex structures.
///
/// It is typically used after schema interpretation and before code generation,
/// to ensure that only necessary and well-structured types are preserved in the final output.
///
/// Optimization is performed lazily; the resulting [`MetaTypes`] can be retrieved
/// using [`finish`](Self::finish).
#[must_use]
#[derive(Debug)]
pub struct Optimizer {
    types: MetaTypes,
    bases: Option<BaseMap>,
    typedefs: Option<TypedefMap>,
}

/// Error that is raised by the [`Optimizer`].
#[derive(Error, Debug)]
pub enum Error {
    /// Unknown type identifier.
    ///
    /// Is raised if a specific identifier could not be resolved to it's
    /// corresponding type information.
    #[error("Unknown type identifier: {0}!")]
    UnknownType(TypeIdent),

    /// The type is not a union type.
    ///
    /// Is raised if a type is expected to be a union, but it is not.
    #[error("The type is not a union type: {0}!")]
    ExpectedUnion(TypeIdent),

    /// The type is not a complex choice type.
    ///
    /// Is raised if a type is expected to be a complex choice, but it is not.
    #[error("The type is not a complex choice type: {0}!")]
    ExpectedComplexChoice(TypeIdent),

    /// The type is not a complex type.
    ///
    /// Is raised if a type is expected to be a complex type, but it is not.
    #[error("The type is not a complex type: {0}!")]
    ExpectedComplexType(TypeIdent),

    /// The complex type is missing a content type.
    ///
    /// Is raised if the content type of a complex type could not be resolved.
    #[error("Complex type {0} is missing a content type!")]
    MissingContentType(TypeIdent),

    /// The complex type is expected to have a choice content.
    ///
    /// Is raised if the content type of a complex type it not a choice.
    #[error("Complex type {0} is expected to have a choice content!")]
    ExpectedChoiceContent(TypeIdent),

    /// The complex type is expected to have content with [`MaxOccurs::Unbounded`](crate::models::schema::MaxOccurs::Unbounded).
    ///
    /// Is raised if the content of a complex type does nor have unbounded occurrence.
    #[error("Complex type {0} is expected to have content with unbound occurrence!")]
    ExpectedUnboundContent(TypeIdent),

    /// The complex type has an unexpected content type.
    ///
    /// Is raised if the content type of a complex type does not match the expectations.
    #[error("Complex type {0} has an unexpected content type!")]
    UnexpectedContentType(TypeIdent),

    /// The complex type contains an unexpected element in it's content type.
    ///
    /// Is raised if any element of the content of a complex type does not match the
    /// expectations.
    #[error("Complex type {0} contains an unexpected element in it's content type!")]
    UnexpectedElementInContent(TypeIdent),

    /// Custom Error
    #[error("{0}")]
    Custom(String),
}

macro_rules! get_bases {
    ($this:expr) => {{
        if $this.bases.is_none() {
            $this.bases = Some(crate::pipeline::optimizer::BaseMap::new(&$this.types));
        }

        $this.bases.as_ref().unwrap()
    }};
}

macro_rules! get_typedefs {
    ($this:expr) => {{
        if $this.typedefs.is_none() {
            $this.typedefs = Some(crate::pipeline::optimizer::TypedefMap::new(&$this.types));
        }

        $this.typedefs.as_ref().unwrap()
    }};
}

pub(super) use get_bases;
pub(super) use get_typedefs;

impl Optimizer {
    /// Create a new [`Optimizer`] instance from the passed `types`.
    pub fn new(types: MetaTypes) -> Self {
        Self {
            types,
            bases: None,
            typedefs: None,
        }
    }

    /// Finish the optimization and return the resulting [`MetaTypes`].
    #[must_use]
    pub fn finish(self) -> MetaTypes {
        self.types
    }
}
