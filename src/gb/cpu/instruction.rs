use std::fmt::Debug;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // Load
    LD_r8_r8(R8, R8),
    LD_r16_r16(R16, R16),
    LD_mem_r8(Mem, R8),
    LD_r8_mem(R8, Mem),

    // Load high (in memory from FF00 to FFFF)
    LDH_A_mem(Mem),
    LDH_mem_A(Mem),

    // 8-bit arithmetic
    ADD_r8(R8),
    ADC_r8(R8),
    SUB_r8(R8),
    SBC_r8(R8),
    INC_r8(R8),
    DEC_r8(R8),
    CP_r8(R8),

    // 16-bit arithmetic
    ADD_r16(R16),
    INC_r16(R16),
    DEC_r16(R16),

    // Logic
    AND_r8(R8),
    OR_r8(R8),
    XOR_r8(R8),
    CPL,

    // Bit flags
    BIT_r8(u8, R8),
    SET_r8(u8, R8),
    RES_r8(u8, R8),

    // Bit shifts
    RL(R8),
    RLA,
    RLC(R8),
    RLCA,
    RR(R8),
    RRA,
    RRC(R8),
    RRCA,
    SLA(R8),
    SRA(R8),
    SRL(R8),
    SWAP(R8),

    // Jumps and subroutines
    CALL(Cond, Mem),
    JP(Cond, Mem),
    JR(Cond, i8),
    RET(Cond),
    RETI,
    RST(Mem),

    // Carry flag
    CCF,
    SCF,

    // Stack manipulation
    ADD_SP_e8(i8),
    LD_a16_SP(Mem),
    LD_HL_SPe8(i8),
    POP(R16),
    PUSH(R16),

    // Interrupts
    DI,
    EI,
    HALT,

    // Misc
    DAA,
    NOP,
    STOP(u8),

    // Meta
    PREFIX,
    INVALID(MetaInstruction),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaInstruction {
    NONE,

    SHOW_CPU,
    TERMINATE,
    DUMP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    MHL,
    A,

    IMM(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
    AF,

    IMM(u16),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mem {
    BC,
    DE,
    HL,
    HLI,
    HLD,

    IMM(u16),

    HIGH_C,
    HIGH_IMM(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
    ALWAYS,
}
