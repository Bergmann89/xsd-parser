use std::fmt::{Debug, Formatter, Result as FmtResult};

use quick_xml::{
    events::Event,
    name::{LocalName, PrefixIter, QName, ResolveResult},
    NsReader,
};

use super::super::{Error, XmlReader, XmlReaderSync};

/// Implements a [`XmlReader`] for string slices.
pub struct SliceReader<'a> {
    inner: NsReader<&'a [u8]>,
}

impl<'a> SliceReader<'a> {
    /// Creates a new [`SliceReader`] instance from the passed string slice `s`.
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        let inner = NsReader::from_str(s);

        Self { inner }
    }
}

impl XmlReader for SliceReader<'_> {
    fn resolve<'n>(&self, name: QName<'n>, attribute: bool) -> (ResolveResult<'_>, LocalName<'n>) {
        self.inner.resolve(name, attribute)
    }

    fn prefixes(&self) -> PrefixIter<'_> {
        self.inner.prefixes()
    }

    fn current_position(&self) -> u64 {
        self.inner.buffer_position()
    }

    fn error_position(&self) -> u64 {
        self.inner.error_position()
    }
}

impl<'a> XmlReaderSync<'a> for SliceReader<'a> {
    fn read_event(&mut self) -> Result<Event<'a>, Error> {
        self.inner
            .read_event()
            .map_err(|error| self.map_error(error))
    }
}

#[cfg(feature = "async")]
impl<'a> super::super::XmlReaderAsync<'a> for SliceReader<'a> {
    type ReadEventFut<'x>
        = std::future::Ready<Result<Event<'a>, Error>>
    where
        Self: 'x;

    fn read_event_async(&mut self) -> Self::ReadEventFut<'_> {
        std::future::ready(
            self.inner
                .read_event()
                .map_err(|error| self.map_error(error)),
        )
    }
}

impl Debug for SliceReader<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "SliceReader")
    }
}
