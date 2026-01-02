#![allow(dead_code, unused_variables)]

use ftail::Ftail;
use log::{LevelFilter, debug, error};
use std::{env, fs, panic, path::Path};

use crate::{gb::GameBoy, logging::error_panic, opdef::define_options, options::Options};

mod gb;
mod immut;
mod logging;
mod opdef;

define_options!(
    brief: "Usage: gbemu [options] ROM_FILE"
    flags:
        help,      "h", "help",    "Show this help menu."
        meta_inst, "m", "meta",    "Enable CPU meta-instructions."
        do_boot,   "b", "do-boot", "If not set, the boot ROM is skipped and execution begins with the cartridge ROM."
);

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
    let options = match Options::parse(&args[1..]) {
        Ok(o) => o,
        Err(e) => error_panic!("{e}"),
    };

    // If help was set, show help and exit
    if options.flags.help {
        print!("{}", options.usage());
        return;
    }

    GameBoy::new(options).run();
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
