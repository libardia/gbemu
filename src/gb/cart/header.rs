use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use crate::{
    gb::mmu::{
        header_data::*,
        region::{HEADER, HEADER_BEGIN},
    },
    macros::make_word,
};

pub struct CartHeader {
    pub entry: Vec<u8>,
    pub logo: Vec<u8>,
    pub title: String,
    pub manufacturer: String,
    pub cgb: CGBSupport,
    pub new_licensee: String,
    pub sgb: bool,
    pub cart_type: CartType,
    pub rom_banks: usize,
    pub ram_banks: usize,
    pub destination: u8,
    pub old_licensee: u8,
    pub version: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

pub enum CGBSupport {
    CGBOnly,
    Compatible,
    NotCompatible,
}

#[derive(Debug)]
pub enum CartType {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    MBC2,
    MBC2Battery,
    RomRam,
    RomRamBattery,
    MMM01,
    MMM01Ram,
    MMM01RamBattery,
    MBC3TimerBattery,
    MBC3TimerRamBattery,
    MBC3,
    MBC3Ram,
    MBC3RamBattery,
    MBC5,
    MBC5Ram,
    MBC5RamBattery,
    MBC5Rumble,
    MBC5RumbleRam,
    MBC5RumbleRamBattery,
    MBC6,
    MBC7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
    Unknown(u8),
}

pub fn read_header(f: &mut File) -> io::Result<CartHeader> {
    let mut buf = [0; HEADER.usize()];

    /* #region Utility macros */
    macro_rules! buf_addr {
        ($address:expr) => {
            (($address) as u16 - 0x100) as usize
        };
    }

    macro_rules! buf_at {
        ($address:expr) => {
            buf[buf_addr!($address)]
        };
    }

    macro_rules! buf_range {
        ($range:expr) => {
            &buf[buf_addr!($range.begin)..buf_addr!($range.end)]
        };
    }

    macro_rules! buf_vec {
        ($range:expr) => {{
            let mut vec = Vec::new();
            let range = buf_range!($range);
            vec.resize(range.len(), 0);
            vec.copy_from_slice(range);
            vec
        }};
    }

    macro_rules! buf_string {
        ($range:expr) => {
            String::from_utf8_lossy(buf_range!($range)).to_string()
        };
    }
    /* #endregion */

    f.seek(SeekFrom::Start(HEADER_BEGIN as u64))?;
    f.read_exact(buf.as_mut_slice())?;

    let header = CartHeader {
        entry: buf_vec!(ENTRY_POINT),
        logo: buf_vec!(NINTENDO_LOGO),
        title: buf_string!(TITLE),
        manufacturer: buf_string!(MANUFACTURER),
        cgb: match buf_at!(CGB_FLAG) {
            0xC0 => CGBSupport::CGBOnly,
            0x80 => CGBSupport::Compatible,
            _ => CGBSupport::NotCompatible,
        },
        new_licensee: buf_string!(NEW_LICENSEE),
        sgb: buf_at!(SGB_FLAG) == 0x03,
        cart_type: match buf_at!(CART_TYPE) {
            0x00 => CartType::RomOnly,                    // ROM ONLY
            0x01 => CartType::MBC1,                       // MBC1
            0x02 => CartType::MBC1Ram,                    // MBC1+RAM
            0x03 => CartType::MBC1RamBattery,             // MBC1+RAM+BATTERY
            0x05 => CartType::MBC2,                       // MBC2
            0x06 => CartType::MBC2Battery,                // MBC2+BATTERY
            0x08 => CartType::RomRam,                     // ROM+RAM
            0x09 => CartType::RomRamBattery,              // ROM+RAM+BATTERY
            0x0B => CartType::MMM01,                      // MMM01
            0x0C => CartType::MMM01Ram,                   // MMM01+RAM
            0x0D => CartType::MMM01RamBattery,            // MMM01+RAM+BATTERY
            0x0F => CartType::MBC3TimerBattery,           // MBC3+TIMER+BATTERY
            0x10 => CartType::MBC3TimerRamBattery,        // MBC3+TIMER+RAM+BATTERY
            0x11 => CartType::MBC3,                       // MBC3
            0x12 => CartType::MBC3Ram,                    // MBC3+RAM
            0x13 => CartType::MBC3RamBattery,             // MBC3+RAM+BATTERY
            0x19 => CartType::MBC5,                       // MBC5
            0x1A => CartType::MBC5Ram,                    // MBC5+RAM
            0x1B => CartType::MBC5RamBattery,             // MBC5+RAM+BATTERY
            0x1C => CartType::MBC5Rumble,                 // MBC5+RUMBLE
            0x1D => CartType::MBC5RumbleRam,              // MBC5+RUMBLE+RAM
            0x1E => CartType::MBC5RumbleRamBattery,       // MBC5+RUMBLE+RAM+BATTERY
            0x20 => CartType::MBC6,                       // MBC6
            0x22 => CartType::MBC7SensorRumbleRamBattery, // MBC7+SENSOR+RUMBLE+RAM+BATTERY
            0xFC => CartType::PocketCamera,               // POCKET CAMERA
            0xFD => CartType::BandaiTama5,                // BANDAI TAMA5
            0xFE => CartType::HuC3,                       // HuC3
            0xFF => CartType::HuC1RamBattery,             // HuC1+RAM+BATTERY
            byte => CartType::Unknown(byte),
        },
        rom_banks: match buf_at!(ROM_SIZE) {
            value if (value <= 8) => 1 << value,
            _ => 1, // Assume 1 for invalid bytes
        },
        ram_banks: match buf_at!(RAM_SIZE) {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => 0, // Assume 0 for invalid bytes
        },
        destination: buf_at!(DESTINATION),
        old_licensee: buf_at!(OLD_LICENSEE),
        version: buf_at!(VERSION),
        header_checksum: buf_at!(HEADER_CHECKSUM),
        global_checksum: make_word!(buf_at!(GLOBAL_CHECKSUM_H), buf_at!(GLOBAL_CHECKSUM_L)),
    };

    f.seek(SeekFrom::Start(0))?;

    Ok(header)
}
