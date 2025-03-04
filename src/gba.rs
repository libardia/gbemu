pub mod cpu;
pub mod decoder;
pub mod mmu;

use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

use cpu::{instructions::Instruction, CPU};
use decoder::decode;
use mmu::MMU;

#[derive(Debug)]
pub struct GBA {
    pub cpu: CPU,
    pub mmu: MMU,

    // Emulating the HALT bug
    skip_next_pc: bool,

    // Debugging
    pub debug_mode: bool,
    breakpoint_mode: bool,
    breakpoints: Vec<u16>,
}

impl GBA {
    pub fn new() -> Self {
        GBA {
            cpu: CPU::new(),
            mmu: MMU::new(),
            skip_next_pc: false,
            debug_mode: false,
            breakpoint_mode: false,
            breakpoints: vec![],
        }
    }

    pub fn load(&mut self, begin: u16, rom: &[u8]) {
        for (i, b) in rom.iter().enumerate() {
            self.mmu.set(begin + (i as u16), *b);
        }
    }

    pub fn set_breakpoints(&mut self, breakpoints: &[u16]) {
        for bp in breakpoints {
            self.breakpoints.push(*bp);
        }
    }

    pub fn run(&mut self) {
        // TODO: load boot rom
        while !self.cpu.terminate {
            let pc_before = self.cpu.pc;
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);

            if self.debug_mode && (self.breakpoint_mode || self.breakpoints.contains(&self.cpu.pc))
            {
                self.breakpoint_mode = true;
                println!("STATE BEFORE THIS INSTRUCTION:");
                self.debug_print(pc_before, inst, inst_length, true);
                self.debug_wait();
            }

            self.cpu.execute(&mut self.mmu, inst, inst_length);

            if self.debug_mode && self.cpu.debug_print {
                self.debug_print(pc_before, inst, inst_length, true);
            }
        }
    }

    pub fn translate(&mut self, limit: u16) {
        while self.cpu.pc <= limit {
            let (inst, inst_length) = decode(&self.mmu, self.cpu.pc);
            println!(
                "{:0>4X} = {:0>3}, {inst_length}b: {inst:?}",
                self.cpu.pc, self.cpu.pc
            );
            self.cpu.pc += inst_length;
        }
    }

    fn debug_print(&self, pc_before: u16, inst: Instruction, inst_length: u16, print_mmu: bool) {
        if print_mmu {
            println!("{:?}", self.mmu)
        }
        println!(
            "pc 0x{:0>4X}: {:?}, {} bytes\n{}\n",
            pc_before, inst, inst_length, self.cpu
        );
    }

    fn debug_wait(&mut self) {
        let mut input = String::new();
        print!("\nLeave blank to step, x to continue: ");
        stdout().flush().ok();
        stdin().read_line(&mut input).ok();
        if input.trim() == "x" {
            self.breakpoint_mode = false;
        }
        println!()
    }
}

impl Display for GBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.cpu, self.mmu)
    }
}
