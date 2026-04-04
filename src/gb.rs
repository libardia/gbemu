use crate::gb::{cpu::CPU, mmu::MMU};

mod cpu;
mod mmu;

#[derive(Debug)]
pub struct GameBoy {
    cpu: CPU,
    mmu: MMU,

    debug_timer: u64,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(),

            debug_timer: Default::default(),
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

    fn tick(&mut self) {
        self.debug_timer += 1;
    }
}
