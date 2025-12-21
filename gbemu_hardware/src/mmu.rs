use crate::{
    cart::Cart,
    mmu::{
        regions::{HIGH_RAM, MappedMemoryRegion, OAM, VRAM, WORK_RAM},
        regs::HardwareRegs,
    },
};

pub mod regions;
pub mod regs;

pub const OPEN_BUS_VALUE: u8 = 0xFF;
pub const UNINIT_VALUE: u8 = 0xFF;

pub struct MMU {
    cart: Box<dyn Cart>,

    // RAM areas
    vram: MappedMemoryRegion,
    wram: MappedMemoryRegion,
    oam: MappedMemoryRegion,
    hram: MappedMemoryRegion,

    io: HardwareRegs,
}

impl MMU {
    pub fn new(cart: Box<dyn Cart>) -> Self {
        Self {
            cart,
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),
            io: HardwareRegs::new(),
        }
    }

    // Return the value of memory at the given address, but without side effects that would
    // otherwise occur if it was a true read. For debug and display purposes
    pub fn peek(&self, address: u16) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
