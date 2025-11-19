mod cpu;
mod mmu;

use crate::macros::new;
use cpu::CPU;
use log::debug;
use mmu::{AccessMode, MMU};

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

    pub fn test_decode(&mut self) {
        self.mmu.set_access_mode(AccessMode::CPU);
        debug!("{:X?}", self.cpu.decode(&self.mmu));
    }
}
