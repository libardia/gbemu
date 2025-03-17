#![allow(dead_code)]

use std::env;

use gb::{gpu::REAL_GB_FPS, GB};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod gb;
mod mem_region;
mod util;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        env::set_var("RUST_LOG", &args[1]);
    }
    env::set_var("RUST_BACKTRACE", "1");

    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .env()
        .init()
        .unwrap();

    let mut gb = GB::new(REAL_GB_FPS, 5);
    // let mut gb = GB::new(9999999.0, 5);

    gb.load_prog(&[0xED, 0xEC]);
    // gb.load(r"D:\Emulation\ROMs\GB\Tetris (World) (Rev 1).gb");
    // gb.load("/run/media/tonyl/Data/Emulation/ROMs/GB/Tetris (World) (Rev 1).gb");

    gb.set_debug_mode(true);
    // gb.set_breakpoints(&[0x150]);

    gb.boot();
}
