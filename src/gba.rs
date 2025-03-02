mod cpu;
mod decoder;
mod mmu;

use std::fmt::{self, Display, Formatter};

use cpu::CPU;
use decoder::decode;
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

    pub fn run(&mut self, debug_print: bool /*, ROM */) {
        // TODO: boot sequence
        // TODO: load ROM
        while !self.cpu.terminate {
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);
            self.cpu.execute(&mut self.mmu, inst, inst_length);
            if debug_print {
                println!("{inst:?}, {inst_length} bytes\n{self}\n\n");
            }
        }
    }

    pub fn test(&mut self) {
        let prog = [
            0x01, 0xAD, 0xDE, // Write 0xDEAD into BC
            0x80, // A += B (0xDE)
            0x81, // A += C (0x8B)
            0xEA, 0xAD, 0xDE, // Write A to [0xDEAD]
            0xEC, // Terminate
        ];

        for (i, b) in prog.iter().enumerate() {
            self.mmu.set(i as u16, *b);
        }

        self.run(true);
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.cpu, self.mmu)
    }
}
