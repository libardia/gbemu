use crate::gb::{
    GameBoy,
    hardware::{
        memory::Memory,
        processor::{
            Processor,
            instructions::{
                Byte,
                Instruction::{self, *},
                Mem, Offset, R8, R16, Word,
            },
            optable::{OP_TABLE, PREFIX_TABLE},
        },
    },
};

impl Processor {
    pub fn decode(ctx: &mut GameBoy) -> Instruction {
        let first_byte = Self::next_byte(ctx);

        let mut inst = OP_TABLE[first_byte.0 as usize];
        if inst == PREFIX {
            inst = PREFIX_TABLE[Self::next_byte(ctx).0 as usize];
        }

        // Fill any constants in the instruction
        match inst {
            // 0x
            LD_r16_r16(first, R16::IMM(_)) => LD_r16_r16(first, R16::IMM(Self::next_word(ctx))),
            LD_r8_r8(first, R8::IMM(_)) => LD_r8_r8(first, R8::IMM(Self::next_byte(ctx))),
            LD_a16_SP(_) => LD_a16_SP(Self::next_word(ctx)),

            // 1x
            STOP(_) => STOP(Self::next_byte(ctx)),
            JR(first, _) => JR(first, Self::next_signed(ctx)),

            // Cx
            JP(first, Mem::IMM(_)) => JP(first, Mem::IMM(Self::next_word(ctx))),
            CALL(first, _) => CALL(first, Self::next_word(ctx)),
            ADD_r8(R8::IMM(_)) => ADD_r8(R8::IMM(Self::next_byte(ctx))),
            ADC_r8(R8::IMM(_)) => ADC_r8(R8::IMM(Self::next_byte(ctx))),

            // Dx
            SUB_r8(R8::IMM(_)) => SUB_r8(R8::IMM(Self::next_byte(ctx))),
            SBC_r8(R8::IMM(_)) => SBC_r8(R8::IMM(Self::next_byte(ctx))),

            // Ex
            LDH_mem_A(Mem::HIGH_IMM(_)) => LDH_mem_A(Mem::HIGH_IMM(Self::next_byte(ctx))),
            AND(R8::IMM(_)) => AND(R8::IMM(Self::next_byte(ctx))),
            ADD_SP_e8(_) => ADD_SP_e8(Self::next_signed(ctx)),
            LD_mem_r8(Mem::IMM(_), second) => LD_mem_r8(Mem::IMM(Self::next_word(ctx)), second),
            XOR(R8::IMM(_)) => XOR(R8::IMM(Self::next_byte(ctx))),

            // Fx
            LDH_A_mem(Mem::HIGH_IMM(_)) => LDH_A_mem(Mem::HIGH_IMM(Self::next_byte(ctx))),
            OR(R8::IMM(_)) => OR(R8::IMM(Self::next_byte(ctx))),
            LD_HL_SPe8(_) => LD_HL_SPe8(Self::next_signed(ctx)),
            LD_r8_mem(first, Mem::IMM(_)) => LD_r8_mem(first, Mem::IMM(Self::next_word(ctx))),
            CP_r8(R8::IMM(_)) => CP_r8(R8::IMM(Self::next_byte(ctx))),

            // Any other instruction
            _ => inst,
        }
    }

    fn next_u8(ctx: &mut GameBoy) -> u8 {
        let byte = Memory::read(ctx, ctx.cpu.pc);
        if ctx.cpu.halt_bug {
            // Don't increment PC, whoops!
            ctx.cpu.halt_bug = false
        } else {
            ctx.cpu.pc = ctx.cpu.pc.wrapping_add(1);
        }
        byte
    }

    fn next_byte(ctx: &mut GameBoy) -> Byte {
        Byte(Self::next_u8(ctx))
    }

    fn next_signed(ctx: &mut GameBoy) -> Offset {
        Offset(Self::next_u8(ctx) as i8)
    }

    fn next_word(ctx: &mut GameBoy) -> Word {
        let low = Self::next_u8(ctx) as u16;
        let high = Self::next_u8(ctx) as u16;
        Word((high << 8) | low)
    }
}
