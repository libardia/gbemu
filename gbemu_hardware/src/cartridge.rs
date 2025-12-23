use crate::{byte_fmt, cartridge::cartridge_romonly::CartRomOnly};
use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Result, Seek, SeekFrom},
    path::Path,
};

mod cartridge_romonly;

const CART_INFO_START: u64 = 0x0147;

pub trait Cartridge {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);

    fn load_from_file(&mut self, cart_file: &File) -> Result<()>;
}

pub fn load_cart(cart_path: &Path) -> Result<Box<dyn Cartridge>> {
    let mut cart_file = File::open(cart_path)?;

    let rom_info = get_rom_info(&mut cart_file)?;
    let mut cart = make_cart_from_info(rom_info)?;
    cart.load_from_file(&cart_file)?;

    Ok(cart)
}

fn get_rom_info(cart_file: &mut File) -> Result<(u8, u8, u8)> {
    let mut cart_info = [0; 3];
    cart_file.seek(SeekFrom::Start(CART_INFO_START))?;
    cart_file.read_exact(&mut cart_info)?;
    cart_file.rewind()?;
    Ok((cart_info[0], cart_info[1], cart_info[2]))
}

fn make_cart_from_info(rom_info: (u8, u8, u8)) -> Result<Box<dyn Cartridge>> {
    macro_rules! okbox {
        ($inner:expr) => {
            Ok(Box::new($inner))
        };
    }

    let (cart_type, crom, cram) = rom_info;
    let rom_size = decode_rom_banks(crom)?;
    let ram_size = decode_ram_size(cram)?;

    match cart_type {
        // Note for the marked lines below (*):
        // MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version
        // (the Japanese version of Pokémon Crystal Version).
        0x00 => okbox!(CartRomOnly::default()), // ROM ONLY
        //TODO: 0x01 => make_result!(/* todo */), // MBC1
        //TODO: 0x02 => make_result!(/* todo */), // MBC1+RAM
        //TODO: 0x03 => make_result!(/* todo */), // MBC1+RAM+BATTERY
        //TODO: 0x05 => make_result!(/* todo */), // MBC2
        //TODO: 0x06 => make_result!(/* todo */), // MBC2+BATTERY
        //TODO: 0x0B => make_result!(/* todo */), // MMM01
        //TODO: 0x0C => make_result!(/* todo */), // MMM01+RAM
        //TODO: 0x0D => make_result!(/* todo */), // MMM01+RAM+BATTERY
        //TODO: 0x0F => make_result!(/* todo */), // MBC3+TIMER+BATTERY
        //TODO: 0x10 => make_result!(/* todo */), // MBC3+TIMER+RAM+BATTERY*
        //TODO: 0x11 => make_result!(/* todo */), // MBC3
        //TODO: 0x12 => make_result!(/* todo */), // MBC3+RAM*
        //TODO: 0x13 => make_result!(/* todo */), // MBC3+RAM+BATTERY*
        //TODO: 0x19 => make_result!(/* todo */), // MBC5
        //TODO: 0x1A => make_result!(/* todo */), // MBC5+RAM
        //TODO: 0x1B => make_result!(/* todo */), // MBC5+RAM+BATTERY
        //TODO: 0x1C => make_result!(/* todo */), // MBC5+RUMBLE
        //TODO: 0x1D => make_result!(/* todo */), // MBC5+RUMBLE+RAM
        //TODO: 0x1E => make_result!(/* todo */), // MBC5+RUMBLE+RAM+BATTERY
        //TODO: 0x20 => make_result!(/* todo */), // MBC6
        //TODO: 0x22 => make_result!(/* todo */), // MBC7+SENSOR+RUMBLE+RAM+BATTERY
        //TODO: 0xFC => make_result!(/* todo */), // POCKET CAMERA
        //TODO: 0xFD => make_result!(/* todo */), // BANDAI TAMA5
        //TODO: 0xFE => make_result!(/* todo */), // HuC3
        //TODO: 0xFF => make_result!(/* todo */), // HuC1+RAM+BATTERY
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unsupported cart type: {}", byte_fmt!(cart_type)),
        )),
    }
}

fn decode_rom_banks(code: u8) -> Result<usize> {
    match code {
        0x00 => Ok(2),   // 2 banks (32 KiB)
        0x01 => Ok(4),   // 4 banks (64 KiB)
        0x02 => Ok(8),   // 8 banks (128 KiB)
        0x03 => Ok(16),  // 16 banks (256 KiB)
        0x04 => Ok(32),  // 32 banks (512 KiB)
        0x05 => Ok(64),  // 64 banks (1 MiB)
        0x06 => Ok(128), // 128 banks (2 MiB)
        0x07 => Ok(256), // 256 banks (4 MiB)
        0x08 => Ok(512), // 512 banks (8 MiB)

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unsupported cart ROM size: {}", byte_fmt!(code)),
        )),
    }
}

fn decode_ram_size(code: u8) -> Result<usize> {
    match code {
        0x00 => Ok(0),          // None
        0x02 => Ok(8 * 1024),   // 8kib
        0x03 => Ok(32 * 1024),  // 32kib
        0x04 => Ok(128 * 1024), // 128kib
        0x05 => Ok(64 * 1024),  // 64kib

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unsupported cart RAM size: {}", byte_fmt!(code)),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    fn file(path: &str) -> File {
        File::open(Path::new(path)).unwrap()
    }

    #[test]
    fn test_get_rom_info() {
        let mut f = file("../res/dummy_cartromonly.bin");
        let (ct, rom, ram) = get_rom_info(&mut f).unwrap();
        assert_eq!(ct, 0x11);
        assert_eq!(rom, 0x22);
        assert_eq!(ram, 0x33);
    }
}
