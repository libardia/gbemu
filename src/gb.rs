use std::{cell::RefCell, fs, rc::Rc};

use crate::{cpu::CPU, mmu::MMU, ppu::PPU};

pub struct GB<C, P, M: MMU>
where
    C: CPU<M>,
    P: PPU<M>,
{
    cpu: C,
    ppu: P,
    mmu: Rc<RefCell<M>>,
}

impl<C, P, M: MMU> GB<C, P, M>
where
    C: CPU<M>,
    P: PPU<M>,
{
    /// Create a new GameBoy.
    pub fn new() -> Self {
        let mmu = Rc::new(RefCell::new(M::new()));
        let cpu = C::new(mmu.clone());
        let gpu = P::new(mmu.clone(), 3);
        Self { cpu, ppu: gpu, mmu }
    }

    /// Set debug mode. When `true`, breakpoints and debug printing is enabled.
    pub fn set_debug_mode(&mut self, mode: bool) {
        self.cpu.set_debug_mode(mode);
    }

    /// Set breakpoints. Breakpoints only have an effect in debug mode.
    pub fn set_breakpoints(&mut self, breakpoints: &[u16]) {
        self.cpu.set_breakpoints(breakpoints);
    }

    /// Load a program into ROM.
    pub fn load_rom(&self, load_at: u16, rom: &[u8]) {
        self.mmu.borrow_mut().load_rom(load_at, rom);
    }

    /// Load a program into ROM from file.
    pub fn load_rom_file(&self, file_path: &str) {
        let bytes =
            fs::read(file_path).expect(format!("Failed to read file {}", file_path).as_str());
        self.load_rom(0, &bytes);
    }

    /// Begin emulation, starting execution at 0.
    pub fn execute(&mut self) {
        self.execute_at(0);
    }

    /// Begin emulation, starting execution at the given address.
    pub fn execute_at(&mut self, address: u16) {
        self.cpu.set_pc(address);
        while !self.cpu.should_terminate() && !self.ppu.should_terminate() {
            let dm = self.cpu.step();
            self.ppu.step_dots(dm);
        }
    }
}
