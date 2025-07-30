//! Defines the [`Resolver`] trait and different resolver implementations.

mod file_resolver;
mod many_resolver;
mod noop_resolver;

#[cfg(feature = "web-resolver")]
mod web_resolver;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use url::Url;

use crate::models::schema::Namespace;

pub use self::file_resolver::FileResolver;
pub use self::many_resolver::ManyResolver;
pub use self::noop_resolver::NoOpResolver;

#[cfg(feature = "web-resolver")]
pub use self::web_resolver::WebResolver;

/// Trait that defines a so called resolver that can be used to load schema
/// information from different sources.
pub trait Resolver: Debug {
    /// Buffer that contains the data of the resolved schema.
    type Buffer;

    /// Error that is returned by the resolver.
    type Error: Display;

    /// Try to resolve the schema information from the passed request.
    ///
    /// This methods tries to resolve the schema information by using the
    /// information provided by the passed request. If the operation was
    /// successful the url and a buffer that contains the schema information
    /// is returned in `Ok(Some((url, buffer)))`. If the request could not be
    /// resolved `Ok(None)` is returned.
    ///
    /// # Errors
    ///
    /// May return any error if resolving the schema information was not
    /// successful.
    fn resolve(&mut self, req: &ResolveRequest)
        -> Result<Option<(Url, Self::Buffer)>, Self::Error>;
}

/// Contains information about the requested resolve action.
#[must_use]
#[derive(Debug)]
pub struct ResolveRequest {
    /// Location of the requested schema.
    pub requested_location: String,

    /// Namespace of the requested schema.
    pub requested_ns: Option<Namespace>,

    /// Location of the current processed schema.
    pub current_location: Option<Url>,

    /// Namespace of the current processed schema.
    pub current_ns: Option<Namespace>,

    /// Indicates wether the request was initiated
    /// by the user, an import or an include.
    pub request_type: ResolveRequestType,
}

/// Distinction of included vs imported schemas
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResolveRequestType {
    /// Schema was added directly by the user
    #[default]
    UserDefined,
    /// Schema has been included
    IncludeRequest,
    /// Schema has been imported
    ImportRequest,
}

impl ResolveRequest {
    /// Create a new [`ResolveRequest`] instance from the passed `requested_location` and `request_type`.
    pub fn new<X: Into<String>>(requested_location: X, request_type: ResolveRequestType) -> Self {
        Self {
            requested_location: requested_location.into(),
            requested_ns: None,

            current_location: None,
            current_ns: None,
            request_type,
        }
    }

    /// Set the `requested_ns` field of this [`ResolveRequest`] instance.
    pub fn requested_ns(mut self, ns: Namespace) -> Self {
        self.requested_ns = Some(ns);

        self
    }

    /// Set the `current_ns` field of this [`ResolveRequest`] instance.
    pub fn current_ns(mut self, ns: Namespace) -> Self {
        self.current_ns = Some(ns);

        self
    }

    /// Set the `current_location` field of this [`ResolveRequest`] instance.
    pub fn current_location<X: Into<Url>>(mut self, location: X) -> Self {
        self.current_location = Some(location.into());

        self
    }
}

impl Display for ResolveRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Location={}", self.requested_location)?;

        if let Some(ns) = &self.requested_ns {
            write!(f, "({})", ns)?;
        }

        if let Some(current) = &self.current_location {
            write!(f, "Current={}", current)?;
        }

        if let Some(current_ns) = &self.current_ns {
            write!(f, "({})", current_ns)?;
        }

        Ok(())
    }
}
