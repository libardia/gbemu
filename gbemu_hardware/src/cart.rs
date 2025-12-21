use crate::cart_types::cart_romonly::CartRomOnly;
use std::{
    fs::File,
    io::{Error, ErrorKind, Result},
    os::unix::fs::FileExt,
    path::Path,
};

const CART_INFO_START: u64 = 0x0147;

pub trait Cart {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, value: u8);
}

pub fn load_cart(cart_path: &Path) -> Result<Box<dyn Cart>> {
    let cart_file = File::open(cart_path)?;

    let mut cart_info = [0; 3];
    cart_file.read_exact_at(&mut cart_info, CART_INFO_START)?;

    let cart_type = cart_info[0];
    let cart_rom_size = decode_rom_size(cart_info[1])?;
    let cart_ram_size = decode_ram_size(cart_info[2])?;

    macro_rules! make_result {
        ($inner:expr) => {
            Ok(Box::new($inner?))
        };
    }

    match cart_type {
        // Note for the marked lines below (*):
        // MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version
        // (the Japanese version of Pokémon Crystal Version).
        0x00 => make_result!(CartRomOnly::new(&cart_file)), // ROM ONLY
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
            format!("Unsupported code for cart type: ${cart_type:0>2X}"),
        )),
    }
}

fn decode_rom_size(code: u8) -> Result<usize> {
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
            format!("Unsupported code for cart ROM size: ${code:0>2X}"),
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
            format!("Unsupported code for cart RAM size: ${code:0>2X}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
