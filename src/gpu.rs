use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::mmu::MMU;

pub trait GPU<M: MMU>: Debug + Display {
    /// Return a new instance of the GPU.
    fn new(mmu: Rc<RefCell<M>>) -> Self;
}
