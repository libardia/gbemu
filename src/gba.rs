mod cpu;
mod mmu;

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
        self.cpu.execute(&mut self.mmu, Instruction::ADC_a_r8(ArgR8::A));
    }
}
