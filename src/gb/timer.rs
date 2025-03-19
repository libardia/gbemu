use std::{cell::RefCell, rc::Rc};

use crate::util::new;

use super::{mmu::MMU, time_types::MTime};

#[derive(Debug, Default)]
pub struct Timer {
    mmu: Rc<RefCell<MMU>>,
    div: u16,
}

impl Timer {
    new!(mmu: Rc<RefCell<MMU>>);

    pub fn step(&mut self, dt: MTime) {}
}
