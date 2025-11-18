mod cpu;
mod mmu;

use cpu::CPU;
use mmu::MMU;

use crate::{gb::cpu::instruction::Instruction, macros::new};

#[derive(Debug, Default)]
pub struct GB {
    mmu: MMU,
    cpu: CPU,
}

impl GB {
    new!();

    pub fn test_decode(&mut self, bytes: &[u8]) -> Instruction {
        self.cpu.decode(bytes)
    }
}
