use crate::{
    cpu_log,
    gb::{
        GameBoy,
        hardware::{memory::Memory, processor::Processor},
        registers::IO_IF,
    },
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
        if ctx.cpu.ime {
            let pending = Processor::pending_interrupts(ctx);
            for (int_mask, handler_address, name) in INT_ORDER {
                if pending & int_mask != 0 {
                    cpu_log!(debug, ctx, "Interrupt fired: {}", name);

                    // Reset the corresponding IF bit
                    Memory::write_masked(ctx, IO_IF, 0, int_mask);

                    // Disable interrupts
                    ctx.cpu.ime = false;

                    // Push PC on the stack and jump to the handler
                    Processor::push_stack(ctx, ctx.cpu.pc);
                    ctx.cpu.pc = handler_address;

                    // Break the loop and return early: an interrupt was fired
                    return true;
                }
            }
        }

        false
    }
}
