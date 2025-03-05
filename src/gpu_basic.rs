use std::{cell::RefCell, fmt::Display, rc::Rc};

use minifb::{Window, WindowOptions};

use crate::{gpu::GPU, mmu::MMU};

#[derive(Debug)]
pub struct BasicGPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
    win: Window,
}

impl<M: MMU> GPU<M> for BasicGPU<M> {
    fn new(mmu: Rc<RefCell<M>>) -> Self {
        let win = Window::new("gbemu", 160, 144, WindowOptions::default())
            .expect("Failed to create window");
        Self {
            mmu: mmu.clone(),
            win,
        }
    }
}

impl<M: MMU> Display for BasicGPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
