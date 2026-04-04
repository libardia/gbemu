use std::{fs, panic, path::Path};

use ftail::Ftail;
use log::{LevelFilter, debug, error};

use crate::gb::GameBoy;

pub mod gb;
pub mod macros;

#[cfg(test)]
mod testutil;

fn main() {
    init_logging("logs");

    // Log on panic instead of a simple print
    panic::set_hook(Box::new(|info| {
        match (info.location(), info.payload_as_str()) {
            (Some(loc), Some(str)) => {
                error!("Panic at '{}' line {}: {str}", loc.file(), loc.line())
            }
            (Some(loc), None) => error!("Unknown panic at '{}' line {}", loc.file(), loc.line()),
            (None, Some(str)) => error!("Panic: {}", str),
            (None, None) => error!("Unknown panic"),
        }
    }));

    let mut gb = GameBoy::new();
    debug!("{gb:?}");
    gb.run();
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
