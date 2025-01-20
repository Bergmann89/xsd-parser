//! The `parser` module contains the schema [`Parser`] and all related types.

pub mod resolver;

mod error;

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::io::BufRead;
use std::path::Path;

use quick_xml::{
    events::Event,
    name::{LocalName, PrefixIter, QName, ResolveResult},
};
use resolver::{FileResolver, NoOpResolver, ResolveRequest, Resolver};
use tracing::instrument;
use url::{ParseError as UrlParseError, Url};

use crate::quick_xml::{
    DeserializeSync, Error as QuickXmlError, IoReader, SliceReader, XmlReader, XmlReaderSync,
};
use crate::schema::xs::{Import, Include, Schema, SchemaContent};
use crate::schema::{Namespace, NamespacePrefix, Schemas};

pub use self::error::Error;

/// The [`Parser`] is used to load and parse XML schema information from different
/// sources.
///
/// This structure can be used to load XML schemas information from different
/// sources using so called [`Resolver`]s. After the content of a schema was load
/// is it parsed and added to the list of schemas, managed by this parser.
///
/// The resulting [`Schemas`] instance can then be used by an
/// [`Interpreter`](crate::interpreter::Interpreter), to generate the more common
/// [`Types`](crate::types::Types) structure out of it.
#[must_use]
#[derive(Default, Debug)]
pub struct Parser<TResolver = NoOpResolver> {
    cache: HashSet<Url>,
    schemas: Schemas,
    pending: VecDeque<ResolveRequest>,

    resolver: TResolver,
    resolve_includes: bool,
}

impl Parser {
    /// Create a new [`Parser`] instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<TResolver> Parser<TResolver> {
    /// Set the default resolver for this parser.
    ///
    /// The default resolver is just a simple [`FileResolver`].
    pub fn with_default_resolver(self) -> Parser<FileResolver> {
        self.with_resolver(FileResolver::default())
    }

    /// Set a custom defined resolver for this parser.
    pub fn with_resolver<XResolver: Resolver + 'static>(
        self,
        resolver: XResolver,
    ) -> Parser<XResolver> {
        let Self { schemas, .. } = self;

        let cache = HashSet::new();
        let pending = VecDeque::new();

        Parser {
            cache,
            schemas,
            pending,

            resolver,
            resolve_includes: true,
        }
    }

    /// Enable or disable resolving includes of parsed XML schemas.
    pub fn resolve_includes(mut self, value: bool) -> Self {
        self.resolve_includes = value;

        self
    }

    /// Finish the parsing process by returning the generated [`Schemas`] instance
    /// containing all parsed schemas.
    pub fn finish(self) -> Schemas {
        self.schemas
    }
}

impl<TResolver> Parser<TResolver>
where
    TResolver: Resolver,
{
    /// Add the default namespaces to this parser.
    ///
    /// The default namespaces are:
    /// - [`NamespacePrefix::XS`] [`Namespace::XS`]
    /// - [`NamespacePrefix::XML`] [`Namespace::XML`]
    ///
    /// # Errors
    ///
    /// Forwards the errors from [`with_namespace`](Self::with_namespace).
    pub fn with_default_namespaces(self) -> Self {
        self.with_namespace(NamespacePrefix::XS, Namespace::XS)
            .with_namespace(NamespacePrefix::XML, Namespace::XML)
    }

    /// Add a new namespace to this parser.
    ///
    /// This method will add a new namespace to the parser. This can be useful to
    /// pre-heat the prefixes for known namespace, or to define namespaces for
    /// custom defined types.
    ///
    /// This will not add any schema information. It's just a namespace definition.
    ///
    /// # Errors
    ///
    /// Will return an error if a problem or mismatch with the already existing
    /// namespaces was encountered.
    pub fn with_namespace(mut self, prefix: NamespacePrefix, namespace: Namespace) -> Self {
        self.schemas
            .get_or_create_namespace_info_mut(Some(prefix), namespace);

        self
    }
}

