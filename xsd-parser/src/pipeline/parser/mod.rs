//! XML Schema (XSD) parser and resolver infrastructure.
//!
//! This module defines the [`Parser`] type and supporting logic for loading,
//! resolving, and parsing XML Schema documents (`.xsd`) into structured [`Schemas`].
//!
//! The parser supports various input sources (e.g., files, strings, URLs) and
//! handles `<import>` and `<include>` logic using pluggable [`Resolver`] implementations.
//!
//! Parsed schemas can be passed to the [`Interpreter`](crate::Interpreter) for
//! further transformation into semantic types.
//!
//! # Example
//! ```rust,ignore
//! let schemas = Parser::new()
//!     .with_default_resolver()
//!     .add_schema_from_file("schema.xsd")?
//!     .finish();
//! ```

pub mod resolver;

mod error;

use std::borrow::Cow;
use std::collections::{btree_map::Entry, BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::io::BufRead;
use std::path::Path;

use quick_xml::events::Event;
use resolver::{FileResolver, NoOpResolver, ResolveRequest};
use tracing::instrument;
use url::Url;

use xsd_parser_types::misc::{Namespace, NamespacePrefix};
use xsd_parser_types::quick_xml::{
    DeserializeSync, Error as QuickXmlError, IoReader, SliceReader, XmlReader, XmlReaderSync,
};

use crate::models::schema::{
    xs::{Import, Include, Schema, SchemaContent},
    NamespaceId, NamespaceInfo, Schemas,
};
use crate::models::schema::{SchemaId, SchemaInfo};
use crate::pipeline::parser::resolver::ResolveRequestType;

pub use self::error::{Error, XmlErrorWithLocation};
pub use self::resolver::Resolver;

/// The [`Parser`] is responsible for loading and parsing XML Schema documents into
/// a structured [`Schemas`] representation.
///
/// It supports resolution of schema references such as `<import>` and `<include>`
/// using a pluggable [`Resolver`], and can read schema content from strings, files,
/// or URLs.
///
/// Internally, the parser maintains a queue of pending schema loads and a cache
/// to prevent duplicate resolutions. Once all schemas are processed, the
/// [`finish`](Self::finish) method returns the final [`Schemas`] collection.
///
/// A generic resolver type `TResolver` controls how external schemas are fetched.
/// By default, a no-op resolver is used, but file-based or custom resolvers can
/// be injected using [`with_resolver`](Self::with_resolver).
#[must_use]
#[derive(Default, Debug)]
pub struct Parser<TResolver = NoOpResolver> {
    cache: HashSet<Url>,
    entries: Vec<ParserEntry>,
    pending: VecDeque<ResolveRequest>,

    resolver: TResolver,
    resolve_includes: bool,
    generate_prefixes: bool,
    alternative_prefixes: bool,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum ParserEntry {
    AnonymousNamespace,
    Namespace {
        prefix: NamespacePrefix,
        namespace: Namespace,
    },
    Schema {
        name: Option<String>,
        schema: Schema,
        location: Option<Url>,
        target_ns: Option<Namespace>,
        namespaces: Namespaces,
    },
}

#[derive(Debug)]
struct SchemasBuilder {
    schemas: Schemas,
    prefix_cache: HashMap<Option<Namespace>, PrefixEntry>,

    generate_prefixes: bool,
    alternative_prefixes: bool,
}

#[derive(Default, Debug)]
struct PrefixEntry {
    prefix: Option<NamespacePrefix>,
    alt_prefixes: HashSet<NamespacePrefix>,
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
        self.with_resolver(FileResolver)
    }

    /// Set a custom defined resolver for this parser.
    pub fn with_resolver<XResolver: Resolver + 'static>(
        self,
        resolver: XResolver,
    ) -> Parser<XResolver> {
        let Self { entries, .. } = self;

        let cache = HashSet::new();
        let pending = VecDeque::new();

        Parser {
            cache,
            entries,
            pending,

            resolver,
            resolve_includes: true,
            generate_prefixes: true,
            alternative_prefixes: true,
        }
    }

    /// Enable or disable resolving includes of parsed XML schemas.
    pub fn resolve_includes(mut self, value: bool) -> Self {
        self.resolve_includes = value;

        self
    }

    /// Instructs the parser to generate unique prefixes for a certain namespace
    /// if its actual prefix is already used.
    pub fn generate_prefixes(mut self, value: bool) -> Self {
        self.generate_prefixes = value;

        self
    }

    /// Instructs the parser to use alternate prefixes known from other
    /// schemas for a certain namespace if its actual prefix is unknown or
    /// already used.
    pub fn alternative_prefixes(mut self, value: bool) -> Self {
        self.alternative_prefixes = value;

        self
    }

    /// Finish the parsing process by returning the generated [`Schemas`] instance
    /// containing all parsed schemas.
    pub fn finish(self) -> Schemas {
        let builder = SchemasBuilder {
            schemas: Schemas::default(),
            prefix_cache: HashMap::new(),

            generate_prefixes: self.generate_prefixes,
            alternative_prefixes: self.alternative_prefixes,
        };

        builder.build(self.entries)
    }
}

