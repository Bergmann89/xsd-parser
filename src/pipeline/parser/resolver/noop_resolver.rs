use std::io::{BufRead, Error, ErrorKind};

use url::Url;

use super::{ResolveRequest, Resolver};

/// Implements a [`Resolver`] that resolves nothing.
///
/// This is the default resolver that is used if no other resolver was provided.
#[must_use]
#[derive(Default, Debug)]
pub struct NoOpResolver;

impl NoOpResolver {
    /// Create a new [`NoOpResolver`] resolver instance.
    pub fn new() -> Self {
        Self
    }
}

impl Resolver for NoOpResolver {
    type Buffer = Box<dyn BufRead + 'static>;
    type Error = Error;

    fn resolve(
        &mut self,
        req: &ResolveRequest,
    ) -> Result<Option<(Url, Self::Buffer)>, Self::Error> {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Unable to resolve requested location: {req}"),
        ))
    }
}
