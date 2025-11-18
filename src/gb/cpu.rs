use crate::macros::new;

mod decode;
pub mod instruction;
mod optable;

#[derive(Debug, Default)]
pub struct CPU {
    // Registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
    f: u8,
    pc: u16,
    sp: u16,
    // Flags
    ime: bool,
}

impl CPU {
    new!();
}
