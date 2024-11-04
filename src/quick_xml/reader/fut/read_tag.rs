use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use quick_xml::events::Event;

use super::super::{Error, XmlReaderAsync};

/// Implements a [`Future`] that reads the next tag from any [`XmlReaderAsync`].
#[derive(Debug)]
pub struct ReadTag<'a, 'x, R>
where
    R: XmlReaderAsync<'a> + 'x,
{
    fut: R::ReadEventFut<'x>,
    reader: *mut R,
}

impl<'a, 'x, R> ReadTag<'a, 'x, R>
where
    R: XmlReaderAsync<'a>,
{
    /// Create a [`ReadTag`] future from the passed `reader`.
    pub fn new(reader: &'x mut R) -> Self {
        let r = reader as *mut _;
        let fut = reader.read_event_async();

        Self { fut, reader: r }
    }
}

impl<'a, 'x, R> Future for ReadTag<'a, 'x, R>
where
    Self: Unpin,
    R: XmlReaderAsync<'a>,
    R::ReadEventFut<'x>: Unpin,
{
    type Output = Result<Event<'a>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        loop {
            match ready!(Pin::new(&mut this.fut).poll(cx)) {
                Ok(e @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                    return Poll::Ready(Ok(e))
                }
                Err(error) => return Poll::Ready(Err(error)),
                _ => this.fut = unsafe { &mut *this.reader }.read_event_async(),
            }
        }
    }
}