impl<TResolver> Parser<TResolver>
where
    TResolver: Resolver,
    TResolver::Buffer: BufRead,
{
    /// Add a new XML schema from the passed string.
    ///
    /// This will parse the XML schema represented by the provided string and add
    /// all schema information to the resulting [`Schemas`] structure.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not parse the provided
    /// schema.
    #[instrument(err, level = "trace", skip(self, schema))]
    pub fn add_schema_from_str(mut self, schema: &str) -> Result<Self, Error<TResolver::Error>> {
        let reader = SliceReader::new(schema);
        let mut reader = SchemaReader::new(reader);

        let schema = Schema::deserialize(&mut reader)?;

        self.add_schema(schema, &reader.namespaces, None)?;
        self.resolve_pending()?;

        Ok(self)
    }

    /// Add a new XML schema from the passed `reader`.
    ///
    /// This will parse the XML schema represented by the provided reader and add
    /// all schema information to the resulting [`Schemas`] structure.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not read the data from
    /// the reader, or parse the schema provided by the reader.
    #[instrument(err, level = "trace", skip(self, reader))]
    pub fn add_schema_from_reader<R: BufRead>(
        mut self,
        reader: R,
    ) -> Result<Self, Error<TResolver::Error>> {
        let reader = IoReader::new(reader);
        let mut reader = SchemaReader::new(reader);

        let schema = Schema::deserialize(&mut reader)?;

        self.add_schema(schema, &reader.namespaces, None)?;
        self.resolve_pending()?;

        Ok(self)
    }

    /// Add a new XML schema from the passed file `path`.
    ///
    /// This will parse the XML schema represented by the provided filepath and
    /// add all schema information to the resulting [`Schemas`] structure.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not read the data from
    /// the file, or parse the schema content.
    #[instrument(err, level = "trace", skip(self))]
    pub fn add_schema_from_file<P: AsRef<Path> + Debug>(
        self,
        path: P,
    ) -> Result<Self, Error<TResolver::Error>> {
        let path = path.as_ref().canonicalize()?;
        let url = Url::from_file_path(&path).map_err(|()| Error::InvalidFilePath(path))?;

        self.add_schema_from_url(url)
    }

    /// Add a new XML schema from the passed file `url`.
    ///
    /// This will parse the XML schema represented by the provided url and
    /// add all schema information to the resulting [`Schemas`] structure.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not resolve the URL
    /// using the provided resolver or the data from the resolver could not be
    /// parsed.
    #[instrument(err, level = "trace", skip(self))]
    pub fn add_schema_from_url(mut self, url: Url) -> Result<Self, Error<TResolver::Error>> {
        let req = ResolveRequest::new(url);

        self.resolve_location(req)?;
        self.resolve_pending()?;

        Ok(self)
    }

    fn resolve_pending(&mut self) -> Result<(), Error<TResolver::Error>> {
        while let Some(req) = self.pending.pop_front() {
            self.resolve_location(req)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn resolve_location(&mut self, req: ResolveRequest) -> Result<(), Error<TResolver::Error>> {
        let Some((location, buffer)) = self.resolver.resolve(&req).map_err(Error::resolver)? else {
            return Err(Error::UnableToResolve(req.requested_location));
        };
        if self.cache.contains(&location) {
            return Ok(());
        }

        let reader = IoReader::new(buffer);
        let mut reader = SchemaReader::new(reader);

        let schema = Schema::deserialize(&mut reader)?;

        self.add_schema(schema, &reader.namespaces, Some(&location))?;
        self.cache.insert(location);

        Ok(())
    }

    fn add_schema(
        &mut self,
        schema: Schema,
        namespaces: &Namespaces,
        current_location: Option<&Url>,
    ) -> Result<(), Error<TResolver::Error>> {
        let Some(target_ns) = &schema.target_namespace else {
            return Err(Error::MissingTargetNamespace);
        };
        let target_ns = Namespace::from(target_ns.as_bytes().to_owned());

        let prefix = namespaces.get(&target_ns).cloned();

        if self.resolve_includes {
            for content in &schema.content {
                match content {
                    SchemaContent::Import(x) => {
                        if let Some(req) = import_req(x, target_ns.clone(), current_location)? {
                            self.pending.push_back(req);
                        }
                    }
                    SchemaContent::Include(x) => {
                        self.pending.push_back(include_req(
                            x,
                            target_ns.clone(),
                            current_location,
                        )?);
                    }
                    _ => (),
                }
            }
        }

        self.schemas.add_schema(prefix, target_ns, schema);

        Ok(())
    }
}

struct SchemaReader<R> {
    inner: R,
    namespaces: Namespaces,
}

type Namespaces = BTreeMap<Namespace, NamespacePrefix>;

impl<R> SchemaReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            namespaces: BTreeMap::new(),
        }
    }
}

impl<R> XmlReader for SchemaReader<R>
where
    R: XmlReader,
{
    fn resolve<'n>(&self, name: QName<'n>, attribute: bool) -> (ResolveResult<'_>, LocalName<'n>) {
        self.inner.resolve(name, attribute)
    }

    fn prefixes(&self) -> PrefixIter<'_> {
        self.inner.prefixes()
    }

    fn current_position(&self) -> u64 {
        self.inner.current_position()
    }

    fn error_position(&self) -> u64 {
        self.inner.error_position()
    }
}

impl<'a, R> XmlReaderSync<'a> for SchemaReader<R>
where
    R: XmlReaderSync<'a>,
{
    fn read_event(&mut self) -> Result<Event<'a>, QuickXmlError> {
        let event = self.inner.read_event()?;

        if let Event::Start(x) | Event::Empty(x) = &event {
            for a in x.attributes() {
                let a = a?;
                if matches!(a.key.prefix(), Some(x) if x.as_ref() == b"xmlns") {
                    let prefix = NamespacePrefix::new(a.key.local_name().as_ref().to_owned());
                    let namespace = Namespace::new(a.value.into_owned());

                    self.namespaces.entry(namespace).or_insert(prefix);
                }
            }
        }

        Ok(event)
    }
}

fn import_req(
    import: &Import,
    current_ns: Namespace,
    current_location: Option<&Url>,
) -> Result<Option<ResolveRequest>, UrlParseError> {
    let Some(location) = import.schema_location.as_ref() else {
        return Ok(None);
    };

    let location = if location.starts_with("http") {
        Url::parse(location)?
    } else {
        Url::parse(&format!("file://./{location}"))?
    };

    let mut req = ResolveRequest::new(location).current_ns(current_ns);

    if let Some(ns) = &import.namespace {
        req = req.requested_ns(Namespace::from(ns.as_bytes().to_owned()));
    }

    if let Some(current_location) = current_location {
        req = req.current_location(current_location.clone());
    }

    Ok(Some(req))
}

fn include_req(
    include: &Include,
    current_ns: Namespace,
    current_location: Option<&Url>,
) -> Result<ResolveRequest, UrlParseError> {
    let location = if include.schema_location.starts_with("http") {
        Url::parse(&include.schema_location)?
    } else {
        Url::parse(&format!("file://./{}", include.schema_location))?
    };

    let mut req = ResolveRequest::new(location).current_ns(current_ns);

    if let Some(current_location) = current_location {
        req = req.current_location(current_location.clone());
    }

    Ok(req)
}
