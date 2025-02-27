// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// and
// https://rgbds.gbdev.io/docs/v0.9.1/gbz80.7

#[allow(non_camel_case_types)]
pub enum Instruction {
    // Load (LD_dest_source)
    LD_r8_r8(ArgR8, ArgR8),
    LD_r8_n8(ArgR8, u8),
    LD_r16_n16(ArgR16, u16),
    LD_mr16_a(ArgR16MEM),
    LD_mn16_a(u16),
    LDH_mn8_a(u8),
    LDH_mc_a,
    LD_a_mr16(ArgR16MEM),
    LD_a_mn16(u16),
    LDH_a_mn16(u8),
    LDH_a_mc,

    // 8-bit arithmetic
    ADC_a_r8(ArgR8),
    ADC_a_n8(u8),
    ADD_a_r8(ArgR8),
    ADD_a_n8(u8),
    CP_a_r8(ArgR8),
    CP_a_n8(u8),
    DEC_r8(ArgR8),
    INC_r8(ArgR8),
    SBC_a_r8(ArgR8),
    SBC_a_n8(u8),
    SUB_a_r8(ArgR8),
    SUB_a_n8(u8),

    // 16-bit arithmetic
    ADD_hl_r16(ArgR16),
    DEC_r16(ArgR16),
    INC_r16(ArgR16),

    // Bitwise logic
    AND_a_r8(ArgR8),
    AND_a_n8(u8),
    CPL,
    OR_a_r8(ArgR8),
    OR_a_n8(u8),
    XOR_a_r8(ArgR8),
    XOR_a_n8(u8),

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
    JR_n16(u16),
    JR_cc_n16(ArgCOND, u16),
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
    POP_af,
    POP_r16(ArgR16STK),
    PUSH_af,
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

pub enum ArgR8 {
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    A,
}

pub enum ArgR16 {
    BC,
    DE,
    HL,
}

pub enum ArgR16STK {
    BC,
    DE,
    HL,
    AF,
}

pub enum ArgR16MEM {
    BC,
    DE,
    HLI,
    HLD,
}

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

pub enum ArgCOND {
    NZ,
    Z,
    NC,
    C,
}

// pub fn inst_len(inst: Instruction) -> u8 {
//     use Instruction::*;
//     match inst {
//         // Load (LD_dest_source)
//         LD_r8_r8(_, _) => 1,
//         LD_r8_n8(_, _) => 1,
//         LD_r16_n16(_, _) => 1,
//         LD_mr16_a(_) => 1,
//         LD_mn16_a(_) => 1,
//         LDH_mn8_a(_) => 1,
//         LDH_mc_a => 1,
//         LD_a_mr16(_) => 1,
//         LD_a_mn16(_) => 1,
//         LDH_a_mn16(_) => 1,
//         LDH_a_mc => 1,
//         // 8-bit arithmetic
//         ADC_a_r8(_) => 1,
//         ADC_a_n8(_) => 1,
//         ADD_a_r8(_) => 1,
//         ADD_a_n8(_) => 1,
//         CP_a_r8(_) => 1,
//         CP_a_n8(_) => 1,
//         DEC_r8(_) => 1,
//         INC_r8(_) => 1,
//         SBC_a_r8(_) => 1,
//         SBC_a_n8(_) => 1,
//         SUB_a_r8(_) => 1,
//         SUB_a_n8(_) => 1,
//         // 16-bit arithmetic
//         ADD_hl_r16(_) => 1,
//         DEC_r16(_) => 1,
//         INC_r16(_) => 1,
//         // Bitwise logic
//         AND_a_r8(_) => 1,
//         AND_a_n8(_) => 1,
//         CPL => 1,
//         OR_a_r8(_) => 1,
//         OR_a_n8(_) => 1,
//         XOR_a_r8(_) => 1,
//         XOR_a_n8(_) => 1,
//         // Bit flags
//         BIT_u3_r8(_, _) => 1,
//         RES_u3_r8(_, _) => 1,
//         SET_u3_r8(_, _) => 1,
//         // Bit shift
//         RL_r8(_) => 1,
//         RLA => 1,
//         RLC_r8(_) => 1,
//         RLCA => 1,
//         RR_r8(_) => 1,
//         RRA => 1,
//         RRC_r8(_) => 1,
//         RRCA => 1,
//         SLA_r8(_) => 1,
//         SRA_r8(_) => 1,
//         SRL_r8(_) => 1,
//         SWAP_r8(_) => 1,
//         // Jumps and subroutines
//         CALL_n16(_) => 1,
//         CALL_cc_n16(_, _) => 1,
//         JP_hl => 1,
//         JP_n16(_) => 1,
//         JP_cc_n16(_, _) => 1,
//         JR_n16(_) => 1,
//         JR_cc_n16(_, _) => 1,
//         RET_cc(_) => 1,
//         RET => 1,
//         RETI => 1,
//         RST_vec(_) => 1,
//         // Carry flag
//         CCF => 1,
//         SCF => 1,
//         // Stack manipulation
//         ADD_hl_sp => 1,
//         ADD_sp_e8(_) => 1,
//         DEC_sp => 1,
//         INC_sp => 1,
//         LD_sp_n16(_) => 1,
//         LD_mn16_sp(_) => 1,
//         LD_hl_sp_plus_e8(_) => 1,
//         LD_sp_hl => 1,
//         POP_af => 1,
//         POP_r16(_) => 1,
//         PUSH_af => 1,
//         PUSH_r16(_) => 1,
//         // Interrupt-related
//         DI => 1,
//         EI => 1,
//         HALT => 1,
//         // Miscellaneous
//         DAA => 1,
//         NOP => 1,
//         STOP => 1
//     }
// }
