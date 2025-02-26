mod cpu;
mod mmu;

use cpu::CPU;
use mmu::MMU;

#[derive(Debug, Default)]
pub struct GBA {
    pub cpu: CPU,
    pub mmu: MMU,
}
