use super::{
    cpu::instructions::{Instruction, Instruction::*, *},
    mmu::MMU,
};

const OP_TABLE: [[Instruction; 16]; 16] = [
    [
        // 0x
        NOP,                                 // x0
        LD_r16_n16(ArgR16::BC, 0),           // x1
        LD_mr16_a(ArgR16MEM::BC),            // x2
        INC_r16(ArgR16::BC),                 // x3
        INC_r8(ArgR8::B),                    // x4
        DEC_r8(ArgR8::B),                    // x5
        LD_r8_r8(ArgR8::B, ArgR8::CONST(0)), // x6
        RLCA,                                // x7
        LD_mn16_sp(0),                       // x8
        ADD_hl_r16(ArgR16::BC),              // x9
        LD_a_mr16(ArgR16MEM::BC),            // xA
        DEC_r16(ArgR16::BC),                 // xB
        INC_r8(ArgR8::C),                    // xC
        DEC_r8(ArgR8::C),                    // xD
        LD_r8_r8(ArgR8::C, ArgR8::CONST(0)), // xE
        RRCA,                                // xF
    ],
    [
        // 1x
        STOP,                                // x0
        LD_r16_n16(ArgR16::DE, 0),           // x1
        LD_mr16_a(ArgR16MEM::DE),            // x2
        INC_r16(ArgR16::DE),                 // x3
        INC_r8(ArgR8::D),                    // x4
        DEC_r8(ArgR8::D),                    // x5
        LD_r8_r8(ArgR8::D, ArgR8::CONST(0)), // x6
        RLA,                                 // x7
        JR_e8(0),                            // x8
        ADD_hl_r16(ArgR16::DE),              // x9
        LD_a_mr16(ArgR16MEM::DE),            // xA
        DEC_r16(ArgR16::DE),                 // xB
        INC_r8(ArgR8::E),                    // xC
        DEC_r8(ArgR8::E),                    // xD
        LD_r8_r8(ArgR8::E, ArgR8::CONST(0)), // xE
        RRA,                                 // xF
    ],
    [
        // 2x
        JR_cc_e8(ArgCOND::NZ, 0),            // x0
        LD_r16_n16(ArgR16::HL, 0),           // x1
        LD_mr16_a(ArgR16MEM::HLI),           // x2
        INC_r16(ArgR16::HL),                 // x3
        INC_r8(ArgR8::H),                    // x4
        DEC_r8(ArgR8::H),                    // x5
        LD_r8_r8(ArgR8::H, ArgR8::CONST(0)), // x6
        DAA,                                 // x7
        JR_cc_e8(ArgCOND::Z, 0),             // x8
        ADD_hl_r16(ArgR16::HL),              // x9
        LD_a_mr16(ArgR16MEM::HLI),           // xA
        DEC_r16(ArgR16::HL),                 // xB
        INC_r8(ArgR8::L),                    // xC
        DEC_r8(ArgR8::L),                    // xD
        LD_r8_r8(ArgR8::L, ArgR8::CONST(0)), // xE
        CPL,                                 // xF
    ],
    [
        // 3x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 4x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 5x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 6x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 7x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 8x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 9x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Ax
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Bx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Cx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Dx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Ex
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Fx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
];

const PREFIX_TABLE: [[Instruction; 16]; 16] = [
    [
        // 0x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 1x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 2x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 3x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 4x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 5x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 6x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 7x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 8x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // 9x
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Ax
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Bx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Cx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Dx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Ex
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
    [
        // Fx
        NOP, // x0
        NOP, // x1
        NOP, // x2
        NOP, // x3
        NOP, // x4
        NOP, // x5
        NOP, // x6
        NOP, // x7
        NOP, // x8
        NOP, // x9
        NOP, // xA
        NOP, // xB
        NOP, // xC
        NOP, // xD
        NOP, // xE
        NOP, // xF
    ],
];

pub fn decode(mmu: &MMU, pc: u16) -> (Instruction, u16) {
    let mut inst_length: u16 = 1;
    let code = mmu.read_byte(pc);
    let upper = ((code & 0xF0) >> 4) as usize;
    let lower = (code & 0xF) as usize;
    let mut inst = OP_TABLE[upper][lower];

    if inst == PREFIX {}

    (inst, inst_length)
}
