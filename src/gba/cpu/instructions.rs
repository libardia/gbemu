// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// and
// https://rgbds.gbdev.io/docs/v0.9.1/gbz80.7

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Load (LD_dest_source)
    LD_r8_r8(ArgR8, ArgR8),
    LD_r16_n16(ArgR16, u16),
    LD_mr16_a(ArgR16MEM),
    LDH_mn16_a(u8),
    LDH_mc_a,
    LD_a_mr16(ArgR16MEM),
    LDH_a_mn16(u8),
    LDH_a_mc,

    // 8-bit arithmetic
    ADC_a_r8(ArgR8),
    ADD_a_r8(ArgR8),
    CP_a_r8(ArgR8),
    DEC_r8(ArgR8),
    INC_r8(ArgR8),
    SBC_a_r8(ArgR8),
    SUB_a_r8(ArgR8),

    // 16-bit arithmetic
    ADD_hl_r16(ArgR16),
    DEC_r16(ArgR16),
    INC_r16(ArgR16),

    // Bitwise logic
    AND_a_r8(ArgR8),
    CPL,
    OR_a_r8(ArgR8),
    XOR_a_r8(ArgR8),

    // Bit flags
    BIT_u3_r8(ArgU3, ArgR8),
    RES_u3_r8(ArgU3, ArgR8),
    SET_u3_r8(ArgU3, ArgR8),

    // Bit shift
    RL_r8(ArgR8),
    RLA,
    RLC_r8(ArgR8),
    RLCA,
    RR_r8(ArgR8),
    RRA,
    RRC_r8(ArgR8),
    RRCA,
    SLA_r8(ArgR8),
    SRA_r8(ArgR8),
    SRL_r8(ArgR8),
    SWAP_r8(ArgR8),

    // Jumps and subroutines
    CALL_n16(u16),
    CALL_cc_n16(ArgCOND, u16),
    JP_hl,
    JP_n16(u16),
    JP_cc_n16(ArgCOND, u16),
    JR_e8(i8),
    JR_cc_e8(ArgCOND, i8),
    RET_cc(ArgCOND),
    RET,
    RETI,
    RST_vec(ArgVEC),

    // Carry flag
    CCF,
    SCF,

    // Stack manipulation
    ADD_hl_sp,
    ADD_sp_e8(i8),
    DEC_sp,
    INC_sp,
    LD_sp_n16(u16),
    LD_mn16_sp(u16),
    LD_hl_sp_plus_e8(i8),
    LD_sp_hl,
    POP_r16(ArgR16STK),
    PUSH_r16(ArgR16STK),

    // Interrupt-related
    DI,
    EI,
    HALT,

    // Miscellaneous
    DAA,
    NOP,
    STOP,
}

#[derive(Debug, Clone, Copy)]
pub enum ArgR8 {
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    A,

    CONST(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum ArgR16 {
    BC,
    DE,
    HL,

    CONST(u16),
}

#[derive(Debug, Clone, Copy)]
pub enum ArgR16STK {
    BC,
    DE,
    HL,
    AF,
}

#[derive(Debug, Clone, Copy)]
pub enum ArgR16MEM {
    BC,
    DE,
    HLI,
    HLD,

    CONST(u16),
}

#[derive(Debug, Clone, Copy)]
pub enum ArgU3 {
    Bit0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
}

#[derive(Debug, Clone, Copy)]
pub enum ArgVEC {
    vec0x00,
    vec0x08,
    vec0x10,
    vec0x18,
    vec0x20,
    vec0x28,
    vec0x30,
    vec0x38,
}

#[derive(Debug, Clone, Copy)]
pub enum ArgCOND {
    NZ,
    Z,
    NC,
    C,
}

pub fn instruction_length(inst: Instruction) -> u16 {
    use Instruction::*;
    match inst {
        // Load (LD_dest_source)
        LD_r8_r8(_, ArgR8::CONST(_)) => 2,
        LD_r8_r8(_, _) => 1,
        LD_r16_n16(_, _) => 3,
        LD_mr16_a(ArgR16MEM::CONST(_)) => 3,
        LD_mr16_a(_) => 1,
        LDH_mn16_a(_) => 2,
        LDH_mc_a => 1,
        LD_a_mr16(ArgR16MEM::CONST(_)) => 3,
        LD_a_mr16(_) => 1,
        LDH_a_mn16(_) => 2,
        LDH_a_mc => 1,

        // 8-bit arithmetic
        ADC_a_r8(ArgR8::CONST(_)) => 2,
        ADC_a_r8(_) => 1,
        ADD_a_r8(ArgR8::CONST(_)) => 2,
        ADD_a_r8(_) => 1,
        CP_a_r8(ArgR8::CONST(_)) => 2,
        CP_a_r8(_) => 1,
        DEC_r8(_) => 1,
        INC_r8(_) => 1,
        SBC_a_r8(ArgR8::CONST(_)) => 2,
        SBC_a_r8(_) => 1,
        SUB_a_r8(ArgR8::CONST(_)) => 2,
        SUB_a_r8(_) => 1,

        // 16-bit arithmetic
        ADD_hl_r16(_) => 1,
        DEC_r16(_) => 1,
        INC_r16(_) => 1,

        // Bitwise logic
        AND_a_r8(ArgR8::CONST(_)) => 2,
        AND_a_r8(_) => 1,
        CPL => 1,
        OR_a_r8(ArgR8::CONST(_)) => 2,
        OR_a_r8(_) => 1,
        XOR_a_r8(ArgR8::CONST(_)) => 2,
        XOR_a_r8(_) => 1,

        // Bit flags
        BIT_u3_r8(_, _) => 2,
        RES_u3_r8(_, _) => 2,
        SET_u3_r8(_, _) => 2,

        // Bit shift
        RL_r8(_) => 2,
        RLA => 1,
        RLC_r8(_) => 2,
        RLCA => 1,
        RR_r8(_) => 2,
        RRA => 1,
        RRC_r8(_) => 2,
        RRCA => 1,
        SLA_r8(_) => 2,
        SRA_r8(_) => 2,
        SRL_r8(_) => 2,
        SWAP_r8(_) => 2,

        // Jumps and subroutines
        CALL_n16(_) => 3,
        CALL_cc_n16(_, _) => 3,
        JP_hl => 1,
        JP_n16(_) => 3,
        JP_cc_n16(_, _) => 3,
        JR_e8(_) => 2,
        JR_cc_e8(_, _) => 2,
        RET_cc(_) => 1,
        RET => 1,
        RETI => 1,
        RST_vec(_) => 1,

        // Carry flag
        CCF => 1,
        SCF => 1,

        // Stack manipulation
        ADD_hl_sp => 1,
        ADD_sp_e8(_) => 2,
        DEC_sp => 1,
        INC_sp => 1,
        LD_sp_n16(_) => 3,
        LD_mn16_sp(_) => 3,
        LD_hl_sp_plus_e8(_) => 2,
        LD_sp_hl => 1,
        POP_r16(_) => 1,
        PUSH_r16(_) => 1,

        // Interrupt-related
        DI => 1,
        EI => 1,
        HALT => 1,

        // Miscellaneous
        DAA => 1,
        NOP => 1,
        STOP => 2,
    }
}
