use crate::{
    address_fmt,
    cart::Cart,
    error_panic,
    mmu::{
        OPEN_BUS_VALUE,
        regions::{CART_RAM, ROM_SPACE},
    },
};
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Result},
};

const TOTAL_SIZE: usize = 0x8000;

pub struct CartRomOnly {
    rom: [u8; TOTAL_SIZE],
}

impl Cart for CartRomOnly {
    fn peek(&self, address: u16) -> u8 {
        // No difference from read()
        self.read(address)
    }

    fn poke(&mut self, _: u16, _: u8) {
        // Ignore
    }

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
