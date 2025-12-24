use crate::{
    gb::{mbc::MBC, mmu::warn_read_open_bus},
    mem_region::regions::{EXTERNAL_RAM, ROM_SPACE},
    util::{either, min, Hex16, Hex8},
};

use super::{warn_write_open_bus, warn_write_rom};

#[derive(Debug)]
pub struct RomOnlyMBC {
    rom: [u8; ROM_SPACE.usize()],
    ram: Option<[u8; EXTERNAL_RAM.usize()]>,
}
impl RomOnlyMBC {
    pub fn from_arr(rom: &[u8], ram_present: bool) -> Self {
        let mut s = Self {
            rom: [0; ROM_SPACE.usize()],
            ram: either!(ram_present => Some([0; EXTERNAL_RAM.usize()]); None),
        };
        for i in 0..min!(ROM_SPACE.usize(), rom.len()) {
            s.rom[i] = rom[i];
        }
        s
    }
}

impl MBC for RomOnlyMBC {
    fn read_byte(&self, address: u16) -> u8 {
        let hex_address = Hex16::make(address);
        if ROM_SPACE.contains(address) {
            self.rom[address as usize]
        } else if EXTERNAL_RAM.contains(address) {
            match self.ram.as_ref() {
                Some(ram) => ram[EXTERNAL_RAM.uoffset(address)],
                None => warn_read_open_bus!(hex_address, "No external RAM in this cart."),
            }
        } else {
            warn_read_open_bus!(hex_address, "Outside MBC range.")
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        let hex_address = Hex16::make(address);
        // We will only MAYBE do something if the address is in the external ram range
        if EXTERNAL_RAM.contains(address) {
            match self.ram.as_mut() {
                Some(ram) => ram[EXTERNAL_RAM.uoffset(address)] = value,
                None => warn_write_open_bus!(hex_address, "No external RAM in this cart."),
            }
        } else {
            warn_write_rom!(
                hex_address,
                Hex8::make(value),
                "Can't write to rom-only MBC."
            );
        }
    }
}
