use std::fmt::{Debug, Display};

use crate::{hex::HexU8, mmu::MMU};

#[allow(dead_code)]
mod regions {
    #[derive(Debug, Clone, Copy)]
    pub struct MemoryRegion {
        pub begin: u16,
        pub end: u16,
        pub size: u16,
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

    pub const MAIN_ROM: MemoryRegion = MemoryRegion::new(0x0000, 0x3FFF);
    pub const SWITCH_ROM: MemoryRegion = MemoryRegion::new(0x4000, 0x7FFF);
    pub const VRAM: MemoryRegion = MemoryRegion::new(0x8000, 0x9FFF);
    pub const EXT_RAM: MemoryRegion = MemoryRegion::new(0xA000, 0xBFFF);
    pub const WORK_RAM_1: MemoryRegion = MemoryRegion::new(0xC000, 0xCFFF);
    pub const WORK_RAM_2: MemoryRegion = MemoryRegion::new(0xD000, 0xDFFF);
    pub const ECHO_RAM: MemoryRegion = MemoryRegion::new(0xE000, 0xFDFF);
    pub const OAM: MemoryRegion = MemoryRegion::new(0xFE00, 0xFE9F);
    pub const UNUSABLE: MemoryRegion = MemoryRegion::new(0xFEA0, 0xFEFF);
    pub const IO: MemoryRegion = MemoryRegion::new(0xFF00, 0xFF7F);
    pub const HIGH_RAM: MemoryRegion = MemoryRegion::new(0xFF80, 0xFFFE);
    pub const IE: MemoryRegion = MemoryRegion::new(0xFFFF, 0xFFFF);
}

use regions::*;
const APPARENT_MEM_SIZE: usize = 0xFFFF + 1;
const ECHO_RAM_OFFSET: u16 = 0x2000;
const EFFECTIVE_MEM_SIZE: usize = APPARENT_MEM_SIZE - ECHO_RAM.size as usize;

pub struct BasicMMU {
    mem: [u8; EFFECTIVE_MEM_SIZE],
}

impl BasicMMU {
    fn is_in_region(address: u16, region: MemoryRegion) -> bool {
        address >= region.begin && address <= region.end
    }

    fn eff_address(&self, address: u16) -> usize {
        (if address < ECHO_RAM.begin {
            address
        } else if address <= ECHO_RAM.end {
            address - ECHO_RAM_OFFSET
        } else {
            address - ECHO_RAM.size
        }) as usize
    }
}

impl MMU for BasicMMU {
    fn new() -> Self {
        Self {
            mem: [0; EFFECTIVE_MEM_SIZE],
        }
    }

    fn get(&self, address: u16) -> u8 {
        self.mem[self.eff_address(address)]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.mem[self.eff_address(address)] = value;
    }

    fn read_byte(&self, address: u16) -> u8 {
        // TODO: when is OAM block?
        let oam_block = false;
        if Self::is_in_region(address, UNUSABLE) {
            if oam_block {
                // TODO: OAM corruption happens here. Also, what actually gets returned..?
                return 0xFF;
            } else {
                return 0;
            }
        }

        self.get(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address > SWITCH_ROM.end {
            if Self::is_in_region(address, UNUSABLE) {
                // TODO: OAM corruption happens here, I think? And idk what gets written.
                self.set(address, value);
            } else {
                self.set(address, value);
            }
        }
    }
}

impl Debug for BasicMMU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Display for BasicMMU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
