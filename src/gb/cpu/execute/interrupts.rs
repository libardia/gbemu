use crate::gb::{GameBoy, cpu::debug_interrupts};

pub fn di(ctx: &mut GameBoy) {
    // unsetting IME happens immediately
    debug_interrupts!(off);
    ctx.cpu.ime_timer = 0;
    ctx.cpu.ime = false;
}

pub fn ei(ctx: &mut GameBoy) {
    // setting IME needs special handling
    debug_interrupts!(..on);
    ctx.cpu.ime_timer = 2;
}

pub fn halt(_ctx: &mut GameBoy) {
    // TODO: HALT
    todo!("halt instruction");
}

#[cfg(test)]
mod tests {
    use crate::{
        gb::{cpu::CPU, mmu::MMU},
        testutil::{INSTRUCTION_ADDRESS, dummy_ctx, prepare_instruction, step_test},
    };
    use test_log::test;

    #[test]
    fn di() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;

            code: 0xF3, length: 1, cycles: 1
            setup {
                ctx.cpu.ime = true;
            }
            after {
                assert!(!ctx.cpu.ime);
            }
        }
    }

    #[test]
    fn ei() {
        let ctx = &mut dummy_ctx();
        prepare_instruction(ctx, INSTRUCTION_ADDRESS, 0xFB); // first instruction is EI
        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, 0x00); // next instruction is NOP

        assert!(!ctx.cpu.ime);

        CPU::step(ctx);

        assert!(!ctx.cpu.ime);
        assert_eq!(ctx.system_ticks, 4); // 1 mtime
        assert_eq!(ctx.cpu.pc, INSTRUCTION_ADDRESS + 1); // 1 byte long

        CPU::step(ctx);

        assert!(ctx.cpu.ime);
    }
}
