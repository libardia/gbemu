mod cpu;
mod mmu;

use cpu::CPU;
use mmu::MMU;

use crate::macros::new;

#[derive(Debug, Default)]
pub struct GB {
    mmu: MMU,
    cpu: CPU,
}

impl GB {
    new!(
        mmu = MMU::new();
        cpu = CPU::new();
    );
}
