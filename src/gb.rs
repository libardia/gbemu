use std::{cell::RefCell, rc::Rc};

use cpu::CPU;
use gpu::GPU;
use log::debug;
use mmu::{nintendo_logo::NINTENDO_LOGO, MMU};

use crate::{mem_region::regions::ROM_SPACE, util::new};

mod cpu;
mod gpu;
mod mbc;
mod mmu;
mod time_types;

#[derive(Debug)]
pub struct GB {
    cpu: CPU,
    gpu: GPU,
    mmu: Rc<RefCell<MMU>>,
}
impl GB {
    new!({
        let mmu = Rc::new(RefCell::new(MMU::new()));

        Self {
            cpu: CPU::new(mmu.clone()),
            gpu: GPU::new(mmu.clone()),
            mmu,
        }
    });

    pub fn set_debug_mode(&mut self, mode: bool) {
        self.cpu.debug_mode = mode;
    }

    pub fn set_breakpoints(&mut self, breakpoints: &[u16]) {
        self.cpu.breakpoints = breakpoints.to_vec();
    }

    pub fn load(&mut self, path: &str) {
        self.mmu.borrow_mut().load_from_file(path);
    }

    pub fn load_bytes(&mut self, bytes: &[u8]) {
        self.mmu.borrow_mut().load_from_bytes(bytes);
    }

    pub fn load_prog(&mut self, prog: &[u8]) {
        self.load_bytes(&Self::make_dummy_cart(prog));
    }

    pub fn boot(&mut self) {
        while !self.cpu.terminate {
            let dt = self.cpu.step();
            self.gpu.step(dt);
        }
    }

    fn make_dummy_cart(prog: &[u8]) -> Vec<u8> {
        debug!("Constructing dummy cart for provided program...");

        let mut cart = vec![0; ROM_SPACE.usize()];

        // Entry point; this will jump to address 0x150 where the program will be written
        cart[0x100] = 0x00; // NOP
        cart[0x101] = 0xC3; // JP $150
        cart[0x102] = 0x50;
        cart[0x103] = 0x01;

        // Copy the nintendo logo into the header (necessary for boot ROM to function)
        const LOGO_START: usize = 0x104;
        for (i, b) in NINTENDO_LOGO.iter().enumerate() {
            cart[LOGO_START + i] = *b;
        }

        // Some dummy data. This is just here so the checksum has something to check

        // Title
        let title = "DUMMY";
        const TITLE_START: usize = 0x134;
        const TITLE_LENGTH: usize = 16;
        for (i, b) in title.to_ascii_uppercase().as_bytes().iter().enumerate() {
            if i < TITLE_LENGTH {
                cart[TITLE_START + i] = *b;
            }
        }

        // Old licensee code = Nintendo
        cart[0x14B] = 0x01;

        // Calculate checksum
        let mut checksum = 0u8;
        for a in 0x134..=0x14C {
            checksum = checksum.wrapping_sub(cart[a]).wrapping_sub(1);
        }
        cart[0x14D] = checksum;

        // Copy prog
        const PROG_START: usize = 0x150;
        for (i, b) in prog.iter().enumerate() {
            let a = PROG_START + i;
            if a < cart.len() {
                cart[a] = *b;
            }
        }

        cart
    }
}
