use crate::{cartridge::Cartridge, memory::OPEN_BUS_VALUE};
use log::debug;
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Result},
};

const TOTAL_SIZE: usize = 0x8000;

#[derive(Debug, Default)]
pub struct CartRomOnly {
    rom: Vec<u8>,
}

impl Cartridge for CartRomOnly {
    fn read_rom(&self, address: u16) -> u8 {
        // ROM_SPACE begins at 0 so no need to transform the address
        self.rom[address as usize]
    }

    fn read_ram(&self, address: u16) -> u8 {
        // No ram in this cart
        OPEN_BUS_VALUE
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        // Ignore writes
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        // Ignore writes
    }

    fn load_from_file(&mut self, cart_file: &File) -> Result<()> {
        let mut reader = BufReader::new(cart_file);

        // Fill the whole rom
        let mut rom_raw = [0; TOTAL_SIZE];
        reader.read_exact(&mut rom_raw)?;

        // Ensure EOF
        if reader.read(&mut rom_raw)? != 0 {
            return Err(Error::new(
                ErrorKind::FileTooLarge,
                "Simple cartridges (no MBC, ROM only) should be exactly 32 KiB (32,768 bytes)",
            ));
        };

        self.rom.extend_from_slice(&rom_raw);

        debug!("{:?}", self.rom);

        Ok(())
    }
}
