mod instruction;
mod optable;

use crate::gb::{
    macros::{address_fmt, bit_flag, byte_fmt, new},
    mmu::{AccessMode, MMU},
    regions::{IO_IF, IO_JOYP},
    types::MTime,
};

#[derive(Debug, Default)]
enum EIState {
    #[default]
    Idle,
    Waiting,
    Now,
}

#[derive(Debug, Default)]
enum CPUMode {
    #[default]
    Normal,
    Halt,
    Stop,
}

#[derive(Debug, Default)]
pub struct CPU {
    // Registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
    f: u8,
    pc: u16,
    sp: u16,

    // Flags
    ime: bool,
    mode: CPUMode,

    // Helper
    ei_state: EIState,
    halt_bug: bool,

    // Meta
    pub enable_meta_instructions: bool,
}

impl CPU {
    new!();

    // Step
    pub fn step(&mut self, mmu: &mut MMU) -> MTime {
        // Tell the MMU that the CPU is accessing it
        mmu.access_mode = AccessMode::CPU;

        // Delayed effect of EI
        match self.ei_state {
            EIState::Idle => (), // Do nothing
            EIState::Waiting => self.ei_state = EIState::Now,
            EIState::Now => {
                self.ime = true;
                self.ei_state = EIState::Idle;
            }
        }

        match self.mode {
            CPUMode::Normal => (), // Do nothing
            CPUMode::Halt => {
                // HALT ends whenever any interrupt is triggered (whether or not it would be handled)
                if mmu.get(IO_IF) != 0 {
                    self.mode = CPUMode::Normal;
                }
            }
            CPUMode::Stop => {
                // STOP ends when any input bit of IO_JOYP goes low (a button is pressed)
                if (mmu.get(IO_JOYP) & 0xF) != 0xF {
                    self.mode = CPUMode::Normal;
                }
            }
        }

        match self.mode {
            CPUMode::Normal => {
                // Handle interruptions
                // TODO: interruptions

                // For logging
                let inst_add = self.pc;
                let inst_byte = mmu.get(inst_add);

                // Decode and execute instruction at PC
                let inst = self.decode(mmu);

                trace!(
                    "execute {} @ {}: {inst:?}",
                    byte_fmt!(inst_byte),
                    address_fmt!(inst_add)
                );

                // Execute
                self.execute(mmu, inst)
            }
            _ => 1,
        }
    }

    // Accessors
    getset_r16!(get_bc, set_bc, b, c);
    getset_r16!(get_de, set_de, d, e);
    getset_r16!(get_hl, set_hl, h, l);
    getset_r16!(get_af, set_af, a, f);

    // F register flags
    bit_flag!(getf_z, setf_z, f, 7);
    bit_flag!(getf_n, setf_n, f, 6);
    bit_flag!(getf_h, setf_h, f, 5);
    bit_flag!(getf_c, setf_c, f, 4);

    fn set_all_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.f = if z { 1 << 7 } else { 0 }
            | if n { 1 << 6 } else { 0 }
            | if h { 1 << 5 } else { 0 }
            | if c { 1 << 4 } else { 0 }
    }

    // HL +/-
    fn get_hli(&mut self) -> u16 {
        let orig = self.get_hl();
        self.set_hl(self.get_hl().wrapping_add(1));
        orig
    }

    fn get_hld(&mut self) -> u16 {
        let orig = self.get_hl();
        self.set_hl(self.get_hl().wrapping_sub(1));
        orig
    }

    // Stack
    fn push_stack(&mut self, mmu: &mut MMU, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;

        self.sp = self.sp.wrapping_sub(1);
        mmu.set(self.sp, high);
        self.sp = self.sp.wrapping_sub(1);
        mmu.set(self.sp, low);
    }

    fn pop_stack(&mut self, mmu: &mut MMU) -> u16 {
        let low = mmu.get(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high = mmu.get(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (high << 8) | low
    }
}

mod decode;
mod execute;

macro_rules! getset_r16 {
    ($getname:ident, $setname:ident, $r1:ident, $r2:ident) => {
        fn $getname(&self) -> u16 {
            let r1 = self.$r1 as u16;
            let r2 = self.$r2 as u16;
            (r1 << 8 | r2)
        }

        fn $setname(&mut self, value: u16) {
            self.$r1 = crate::gb::macros::byte_of!(value, 1);
            self.$r2 = crate::gb::macros::byte_of!(value, 0);
        }
    };
}
use getset_r16;
use log::trace;
