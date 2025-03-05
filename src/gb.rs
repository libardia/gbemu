use std::{cell::RefCell, fs, rc::Rc};

use crate::{cpu::CPU, gpu::GPU, mmu::MMU};

#[derive(Debug)]
pub struct GB<C, G, M: MMU>
where
    C: CPU<M>,
    G: GPU<M>,
{
    cpu: C,
    gpu: G,
    mmu: Rc<RefCell<M>>,
}

impl<C, G, M: MMU> GB<C, G, M>
where
    C: CPU<M>,
    G: GPU<M>,
{
    /// Create a new GameBoy.
    pub fn new() -> Self {
        let mmu = Rc::new(RefCell::new(M::new()));
        let cpu = C::new(mmu.clone());
        let gpu = G::new(mmu.clone(), 3);
        Self { cpu, gpu, mmu }
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
        self.cpu.execute();
    }

    /// Begin emulation, starting execution at the given address.
    pub fn execute_at(&mut self, address: u16) {
        self.cpu.execute_at(address);
    }
}
