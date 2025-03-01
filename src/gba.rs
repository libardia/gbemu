mod cpu;
mod mmu;

use std::{fmt, fmt::Display, fmt::Formatter};

use cpu::{instructions::*, CPU};
use mmu::MMU;

#[derive(Debug)]
pub struct GBA {
    pub cpu: CPU,
    pub mmu: MMU,
}

impl GBA {
    pub fn new() -> Self {
        GBA {
            cpu: CPU::new(),
            mmu: MMU::new(),
        }
    }

    pub fn mainloop(&mut self) {
        use Instruction::*;
        let ops = [
            LD_r8_r8(ArgR8::A, ArgR8::CONST(0b10001000)),
        ];

        for i in ops {
            self.cpu.execute(&mut self.mmu, i);
            println!("{:?}\n{}\n", i, self);
        }
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cpu)
    }
}
