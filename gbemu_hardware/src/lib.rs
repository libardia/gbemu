#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    cart::{Cart, load_cart},
    cpu::CPU,
    mmu::MMU,
    ppu::PPU,
};
use std::{io::Result, path::Path};

mod cart;
mod cart_types;
mod cpu;
mod macros;
mod mmu;
mod ppu;

pub struct GameBoy {
    cart: Box<dyn Cart>,
    cpu: CPU,
    ppu: PPU,
    mmu: MMU,
}

impl GameBoy {
    pub fn new(cart: Box<dyn Cart>) -> Self {
        Self {
            cart,
            cpu: Default::default(),
            ppu: Default::default(),
            mmu: Default::default(),
        }
    }

    pub fn run(&mut self) {
        //TODO: run
        MMU::peek(self, 0);
    }
}

pub fn start(rom_path: &str) -> Result<()> {
    let cart = load_cart(Path::new(rom_path))?;
    let mut gb = GameBoy::new(cart);
    gb.run();
    Ok(())
}
