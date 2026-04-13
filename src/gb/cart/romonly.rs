use std::{fs::File, io::Read};

use crate::{
    gb::{
        cart::Cart,
        mmu::region::{CART_RAM, MappedMemoryRegion, ROM_SPACE},
    },
    macros::{hex, unwrap_or_error},
};

#[derive(Default)]
pub struct CartRomOnly {
    pub rom: MappedMemoryRegion,
}

impl CartRomOnly {
    pub fn new() -> Self {
        Self {
            rom: MappedMemoryRegion::new(ROM_SPACE),
        }
    }
}

impl Cart for CartRomOnly {
    fn load(&mut self, f: &mut File) {
        unwrap_or_error!(
            f.read_exact(self.rom.as_mut_slice()),
            "failed to read cartridge file"
        );
    }

    fn read(&mut self, address: u16) -> u8 {
        if self.rom.region.contains(address) {
            self.rom.get(address)
        } else if CART_RAM.contains(address) {
            0xFF // this cart has no ram
        } else {
            unimplemented!("cannot read address {} from cart", hex!(address, 4))
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        if self.rom.region.contains(address) || CART_RAM.contains(address) {
            // ignore writes
        } else {
            unimplemented!(
                "cannot write {} to address {} in cart",
                hex!(byte, 2),
                hex!(address, 4)
            )
        }
    }
}
