use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::ready;
use quick_xml::events::Event;

use super::super::{Error, ErrorKind, XmlReaderAsync};

/// Implements a [`Future`] that skips the current element of any [`XmlReaderAsync`].
#[derive(Debug)]
pub struct SkipCurrent<'a, 'x, R>
where
    R: XmlReaderAsync<'a> + 'x,
{
    fut: R::ReadEventFut<'x>,
    depth: usize,
    reader: *mut R,
}

impl<'a, 'x, R> SkipCurrent<'a, 'x, R>
where
    R: XmlReaderAsync<'a>,
{
    /// Create a [`SkipCurrent`] future from the passed `reader`.
    pub fn new(reader: &'x mut R) -> Self {
        let r = reader as *mut _;
        let fut = reader.read_event_async();

        Self {
            fut,
            depth: 0,
            reader: r,
        }
    }
}

impl<'a, 'x, R> Future for SkipCurrent<'a, 'x, R>
where
    Self: Unpin,
    R: XmlReaderAsync<'a>,
    R::ReadEventFut<'x>: Unpin,
{
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        loop {
            match ready!(Pin::new(&mut this.fut).poll(cx)) {
                Ok(event) => match event {
                    Event::Start(_) => this.depth += 1,
                    Event::End(_) if this.depth == 1 => return Poll::Ready(Ok(())),
                    Event::End(_) => this.depth -= 1,
                    Event::Eof => return Poll::Ready(Err(Error::from(ErrorKind::UnexpectedEoF))),
                    _ if this.depth > 0 => (),
                    _ => return Poll::Ready(Ok(())),
                },
                Err(error) => return Poll::Ready(Err(error)),
            }

            this.fut = unsafe { &mut *this.reader }.read_event_async();
        }
    }
}
