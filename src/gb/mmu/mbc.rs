mod nombc;

use std::fmt::Debug;

pub trait MBC: Debug {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, value: u8);
}
