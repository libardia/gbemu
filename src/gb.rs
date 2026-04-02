use crate::gb::{cpu::CPU, mmu::MMU};

mod cpu;
mod mmu;

pub struct GameBoy {
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

    pub fn run(&mut self) {
        loop {
            // CPU step will handle ticking all other hardware,
            // because instructions take variable amounts of time
            CPU::step(self);
            break;
        }
    }

    fn long_tick(&mut self) {
        for _ in 0..4 {
            self.tick();
        }
    }

    fn tick(&mut self) {}
}
