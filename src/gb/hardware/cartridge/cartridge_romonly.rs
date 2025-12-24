use crate::{
    gb::{
        hardware::{cartridge::Cartridge, memory::OPEN_BUS_VALUE},
        regions::{CART_RAM, ROM_SPACE},
    },
    region_guard,
};
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
        region_guard!(address in ROM_SPACE);
        // ROM_SPACE begins at 0 so no need to transform the address
        self.rom[address as usize]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        region_guard!(address in ROM_SPACE);
        // Do nothing; ignore writes
    }

    fn read_ram(&self, address: u16) -> u8 {
        region_guard!(address in CART_RAM);
        // No ram in this cart
        OPEN_BUS_VALUE
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        region_guard!(address in CART_RAM);
        // Do nothing; ignore writes
    }

    fn load_from_file(&mut self, cart_file: &File) -> Result<()> {
        let mut reader = BufReader::new(cart_file);

        // Fill the whole rom
        self.rom.resize(TOTAL_SIZE, 0);
        reader.read_exact(&mut self.rom)?;

        // Ensure EOF
        if reader.read(&mut self.rom)? != 0 {
            return Err(Error::new(
                ErrorKind::FileTooLarge,
                "Simple cartridges (no MBC, ROM only) should be exactly 32 KiB (32,768 bytes)",
            ));
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use test_log::test;

    fn file(path: &str) -> File {
        File::open(Path::new(path)).unwrap()
    }

    #[test]
    fn test_load_cart() {
        let f = file("res/dummy_cartromonly_read_test.bin");
        let mut cart = CartRomOnly::default();
        cart.load_from_file(&f).unwrap();
        assert_eq!(cart.rom.len(), ROM_SPACE.size().into());
        assert_eq!(cart.rom[ROM_SPACE.begin as usize], 0xAA);
        assert_eq!(cart.rom[ROM_SPACE.end as usize], 0xBB);
    }
}
