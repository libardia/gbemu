use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::mmu::MMU;

pub type MTime = u32;

/// Trait defining behavior for all CPUs.
pub trait CPU<M: MMU>: Debug + Display {
    /// Return a new instance of the CPU.
    fn new(mmu: Rc<RefCell<M>>) -> Self;

    /// Execute a single instruction, advancing the program counter. Returns the number of cycles
    /// elapsed in M-time, which is the actual number of cycles elapsed divided by 4. Multply by 4
    /// to get the actual cycles.
    fn step(&mut self) -> MTime;

    /// Get program counter.
    fn get_pc(&self) -> u16;

    /// Set program counter.
    fn set_pc(&mut self, value: u16);

    /// Set debug mode. When `true`, breakpoints and debug printing is enabled.
    fn set_debug_mode(&mut self, mode: bool);

    /// Get debug mode.
    fn get_debug_mode(&self) -> bool;

    /// Set breakpoints. Breakpoints only have an effect in debug mode.
    fn set_breakpoints(&mut self, breakpoints: &[u16]);

    /// Get breakpoints.
    fn get_breakpoints(&self) -> &[u16];

    /// Returns if termination was requested.
    fn should_terminate(&self) -> bool;
}
