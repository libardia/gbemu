mod cpu;
mod mmu;

use cpu::CPU;
use mmu::MMU;

#[derive(Debug, Default)]
pub struct GB {
    mmu: MMU,
    cpu: CPU,
}

impl GB {}
