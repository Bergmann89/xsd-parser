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

use crate::types::{Ident, Types};

pub use self::dynamic_to_choice::ConvertDynamicToChoice;
pub use self::empty_enums::{RemoveEmptyEnumVariants, RemoveEmptyEnums};
pub use self::empty_unions::{RemoveDuplicateUnionVariants, RemoveEmptyUnions};
pub use self::flatten_complex_type::FlattenComplexTypes;
pub use self::flatten_unions::FlattenUnions;
pub use self::merge_choice_cardinality::MergeChoiceCardinalities;
pub use self::merge_enum_unions::MergeEnumUnions;
pub use self::remove_duplicates::RemoveDuplicates;
pub use self::resolve_typedefs::ResolveTypedefs;
pub use self::unrestricted_base::UseUnrestrictedBaseType;

use self::misc::{BaseMap, TypedefMap};

/// The [`TypeTransformer`] trait is used to implement modules that in different ways manipulate the types in a [`Types`] instance. Generally, it's used for transformations that reduce the size or complexity of the types, making them more compatible with tools further down the line.
pub trait TypeTransformer {
    /// The error type that is returned by the [`TypeTransformer::transform`] method.
    type Error: std::fmt::Debug;

    /// Transforms the types in the given [`Types`] instance according to the specific transformer.
    ///
    /// # Errors
    /// If the transformation fails, an error of type [`Self::Error`] is returned which should contain more information about the failure. Since this is entirely transformer specific, you should refer to the documentation of the specific transformer for more information.
    fn transform(self, types: &mut Types) -> Result<(), Error>;
}

impl Types {
    /// Applies the given [`TypeTransformer`] to the types in this [`Types`] instance.
    ///
    /// # Errors
    /// If the transformation fails, its error is returned.
    pub fn apply_transformer<T: TypeTransformer<Error = Error>>(
        mut self,
        transformer: T,
    ) -> Result<Self, T::Error> {
        transformer.transform(&mut self)?;
        Ok(self)
    }

    /// Applies the given [`TypeTransformer`] to the types in this [`Types`] instance if the condition is true.
    /// Otherwise, it returns the types unchanged.
    ///
    /// # Errors
    /// If the transformation fails, its error is returned.
    pub fn apply_transformer_if<T: TypeTransformer<Error = Error>>(
        self,
        transformer: T,
        condition: bool,
    ) -> Result<Self, T::Error> {
        if condition {
            self.apply_transformer(transformer)
        } else {
            Ok(self)
        }
    }
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
