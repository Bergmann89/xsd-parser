use thiserror::Error;

use crate::models::{code::InvalidIdentPath, schema::NamespaceId, Ident};

/// Error that might be raised by the [`Generator`](crate::Generator).
#[derive(Debug, Error)]
pub enum Error {
    /// Unknown type identifier.
    ///
    /// Is raised if a specific identifier could not be resolved to it's
    /// corresponding type information.
    #[error("Unknown type identifier: {0}!")]
    UnknownType(Ident),

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
    #[error("Invalid default value for type {0:?}: {1}!")]
    InvalidDefaultValue(Ident, String),

    /// Invalid identifier.
    ///
    /// Is raised if the user passed a invalid identifier.
    #[error("{0}")]
    InvalidIdentifier(
        #[from]
        #[source]
        InvalidIdentPath,
    ),
}
