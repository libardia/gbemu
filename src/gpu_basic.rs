use std::{cell::RefCell, fmt::Display, rc::Rc};

use minifb::{Window, WindowOptions};

use crate::{gpu::GPU, mmu::MMU};

const BASE_WIDTH: usize = 160;
const BASE_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct BasicGPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
    scale: usize,
    win: Window,
}

impl<M: MMU> GPU<M> for BasicGPU<M> {
    fn new(mmu: Rc<RefCell<M>>, scale: usize) -> Self {
        let win = Window::new(
            "gbemu",
            BASE_WIDTH * scale,
            BASE_HEIGHT * scale,
            WindowOptions::default(),
        )
        .expect("Failed to create window");
        Self {
            mmu: mmu.clone(),
            scale,
            win,
        }
    }
}

impl<M: MMU> Display for BasicGPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
