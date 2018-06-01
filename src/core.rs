
use std::io::{Error, BufReader, Read, Write};
use std::fs::File;
use futures::Stream;
use futures::Async;
use futures::task::Context;
use shipper::TcpShipper;
use streamer::FileStreamer;

pub struct LogListener {
    streamer: FileStreamer<BufReader<File>>, 
    shipper: TcpShipper
}

impl LogListener {
    pub fn new(fp: &str, url: &str) -> Result<Self, Error> {

        let mut file = File::open(fp)?;
        let mut reader = BufReader::new(file);
        let mut shipper = TcpShipper::new(url.to_owned())?;

        Ok(LogListener {
            streamer: FileStreamer::new(reader),
            shipper: shipper
        })
    }
}

impl Stream for LogListener {
    type Item = bool;
    type Error = Error;

    fn poll_next(&mut self, ctx: &mut Context) -> Result<Async<Option<bool>>, Error> {
        match self.streamer.poll_next(ctx) {
            Ok(val) => match val {
                Async::Ready(sop) => {
                    let written = self.shipper.send(sop.unwrap())?;
                    self.shipper.flush()?;
                    Ok(Async::Ready(Some(true)))
                },
                _ => Ok(Async::Pending)
            },
            Err(e) => Err(e)
        }
    }
}