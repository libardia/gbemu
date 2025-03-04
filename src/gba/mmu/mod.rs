use std::fmt::{self, Debug, Display, Formatter};

use crate::hex::HexU8;

pub const BOOT_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];

#[derive(Debug, Clone, Copy)]
struct MemoryRegion {
    begin: u16,
    end: u16,
    size: u16,
}

impl MemoryRegion {
    pub const fn new(begin: u16, end: u16) -> Self {
        MemoryRegion {
            begin,
            end,
            size: end - begin + 1,
        }
    }
}

const TOTAL_MEM_SIZE: usize = 0xFFFF + 1;

const MAIN_ROM: MemoryRegion = MemoryRegion::new(0x0000, 0x3FFF);
const SWITCH_ROM: MemoryRegion = MemoryRegion::new(0x4000, 0x7FFF);
const VRAM: MemoryRegion = MemoryRegion::new(0x8000, 0x9FFF);
const EXT_RAM: MemoryRegion = MemoryRegion::new(0xA000, 0xBFFF);
const WORK_RAM_1: MemoryRegion = MemoryRegion::new(0xC000, 0xCFFF);
const WORK_RAM_2: MemoryRegion = MemoryRegion::new(0xD000, 0xDFFF);
const ECHO_RAM: MemoryRegion = MemoryRegion::new(0xE000, 0xFDFF);
const OAM: MemoryRegion = MemoryRegion::new(0xFE00, 0xFE9F);
const UNUSABLE: MemoryRegion = MemoryRegion::new(0xFEA0, 0xFEFF);
const IO: MemoryRegion = MemoryRegion::new(0xFF00, 0xFF7F);
const HIGH_RAM: MemoryRegion = MemoryRegion::new(0xFF80, 0xFFFE);
const IE: MemoryRegion = MemoryRegion::new(0xFFFF, 0xFFFF);

const ECHO_RAM_OFFSET: u16 = 0x2000;

const EFFECTIVE_MEM_SIZE: usize = TOTAL_MEM_SIZE - ECHO_RAM.size as usize;

pub struct MMU {
    pub mem: [u8; EFFECTIVE_MEM_SIZE],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            mem: [0; EFFECTIVE_MEM_SIZE],
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        let eff = self.calc_eff_address(address);
        self.mem[eff]
    }

    pub fn set(&mut self, address: u16, value: u8) {
        let eff = self.calc_eff_address(address);
        self.mem[eff] = value;
    }

    fn is_in_region(&self, address: u16, region: MemoryRegion) -> bool {
        address >= region.begin && address <= region.end
    }

    fn calc_eff_address(&self, address: u16) -> usize {
        (if address < ECHO_RAM.begin {
            address
        } else if self.is_in_region(address, ECHO_RAM) {
            address - ECHO_RAM_OFFSET
        } else {
            address - ECHO_RAM.size
        }) as usize
    }

    // 8-bit ======================================================================================

    pub fn read_byte(&self, address: u16) -> u8 {
        // TODO: when is OAM block?
        let oam_block = false;
        if self.is_in_region(address, UNUSABLE) {
            if oam_block {
                // TODO: OAM corruption happens here. Also, what actually gets returned..?
                return 0xFF;
            } else {
                return 0;
            }
        }

        self.get(address)
    }

    pub fn read_signed_byte(&self, address: u16) -> i8 {
        self.read_byte(address) as i8
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address > SWITCH_ROM.end {
            if self.is_in_region(address, UNUSABLE) {
                // TODO: OAM corruption happens here, I think? And idk what gets written.
                self.set(address, value);
            } else {
                self.set(address, value);
            }
        }
    }

    // 16-bit =====================================================================================

    // NOTE: LITTLE-ENDIAN: second byte of value is stored in address, first byte is stored in
    // address + 1. This is very important because virtual 16-bit registers are BIG-ENDIAN.

    pub fn read_word(&self, address: u16) -> u16 {
        let ls = self.read_byte(address);
        let ms = self.read_byte(address + 1);
        ((ms as u16) << 8) + (ls as u16)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, ((value & 0xFF00) >> 8) as u8);
    }
}

impl Debug for MMU {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let addr_end = 0xFFFF + 1;
        writeln!(f, "MMU: [")?;
        for a in 0..addr_end {
            let b = self.get(a as u16);
            if b != 0 {
                writeln!(f, "\t0x{:0>4X}: {:?}", a, HexU8(b))?;
            }
        }
        write!(f, "]")
    }
}

impl Display for MMU {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let addr_end = 0xFFFF + 1;
        let u8_end = 0xFF + 1;
        write!(f, "MMU:")?;
        for i in 0..u8_end {
            write!(f, " xx{:0>2X}", i)?;
        }
        let mut a = 0;
        while a < addr_end {
            write!(f, "\n")?;
            for i in 0..(0xFF + 1) {
                if i == 0 {
                    write!(f, "{:0>2X}xx", (a & 0xFF00) >> 8)?;
                }
                let b = self.get(a as u16);
                if b != 0 {
                    write!(f, "  {:0>2X} ", b)?;
                } else {
                    write!(f, "     ")?;
                }
                a += 1;
            }
        }
        write!(f, "")
    }
}
