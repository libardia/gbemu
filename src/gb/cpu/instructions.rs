// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// and
// https://rgbds.gbdev.io/docs/v0.9.1/gbz80.7

use crate::util::{Hex16, Hex8};

/* #region Instructions ======================================================================== */

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // Load (LD_dest_source)
    LD_r8_r8(ArgR8, ArgR8),
    LD_r16_n16(ArgR16, Hex16),
    LD_mr16_a(ArgR16MEM),
    LDH_mn16_a(Hex8),
    LDH_mc_a,
    LD_a_mr16(ArgR16MEM),
    LDH_a_mn16(Hex8),
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
    CALL_n16(Hex16),
    CALL_cc_n16(ArgCOND, Hex16),
    JP_hl,
    JP_n16(Hex16),
    JP_cc_n16(ArgCOND, Hex16),
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
    LD_sp_n16(Hex16),
    LD_mn16_sp(Hex16),
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
    #[default]
    NOP,
    STOP(Hex8),

    // Meta
    PREFIX,
    INVALID,
    TERMINATE,
    DEBUG_PRINT,
}

/* #endregion */

/* #region Argument types ====================================================================== */

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgR8 {
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    A,

    CONST(Hex8),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgR16 {
    BC,
    DE,
    HL,
    SP,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgR16STK {
    BC,
    DE,
    HL,
    AF,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgR16MEM {
    BC,
    DE,
    HLI,
    HLD,

    CONST(Hex16),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgU3 {
    Bit0 = 0b0000_0001,
    Bit1 = 0b0000_0010,
    Bit2 = 0b0000_0100,
    Bit3 = 0b0000_1000,
    Bit4 = 0b0001_0000,
    Bit5 = 0b0010_0000,
    Bit6 = 0b0100_0000,
    Bit7 = 0b1000_0000,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgVEC {
    Vec0x00 = 0x00,
    Vec0x08 = 0x08,
    Vec0x10 = 0x10,
    Vec0x18 = 0x18,
    Vec0x20 = 0x20,
    Vec0x28 = 0x28,
    Vec0x30 = 0x30,
    Vec0x38 = 0x38,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgCOND {
    NZ,
    Z,
    NC,
    C,
    ALWAYS,
}

/* #endregion */
