use crate::{
    error_panic,
    gb::{
        GameBoy,
        hardware::processor::instructions::{
            Instruction::{self, *},
            MetaInstruction::*,
        },
    },
};

pub fn execute(ctx: &mut GameBoy, inst: Instruction) -> u16 {
    match inst {
        // Load
        LD_r8_r8(dest, src) => todo!(),
        LD_r8_mem(dest, src) => todo!(),
        LD_mem_r8(dest, src) => todo!(),
        LD_r16_r16(dest, src) => todo!(),

        // Load high
        LDH_A_mem(src) => todo!(),
        LDH_mem_A(dest) => todo!(),

        // 8-bit arithmetic
        ADD_r8(op) => todo!(),
        ADC_r8(op) => todo!(),
        SUB_r8(op) => todo!(),
        SBC_r8(op) => todo!(),
        INC_r8(target) => todo!(),
        DEC_r8(target) => todo!(),
        CP_r8(op) => todo!(),

        // 16-bit arithmetic
        ADD_r16(op) => todo!(),
        INC_r16(target) => todo!(),
        DEC_r16(target) => todo!(),

        // Logic
        AND(op) => todo!(),
        OR(op) => todo!(),
        XOR(op) => todo!(),
        CPL => todo!(),

        // Bit flags
        BIT(bit, target) => todo!(),
        SET(bit, target) => todo!(),
        RES(bit, target) => todo!(),

        // Bit shifts
        RL(target) => todo!(),
        RLA => todo!(),
        RLC(target) => todo!(),
        RLCA => todo!(),
        RR(target) => todo!(),
        RRA => todo!(),
        RRC(target) => todo!(),
        RRCA => todo!(),
        SLA(target) => todo!(),
        SRA(target) => todo!(),
        SRL(target) => todo!(),
        SWAP(target) => todo!(),

        // Jumps and subroutines
        CALL(cond, address) => todo!(),
        JP(cond, address) => todo!(),
        JR(cond, off) => todo!(),
        RET(cond) => todo!(),
        RETI => todo!(),
        RST(address) => todo!(),

        // Carry flag
        CCF => todo!(),
        SCF => todo!(),

        // Stack manipulation
        ADD_SP_e8(off) => todo!(),
        LD_a16_SP(address) => todo!(),
        LD_HL_SPe8(off) => todo!(),
        POP(target) => todo!(),
        PUSH(target) => todo!(),

        // Interrupts
        DI => todo!(),
        EI => todo!(),
        HALT => todo!(),

        // Misc
        DAA => todo!(),
        NOP => 1, // Do nothing for 1 MTime
        STOP(_) => todo!(),
        PREFIX => error_panic!("Tried to execute PREFIX, which is only used as a marker."),

        // Meta
        INVALID(meta) => match meta {
            SHOW_CPU if ctx.cpu.meta_inst => todo!(),
            TERMINATE if ctx.cpu.meta_inst => todo!(),
            DUMP if ctx.cpu.meta_inst => todo!(),

            _ => error_panic!("Tried to execute an invalid instruction."),
        },
    }
}
