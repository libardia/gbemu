mod registers;

use registers::*;
use std::{fmt, fmt::Display, fmt::Formatter};

#[derive(Default, Debug)]
pub struct CPU  {
    pub registers: Registers
}

impl Display for CPU {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let Registers {a, f, b, c, d, e, h, l} = &self.registers;
        write!(formatter, "+--------------------+\n")?;
        write!(formatter, "| A: 0x{a:0>2X}    F: 0x{f:0>2X} |\n")?;
        // write!(formatter, "|     {a:0>3}        {f:0>3} |\n")?;
        write!(formatter, "| B: 0x{b:0>2X}    C: 0x{c:0>2X} |\n")?;
        // write!(formatter, "|     {b:0>3}        {c:0>3} |\n")?;
        write!(formatter, "| D: 0x{d:0>2X}    E: 0x{e:0>2X} |\n")?;
        // write!(formatter, "|     {d:0>3}        {e:0>3} |\n")?;
        write!(formatter, "| H: 0x{h:0>2X}    L: 0x{l:0>2X} |\n")?;
        // write!(formatter, "|     {h:0>3}        {l:0>3} |\n")?;
        write!(formatter, "+--------------------+\n")
    }
}