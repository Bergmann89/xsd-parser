use std::error::Error as StdError;
use std::io::BufRead;

use anyhow::Error;
use url::Url;

use super::{ResolveRequest, Resolver};

/// Implements a [`Resolver`] that can combines multiple different other resolvers
/// into one single resolver.
#[derive(Default, Debug)]
pub struct ManyResolver {
    resolvers: Vec<Box<dyn Resolver<Buffer = BoxedBuffer, Error = Error>>>,
}

type BoxedBuffer = Box<dyn BufRead + 'static>;

#[derive(Debug)]
#[must_use]
struct MappedResolver<T>(T);

impl ManyResolver {
    /// Create a new empty [`ManyResolver`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `resolver` to this [`ManyResolver`].
    #[must_use]
    pub fn add_resolver<R>(mut self, resolver: R) -> Self
    where
        R: Resolver + 'static,
        R::Buffer: BufRead + 'static,
        R::Error: StdError + Send + Sync + 'static,
    {
        self.resolvers.push(Box::new(MappedResolver(resolver)));

        self
    }
}

impl Resolver for ManyResolver {
    type Buffer = BoxedBuffer;
    type Error = Error;

    fn resolve(
        &mut self,
        req: &ResolveRequest,
    ) -> Result<Option<(Url, Self::Buffer)>, Self::Error> {
        for resolver in self.resolvers.iter_mut() {
            if let Some((location, buffer)) = resolver.resolve(req)? {
                return Ok(Some((location, buffer)));
            }
        }

        Ok(None)
    }
}

/* MappedResolver */

impl<R> Resolver for MappedResolver<R>
where
    R: Resolver + 'static,
    R::Buffer: BufRead + 'static,
    R::Error: StdError + Send + Sync + 'static,
{
    type Buffer = BoxedBuffer;
    type Error = Error;

    fn resolve(
        &mut self,
        req: &ResolveRequest,
    ) -> Result<Option<(Url, Self::Buffer)>, Self::Error> {
        match self.0.resolve(req) {
            Ok(Some((location, buffer))) => Ok(Some((location, Box::new(buffer)))),
            Ok(None) => Ok(None),
            Err(error) => Err(Error::from(error)),
        }
    }
}
