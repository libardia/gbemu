use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{cpu::MTime, mmu::MMU};

pub trait GPU<M: MMU>: Debug + Display {
    /// Return a new instance of the GPU.
    fn new(mmu: Rc<RefCell<M>>, scale: usize) -> Self;

    /// Process a number of "dots". There are 4 dots per M-time.
    fn draw(&self, m_time: MTime);
}
