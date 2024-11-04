use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::io::BufRead;

use quick_xml::{
    events::Event,
    name::{LocalName, PrefixIter, QName, ResolveResult},
    NsReader,
};

#[cfg(feature = "async")]
use tokio::io::AsyncBufRead;

use super::super::{Error, XmlReader, XmlReaderSync};

/// Implements an [`XmlReader`] for any kind of [`BufRead`].
pub struct IoReader<R> {
    inner: NsReader<R>,
    buffer: Vec<u8>,
}

impl<R> IoReader<R> {
    /// Create a new [`IoReader`] instance from the passed `reader`.
    pub fn new(reader: R) -> Self {
        let inner = NsReader::from_reader(reader);
        let buffer = Vec::new();

        Self { inner, buffer }
    }
}

impl<R> XmlReader for IoReader<R> {
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

impl<R> XmlReaderSync<'static> for IoReader<R>
where
    R: BufRead,
{
    fn read_event(&mut self) -> Result<Event<'static>, Error> {
        self.inner
            .read_event_into(&mut self.buffer)
            .map(Event::into_owned)
            .map_err(|error| self.map_error(error))
    }
}

#[cfg(feature = "async")]
impl<R> super::super::XmlReaderAsync<'static> for IoReader<R>
where
    R: AsyncBufRead + Unpin,
{
    type ReadEventFut<'x>
        = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Event<'static>, Error>> + 'x>>
    where
        Self: 'x;

    fn read_event_async(&mut self) -> Self::ReadEventFut<'_> {
        Box::pin(async move {
            match self.inner.read_event_into_async(&mut self.buffer).await {
                Ok(event) => Ok(event.into_owned()),
                Err(error) => Err(self.map_error(error)),
            }
        })
    }
}

impl<R> Debug for IoReader<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "IoReader")
    }
}
