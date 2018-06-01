#[macro_use]
extern crate log;
extern crate env_logger;

extern crate futures;
extern crate clap;

use std::fs::canonicalize;
use std::path::PathBuf;
use clap::{Arg, App};
use futures::executor::block_on_stream;

pub mod streamer;
pub mod shipper;
pub mod core;


fn main() {
    env_logger::init();
    let app = App::new("Pichu")
                  .version("0.0.1")
                  .arg(Arg::with_name("file").required(true).takes_value(true))
                  .arg(Arg::with_name("address").required(true).takes_value(true))
                  .get_matches();
    
    let filepath = app.value_of("file").unwrap();
    let address = app.value_of("address").unwrap();
    // File path

    let mut file_path_input = PathBuf::from(filepath);

    if file_path_input.is_relative(){
        file_path_input = canonicalize(file_path_input).unwrap();
    }

    let mut lresult = core::LogListener::new(file_path_input.to_str().unwrap(), address);
    
    match lresult {
        Ok(listener) => {
            for _ in block_on_stream(listener) {}
        },
        Err(e) => error!("{:?}", e)
    };

}