use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::path::PathBuf;

use thiserror::Error;
use url::{ParseError as UrlParseError, Url};

use xsd_parser_types::misc::Namespace;
use xsd_parser_types::quick_xml::Error as XmlError;

use super::resolver::ResolveRequest;

/// Represents the errors that are raised by the [`Parser`](super::Parser).
#[derive(Debug, Error)]
pub enum Error<E> {
    /// An IO error occurred.
    #[error("IO Error: {0}")]
    IoError(#[from] IoError),

    /// Error while interpreting the XML for the schema.
    #[error("{0}")]
    XmlError(#[from] XmlErrorWithLocation),

    /// Error while paring the URL.
    #[error("URL Parse Error: {0}")]
    UrlParseError(#[from] UrlParseError),

    /// Unable to resolve the requested resource.
    #[error("Unable to resolve requested resource: {0}")]
    UnableToResolve(Box<ResolveRequest>),

    /// The target namespace of the included schema does not
    /// match the target namespace of the including schema.
    #[error(
        "Mismatching target namespace (location={location}, found={found}, expected={expected})"
    )]
    MismatchingTargetNamespace {
        /// Location of the schema that was processed.
        location: Url,

        /// Target namespace that was found in the included schema.
        found: Namespace,

        /// Target namespace that was expected (i.e. the target namespace of the including schema).
        expected: Namespace,
    },

    /// Error while resolving the requested resource.
    #[error("Resolver Error: {0}")]
    Resolver(E),

    /// Invalid file path!
    ///
    /// Is raised if the file path could not be converted to an [`Url`](url::Url).
    #[error("Invalid file path: {0}!")]
    InvalidFilePath(PathBuf),
}

impl<E> Error<E> {
    /// Create a [`Error::Resolver`] from the passed `error`.
    pub fn resolver<X: Into<E>>(error: X) -> Self {
        Self::Resolver(error.into())
    }
}

/// Error that is raised during deserialization of XSD structures.
#[derive(Debug, Error)]
pub struct XmlErrorWithLocation {
    /// Error that was raised.
    pub error: XmlError,

    /// Location of the schema that was processed.
    pub location: Option<Url>,
}

impl From<XmlError> for XmlErrorWithLocation {
    fn from(error: XmlError) -> Self {
        Self {
            error,
            location: None,
        }
    }
}

impl Display for XmlErrorWithLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { error, location } = self;

        write!(f, "XML Error: ")?;
        error.fmt(f)?;

        if let Some(location) = location {
            write!(f, "\n    in schema ")?;
            location.fmt(f)?;
        }

        Ok(())
    }
}
