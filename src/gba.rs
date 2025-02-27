mod cpu;
mod mmu;

use std::{fmt, fmt::Display, fmt::Formatter};

use cpu::{CPU, instructions::*};
use mmu::MMU;

#[derive(Debug)]
pub struct GBA {
    pub cpu: CPU,
    pub mmu: MMU,
}

impl GBA {
    pub fn new() -> Self {
        GBA { cpu: CPU::new(), mmu: MMU::new() }
    }

    pub fn mainloop(&mut self) {
        self.cpu.execute(&mut self.mmu, Instruction::ADD_a_r8(ArgR8::CONST(15)));
    }
}


impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cpu)
    }
}
