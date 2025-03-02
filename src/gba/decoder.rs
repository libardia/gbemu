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
        JR_cc_e8(ArgCOND::NC, 0),              // x0
        LD_sp_n16(0),                          // x1
        LD_mr16_a(ArgR16MEM::HLD),             // x2
        INC_sp,                                // x3
        INC_r8(ArgR8::MHL),                    // x4
        DEC_r8(ArgR8::MHL),                    // x5
        LD_r8_r8(ArgR8::MHL, ArgR8::CONST(0)), // x6
        SCF,                                   // x7
        JR_cc_e8(ArgCOND::C, 0),               // x8
        ADD_hl_sp,                             // x9
        LD_a_mr16(ArgR16MEM::HLD),             // xA
        DEC_sp,                                // xB
        INC_r8(ArgR8::A),                      // xC
        DEC_r8(ArgR8::A),                      // xD
        LD_r8_r8(ArgR8::A, ArgR8::CONST(0)),   // xE
        CCF,                                   // xF
    ],
    [
        // 4x
        LD_r8_r8(ArgR8::B, ArgR8::B),   // x0
        LD_r8_r8(ArgR8::B, ArgR8::C),   // x1
        LD_r8_r8(ArgR8::B, ArgR8::D),   // x2
        LD_r8_r8(ArgR8::B, ArgR8::E),   // x3
        LD_r8_r8(ArgR8::B, ArgR8::H),   // x4
        LD_r8_r8(ArgR8::B, ArgR8::L),   // x5
        LD_r8_r8(ArgR8::B, ArgR8::MHL), // x6
        LD_r8_r8(ArgR8::B, ArgR8::A),   // x7
        LD_r8_r8(ArgR8::C, ArgR8::B),   // x8
        LD_r8_r8(ArgR8::C, ArgR8::C),   // x9
        LD_r8_r8(ArgR8::C, ArgR8::D),   // xA
        LD_r8_r8(ArgR8::C, ArgR8::E),   // xB
        LD_r8_r8(ArgR8::C, ArgR8::H),   // xC
        LD_r8_r8(ArgR8::C, ArgR8::L),   // xD
        LD_r8_r8(ArgR8::C, ArgR8::MHL), // xE
        LD_r8_r8(ArgR8::C, ArgR8::A),   // xF
    ],
    [
        // 5x
        LD_r8_r8(ArgR8::D, ArgR8::B),   // x0
        LD_r8_r8(ArgR8::D, ArgR8::C),   // x1
        LD_r8_r8(ArgR8::D, ArgR8::D),   // x2
        LD_r8_r8(ArgR8::D, ArgR8::E),   // x3
        LD_r8_r8(ArgR8::D, ArgR8::H),   // x4
        LD_r8_r8(ArgR8::D, ArgR8::L),   // x5
        LD_r8_r8(ArgR8::D, ArgR8::MHL), // x6
        LD_r8_r8(ArgR8::D, ArgR8::A),   // x7
        LD_r8_r8(ArgR8::E, ArgR8::B),   // x8
        LD_r8_r8(ArgR8::E, ArgR8::C),   // x9
        LD_r8_r8(ArgR8::E, ArgR8::D),   // xA
        LD_r8_r8(ArgR8::E, ArgR8::E),   // xB
        LD_r8_r8(ArgR8::E, ArgR8::H),   // xC
        LD_r8_r8(ArgR8::E, ArgR8::L),   // xD
        LD_r8_r8(ArgR8::E, ArgR8::MHL), // xE
        LD_r8_r8(ArgR8::E, ArgR8::A),   // xF
    ],
    [
        // 6x
        LD_r8_r8(ArgR8::H, ArgR8::B),   // x0
        LD_r8_r8(ArgR8::H, ArgR8::C),   // x1
        LD_r8_r8(ArgR8::H, ArgR8::D),   // x2
        LD_r8_r8(ArgR8::H, ArgR8::E),   // x3
        LD_r8_r8(ArgR8::H, ArgR8::H),   // x4
        LD_r8_r8(ArgR8::H, ArgR8::L),   // x5
        LD_r8_r8(ArgR8::H, ArgR8::MHL), // x6
        LD_r8_r8(ArgR8::H, ArgR8::A),   // x7
        LD_r8_r8(ArgR8::L, ArgR8::B),   // x8
        LD_r8_r8(ArgR8::L, ArgR8::C),   // x9
        LD_r8_r8(ArgR8::L, ArgR8::D),   // xA
        LD_r8_r8(ArgR8::L, ArgR8::E),   // xB
        LD_r8_r8(ArgR8::L, ArgR8::H),   // xC
        LD_r8_r8(ArgR8::L, ArgR8::L),   // xD
        LD_r8_r8(ArgR8::L, ArgR8::MHL), // xE
        LD_r8_r8(ArgR8::L, ArgR8::A),   // xF
    ],
    [
        // 7x
        LD_r8_r8(ArgR8::MHL, ArgR8::B), // x0
        LD_r8_r8(ArgR8::MHL, ArgR8::C), // x1
        LD_r8_r8(ArgR8::MHL, ArgR8::D), // x2
        LD_r8_r8(ArgR8::MHL, ArgR8::E), // x3
        LD_r8_r8(ArgR8::MHL, ArgR8::H), // x4
        LD_r8_r8(ArgR8::MHL, ArgR8::L), // x5
        HALT,                           // x6
        LD_r8_r8(ArgR8::MHL, ArgR8::A), // x7
        LD_r8_r8(ArgR8::A, ArgR8::B),   // x8
        LD_r8_r8(ArgR8::A, ArgR8::C),   // x9
        LD_r8_r8(ArgR8::A, ArgR8::D),   // xA
        LD_r8_r8(ArgR8::A, ArgR8::E),   // xB
        LD_r8_r8(ArgR8::A, ArgR8::H),   // xC
        LD_r8_r8(ArgR8::A, ArgR8::L),   // xD
        LD_r8_r8(ArgR8::A, ArgR8::MHL), // xE
        LD_r8_r8(ArgR8::A, ArgR8::A),   // xF
    ],
    [
        // 8x
        ADD_a_r8(ArgR8::B),   // x0
        ADD_a_r8(ArgR8::C),   // x1
        ADD_a_r8(ArgR8::D),   // x2
        ADD_a_r8(ArgR8::E),   // x3
        ADD_a_r8(ArgR8::H),   // x4
        ADD_a_r8(ArgR8::L),   // x5
        ADD_a_r8(ArgR8::MHL), // x6
        ADD_a_r8(ArgR8::A),   // x7
        ADC_a_r8(ArgR8::B),   // x8
        ADC_a_r8(ArgR8::C),   // x9
        ADC_a_r8(ArgR8::D),   // xA
        ADC_a_r8(ArgR8::E),   // xB
        ADC_a_r8(ArgR8::H),   // xC
        ADC_a_r8(ArgR8::L),   // xD
        ADC_a_r8(ArgR8::MHL), // xE
        ADC_a_r8(ArgR8::A),   // xF
    ],
    [
        // 9x
        SUB_a_r8(ArgR8::B),   // x0
        SUB_a_r8(ArgR8::C),   // x1
        SUB_a_r8(ArgR8::D),   // x2
        SUB_a_r8(ArgR8::E),   // x3
        SUB_a_r8(ArgR8::H),   // x4
        SUB_a_r8(ArgR8::L),   // x5
        SUB_a_r8(ArgR8::MHL), // x6
        SUB_a_r8(ArgR8::A),   // x7
        SBC_a_r8(ArgR8::B),   // x8
        SBC_a_r8(ArgR8::C),   // x9
        SBC_a_r8(ArgR8::D),   // xA
        SBC_a_r8(ArgR8::E),   // xB
        SBC_a_r8(ArgR8::H),   // xC
        SBC_a_r8(ArgR8::L),   // xD
        SBC_a_r8(ArgR8::MHL), // xE
        SBC_a_r8(ArgR8::A),   // xF
    ],
    [
        // Ax
        AND_a_r8(ArgR8::B),   // x0
        AND_a_r8(ArgR8::C),   // x1
        AND_a_r8(ArgR8::D),   // x2
        AND_a_r8(ArgR8::E),   // x3
        AND_a_r8(ArgR8::H),   // x4
        AND_a_r8(ArgR8::L),   // x5
        AND_a_r8(ArgR8::MHL), // x6
        AND_a_r8(ArgR8::A),   // x7
        XOR_a_r8(ArgR8::B),   // x8
        XOR_a_r8(ArgR8::C),   // x9
        XOR_a_r8(ArgR8::D),   // xA
        XOR_a_r8(ArgR8::E),   // xB
        XOR_a_r8(ArgR8::H),   // xC
        XOR_a_r8(ArgR8::L),   // xD
        XOR_a_r8(ArgR8::MHL), // xE
        XOR_a_r8(ArgR8::A),   // xF
    ],
    [
        // Bx
        OR_a_r8(ArgR8::B),   // x0
        OR_a_r8(ArgR8::C),   // x1
        OR_a_r8(ArgR8::D),   // x2
        OR_a_r8(ArgR8::E),   // x3
        OR_a_r8(ArgR8::H),   // x4
        OR_a_r8(ArgR8::L),   // x5
        OR_a_r8(ArgR8::MHL), // x6
        OR_a_r8(ArgR8::A),   // x7
        CP_a_r8(ArgR8::B),   // x8
        CP_a_r8(ArgR8::C),   // x9
        CP_a_r8(ArgR8::D),   // xA
        CP_a_r8(ArgR8::E),   // xB
        CP_a_r8(ArgR8::H),   // xC
        CP_a_r8(ArgR8::L),   // xD
        CP_a_r8(ArgR8::MHL), // xE
        CP_a_r8(ArgR8::A),   // xF
    ],
    [
        // Cx
        RET_cc(ArgCOND::NZ),         // x0
        POP_r16(ArgR16STK::BC),      // x1
        JP_cc_n16(ArgCOND::NZ, 0),   // x2
        JP_n16(0),                   // x3
        CALL_cc_n16(ArgCOND::NZ, 0), // x4
        PUSH_r16(ArgR16STK::BC),     // x5
        ADD_a_r8(ArgR8::CONST(0)),   // x6
        RST_vec(ArgVEC::Vec0x00),    // x7
        RET_cc(ArgCOND::Z),          // x8
        RET,                         // x9
        JP_cc_n16(ArgCOND::Z, 0),    // xA
        PREFIX,                      // xB
        CALL_cc_n16(ArgCOND::Z, 0),  // xC
        CALL_n16(0),                 // xD
        ADC_a_r8(ArgR8::CONST(0)),   // xE
        RST_vec(ArgVEC::Vec0x08),    // xF
    ],
    [
        // Dx
        RET_cc(ArgCOND::NC),         // x0
        POP_r16(ArgR16STK::DE),      // x1
        JP_cc_n16(ArgCOND::NC, 0),   // x2
        INVALID,                     // x3
        CALL_cc_n16(ArgCOND::NC, 0), // x4
        PUSH_r16(ArgR16STK::DE),     // x5
        SUB_a_r8(ArgR8::CONST(0)),   // x6
        RST_vec(ArgVEC::Vec0x10),    // x7
        RET_cc(ArgCOND::C),          // x8
        RETI,                        // x9
        JP_cc_n16(ArgCOND::C, 0),    // xA
        INVALID,                     // xB
        CALL_cc_n16(ArgCOND::C, 0),  // xC
        INVALID,                     // xD
        SBC_a_r8(ArgR8::CONST(0)),   // xE
        RST_vec(ArgVEC::Vec0x18),    // xF
    ],
    [
        // Ex
        LDH_mn16_a(0),                  // x0
        POP_r16(ArgR16STK::HL),         // x1
        LDH_mc_a,                       // x2
        INVALID,                        // x3
        INVALID,                        // x4
        PUSH_r16(ArgR16STK::HL),        // x5
        AND_a_r8(ArgR8::CONST(0)),      // x6
        RST_vec(ArgVEC::Vec0x20),       // x7
        ADD_sp_e8(0),                   // x8
        JP_hl,                          // x9
        LD_mr16_a(ArgR16MEM::CONST(0)), // xA
        INVALID,                        // xB
        INVALID,                        // xC
        INVALID,                        // xD
        XOR_a_r8(ArgR8::CONST(0)),      // xE
        RST_vec(ArgVEC::Vec0x28),       // xF
    ],
    [
        // Fx
        LDH_a_mn16(0),                  // x0
        POP_r16(ArgR16STK::AF),         // x1
        LDH_a_mc,                       // x2
        DI,                             // x3
        INVALID,                        // x4
        PUSH_r16(ArgR16STK::AF),        // x5
        OR_a_r8(ArgR8::CONST(0)),       // x6
        RST_vec(ArgVEC::Vec0x30),       // x7
        LD_hl_sp_plus_e8(0),            // x8
        LD_sp_hl,                       // x9
        LD_a_mr16(ArgR16MEM::CONST(0)), // xA
        EI,                             // xB
        INVALID,                        // xC
        INVALID,                        // xD
        CP_a_r8(ArgR8::CONST(0)),       // xE
        RST_vec(ArgVEC::Vec0x38),       // xF
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
