use std::{cell::RefCell, rc::Rc};

use cpu::CPU;
use gpu::GPU;
use mmu::MMU;

use crate::util::new;

mod cpu;
mod gpu;
mod mbc;
mod mmu;
mod time_types;

#[derive(Debug)]
pub struct GB {
    pub cpu: CPU,
    pub gpu: GPU,
    pub mmu: Rc<RefCell<MMU>>,
}
impl GB {
    new!({
        let mmu = Rc::new(RefCell::new(MMU::new()));

        Self {
            cpu: CPU::new(mmu.clone()),
            gpu: GPU::new(mmu.clone()),
            mmu,
        }
    });

    pub fn boot(&mut self) {
        while !self.cpu.terminate {
            let dt = self.cpu.step();
            self.gpu.step(dt);
        }
    }
}
