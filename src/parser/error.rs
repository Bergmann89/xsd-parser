use std::io::Error as IoError;
use std::path::PathBuf;

use thiserror::Error;
use url::{ParseError as UrlParseError, Url};

use crate::quick_xml::Error as XmlError;

/// Represents the errors that are raised by the [`Parser`](super::Parser).
#[derive(Debug, Error)]
pub enum Error<E> {
    /// An IO error occurred.
    #[error("IO Error: {0}")]
    IoError(#[from] IoError),

    /// Error while interpreting the XML for the schema.
    #[error("XML Error: {0}")]
    XmlError(#[from] XmlError),

    /// Error while paring the URL.
    #[error("URL Parse Error: {0}")]
    UrlParseError(#[from] UrlParseError),

    /// Unable to resolve the requested resource.
    #[error("Unable to resolve requested resource: {0}")]
    UnableToResolve(Url),

    /// Error while resolving the requested resource.
    #[error("Resolver Error: {0}")]
    Resolver(E),

    /// Invalid file path!
    ///
    /// Is raised if the file path could not be converted to an [`Url`].
    #[error("Invalid file path: {0}!")]
    InvalidFilePath(PathBuf),

    /// Schema is missing a target namespace!
    #[error("Schema is missing a target namespace!")]
    MissingTargetNamespace,

    /// Schema is missing a namespace prefix for the target namespace!
    #[error("Schema is missing a namespace prefix for the target namespace!")]
    MissingTargetNamespacePrefix,
}

impl<E> Error<E> {
    /// Create a [`Error::Resolver`] from the passed `error`.
    pub fn resolver<X: Into<E>>(error: X) -> Self {
        Self::Resolver(error.into())
    }
}
