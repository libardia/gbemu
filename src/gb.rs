use crate::gb::{cpu::CPU, mmu::MMU};

mod cpu;
mod mmu;

struct GameBoy {
    cpu: CPU,
    mmu: MMU,
}
