use crate::{
    gb::{mbc::MBC, mmu::warn_read_open_bus},
    mem_region::regions::{EXTERNAL_RAM, ROM_BANK_0, ROM_BANK_N},
    util::{either, min},
};

use super::{warn_write_open_bus, warn_write_rom};

const ROM_SIZE: usize = ROM_BANK_0.usize() + ROM_BANK_N.usize();
const RAM_SIZE: usize = EXTERNAL_RAM.usize();

#[derive(Debug)]
pub struct NoMBC {
    rom: [u8; ROM_SIZE],
    ram: Option<[u8; RAM_SIZE]>,
}
impl NoMBC {
    pub fn from_arr(rom: &[u8], ram_present: bool) -> Self {
        let mut s = Self {
            rom: [0; ROM_SIZE],
            ram: either!(ram_present => Some([0; RAM_SIZE]); None),
        };
        for i in 0..min!(ROM_SIZE, rom.len()) {
            s.rom[i] = rom[i];
        }
        s
    }
}

impl MBC for NoMBC {
    fn read_byte(&self, address: u16) -> u8 {
        if address < ROM_BANK_N.end() {
            self.rom[address as usize]
        } else if EXTERNAL_RAM.contains(address) {
            match self.ram.as_ref() {
                Some(ram) => ram[EXTERNAL_RAM.uoffset(address)],
                None => warn_read_open_bus!(address, "No external RAM in this cart."),
            }
        } else {
            warn_read_open_bus!(address, "Outside MBC range.")
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        // We will only MAYBE do something if the address is in the external ram range
        if EXTERNAL_RAM.contains(address) {
            match self.ram.as_mut() {
                Some(ram) => ram[EXTERNAL_RAM.uoffset(address)] = value,
                None => warn_write_open_bus!(address, "No external RAM in this cart."),
            }
        } else {
            warn_write_rom!(address, "Can't write to rom-only MBC.");
        }
    }
}
