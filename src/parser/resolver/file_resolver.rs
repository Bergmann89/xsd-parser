use std::env::current_dir;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

use url::{Host, Url};

use super::{ResolveRequest, Resolver};

/// Implements a [`Resolver`] that can be used to load local files.
#[must_use]
#[derive(Debug)]
pub struct FileResolver {
    search_paths: Vec<PathBuf>,
    use_current_path: bool,
}

impl FileResolver {
    /// Create a new [`FileResolver`] instance.
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
            use_current_path: false,
        }
    }

    /// Wether to use the path of the current schema as base path to resolve
    /// other schema files or not.
    pub fn use_current_path(mut self, value: bool) -> Self {
        self.use_current_path = value;

        self
    }

    /// Add an additional search path to the resolver.
    pub fn add_search_path<P>(mut self, path: P) -> Self
    where
        PathBuf: From<P>,
    {
        self.search_paths.push(PathBuf::from(path));

        self
    }

    /// Add additional search paths to the resolver.
    pub fn add_search_paths<P>(mut self, paths: P) -> Self
    where
        P: IntoIterator,
        PathBuf: From<P::Item>,
    {
        self.search_paths
            .extend(paths.into_iter().map(PathBuf::from));

        self
    }
}

impl Default for FileResolver {
    fn default() -> Self {
        Self::new()
            .use_current_path(true)
            .add_search_paths(current_dir().ok())
    }
}

impl Resolver for FileResolver {
    type Buffer = BufReader<File>;
    type Error = Error;

    fn resolve(
        &mut self,
        req: &ResolveRequest,
    ) -> Result<Option<(Url, Self::Buffer)>, Self::Error> {
        macro_rules! try_resolve {
            ($path:expr) => {{
                let path = $path;
                tracing::trace!("Try to resolve file \"{}\"", path.display());

                if let Ok(path) = path.canonicalize() {
                    let location = Url::from_file_path(&path).map_err(|()| {
                        Error::new(
                            ErrorKind::InvalidInput,
                            format!("Unable to crate URL from file: {}", path.display()),
                        )
                    })?;
                    let file = File::open(&path)?;
                    let buffer = BufReader::new(file);

                    return Ok(Some((location, buffer)));
                }
            }};
        }

        tracing::debug!("Received resolve request: {req:#?}");

        if req.requested_location.scheme() != "file" {
            return Ok(None);
        }

        /* HACK:
         *   Relative paths are not supported by url.
         *   The first part of the filepath is interpreted as host, so we join it manually.
         */
        let location = if let Some(Host::Domain(host)) = req.requested_location.host() {
            decode_file_segments(Some(host), req.requested_location.path())
        } else if let Ok(location) = req.requested_location.to_file_path() {
            location
        } else {
            decode_file_segments(None, req.requested_location.path())
        };

        if location.is_absolute() {
            try_resolve!(&location);

            return Ok(None);
        }

        if let Some(path) = self
            .use_current_path
            .then_some(())
            .and(req.current_location.as_ref())
            .filter(|url| url.scheme() == "file")
            .and_then(|url| url.to_file_path().ok())
            .as_deref()
            .and_then(Path::parent)
            .map(|dir| dir.join(&location))
        {
            try_resolve!(path);
        }

        for dir in &self.search_paths {
            try_resolve!(dir.join(&location));
        }

        Ok(None)
    }
}

fn decode_file_segments(host: Option<&str>, path: &str) -> PathBuf {
    let mut buf = String::new();

    if let Some(host) = host {
        buf.push_str(host);
    }

    for segment in path.split('/').filter(|s| !s.is_empty()) {
        #[cfg(windows)]
        buf.push('\\');

        #[cfg(not(windows))]
        buf.push('/');

        buf.push_str(segment);
    }

    PathBuf::from(buf)
}
