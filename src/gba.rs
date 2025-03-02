mod cpu;
mod decoder;
mod mmu;

use std::fmt::{self, Display, Formatter};

use cpu::CPU;
use decoder::decode;
use mmu::MMU;

#[derive(Debug)]
pub struct GBA {
    pub cpu: CPU,
    pub mmu: MMU,

    // Emulating the HALT bug
    skip_next_pc: bool,
}

impl GBA {
    pub fn new() -> Self {
        GBA {
            cpu: CPU::new(),
            mmu: MMU::new(),
            skip_next_pc: false,
        }
    }

    pub fn load(&mut self, rom: &[u8]) {
        for (i, b) in rom.iter().enumerate() {
            self.mmu.set(i as u16, *b);
        }
    }

    pub fn run(&mut self, debug_print: bool) {
        // TODO: boot sequence
        while !self.cpu.terminate {
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);
            self.cpu.execute(&mut self.mmu, inst, inst_length);
            if debug_print {
                println!("{inst:?}, {inst_length} bytes\n{self}\n\n");
            }
        }
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.cpu, self.mmu)
    }
}
