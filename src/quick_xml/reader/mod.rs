//! Defines and implements different helper traits that are needed for the
//! deserialization process with the [`quick_xml`] crate.

mod error_reader;
mod io_reader;
mod slice_reader;

#[cfg(feature = "async")]
mod fut;

#[cfg(feature = "async")]
use std::future::Future;

use std::borrow::Cow;
use std::cell::RefCell;
use std::mem::take;

use quick_xml::{
    events::Event,
    name::{LocalName, PrefixDeclaration, PrefixIter, QName, ResolveResult},
};

use crate::xml::NamespacesShared;

pub use self::error_reader::ErrorReader;
pub use self::io_reader::IoReader;
pub use self::slice_reader::SliceReader;

#[cfg(feature = "async")]
pub use self::fut::{ReadTag, SkipCurrent};

use super::{Error, ErrorKind};

/// Trait that defines the basics for an XML reader.
pub trait XmlReader: Sized {
    /// Resolves a qname in the current context of the XML file.
    fn resolve<'n>(&self, name: QName<'n>, attribute: bool) -> (ResolveResult<'_>, LocalName<'n>);

    /// Returns a map that contains all namespaces in the current context.
    fn namespaces(&self) -> NamespacesShared<'static>;

    /// Returns the current position (byte offset) in the current XML file.
    fn current_position(&self) -> u64;

    /// Returns the position (byte offset) of the last detected error.
    fn error_position(&self) -> u64;

    /// Add the error position to the passed error and return it.
    fn extend_error(&self, error: Error) -> Error {
        error.with_pos(self.error_position())
    }

    /// Converts the passed `error` to an [`Error`], adds the error information
    /// using `extend_error` and returns it.
    fn map_error<E>(&self, error: E) -> Error
    where
        Error: From<E>,
    {
        self.extend_error(Error::from(error))
    }

    /// Same as `map_error`, but for the passed `result`.
    #[allow(clippy::missing_errors_doc)]
    fn map_result<T, E>(&self, result: Result<T, E>) -> Result<T, Error>
    where
        Error: From<E>,
    {
        result.map_err(|error| self.map_error(error))
    }

    /// Create a result from the passed `error` using `map_error` and returns it.
    #[allow(clippy::missing_errors_doc)]
    fn err<E>(&self, error: E) -> Result<(), Error>
    where
        Error: From<E>,
    {
        Err(self.map_error(error))
    }

    /// Wraps the current reader in a new [`ErrorReader`].
    fn with_error_info(self) -> ErrorReader<Self> {
        ErrorReader::new(self)
    }
}

/// Trait that defines a synchronous XML reader.
pub trait XmlReaderSync<'a>: XmlReader {
    /// Reads the next [`Event`] from the reader.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the event could not been read.
    fn read_event(&mut self) -> Result<Event<'a>, Error>;

    /// Reads a new XML tag ([`Event::Start`], [`Event::Empty`] or [`Event::End`])
    /// from the reader.
    ///
    /// # Errors
    ///
    /// Forwards the errors from `read_event`.
    fn read_tag(&mut self) -> Result<Event<'a>, Error> {
        loop {
            if let e @ (Event::Start(_) | Event::Empty(_) | Event::End(_)) = self.read_event()? {
                break Ok(e);
            }
        }
    }

    /// Skips the current event with respect to the level of the different XML tags.
    ///
    /// # Errors
    ///
    /// Forwards the errors from `read_event`.
    fn skip_current(&mut self) -> Result<(), Error> {
        let mut depth = 0usize;

        loop {
            let event = self.read_event()?;

            match event {
                Event::Start(_) => depth += 1,
                Event::End(_) if depth == 1 => return Ok(()),
                Event::End(_) => depth -= 1,
                Event::Eof => Err(ErrorKind::UnexpectedEof)?,
                _ if depth > 0 => (),
                _ => return Ok(()),
            }
        }
    }
}

/// Trait that defines a asynchronous XML reader.
#[cfg(feature = "async")]
pub trait XmlReaderAsync<'a>: XmlReader {
    /// Future that is returned by the [`read_event_async`] method.
    type ReadEventFut<'x>: Future<Output = Result<Event<'a>, Error>> + Unpin
    where
        Self: 'x;

    /// Reads the next [`Event`] from the reader asynchronously.
    fn read_event_async(&mut self) -> Self::ReadEventFut<'_>;

    /// Reads a new XML tag ([`Event::Start`], [`Event::Empty`] or [`Event::End`])
    /// from the reader asynchronously.
    fn read_tag_async(&mut self) -> ReadTag<'a, '_, Self> {
        ReadTag::new(self)
    }

    /// Skips the current event with respect to the level of the different XML
    /// tags asynchronously.
    fn skip_current_async(&mut self) -> SkipCurrent<'a, '_, Self> {
        SkipCurrent::new(self)
    }
}

#[derive(Default, Debug)]
struct NamespacesBuilder(RefCell<NamespacesBuilderState>);

#[derive(Debug)]
struct NamespacesBuilderState {
    level: usize,
    pending: bool,
    elements: Vec<(usize, Option<NamespacesShared<'static>>)>,
}

impl NamespacesBuilder {
    fn get_or_create(&self, prefixes: PrefixIter<'_>) -> NamespacesShared<'static> {
        let mut state = self.0.borrow_mut();

        state
            .elements
            .last_mut()
            .map(|(_, x)| {
                let prefixes = prefixes.map(|(decl, ns)| {
                    let key = match decl {
                        PrefixDeclaration::Default => Cow::Owned(Vec::new()),
                        PrefixDeclaration::Named(x) => Cow::Owned(x.into()),
                    };
                    let value = Cow::Owned(ns.0.into());

                    (key, value)
                });

                x.get_or_insert_with(|| NamespacesShared::new(prefixes.collect()))
                    .clone()
            })
            .unwrap_or_default()
    }

    fn handle_event(&self, event: &Event<'_>) {
        let mut state = self.0.borrow_mut();

        if take(&mut state.pending) {
            state.level = state.level.saturating_sub(1);

            loop {
                if matches!(state.elements.last(), Some((last_level, _)) if *last_level > state.level)
                {
                    state.elements.pop();
                } else {
                    break;
                }
            }
        }

        match event {
            Event::Start(start) | Event::Empty(start) => {
                state.level += 1;
                state.pending = matches!(event, Event::Empty(_));

                for a in start.attributes().with_checks(false).flatten() {
                    if a.key.0.starts_with(b"xmlns") {
                        let level = state.level;

                        state.elements.push((level, None));

                        break;
                    }
                }
            }
            Event::End(_) => {
                state.pending = true;
            }
            _ => (),
        }
    }
}

impl Default for NamespacesBuilderState {
    fn default() -> Self {
        Self {
            level: 0,
            pending: false,
            elements: vec![(0, None)],
        }
    }
}
