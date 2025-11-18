pub mod regions;

use crate::macros::new;
use regions::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct MMU {
    raw_ram: Vec<u8>,
}

impl MMU {
    new!(
        raw_ram = vec![0xFF; ALL_RAM.usize() - ECHO_RAM.usize()];
    );

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
