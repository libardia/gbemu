pub mod regions;

use regions::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct MMU {
    raw_ram: Vec<u8>,
}

impl Default for MMU {
    fn default() -> Self {
        Self {
            raw_ram: vec![0xFF; ALL_RAM.usize() - ECHO_RAM.usize()],
        }
    }
}

impl MMU {
    fn adjust_address(address: u16) -> u16 {
        if address >= ECHO_RAM.begin {
            if address <= ECHO_RAM.end {
                address - (ECHO_RAM.begin - WORK_RAM.begin)
            } else {
                address - ECHO_RAM.size()
            }
        } else {
            address
        }
    }
}

impl Index<u16> for MMU {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.raw_ram[Self::adjust_address(index) as usize]
    }
}

impl IndexMut<u16> for MMU {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.raw_ram[Self::adjust_address(index) as usize]
    }
}
