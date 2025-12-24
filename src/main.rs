#![allow(dead_code, unused_variables)]

use crate::gb::GameBoy;
use getopts::Options;
use log::{LevelFilter, debug, error};
use simple_logger::SimpleLogger;
use std::{env, panic};

mod gb;

fn main() {
    // Initialize logging
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();

    // Log on panic instead of a simple print
    panic::set_hook(Box::new(|info| {
        error!("Unrecoverable error: shutting down.");
    }));

    // Parse commandline arguments
    let args: Vec<String> = env::args().collect();
    debug!("Raw args: {args:?}");
    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{e}"),
    };

    // Make sure a ROM file is provided
    if matches.free.len() < 1 {
        error_panic!("No ROM file provided.");
    }

    let res_gb = GameBoy::new(&matches.free[0]);
    match res_gb {
        Ok(mut gb) => gb.run(),
        Err(e) => error_panic!("Couldn't start: {}", e),
    }
}
