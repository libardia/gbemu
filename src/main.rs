#![allow(dead_code, unused_variables)]

use crate::gb::GameBoy;
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod gb;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();

    let res_gb = GameBoy::new("D:\\Emulation\\ROMs\\GB\\Tetris (World) (Rev 1).gb");
    match res_gb {
        Ok(mut gb) => gb.run(),
        Err(e) => error_panic!("Couldn't start: {}", e),
    }
}
