//! The `optimizer` module contains the type information [`Optimizer`] and all related types.

mod dynamic_to_choice;
mod empty_enums;
mod empty_unions;
mod flatten_complex_type;
mod flatten_unions;
mod merge_choice_cardinality;
mod merge_enum_unions;
mod misc;
mod remove_duplicates;
mod resolve_typedefs;
mod unrestricted_base;

use thiserror::Error;

use crate::models::{meta::MetaTypes, Ident};

use self::misc::{BaseMap, TypedefMap};

/// The [`Optimizer`] is a structure that can be used to reduce the size and the
/// complexity of a [`Types`] instance.
///
/// The optimizer contains different optimizations that could be applied to a
/// [`Types`] instance. Optimizations are usually used to reduce the size or the
/// complexity of the different types.
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
    UnknownType(Ident),

    /// The type is not a union type.
    ///
    /// Is raised if a type is expected to be a union, but it is not.
    #[error("The type is not a union type: {0}!")]
    ExpectedUnion(Ident),

    /// The type is not a complex choice type.
    ///
    /// Is raised if a type is expected to be a complex choice, but it is not.
    #[error("The type is not a complex choice type: {0}!")]
    ExpectedComplexChoice(Ident),

    /// The type is not a complex type.
    ///
    /// Is raised if a type is expected to be a complex type, but it is not.
    #[error("The type is not a complex type: {0}!")]
    ExpectedComplexType(Ident),

    /// The complex type is missing a content type.
    ///
    /// Is raised if the content type of a complex type could not be resolved.
    #[error("Complex type {0} is missing a content type!")]
    MissingContentType(Ident),
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

    /// Finish the optimization and return the resulting [`Types`].
    #[must_use]
    pub fn finish(self) -> MetaTypes {
        self.types
    }
}
