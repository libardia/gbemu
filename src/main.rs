#![allow(dead_code)]

use std::env;

// use gb::cpu::instructions::{ArgR8, Instruction::*};
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

    let mut gb = GB::new(REAL_GB_FPS, 3);
    // let mut gb = GB::new(30.0, 5);
    // let mut gb = GB::new(9999999.0, 5);

    // let print_terminate = gb.compile(vec![DEBUG_PRINT, TERMINATE]);
    // let mut timer_cart = GB::make_dummy_cart();
    // let timer_test = gb.compile(vec![
    //     LD_r8_r8(ArgR8::A, ArgR8::CONST(0b100.into())), // A = b100
    //     LDH_mn16_a(0xFF.into()), // [$FFFF] = A = b100 (enable timer interrupt)
    //     LD_r8_r8(ArgR8::A, ArgR8::CONST(0b101.into())), // A = b101
    //     LDH_mn16_a(0x07.into()), // [$FF07] = A = b101 (enable timer, clock = 4)
    //     EI,
    //     JR_e8(-2),
    // ]);
    // let timer_int = gb.compile(vec![DEBUG_PRINT, RETI]);
    // for (i, b) in timer_test.iter().enumerate() {
    //     timer_cart[i + 0x150] = *b;
    // }
    // for (i, b) in timer_int.iter().enumerate() {
    //     timer_cart[i + 0x50] = *b;
    // }

    // gb.load_prog(&print_terminate);
    // gb.load_cart_bytes(&timer_cart);
    gb.load(r"D:\Emulation\ROMs\GB\Tetris (World) (Rev 1).gb");
    // gb.load("/run/media/tonyl/Data/Emulation/ROMs/GB/Tetris (World) (Rev 1).gb");

    gb.set_debug_mode(true);
    // gb.set_breakpoints(&[0x150]);
    // gb.set_breakpoints(&[0x2BA]);

    gb.boot();
}
