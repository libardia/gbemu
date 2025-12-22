use crate::mmu::MMU;
use std::{cell::RefCell, rc::Rc};

pub struct PPU {
    mmu: Rc<RefCell<MMU>>,
}
