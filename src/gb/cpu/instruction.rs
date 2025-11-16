#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arg {
    // 8-bit registers
    R8_B,
    R8_C,
    R8_D,
    R8_E,
    R8_H,
    R8_L,
    R8_A,

    // 16-bit registers
    R16_BC,
    R16_DE,
    R16_HL,
    R16_AF,
    R16_SP,

    // Memory pointed at by address
    M_BC,
    M_DE,
    M_HL,
    M_HLI,
    M_HLD,

    // Conditions
    C_A, // (always)
    C_NZ,
    C_Z,
    C_NC,
    C_C,

    // Constants
    CONST_8(u8),
    CONST_16(u16),
    M_CONST_16(u16),
    CONST_i8(i8),
    CONST_i16(i16),

    // Constant placeholders
    IMM_8,
    IMM_16,
    M_IMM_16,
    IMM_i8,
    IMM_i16,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // Load
    LD(Arg, Arg),
    LDH(Arg, Arg),

    // Arithmetic
    ADD(Arg),
    ADD_16(Arg),
    ADC(Arg),
    SUB(Arg),
    SBC(Arg),
    INC(Arg),
    DEC(Arg),
    CP(Arg),

    // Logic
    AND(Arg),
    OR(Arg),
    XOR(Arg),
    CPL,

    // Bit flags
    BIT(Arg, Arg),
    RES(Arg, Arg),
    SET(Arg, Arg),

    // Bit shifts
    RL(Arg),
    RLC(Arg),
    RLA,
    RLCA,
    RR(Arg),
    RRC(Arg),
    RRA,
    RRCA,
    SLA(Arg),
    SRA(Arg),
    SRL(Arg),
    SWAP(Arg, Arg),

    // Jumps
    JP(Arg, Arg),
    JR(Arg, Arg),
    CALL(Arg, Arg),
    RST(Arg),
    RET(Arg),
    RETI,

    // Carry flag
    CCF,
    SCF,

    // Stack
    POP(Arg),
    PUSH(Arg),

    // System
    DI,
    EI,
    HALT,

    // Misc
    #[default]
    NOP,
    DAA,
    STOP(Arg),
    PREFIX,
    INVALID,
}
