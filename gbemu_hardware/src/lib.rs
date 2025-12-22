#![allow(dead_code)]
#![allow(unused_variables)]

use bumpalo::Bump;

use crate::{
    cart::{Cart, load_cart},
    cpu::CPU,
    mmu::MMU,
};
use std::{cell::RefCell, io::Result, path::Path, rc::Rc};

mod cart;
mod cart_types;
mod cpu;
mod macros;
mod mmu;
mod ppu;

pub struct GameBoy {
    arena: Bump,

    cpu: CPU,
    mmu: Rc<RefCell<MMU>>,
}

impl GameBoy {
    pub fn new(cart: Box<dyn Cart>) -> Self {
        let mmu = rcref!(MMU::new(cart));
        let cpu = CPU::new(mmu.clone());
        Self { cpu, mmu }
        //TODO: Okay so I think MMU should have a rcref to everything that uses a hardware register,
        // but then everything else just takes a reference to mmu as a parameter
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
