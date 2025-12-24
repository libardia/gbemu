use crate::{address_fmt, byte_fmt, number_type};
use std::fmt::Debug;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // Load
    LD_r8_r8(R8, R8),
    LD_r8_mem(R8, Mem),
    LD_mem_r8(Mem, R8),
    LD_r16_r16(R16, R16),

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
    AND(R8),
    OR(R8),
    XOR(R8),
    CPL,

    // Bit flags
    BIT(u8, R8),
    SET(u8, R8),
    RES(u8, R8),

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
    CALL(Cond, Word),
    JP(Cond, Mem),
    JR(Cond, Offset),
    RET(Cond),
    RETI,
    RST(Word),

    // Carry flag
    CCF,
    SCF,

    // Stack manipulation
    ADD_SP_e8(Offset),
    LD_a16_SP(Word),
    LD_HL_SPe8(Offset),
    POP(R16),
    PUSH(R16),

    // Interrupts
    DI,
    EI,
    HALT,

    // Misc
    DAA,
    NOP,
    STOP(Byte),

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

    IMM(Byte),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
    AF,

    IMM(Word),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mem {
    BC,
    DE,
    HL,
    HLI,
    HLD,

    IMM(Word),

    HIGH_C,
    HIGH_IMM(Byte),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
    ALWAYS,
}

number_type!(pub Byte: u8);
impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&byte_fmt!(&self.0))
    }
}

number_type!(pub Word: u16);
impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&address_fmt!(&self.0))
    }
}

number_type!(pub Offset: i8);
impl Debug for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self.0)
    }
}
