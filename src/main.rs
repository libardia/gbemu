#![allow(dead_code, unused_variables)]

use crate::gb::GameBoy;
use ftail::Ftail;
use getopts::Options;
use log::{LevelFilter, debug, error};
use std::{env, fs, panic, path::Path};

mod gb;

fn main() {
    init_logging("logs");

    // Log on panic instead of a simple print
    panic::set_hook(Box::new(|info| match info.location() {
        Some(loc) => error!(
            "Unrecoverable error at '{}' line {}; shutting down.",
            loc.file(),
            loc.line()
        ),
        None => error!("Unrecoverable error; shutting down."),
    }));

    // Parse commandline arguments
    let args: Vec<String> = env::args().collect();
    debug!("Raw args: {args:?}");
    let mut opts = Options::new();

    // Define options
    opts.optflag("m", "meta", "Enable meta-instructions.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => error_panic!("{e}"),
    };

    GameBoy::new(matches).run();
}

fn init_logging(base_dir: &str) {
    fs::create_dir(base_dir).ok();
    Ftail::new()
        .console_env_level()
        .single_file(
            &Path::new(base_dir).join("trace.log"),
            false,
            LevelFilter::Trace,
        )
        .single_file(
            &Path::new(base_dir).join("debug.log"),
            false,
            LevelFilter::Debug,
        )
        .init()
        .unwrap();
}
