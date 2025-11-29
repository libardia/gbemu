mod cpu;
mod macros;
mod mmu;
mod regions;

use cpu::CPU;
use macros::new;
use mmu::MMU;

type MTime = u16;
type TTime = u16;

#[derive(Debug, Default)]
pub struct GB {
    mmu: MMU,
    cpu: CPU,
    skip_boot: bool,
}

impl GB {
    new!(
        mmu = MMU::new();
        cpu = CPU::new();
        ...
    );

    pub fn skip_boot(&mut self, boot: bool) {
        self.skip_boot = boot;
    }

    pub fn enable_meta_instructions(&mut self, enable: bool) {
        self.cpu.enable_meta_instructions = enable;
    }

    pub fn load_cart(&mut self, cart_path: String) {}

    pub fn run(&mut self) {
        loop {
            let m_dt = self.cpu.step(&mut self.mmu);
            let t_dt = m_dt * 4;

            // TODO: dmau.step(m_dt)
            // TODO: ppu.step(t_dt)
            // TODO: apu.step(t_dt)
        }
    }
}
