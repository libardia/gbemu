use crate::{
    cpu_log,
    gb::{GameBoy, hardware::processor::Processor},
};

const VBLANK: u8 = 0x1;
const STAT: u8 = 0x2;
const TIMER: u8 = 0x4;
const SERIAL: u8 = 0x8;
const JOYPAD: u8 = 0x10;

const VBLANK_HANDLER_ADDRESS: u16 = 0x40;
const STAT_HANDLER_ADDRESS: u16 = 0x48;
const TIMER_HANDLER_ADDRESS: u16 = 0x50;
const SERIAL_HANDLER_ADDRESS: u16 = 0x58;
const JOYPAD_HANDLER_ADDRESS: u16 = 0x60;

const INT_ORDER: [(u8, u16, &str); 5] = [
    (VBLANK, VBLANK_HANDLER_ADDRESS, "VBlank"),
    (STAT, STAT_HANDLER_ADDRESS, "STAT"),
    (TIMER, TIMER_HANDLER_ADDRESS, "Timer"),
    (SERIAL, SERIAL_HANDLER_ADDRESS, "Serial"),
    (JOYPAD, JOYPAD_HANDLER_ADDRESS, "Joypad"),
];

impl Processor {
    pub fn maybe_interrupt(ctx: &mut GameBoy) -> bool {
        let pending = Processor::pending_interrupts(ctx);
        for (int_mask, handler_address, name) in INT_ORDER {
            if pending & int_mask != 0 {
                cpu_log!(debug, ctx, "Interrupt fired: {}", name);

                // Push PC on the stack and jump to the handler
                Processor::push_stack(ctx, ctx.cpu.pc);
                ctx.cpu.pc = handler_address;

                // Return early: an interrupt was fired
                return true;
            }
        }

        false
    }
}
