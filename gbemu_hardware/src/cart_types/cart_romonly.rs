use crate::cart::Cart;
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
        self.rom[address as usize]
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
                "Simple cartridges (no MBC, ROM only) should be exactly 32 KB",
            ));
        };

        Ok(Self { rom })
    }
}
