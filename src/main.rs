use std::{fs, panic, path::Path};

use ftail::Ftail;
use log::{LevelFilter, error};

use crate::gb::GameBoy;

pub mod gb;
pub mod macros;

#[cfg(test)]
pub mod testutil;

fn main() {
    init_logging("logs");

    // Log on panic instead of a simple print
    panic::set_hook(Box::new(|info| {
        match (info.location(), info.payload_as_str()) {
            (Some(loc), Some(str)) => {
                error!("panic at '{}' line {}: {str}", loc.file(), loc.line())
            }
            (Some(loc), None) => error!("unknown panic at '{}' line {}", loc.file(), loc.line()),
            (None, Some(str)) => error!("panic: {}", str),
            (None, None) => error!("unknown panic"),
        }
    }));

    let mut gb = GameBoy::new("res/cart_romonly_terminate.bin");
    gb.debug_isntructions = true;
    gb.load_rom("gb-bootroms/bin/dmg.bin");
    gb.skip_boot();
    gb.run();
}

fn init_logging(base_dir: &str) {
    fs::create_dir(base_dir).ok();
    let trace_file = Path::new(base_dir).join("trace.log");
    let debug_file = Path::new(base_dir).join("debug.log");
    fs::remove_file(&trace_file).ok();
    fs::remove_file(&debug_file).ok();
    Ftail::new()
        .console_env_level()
        .single_file(&trace_file, false, LevelFilter::Trace)
        .single_file(&debug_file, false, LevelFilter::Debug)
        .init()
        .unwrap();
}
