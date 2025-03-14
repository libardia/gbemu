#![allow(dead_code)]

use std::env;

use gb::GB;
use log::LevelFilter;
use mem_region::{header_data::CART_TYPE, regions::ROM_BANK_N};
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
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let mut gb = GB::new();

    let mut rom = [0; ROM_BANK_N.uend() + 1];
    rom[CART_TYPE as usize] = 0x08;
    // gb.mmu.borrow_mut().set_mbc_from_arr(&rom);
    gb.mmu
        .borrow_mut()
        .set_mbc_from_file(r"D:\Emulation\ROMs\GB\Tetris (World) (Rev 1).gb");

    gb.boot();
}
