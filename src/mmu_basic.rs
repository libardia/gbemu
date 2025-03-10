use std::fmt::{Debug, Display};

use crate::{
    hex::HexU8,
    mem_region::{
        io_regs::REG_BANK,
        regions::{BOOT_ROM_BANK, ECHO_RAM, UNUSABLE_MEM},
        MemoryRegion,
    },
    mmu::{BOOT_ROM, MMU},
};

const APPARENT_MEM_SIZE: usize = 0xFFFF + 1;
const EFFECTIVE_MEM_SIZE: usize = APPARENT_MEM_SIZE - ECHO_RAM.size() as usize;

pub struct BasicMMU {
    mem: [u8; EFFECTIVE_MEM_SIZE],
    blocked_regions: Vec<MemoryRegion>,
}

impl BasicMMU {
    fn eff_address(&self, address: u16) -> usize {
        const ECHO_RAM_OFFSET: u16 = 0x2000;

        let eff_address = if address < ECHO_RAM.begin {
            address
        } else if ECHO_RAM.contains(address) {
            address - ECHO_RAM_OFFSET
        } else {
            address - ECHO_RAM.size()
        };

        eff_address as usize
    }

    fn internal_read_byte(&self, address: u16, read_blocked: bool) -> u8 {
        if !read_blocked {
            for blocked in &self.blocked_regions {
                if blocked.contains(address) {
                    // Reads to blocked ranges return 0xFF
                    return 0xFF;
                }
            }
        }

        if BOOT_ROM_BANK.contains(address) {
            if self.get(REG_BANK) == 0 {
                BOOT_ROM[address as usize]
            } else {
                self.get(address)
            }
        } else if UNUSABLE_MEM.contains(address) {
            // Reads in the unusable range return 0xFF
            0xFF
        } else {
            self.get(address)
        }
    }
}

impl MMU for BasicMMU {
    fn new() -> Self {
        let mut blocked = Vec::new();
        blocked.reserve(10);

        Self {
            mem: [0; EFFECTIVE_MEM_SIZE],
            blocked_regions: blocked,
        }
    }

    fn get(&self, address: u16) -> u8 {
        self.mem[self.eff_address(address)]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.mem[self.eff_address(address)] = value;
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.internal_read_byte(address, false)
    }

    fn read_blocked_byte(&self, address: u16) -> u8 {
        self.internal_read_byte(address, true)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        for blocked in &self.blocked_regions {
            if blocked.contains(address) {
                // Writes to blocked ranges are ignored
                return;
            }
        }

        // Writes in the unusable range are ignored
        if !UNUSABLE_MEM.contains(address) {
            self.set(address, value);
        }
    }

    fn block_range(&mut self, region: MemoryRegion) {
        self.blocked_regions.push(region);
    }

    fn unblock_range(&mut self, region: MemoryRegion) {
        self.blocked_regions.retain(|&x| x != region);
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
