#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    cartridge::{Cartridge, load_cart},
    graphics::Graphics,
    memory::Memory,
    processor::Processor,
};
use std::{io::Result, path::Path};

mod cartridge;
mod cartridge_types;
mod graphics;
mod macros;
mod memory;
mod processor;

pub struct GameBoy {
    cart: Box<dyn Cartridge>,
    cpu: Processor,
    ppu: Graphics,
    mmu: Memory,
}

impl GameBoy {
    pub fn new(cart: Box<dyn Cartridge>) -> Self {
        Self {
            cart,
            cpu: Default::default(),
            ppu: Default::default(),
            mmu: Default::default(),
        }
    }

    pub fn run(&mut self) {
        //TODO: run
    }
}

pub fn start(rom_path: &str) -> Result<()> {
    let cart = load_cart(Path::new(rom_path))?;
    let mut gb = GameBoy::new(cart);
    gb.run();
    Ok(())
}
