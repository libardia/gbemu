mod cpu;
mod decoder;
mod mmu;

use std::{fmt, fmt::Display, fmt::Formatter};

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

    pub fn run(&mut self /*, ROM */) {
        // TODO: boot sequence
        // TODO: load ROM
        loop {
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);
            // TODO: advance PC (emulate HALT bug)
            if self.skip_next_pc {
                self.cpu.pc += inst_length;
                self.skip_next_pc = false;
            }

            self.cpu.execute(&mut self.mmu, inst);

            // Terminate emulator
            if self.cpu.terminate {
                break;
            }
        }
    }

    pub fn test(&mut self) {
        let prog = [
            0x04u8, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x60, 0x68, 0x29, 0x29,
            0x29, 0x70,
        ];

        for (i, b) in prog.iter().enumerate() {
            self.mmu.set(i as u16, *b);
        }

        while self.mmu.get(self.cpu.pc) != 0 {
            let (inst, inst_length) = decoder::decode(&self.mmu, self.cpu.pc);
            self.cpu.pc += inst_length;
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
