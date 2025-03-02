use std::{fmt::write, path::Display};

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

#[derive(Debug)]
pub struct MMU {
    pub mem: [u8; EFFECTIVE_MEM_SIZE],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            mem: [0; EFFECTIVE_MEM_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.mem = [0; EFFECTIVE_MEM_SIZE];
    }

    fn get(&self, address: u16) -> u8 {
        let eff = self.calc_eff_address(address);

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

        self.mem[eff]
    }

    fn set(&mut self, address: u16, value: u8) {
        let eff = self.calc_eff_address(address);
        if address > SWITCH_ROM.end {
            if self.is_in_region(address, UNUSABLE) {
                // TODO: OAM corruption happens here, I think? And idk what gets written.
                self.mem[eff] = value;
            } else {
                self.mem[eff] = value;
            }
        }
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
        self.get(address)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.set(address, value);
    }

    // 16-bit =====================================================================================

    // NOTE: LITTLE-ENDIAN: second byte of value is stored in address, first byte is stored in
    // address + 1. This is very important because virtual 16-bit registers are BIG-ENDIAN.

    pub fn read_word(&self, address: u16) -> u16 {
        let ls = self.get(address);
        let ms = self.get(address + 1);
        ((ms as u16) << 8) + (ls as u16)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.set(address, (value & 0xFF) as u8);
        self.set(address + 1, ((value & 0xFF00) >> 8) as u8);
    }
}

impl std::fmt::Display for MMU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MMU: [")?;
        let mut one = false;
        for a in 0..TOTAL_MEM_SIZE {
            let b = self.read_byte(a as u16);
            if b != 0 {
                one = true;
                write!(f, "\n\t0x{:0>4X} = {}", a, b)?;
            }
        }
        if one {
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}
