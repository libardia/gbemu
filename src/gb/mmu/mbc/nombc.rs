use crate::{gb::mmu::regions::ROM_SPACE, macros::new};

use super::*;

#[derive(Debug, Default)]
pub struct NoMBC {
    rom: Vec<u8>,
}

impl NoMBC {
    new!(rom = vec![0xFF; ROM_SPACE.usize()];);
}

impl MBC for NoMBC {
    fn get(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn set(&mut self, _address: u16, _value: u8) {
        // Do nothing (ignore writes)
    }
}
