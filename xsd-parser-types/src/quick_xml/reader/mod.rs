//! Defines and implements different helper traits that are needed for the
//! deserialization process with the [`quick_xml`] crate.

mod error_reader;
mod io_reader;
mod slice_reader;

#[cfg(feature = "async")]
use std::future::Future;

use quick_xml::events::Event;

pub use self::error_reader::ErrorReader;
pub use self::io_reader::IoReader;
pub use self::slice_reader::SliceReader;

use super::Error;

/// Trait that defines the basics for an XML reader.
pub trait XmlReader: Sized {
    /// Add the error position to the passed error and return it.
    fn extend_error(&self, error: Error) -> Error;

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
}
