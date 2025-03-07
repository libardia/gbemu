use std::fmt::{Debug, Display};

use crate::{hex::HexU8, mmu::MMU};

const ECHO_RAM_BEGIN: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;
const ECHO_RAM_SIZE: u16 = 0x1E00;

const UNUSABLE_RAM_BEGIN: u16 = 0xFEA0;
const UNUSABLE_RAM_END: u16 = 0xFEFF;

const ECHO_RAM_OFFSET: u16 = 0x2000;

const APPARENT_MEM_SIZE: usize = 0xFFFF + 1;
const EFFECTIVE_MEM_SIZE: usize = APPARENT_MEM_SIZE - ECHO_RAM_SIZE as usize;

pub struct BasicMMU {
    mem: [u8; EFFECTIVE_MEM_SIZE],
}

impl BasicMMU {
    fn eff_address(&self, address: u16) -> usize {
        (match address {
            ..ECHO_RAM_BEGIN => address,
            ..=ECHO_RAM_END => address - ECHO_RAM_OFFSET,
            _ => address - ECHO_RAM_SIZE,
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
        match address {
            ..UNUSABLE_RAM_BEGIN => self.get(address),
            ..=UNUSABLE_RAM_END => 0xFF,
            _ => self.get(address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            ..UNUSABLE_RAM_BEGIN => self.set(address, value),
            ..=UNUSABLE_RAM_END => (),
            _ => self.set(address, value),
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
