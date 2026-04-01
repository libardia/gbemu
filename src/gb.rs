use crate::gb::{cpu::CPU, mmu::MMU};

mod cpu;
mod mmu;

struct GameBoy {
    cpu: CPU,
    mmu: MMU,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(),
        }
    }
}
