use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Error};

use url::Url;

use super::{strip_name_ext, ResolveRequest, ResolveResult, Resolver};

/// Implements a [`Resolver`] that can be used to load local files.
#[must_use]
#[derive(Default, Debug)]
pub struct FileResolver;

impl FileResolver {
    /// Create a new [`FileResolver`] instance.
    pub fn new() -> Self {
        Self
    }
}

impl Resolver for FileResolver {
    type Buffer = BufReader<File>;
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

        let Ok(path) = url.to_file_path() else {
            return Ok(None);
        };

        let file = File::open(&path)?;
        let buffer = BufReader::new(file);

        let name = path
            .iter()
            .next_back()
            .and_then(OsStr::to_str)
            .map(strip_name_ext)
            .map(ToOwned::to_owned);

        Ok(Some((name, url, buffer)))
    }
}
