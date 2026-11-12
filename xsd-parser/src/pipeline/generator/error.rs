use thiserror::Error;

use crate::{
    models::{code::InvalidIdentPath, schema::NamespaceId, TypeIdent},
    pipeline::generator::ValueGeneratorMode,
};

/// Error that might be raised by the [`Generator`](super::Generator).
#[derive(Debug, Error)]
pub enum Error {
    /// Unknown type identifier.
    ///
    /// Is raised if a specific identifier could not be resolved to it's
    /// corresponding type information.
    #[error("Unknown type identifier: {0}!")]
    UnknownType(TypeIdent),

    /// A custom type alias was requested without an included Rust type path.
    #[error("Custom type alias {0} requires an include path!")]
    MissingCustomAliasTarget(TypeIdent),

    /// Unknown namespace.
    ///
    /// Is raised if a specific namespace id could not be resolved to it's
    /// corresponding namespace information.
    #[error("Unknown namespace: {0:?}!")]
    UnknownNamespace(NamespaceId),

    /// Invalid default value.
    ///
    /// Is raised if the default value for an attribute defined in the schema
    /// could not be converted to a suitable default code snippet.
    #[error("Invalid default value for type (ident={ident:?}, value={value}, mode: {mode:?})!")]
    InvalidDefaultValue {
        /// The identifier of the type for which the value should be generated.
        ident: TypeIdent,

        /// The value that should be generated.
        value: String,

        /// The mode that determines how the value should be generated.
        mode: ValueGeneratorMode,
    },

    /// Invalid identifier.
    ///
    /// Is raised if the user passed a invalid identifier.
    #[error("{0}")]
    InvalidIdentifier(
        #[from]
        #[source]
        InvalidIdentPath,
    ),

    /// Duplicate type name.
    ///
    /// Is raised if two different schema types would be generated with the
    /// same Rust path, which results in code that does not compile.
    #[error(
        "Type name `{path}` is generated for both {first} and {second}! \
         Use different module flags or a custom naming to tell them apart."
    )]
    DuplicateTypeName {
        /// The Rust path that is used by both types.
        path: String,

        /// Description of the schema type that claimed the path first.
        first: String,

        /// Description of the schema type that collides with `first`.
        second: String,
    },

    /// Generated type shadows a build-in type.
    ///
    /// Is raised if a schema type would be generated with the name of a
    /// build-in Rust type or trait (like `String` or `Option`) that the
    /// generated code refers to, which results in code that does not compile.
    #[error(
        "Type name `{name}` is generated for {type_} and shadows the build-in `{name}`! \
         Enable `GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS` or use a custom naming."
    )]
    ShadowedBuildIn {
        /// The name of the shadowed build-in type or trait.
        name: String,

        /// Description of the schema type that uses the name.
        type_: String,
    },
}
