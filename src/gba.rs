mod cpu;
mod mmu;

use std::{fmt, fmt::Display, fmt::Formatter};

use cpu::{instructions::*, CPU};
use mmu::MMU;

#[derive(Debug)]
pub struct GBA {
    pub cpu: CPU,
    pub mmu: MMU,

    // Emulating the HALT bug
    skip_next_pc: bool,
}

impl GBA {
    pub fn new() -> Self {
        GBA {
            cpu: CPU::new(),
            mmu: MMU::new(),
            skip_next_pc: false,
        }
    }

    pub fn run(&mut self /*, ROM */) {
        // TODO: boot sequence
        // TODO: load ROM
        loop {
            // TODO: get instruction
            // TODO: decode instruction
            // TODO: advance PC (emulate HALT bug)
            // TODO: execute
        }
    }

    pub fn test(&mut self) {
        use Instruction::*;
        let prog = [
            NOP,
            NOP,
            EI,
            NOP,
            NOP,
            DI,
            NOP,
            NOP,
            LD_r8_r8(ArgR8::B, ArgR8::CONST(0x12)),
            LD_r8_r8(ArgR8::C, ArgR8::CONST(0x13)),
            LD_r8_r8(ArgR8::D, ArgR8::CONST(0x14)),
            LD_r8_r8(ArgR8::E, ArgR8::CONST(0x15)),
            LD_r8_r8(ArgR8::H, ArgR8::CONST(0xD5)),
            LD_r8_r8(ArgR8::L, ArgR8::CONST(0x10)),
            LD_r8_r8(ArgR8::MHL, ArgR8::CONST(0x18)),
            LD_r16_n16(ArgR16::HL, 0xDEAD),
            LD_r8_r8(ArgR8::MHL, ArgR8::CONST(255)),
            ADD_a_r8(ArgR8::B),
            ADD_a_r8(ArgR8::C),
            LD_r8_r8(ArgR8::B, ArgR8::A),
            SUB_a_r8(ArgR8::A),
        ];

        for inst in prog {
            self.cpu.pc += instruction_length(inst);
            self.cpu.execute(&mut self.mmu, inst);
            println!("{:?}\n{}\n\n", inst, self);
        }
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.cpu, self.mmu)
    }
}
