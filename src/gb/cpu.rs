mod instruction;
mod optable;

use crate::{
    gb::{
        mmu::{AccessMode, MMU},
        MachineCycles,
    },
    macros::new,
};

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

    // Helper
    pub enable_meta_instructions: bool,
}

impl CPU {
    new!();

    // Accessors
    getset_r16!(get_bc, set_bc, b, c);
    getset_r16!(get_de, set_de, d, e);
    getset_r16!(get_hl, set_hl, h, l);
    getset_r16!(get_af, set_af, a, f);

    fn get_hli(&mut self) -> u16 {
        let orig = self.get_hl();
        self.set_hl(self.get_hl() + 1);
        orig
    }

    fn get_hld(&mut self) -> u16 {
        let orig = self.get_hl();
        self.set_hl(self.get_hl() - 1);
        orig
    }

    // Step
    pub fn step(&mut self, mmu: &mut MMU) -> MachineCycles {
        // Tell the MMU that the CPU is accessing it
        mmu.access_mode = AccessMode::CPU;

        // Decode instruction at PC
        let inst = self.decode(mmu);

        self.execute(mmu, inst)
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
            self.$r1 = crate::macros::byte_of!(value, 1);
            self.$r2 = crate::macros::byte_of!(value, 0);
        }
    };
}
use getset_r16;
