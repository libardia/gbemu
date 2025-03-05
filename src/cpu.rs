use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::mmu::MMU;

/// Trait defining behavior for all CPUs.
pub trait CPU<M: MMU>: Debug + Display {
    /// Return a new instance of the CPU.
    fn new(mmu: Rc<RefCell<M>>) -> Self;

    /// Begin execution. Starts at address 0.
    fn execute(&mut self) {
        self.execute_at(0);
    }

    /// Begin execution at a specified memory address.
    fn execute_at(&mut self, address: u16);

    /// Set debug mode. When `true`, breakpoints and debug printing is enabled.
    fn set_debug_mode(&mut self, mode: bool);

    /// Set breakpoints. Breakpoints only have an effect in debug mode.
    fn set_breakpoints(&mut self, breakpoints: &[u16]);
}
