use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{cpu::MTime, mmu::MMU};

pub trait PPU<M: MMU>: Debug + Display {
    /// Return a new instance of the PPU.
    fn new(mmu: Rc<RefCell<M>>, scale: usize, frame_rate: f32) -> Self;

    /// Process a number of "dots" based on M-time taken by the CPU. There are 4 dots per M-time.
    fn step_dots(&mut self, dm: MTime);

    /// Returns if termination was requested. Probably because the window was closed.
    fn should_terminate(&self) -> bool;
}
