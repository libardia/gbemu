use crate::gb::{GameBoy, cpu::CPU};

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

pub const VBLANK: Interrupt = Interrupt::new(0x01, 0x40, "vblank");
pub const LCD: Interrupt = Interrupt::new(0x02, 0x48, "LCD");
pub const TIMER: Interrupt = Interrupt::new(0x04, 0x50, "timer");
pub const SERIAL: Interrupt = Interrupt::new(0x08, 0x58, "serial");
pub const JOYPAD: Interrupt = Interrupt::new(0x10, 0x60, "joypad");
pub const ALL_INTERRUPS: [Interrupt; 5] = [VBLANK, LCD, TIMER, SERIAL, JOYPAD];

impl CPU {
    pub fn handle_interrupts(ctx: &mut GameBoy) {
        if ctx.cpu.ime {
            let pending = ctx.cpu.pending_interrupts();
            for int in ALL_INTERRUPS {
                if pending & int.flag != 0 {
                    // TODO: fire interrupts
                }
            }
        }
    }

    pub fn pending_interrupts(&self) -> u8 {
        self.io_ie & self.io_if & INT_FLAGS_MASK
    }
}
