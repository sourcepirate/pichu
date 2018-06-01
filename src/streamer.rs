//! Futures for listening to files;

use futures::task::Context;
use futures::Async;
use futures::Stream;
use std::io::{BufRead, Error};
use std::thread;
use std::time::Duration;

const MIN_DURATION: u32 = 100; // time in milli seconds

pub struct FileStreamer<T: BufRead> {
    inner: T,
}

impl<T> FileStreamer<T>
where
    T: BufRead,
{
    pub fn new(p: T) -> Self {
        FileStreamer { inner: p }
    }
}

impl<T> Stream for FileStreamer<T>
where
    T: BufRead,
{
    type Item = String;
    type Error = Error;

    fn poll_next(&mut self, ctx: &mut Context) -> Result<Async<Option<String>>, Error> {
        // get the content file if exists
        let mut content = String::new();
        self.inner.read_line(&mut content)?;
        if !content.is_empty() {
            debug!("DEBUG[/]: {:?}", content);
            Ok(Async::Ready(Some(content)))
        } else {
            let waker = ctx.waker();
            let notifier = waker.clone();
            thread::spawn(move || {
                thread::sleep(Duration::new(0, MIN_DURATION));
                notifier.wake();
            });
            Ok(Async::Pending)
        }
    }
}
