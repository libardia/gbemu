// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// and
// https://rgbds.gbdev.io/docs/v0.9.1/gbz80.7

#[allow(non_camel_case_types)]
pub enum Instruction {
    NOP,
    LD_r16_n16(ArgR16, u16),
    LD_m16_a(ArgR16MEM),
    INC_r16(ArgR16),
    INC_r8(ArgR8),
    DEC_r8(ArgR8),
    LD_r8_n8(ArgR8, u8),

    ADD_a_r8(ArgR8),
    ADC_a_r8(ArgR8),
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
    SP
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
