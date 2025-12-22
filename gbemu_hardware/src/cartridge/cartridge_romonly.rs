use crate::{
    HardwareInterface, address_fmt,
    cartridge::Cartridge,
    error_panic,
    memory::OPEN_BUS_VALUE,
    regions::{CART_RAM, ROM_SPACE},
};
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

impl HardwareInterface for CartRomOnly {
    fn read(&self, address: u16) -> u8 {
        if ROM_SPACE.contains(address) {
            // ROM_SPACE begins at 0 so no need to transform the address
            self.rom[address as usize]
        } else if CART_RAM.contains(address) {
            // No RAM in this cart
            OPEN_BUS_VALUE
        } else {
            error_panic!(
                "Tried to read an address outside the cart's range: {}",
                address_fmt!(address)
            );
        }
    }

    fn write(&mut self, _: u16, _: u8) {
        // Ignore writes.
    }
}

impl Cartridge for CartRomOnly {
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
