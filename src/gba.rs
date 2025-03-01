mod cpu;
mod mmu;

use std::{fmt, fmt::Display, fmt::Formatter};

use cpu::{instructions::*, CPU};
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

    pub fn run(&mut self /*, ROM */) {
        // TODO: boot sequence
        // TODO: load ROM
        loop {
            // TODO: get instruction
            // TODO: decode instruction
            // TODO: advance PC (emulate HALT bug)
            // TODO: execute
        }
    }

    pub fn test(&mut self) {
        use Instruction::*;
        let prog = [NOP, NOP, EI, NOP, NOP, DI, NOP, NOP];

        for inst in prog {
            self.cpu.pc += instruction_length(inst);
            self.cpu.execute(&mut self.mmu, inst);
            println!("{:?}\n{}\n", inst, self);
        }
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cpu)
    }
}
