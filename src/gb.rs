use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{cpu::CPU, mmu::MMU};

#[derive(Debug)]
pub struct GB<C, M: MMU>
where
    C: CPU<M>,
{
    cpu: C,
    mmu: Rc<RefCell<M>>,
}

impl<C, M: MMU> GB<C, M>
where
    C: CPU<M>,
{
    /// Create a new GameBoy.
    pub fn new() -> Self {
        let mmu = Rc::new(RefCell::new(M::new()));
        let cpu = C::new(mmu.clone());
        Self { cpu, mmu }
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

    /// Begin emulation, starting execution at 0.
    pub fn execute(&mut self) {
        self.cpu.execute();
    }

    /// Begin emulation, starting execution at the given address.
    pub fn execute_at(&mut self, address: u16) {
        self.cpu.execute_at(address);
    }
}
