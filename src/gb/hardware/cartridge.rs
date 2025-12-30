use crate::{
    byte_fmt, error_panic,
    gb::{hardware::cartridge::cartridge_romonly::CartRomOnly, regions::MemoryRegion},
    unwrap_or_log,
};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

pub mod cartridge_romonly;

pub const CART_ENTRY: u16 = 0x0100;
pub const HEADER_LOGO: MemoryRegion = MemoryRegion::new(0x0104, 0x0133);
pub const HEADER_TITLE: MemoryRegion = MemoryRegion::new(0x0134, 0x0143);
pub const HEADER_CART_TYPE: u16 = 0x0147;
pub const HEADER_ROM_SIZE: u16 = 0x0148;
pub const HEADER_RAM_SIZE: u16 = 0x0149;
pub const HEADER_CHECKSUM: u16 = 0x014D;
pub const HEADER_GLOBAL_CHECKSUM: MemoryRegion = MemoryRegion::new(0x014E, 0x014F);

pub trait Cartridge {
    fn init(&mut self);

    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);

    fn load_from_file(&mut self, cart_file: &File);
}

pub fn load_cart(cart_path: &str) -> Box<dyn Cartridge> {
    let mut cart_file = unwrap_or_log!(File::open(Path::new(cart_path)));
    let rom_info = get_rom_info(&mut cart_file);
    let mut cart = make_cart_from_info(rom_info);
    cart.load_from_file(&cart_file);
    cart
}

fn get_rom_info(cart_file: &mut File) -> (u8, u8, u8) {
    let mut cart_info = [0; 3];
    unwrap_or_log!(cart_file.seek(SeekFrom::Start(HEADER_CART_TYPE as u64)));
    unwrap_or_log!(cart_file.read_exact(&mut cart_info));
    unwrap_or_log!(cart_file.rewind());
    (cart_info[0], cart_info[1], cart_info[2])
}

fn make_cart_from_info(rom_info: (u8, u8, u8)) -> Box<dyn Cartridge> {
    let (cart_type, crom, cram) = rom_info;
    let rom_size = decode_rom_banks(crom);
    let ram_size = decode_ram_size(cram);

    match cart_type {
        // Note for the marked lines below (*):
        // MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version
        // (the Japanese version of Pokémon Crystal Version).
        0x00 => Box::new(CartRomOnly::default()), // ROM ONLY
        //TODO: 0x01 => Box::new(/* todo */), // MBC1
        //TODO: 0x02 => Box::new(/* todo */), // MBC1+RAM
        //TODO: 0x03 => Box::new(/* todo */), // MBC1+RAM+BATTERY
        //TODO: 0x05 => Box::new(/* todo */), // MBC2
        //TODO: 0x06 => Box::new(/* todo */), // MBC2+BATTERY
        //TODO: 0x0B => Box::new(/* todo */), // MMM01
        //TODO: 0x0C => Box::new(/* todo */), // MMM01+RAM
        //TODO: 0x0D => Box::new(/* todo */), // MMM01+RAM+BATTERY
        //TODO: 0x0F => Box::new(/* todo */), // MBC3+TIMER+BATTERY
        //TODO: 0x10 => Box::new(/* todo */), // MBC3+TIMER+RAM+BATTERY*
        //TODO: 0x11 => Box::new(/* todo */), // MBC3
        //TODO: 0x12 => Box::new(/* todo */), // MBC3+RAM*
        //TODO: 0x13 => Box::new(/* todo */), // MBC3+RAM+BATTERY*
        //TODO: 0x19 => Box::new(/* todo */), // MBC5
        //TODO: 0x1A => Box::new(/* todo */), // MBC5+RAM
        //TODO: 0x1B => Box::new(/* todo */), // MBC5+RAM+BATTERY
        //TODO: 0x1C => Box::new(/* todo */), // MBC5+RUMBLE
        //TODO: 0x1D => Box::new(/* todo */), // MBC5+RUMBLE+RAM
        //TODO: 0x1E => Box::new(/* todo */), // MBC5+RUMBLE+RAM+BATTERY
        //TODO: 0x20 => Box::new(/* todo */), // MBC6
        //TODO: 0x22 => Box::new(/* todo */), // MBC7+SENSOR+RUMBLE+RAM+BATTERY
        //TODO: 0xFC => Box::new(/* todo */), // POCKET CAMERA
        //TODO: 0xFD => Box::new(/* todo */), // BANDAI TAMA5
        //TODO: 0xFE => Box::new(/* todo */), // HuC3
        //TODO: 0xFF => Box::new(/* todo */), // HuC1+RAM+BATTERY
        _ => error_panic!("Unsupported cart type: {}", byte_fmt!(cart_type)),
    }
}

fn decode_rom_banks(code: u8) -> usize {
    match code {
        0x00 => 2,   // 2 banks (32 KiB)
        0x01 => 4,   // 4 banks (64 KiB)
        0x02 => 8,   // 8 banks (128 KiB)
        0x03 => 16,  // 16 banks (256 KiB)
        0x04 => 32,  // 32 banks (512 KiB)
        0x05 => 64,  // 64 banks (1 MiB)
        0x06 => 128, // 128 banks (2 MiB)
        0x07 => 256, // 256 banks (4 MiB)
        0x08 => 512, // 512 banks (8 MiB)

        _ => error_panic!("Unsupported cart ROM size: {}", byte_fmt!(code)),
    }
}

fn decode_ram_size(code: u8) -> usize {
    match code {
        0x00 => 0,          // None
        0x02 => 8 * 1024,   // 8kib
        0x03 => 32 * 1024,  // 32kib
        0x04 => 128 * 1024, // 128kib
        0x05 => 64 * 1024,  // 64kib

        _ => error_panic!("Unsupported cart RAM size: {}", byte_fmt!(code)),
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
        let mut f = file("res/dummy_cartromonly_read_test.bin");
        let (ct, rom, ram) = get_rom_info(&mut f);
        assert_eq!(ct, 0x11);
        assert_eq!(rom, 0x22);
        assert_eq!(ram, 0x33);
    }
}
