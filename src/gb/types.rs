use crate::gb::macros::{address_fmt, byte_fmt};
use std::fmt::Debug;

pub type MTime = u16;
pub type TTime = u16;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Byte(pub u8);
impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&byte_fmt!(&self.0))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Word(pub u16);
impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&address_fmt!(&self.0))
    }
}
