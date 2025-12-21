use crate::{cart::Cart, regions::ROM_SPACE};
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Result},
};

const TOTAL_SIZE: usize = 0x8000;

pub struct CartRomOnly {
    rom: [u8; TOTAL_SIZE],
}

impl Cart for CartRomOnly {
    fn get(&self, address: u16) -> u8 {
        if ROM_SPACE.contains(address) {
            // ROM_SPACE begins at 0 so no need to transform the address
            self.rom[address as usize]
        } else {
            // No RAM in this cart
            0xFF
        }
    }

    fn set(&mut self, _: u16, _: u8) {
        // Ignore writes.
    }
}

impl CartRomOnly {
    pub fn new(cart_file: &File) -> Result<Self> {
        let mut reader = BufReader::new(cart_file);

        // Fill the whole rom
        let mut rom = [0; TOTAL_SIZE];
        reader.read_exact(&mut rom)?;

        // Ensure EOF
        if reader.read(&mut rom)? != 0 {
            return Err(Error::new(
                ErrorKind::FileTooLarge,
                "Simple cartridges (no MBC, ROM only) should be exactly 32 KiB (32,768 bytes)",
            ));
        };

        Ok(Self { rom })
    }
}
