use thiserror::Error;

use crate::models::{code::InvalidIdentPath, schema::NamespaceId};

/// Error that might be raised by the [`Generator`](crate::Generator).
#[derive(Debug, Error)]
pub enum Error {
    /// Unknown namespace.
    ///
    /// Is raised if a specific namespace id could not be resolved to it's
    /// corresponding namespace information.
    #[error("Unknown namespace: {0:?}!")]
    UnknownNamespace(NamespaceId),

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
