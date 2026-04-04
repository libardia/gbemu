use crate::gb::{GameBoy, mmu::MMU};

#[macro_export]
macro_rules! inst_test {
    (
        name $name:ident
        code $code:literal cycles $cycles:literal length $length:literal
        setup $setup:block
        after $after:block
    ) => {
        #[test]
        fn $name {

        }
    };
}

pub fn prepare_instruction(ctx: &mut GameBoy, address: u16, byte: u8) {
    MMU::write(ctx, address, byte);
    ctx.cpu.pc = address;
}
