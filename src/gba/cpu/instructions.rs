// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// and
// https://rgbds.gbdev.io/docs/v0.9.1/gbz80.7

pub enum Instruction {
    NOP,
    ADD(ArgR8),
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
