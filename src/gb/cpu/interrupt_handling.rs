use log::debug;

use crate::gb::{
    GameBoy,
    cpu::{CPU, debug_interrupts},
};

pub struct Interrupt {
    pub flag: u8,
    pub handler: u16,
    pub name: &'static str,
}

impl Interrupt {
    pub const fn new(flag: u8, handler: u16, name: &'static str) -> Self {
        Self {
            flag,
            handler,
            name,
        }
    }
}

pub const INT_FLAGS_MASK: u8 = 0x1F;

pub const VBLANK: Interrupt = Interrupt::new(0x01, 0x40, "VBLANK");
pub const LCD: Interrupt = Interrupt::new(0x02, 0x48, "LCD/STAT");
pub const TIMER: Interrupt = Interrupt::new(0x04, 0x50, "TIMER");
pub const SERIAL: Interrupt = Interrupt::new(0x08, 0x58, "SERIAL");
pub const JOYPAD: Interrupt = Interrupt::new(0x10, 0x60, "JOYPAD");
pub const ALL_INTERRUPS: [Interrupt; 5] = [VBLANK, LCD, TIMER, SERIAL, JOYPAD];

impl CPU {
    pub fn handle_interrupts(ctx: &mut GameBoy) {
        if ctx.cpu.ime {
            let pending = ctx.cpu.pending_interrupts();
            for int in ALL_INTERRUPS {
                if pending & int.flag != 0 {
                    CPU::fire_interrupt(ctx, int);
                    break; // Stop checking interrupts
                }
            }
        }
    }

    pub fn fire_interrupt(ctx: &mut GameBoy, int: Interrupt) {
        ctx.cpu.io_if &= !int.flag; // Unset IF flag
        ctx.cpu.ime = false; // Unset IME
        debug_interrupts!(off);

        debug!("firing {} interrupt handler", int.name);
        ctx.m_tick();
        ctx.m_tick(); // Nothing happens for 2 ticks

        CPU::push_stack(ctx, ctx.cpu.pc); // Push PC to stack, 2 ticks

        ctx.m_tick();
        ctx.cpu.pc = int.handler; // Setting PC takes one last tick
    }

    pub fn pending_interrupts(&self) -> u8 {
        self.io_ie & self.io_if & INT_FLAGS_MASK
    }
}
