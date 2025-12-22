#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{cart::load_cart, cpu::CPU, mmu::MMU};
use std::{io::Result, path::Path};

mod cart;
mod cart_types;
mod cpu;
mod macros;
mod mmu;

struct GameBoy {
    mmu: MMU,
    cpu: CPU,
}

pub fn run(rom_path: &str) -> Result<()> {
    let cart = load_cart(Path::new(rom_path))?;
    let gb = GameBoy {
        mmu: MMU::new(cart),
        cpu: CPU::default(),
    };

    //TODO: run the gb

    Ok(())
}
