use crate::{
    cart::Cart,
    regions::{self, MappedMemoryRegion},
};

mod regs;

pub struct MMU {
    pub cart: Box<dyn Cart>,

    // RAM areas
    vram: MappedMemoryRegion,
}

impl MMU {
    pub fn new(cart: Box<dyn Cart>) -> Self {
        Self {
            cart,
            vram: MappedMemoryRegion::new(regions::VRAM),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
