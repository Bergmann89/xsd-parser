use std::io::BufReader;

use reqwest::{
    blocking::{Client, Response},
    Error,
};
use url::Url;

use super::{ResolveRequest, Resolver};

/// Implements a [`Resolver`] that resolves `http` and `https` resources.
#[must_use]
#[derive(Default, Debug)]
pub struct WebResolver {
    client: Option<Client>,
}

impl WebResolver {
    /// Create a new [`WebResolver`] instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Resolver for WebResolver {
    type Buffer = BufReader<Response>;
    type Error = Error;

    fn resolve(
        &mut self,
        req: &ResolveRequest,
    ) -> Result<Option<(Url, Self::Buffer)>, Self::Error> {
        if !matches!(req.requested_location.scheme(), "http" | "https") {
            return Ok(None);
        }

        tracing::trace!("Try to resolve \"{}\"", req.requested_location);

        let client = self.client.get_or_insert_with(Client::new);

        let res = client.get(req.requested_location.clone()).send()?;
        let buffer = BufReader::new(res);
        let location = req.requested_location.clone();

        Ok(Some((location, buffer)))
    }
}
