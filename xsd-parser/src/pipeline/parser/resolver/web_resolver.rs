use std::io::BufReader;

use reqwest::{
    blocking::{Client, Response},
    Error,
};
use url::Url;

use super::{strip_name_ext, ResolveRequest, ResolveResult, Resolver};

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

    fn resolve(&mut self, req: &ResolveRequest) -> ResolveResult<Self> {
        let url = if let Some(current) = &req.current_location {
            current.join(&req.requested_location)
        } else {
            Url::parse(&req.requested_location)
        };

        let Ok(url) = url else {
            return Ok(None);
        };

        if !matches!(url.scheme(), "http" | "https") {
            return Ok(None);
        }

        let client = self.client.get_or_insert_with(Client::new);

        let res = client.get(url.clone()).send()?;
        let buffer = BufReader::new(res);

        let name = url
            .as_str()
            .split(['\\', '/'])
            .next_back()
            .map(strip_name_ext)
            .map(ToOwned::to_owned);

        Ok(Some((name, url, buffer)))
    }
}
