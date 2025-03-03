pub mod cpu;
pub mod decoder;
pub mod mmu;

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

    pub fn load(&mut self, begin: u16, rom: &[u8]) {
        for (i, b) in rom.iter().enumerate() {
            self.mmu.set(begin + (i as u16), *b);
        }
    }

    pub fn run(&mut self, debug_print: bool) {
        // TODO: load boot rom
        while !self.cpu.terminate {
            let pc_before = self.cpu.pc;
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);
            self.cpu.execute(&mut self.mmu, inst, inst_length);
            if debug_print {
                // println!("{inst:?}, {inst_length} bytes\n{self}\n\n");
                println!("{pc_before}: {inst:?}, {inst_length} bytes");
            }
        }
    }

    pub fn translate(&mut self, rom: &[u8]) {
        self.load(0, rom);
        while self.cpu.pc < rom.len() as u16 {
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);
            println!("{:0>4X} = {:0>3}, {inst_length}b: {inst:?}", self.cpu.pc, self.cpu.pc);
            self.cpu.pc += inst_length;
        }
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.cpu, self.mmu)
        // write!(f, "{}", self.cpu)
    }
}
