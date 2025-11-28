use quick_xml::events::Event;

use crate::quick_xml::{error::ErrorInfo, Error};

use super::{XmlReader, XmlReaderSync};

/// Implements a [`XmlReader`] that is able to provide additional error
/// information like the current cursor position or the chain of currently
/// processed XML tags.
#[derive(Debug)]
pub struct ErrorReader<R> {
    inner: R,
    error_info: ErrorInfo,
}

impl<R> ErrorReader<R>
where
    R: XmlReader,
{
    /// Create a new [`ErrorReader`] instance from the passed reader.
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            error_info: ErrorInfo::default(),
        }
    }

    /// Extracts the internal reader from the instance.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R> XmlReader for ErrorReader<R>
where
    R: XmlReader,
{
    fn extend_error(&self, error: Error) -> Error {
        self.inner
            .extend_error(error)
            .with_error_info(&self.error_info)
    }
}

impl<'a, R> XmlReaderSync<'a> for ErrorReader<R>
where
    R: XmlReaderSync<'a>,
{
    fn read_event(&mut self) -> Result<Event<'a>, Error> {
        let event = self.inner.read_event()?;

        self.error_info.update(&event);

        Ok(event)
    }
}

#[cfg(feature = "async")]
impl<'a, R> super::XmlReaderAsync<'a> for ErrorReader<R>
where
    R: super::XmlReaderAsync<'a>,
    for<'x> R::ReadEventFut<'x>: Unpin,
{
    type ReadEventFut<'x>
        = ReadEventFut<'a, 'x, R>
    where
        R: 'x;

    fn read_event_async(&mut self) -> Self::ReadEventFut<'_> {
        ReadEventFut::new(self)
    }
}

/// Implements a [`Future`] that is emitted by the [`XmlReaderAsync::read_event_async`]
/// implementation of the [`ErrorReader`].
#[derive(Debug)]
#[cfg(feature = "async")]
pub struct ReadEventFut<'a, 'x, R>
where
    R: super::XmlReaderAsync<'a> + 'x,
{
    inner: R::ReadEventFut<'x>,
    error_info: &'x mut ErrorInfo,
}

#[cfg(feature = "async")]
impl<'a, 'x, R> ReadEventFut<'a, 'x, R>
where
    R: super::XmlReaderAsync<'a> + 'x,
{
    fn new(reader: &'x mut ErrorReader<R>) -> Self {
        let inner = reader.inner.read_event_async();
        let error_info = &mut reader.error_info;

        Self { inner, error_info }
    }
}

#[cfg(feature = "async")]
impl<'a, 'x, R> std::future::Future for ReadEventFut<'a, 'x, R>
where
    Self: Unpin,
    R: super::XmlReaderAsync<'a> + 'x,
    R::ReadEventFut<'x>: Unpin,
{
    type Output = Result<Event<'a>, Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use futures::ready;
        use std::pin::Pin;
        use std::task::Poll;

        let this = self.get_mut();

        Poll::Ready(match ready!(Pin::new(&mut this.inner).poll(cx)) {
            Ok(event) => {
                this.error_info.update(&event);

                Ok(event)
            }
            Err(error) => Err(error.with_error_info(this.error_info)),
        })
    }
}
