use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{gpu::GPU, mmu::MMU};

#[derive(Debug)]
pub struct BasicGPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
}

impl<M: MMU> GPU<M> for BasicGPU<M> {
    fn new(mmu: Rc<RefCell<M>>) -> Self {
        Self { mmu: mmu.clone() }
    }
}

impl<M: MMU> Display for BasicGPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