impl<TResolver> Parser<TResolver>
where
    TResolver: Resolver,
{
    /// Add the default namespaces to this parser.
    ///
    /// The default namespaces are:
    /// - The anonymous namespace
    /// - [`NamespacePrefix::XS`] [`Namespace::XS`]
    /// - [`NamespacePrefix::XML`] [`Namespace::XML`]
    ///
    /// # Errors
    ///
    /// Forwards the errors from [`with_namespace`](Self::with_namespace).
    pub fn with_default_namespaces(self) -> Self {
        self.with_anonymous_namespace()
            .with_namespace(NamespacePrefix::XS, Namespace::XS)
            .with_namespace(NamespacePrefix::XML, Namespace::XML)
            .with_namespace(NamespacePrefix::XSI, Namespace::XSI)
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
        self.entries
            .push(ParserEntry::Namespace { prefix, namespace });

        self
    }

    /// Adds the anonymous namespace to the resulting [`Schemas`] structure.
    ///
    /// The anonymous namespace does not have a namespace prefix, or a namespace
    /// URI. It is used for type definitions and schemas that do not provide a
    /// target namespace. Additionally it can be used to provide user defined types.
    pub fn with_anonymous_namespace(mut self) -> Self {
        self.entries.push(ParserEntry::AnonymousNamespace);

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
    pub fn add_schema_from_str(self, schema: &str) -> Result<Self, Error<TResolver::Error>> {
        self.add_named_schema_from_str_impl(None, schema)
    }

    /// Add a new XML schema from the passed string.
    ///
    /// This will parse the XML schema represented by the provided string and add
    /// all schema information to the resulting [`Schemas`] structure using the
    /// passed `name` for the schema.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not parse the provided
    /// schema.
    #[instrument(err, level = "trace", skip(self, schema))]
    pub fn add_named_schema_from_str(
        self,
        name: String,
        schema: &str,
    ) -> Result<Self, Error<TResolver::Error>> {
        self.add_named_schema_from_str_impl(Some(name), schema)
    }

    #[instrument(err, level = "trace", skip(self, schema))]
    fn add_named_schema_from_str_impl(
        mut self,
        name: Option<String>,
        schema: &str,
    ) -> Result<Self, Error<TResolver::Error>> {
        let reader = SliceReader::new(schema);
        let mut reader = SchemaReader::new(reader);

        let schema = Schema::deserialize(&mut reader).map_err(XmlErrorWithLocation::from)?;

        self.add_schema(name, schema, None, reader.namespaces);
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
    pub fn add_schema_from_reader<R: BufRead>(
        self,
        reader: R,
    ) -> Result<Self, Error<TResolver::Error>> {
        self.add_named_schema_from_reader_impl(None, reader)
    }

    /// Add a new XML schema from the passed `reader`.
    ///
    /// This will parse the XML schema represented by the provided reader and add
    /// all schema information to the resulting [`Schemas`] structure using the
    /// passed `name` as name for the schema.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not read the data from
    /// the reader, or parse the schema provided by the reader.
    pub fn add_named_schema_from_reader<R: BufRead>(
        self,
        name: String,
        reader: R,
    ) -> Result<Self, Error<TResolver::Error>> {
        self.add_named_schema_from_reader_impl(Some(name), reader)
    }

    #[instrument(err, level = "trace", skip(self, reader))]
    fn add_named_schema_from_reader_impl<R: BufRead>(
        mut self,
        name: Option<String>,
        reader: R,
    ) -> Result<Self, Error<TResolver::Error>> {
        let reader = IoReader::new(reader);
        let mut reader = SchemaReader::new(reader);

        let schema = Schema::deserialize(&mut reader).map_err(XmlErrorWithLocation::from)?;

        self.add_schema(name, schema, None, reader.namespaces);
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

    /// Add multiple XML schemas from the passed paths iterator.
    ///
    /// # Errors
    ///
    /// Will return an suitable error if the parser could not read the data from
    /// any file, or parse the schema content.
    #[instrument(err, level = "trace", skip(self))]
    pub fn add_schema_from_files<I>(mut self, paths: I) -> Result<Self, Error<TResolver::Error>>
    where
        I: IntoIterator + Debug,
        I::Item: AsRef<Path> + Debug,
    {
        for path in paths {
            self = self.add_schema_from_file(path)?;
        }

        Ok(self)
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
        let req = ResolveRequest::new(url, ResolveRequestType::UserDefined);

        self.resolve_location(req)?;
        self.resolve_pending()?;

        Ok(self)
    }

    fn add_pending(&mut self, req: ResolveRequest) {
        tracing::debug!("Add pending resolve request: {req:#?}");

        self.pending.push_back(req);
    }

    fn resolve_pending(&mut self) -> Result<(), Error<TResolver::Error>> {
        while let Some(req) = self.pending.pop_front() {
            self.resolve_location(req)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn resolve_location(&mut self, req: ResolveRequest) -> Result<(), Error<TResolver::Error>> {
        tracing::debug!("Process resolve request: {req:#?}");

        let Some((name, location, buffer)) =
            self.resolver.resolve(&req).map_err(Error::resolver)?
        else {
            return Err(Error::UnableToResolve(Box::new(req)));
        };
        if self.cache.contains(&location) {
            return Ok(());
        }

        let reader = IoReader::new(buffer);
        let reader = SchemaReader::new(reader);
        let mut reader = reader.with_error_info();

        let mut schema =
            Schema::deserialize(&mut reader).map_err(|error| XmlErrorWithLocation {
                error,
                location: Some(location.clone()),
            })?;

        if schema.target_namespace.is_none()
            && ResolveRequestType::IncludeRequest == req.request_type
        {
            if let Some(current_ns) = req.current_ns {
                let inherited_ns = current_ns.to_string();
                schema.target_namespace = Some(inherited_ns);
            }
        }

        let reader = reader.into_inner();

        self.add_schema(name, schema, Some(location.clone()), reader.namespaces);
        self.cache.insert(location);

        Ok(())
    }

    fn add_schema(
        &mut self,
        name: Option<String>,
        schema: Schema,
        location: Option<Url>,
        namespaces: Namespaces,
    ) {
        tracing::debug!(
            "Process schema (location={:?}, target_namespace={:?}",
            location.as_ref().map(Url::as_str),
            &schema.target_namespace
        );

        let target_ns = schema
            .target_namespace
            .as_deref()
            .map(|ns| Namespace::from(ns.as_bytes().to_owned()));

        if self.resolve_includes {
            for content in &schema.content {
                match content {
                    SchemaContent::Import(x) => {
                        if let Some(req) = import_req(x, target_ns.clone(), location.as_ref()) {
                            self.add_pending(req);
                        }
                    }
                    SchemaContent::Include(x) => {
                        self.add_pending(include_req(x, target_ns.clone(), location.as_ref()));
                    }
                    _ => (),
                }
            }
        }

        self.entries.push(ParserEntry::Schema {
            name,
            schema,
            location,
            target_ns,
            namespaces,
        });
    }
}

struct SchemaReader<R> {
    inner: R,
    namespaces: Namespaces,
}

type Namespaces = BTreeMap<Option<Namespace>, Vec<NamespacePrefix>>;

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
    fn extend_error(&self, error: QuickXmlError) -> QuickXmlError {
        self.inner.extend_error(error)
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

                    self.namespaces
                        .entry(Some(namespace))
                        .or_default()
                        .push(prefix);
                }
            }
        }

        Ok(event)
    }
}

impl SchemasBuilder {
    fn build(mut self, entries: Vec<ParserEntry>) -> Schemas {
        self.build_cache(&entries);

        for entry in entries {
            match entry {
                ParserEntry::AnonymousNamespace => {
                    self.get_or_create_namespace_info_mut(None);
                }
                ParserEntry::Namespace { namespace, .. } => {
                    self.get_or_create_namespace_info_mut(Some(namespace));
                }
                ParserEntry::Schema {
                    name,
                    schema,
                    location,
                    target_ns,
                    ..
                } => {
                    self.add_schema(target_ns, name, location, schema);
                }
            }
        }

        self.determine_prefixes();

        self.schemas
    }

    fn build_cache(&mut self, entries: &[ParserEntry]) {
        for entry in entries {
            match entry {
                ParserEntry::AnonymousNamespace => {
                    self.prefix_cache.entry(None).or_default();
                }
                ParserEntry::Namespace { prefix, namespace } => {
                    self.prefix_cache
                        .entry(Some(namespace.clone()))
                        .or_default()
                        .prefix = Some(prefix.clone());
                }
                ParserEntry::Schema {
                    target_ns,
                    namespaces,
                    ..
                } => {
                    let prefix = namespaces
                        .get(target_ns)
                        .and_then(|prefixes| prefixes.first())
                        .cloned();
                    let entry = self.prefix_cache.entry(target_ns.clone()).or_default();

                    if entry.prefix.is_none() {
                        entry.prefix = prefix;
                    } else if let Some(prefix) = prefix {
                        entry.alt_prefixes.insert(prefix);
                    }

                    for (namespace, prefixes) in namespaces {
                        for prefix in prefixes {
                            self.prefix_cache
                                .entry(namespace.clone())
                                .or_default()
                                .alt_prefixes
                                .insert(prefix.clone());
                        }
                    }
                }
            }
        }
    }

    fn add_schema(
        &mut self,
        namespace: Option<Namespace>,
        name: Option<String>,
        location: Option<Url>,
        schema: Schema,
    ) {
        self.schemas.last_schema_id = self.schemas.last_schema_id.wrapping_add(1);
        let schema_id = SchemaId(self.schemas.last_schema_id);

        let (namespace_id, namespace_info) = self.get_or_create_namespace_info_mut(namespace);
        namespace_info.schemas.push(schema_id);

        match self.schemas.schemas.entry(schema_id) {
            Entry::Vacant(e) => e.insert(SchemaInfo {
                name,
                schema,
                location,
                namespace_id,
            }),
            Entry::Occupied(_) => crate::unreachable!(),
        };
    }

    fn get_or_create_namespace_info_mut(
        &mut self,
        namespace: Option<Namespace>,
    ) -> (NamespaceId, &mut NamespaceInfo) {
        match self.schemas.known_namespaces.entry(namespace) {
            Entry::Occupied(e) => {
                let id = *e.get();
                let info = self.schemas.namespace_infos.get_mut(&id).unwrap();

                (id, info)
            }
            Entry::Vacant(e) => {
                let id = if e.key().is_none() {
                    NamespaceId::ANONYMOUS
                } else {
                    self.schemas.last_namespace_id = self.schemas.last_namespace_id.wrapping_add(1);

                    NamespaceId(self.schemas.last_namespace_id)
                };

                let namespace = e.key().clone();
                e.insert(id);

                let info = match self.schemas.namespace_infos.entry(id) {
                    Entry::Vacant(e) => e.insert(NamespaceInfo::new(namespace)),
                    Entry::Occupied(_) => crate::unreachable!(),
                };

                (id, info)
            }
        }
    }

    fn determine_prefixes(&mut self) {
        // Insert main prefixes
        for (id, info) in &mut self.schemas.namespace_infos {
            if info.prefix.is_some() {
                continue;
            }

            let entry = &mut self.prefix_cache.get(&info.namespace).unwrap();
            if let Some(prefix) = &entry.prefix {
                if let Entry::Vacant(e) = self.schemas.known_prefixes.entry(prefix.clone()) {
                    info.prefix = Some(e.key().clone());
                    e.insert(*id);
                }
            }
        }

        // Fallback to alternate prefixes
        if self.alternative_prefixes {
            for (id, info) in &mut self.schemas.namespace_infos {
                if info.prefix.is_some() {
                    continue;
                }

                let entry = &mut self.prefix_cache.get(&info.namespace).unwrap();
                for alt in &entry.alt_prefixes {
                    if let Entry::Vacant(e) = self.schemas.known_prefixes.entry(alt.clone()) {
                        info.prefix = Some(e.key().clone());
                        e.insert(*id);
                    }
                }
            }
        }

        // Fallback to generated prefix
        if self.generate_prefixes {
            for (id, info) in &mut self.schemas.namespace_infos {
                if info.prefix.is_some() {
                    continue;
                }

                let entry = &mut self.prefix_cache.get(&info.namespace).unwrap();
                let prefix = entry
                    .prefix
                    .clone()
                    .or_else(|| entry.alt_prefixes.iter().next().cloned());
                if let Some(prefix) = prefix {
                    let ext = format!("_{}", id.0);
                    let ext = ext.as_bytes();

                    let mut p = prefix.0.into_owned();
                    p.extend_from_slice(ext);

                    let prefix = NamespacePrefix(Cow::Owned(p));
                    self.schemas.known_prefixes.insert(prefix, *id);
                }
            }
        }
    }
}

fn import_req(
    import: &Import,
    current_ns: Option<Namespace>,
    current_location: Option<&Url>,
) -> Option<ResolveRequest> {
    let location = import.schema_location.as_ref()?;

    let mut req = ResolveRequest::new(location, ResolveRequestType::ImportRequest);

    if let Some(ns) = current_ns {
        req = req.current_ns(ns);
    }

    if let Some(ns) = &import.namespace {
        req = req.requested_ns(Namespace::from(ns.as_bytes().to_owned()));
    }

    if let Some(current_location) = current_location {
        req = req.current_location(current_location.clone());
    }

    Some(req)
}

fn include_req(
    include: &Include,
    current_ns: Option<Namespace>,
    current_location: Option<&Url>,
) -> ResolveRequest {
    let mut req = ResolveRequest::new(&include.schema_location, ResolveRequestType::IncludeRequest);

    if let Some(ns) = current_ns {
        req = req.current_ns(ns);
    }

    if let Some(current_location) = current_location {
        req = req.current_location(current_location.clone());
    }

    req
}
