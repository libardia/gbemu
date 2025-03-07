use std::{cell::RefCell, rc::Rc};

use crate::{hex::*, mmu::MMU};

use super::instructions::{Instruction, Instruction::*, *};

const OP_TABLE: [[Instruction; 16]; 16] = [
    [
        // 0x
        NOP,                                        // x0
        LD_r16_n16(ArgR16::BC, HexU16(0)),          // x1**
        LD_mr16_a(ArgR16MEM::BC),                   // x2
        INC_r16(ArgR16::BC),                        // x3
        INC_r8(ArgR8::B),                           // x4
        DEC_r8(ArgR8::B),                           // x5
        LD_r8_r8(ArgR8::B, ArgR8::CONST(HexU8(0))), // x6*
        RLCA,                                       // x7
        LD_mn16_sp(HexU16(0)),                      // x8**
        ADD_hl_r16(ArgR16::BC),                     // x9
        LD_a_mr16(ArgR16MEM::BC),                   // xA
        DEC_r16(ArgR16::BC),                        // xB
        INC_r8(ArgR8::C),                           // xC
        DEC_r8(ArgR8::C),                           // xD
        LD_r8_r8(ArgR8::C, ArgR8::CONST(HexU8(0))), // xE*
        RRCA,                                       // xF
    ],
    [
        // 1x
        STOP(HexU8(0)),                             // x0*
        LD_r16_n16(ArgR16::DE, HexU16(0)),          // x1**
        LD_mr16_a(ArgR16MEM::DE),                   // x2
        INC_r16(ArgR16::DE),                        // x3
        INC_r8(ArgR8::D),                           // x4
        DEC_r8(ArgR8::D),                           // x5
        LD_r8_r8(ArgR8::D, ArgR8::CONST(HexU8(0))), // x6*
        RLA,                                        // x7
        JR_e8(0),                                   // x8*
        ADD_hl_r16(ArgR16::DE),                     // x9
        LD_a_mr16(ArgR16MEM::DE),                   // xA
        DEC_r16(ArgR16::DE),                        // xB
        INC_r8(ArgR8::E),                           // xC
        DEC_r8(ArgR8::E),                           // xD
        LD_r8_r8(ArgR8::E, ArgR8::CONST(HexU8(0))), // xE*
        RRA,                                        // xF
    ],
    [
        // 2x
        JR_cc_e8(ArgCOND::NZ, 0),                   // x0*
        LD_r16_n16(ArgR16::HL, HexU16(0)),          // x1**
        LD_mr16_a(ArgR16MEM::HLI),                  // x2
        INC_r16(ArgR16::HL),                        // x3
        INC_r8(ArgR8::H),                           // x4
        DEC_r8(ArgR8::H),                           // x5
        LD_r8_r8(ArgR8::H, ArgR8::CONST(HexU8(0))), // x6*
        DAA,                                        // x7
        JR_cc_e8(ArgCOND::Z, 0),                    // x8*
        ADD_hl_r16(ArgR16::HL),                     // x9
        LD_a_mr16(ArgR16MEM::HLI),                  // xA
        DEC_r16(ArgR16::HL),                        // xB
        INC_r8(ArgR8::L),                           // xC
        DEC_r8(ArgR8::L),                           // xD
        LD_r8_r8(ArgR8::L, ArgR8::CONST(HexU8(0))), // xE*
        CPL,                                        // xF
    ],
    [
        // 3x
        JR_cc_e8(ArgCOND::NC, 0),                     // x0*
        LD_sp_n16(HexU16(0)),                         // x1**
        LD_mr16_a(ArgR16MEM::HLD),                    // x2
        INC_sp,                                       // x3
        INC_r8(ArgR8::MHL),                           // x4
        DEC_r8(ArgR8::MHL),                           // x5
        LD_r8_r8(ArgR8::MHL, ArgR8::CONST(HexU8(0))), // x6*
        SCF,                                          // x7
        JR_cc_e8(ArgCOND::C, 0),                      // x8*
        ADD_hl_sp,                                    // x9
        LD_a_mr16(ArgR16MEM::HLD),                    // xA
        DEC_sp,                                       // xB
        INC_r8(ArgR8::A),                             // xC
        DEC_r8(ArgR8::A),                             // xD
        LD_r8_r8(ArgR8::A, ArgR8::CONST(HexU8(0))),   // xE*
        CCF,                                          // xF
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
        RET_cc(ArgCOND::NZ),                 // x0
        POP_r16(ArgR16STK::BC),              // x1
        JP_cc_n16(ArgCOND::NZ, HexU16(0)),   // x2**
        JP_n16(HexU16(0)),                   // x3**
        CALL_cc_n16(ArgCOND::NZ, HexU16(0)), // x4**
        PUSH_r16(ArgR16STK::BC),             // x5
        ADD_a_r8(ArgR8::CONST(HexU8(0))),    // x6*
        RST_vec(ArgVEC::Vec0x00),            // x7
        RET_cc(ArgCOND::Z),                  // x8
        RET,                                 // x9
        JP_cc_n16(ArgCOND::Z, HexU16(0)),    // xA**
        PREFIX,                              // xB
        CALL_cc_n16(ArgCOND::Z, HexU16(0)),  // xC**
        CALL_n16(HexU16(0)),                 // xD**
        ADC_a_r8(ArgR8::CONST(HexU8(0))),    // xE*
        RST_vec(ArgVEC::Vec0x08),            // xF
    ],
    [
        // Dx
        RET_cc(ArgCOND::NC),                 // x0
        POP_r16(ArgR16STK::DE),              // x1
        JP_cc_n16(ArgCOND::NC, HexU16(0)),   // x2**
        INVALID,                             // x3
        CALL_cc_n16(ArgCOND::NC, HexU16(0)), // x4**
        PUSH_r16(ArgR16STK::DE),             // x5
        SUB_a_r8(ArgR8::CONST(HexU8(0))),    // x6*
        RST_vec(ArgVEC::Vec0x10),            // x7
        RET_cc(ArgCOND::C),                  // x8
        RETI,                                // x9
        JP_cc_n16(ArgCOND::C, HexU16(0)),    // xA**
        INVALID,                             // xB
        CALL_cc_n16(ArgCOND::C, HexU16(0)),  // xC**
        INVALID,                             // xD
        SBC_a_r8(ArgR8::CONST(HexU8(0))),    // xE*
        RST_vec(ArgVEC::Vec0x18),            // xF
    ],
    [
        // Ex
        LDH_mn16_a(HexU8(0)),                   // x0*
        POP_r16(ArgR16STK::HL),                 // x1
        LDH_mc_a,                               // x2
        INVALID,                                // x3
        INVALID,                                // x4
        PUSH_r16(ArgR16STK::HL),                // x5
        AND_a_r8(ArgR8::CONST(HexU8(0))),       // x6*
        RST_vec(ArgVEC::Vec0x20),               // x7
        ADD_sp_e8(0),                           // x8*
        JP_hl,                                  // x9
        LD_mr16_a(ArgR16MEM::CONST(HexU16(0))), // xA**
        INVALID,                                // xB
        TERMINATE,                              // xC         WARNING: NON-STAMDARD
        DEBUG_PRINT,                            // xD         WARNING: NON-STAMDARD
        XOR_a_r8(ArgR8::CONST(HexU8(0))),       // xE*
        RST_vec(ArgVEC::Vec0x28),               // xF
    ],
    [
        // Fx
        LDH_a_mn16(HexU8(0)),                   // x0*
        POP_r16(ArgR16STK::AF),                 // x1
        LDH_a_mc,                               // x2
        DI,                                     // x3
        INVALID,                                // x4
        PUSH_r16(ArgR16STK::AF),                // x5
        OR_a_r8(ArgR8::CONST(HexU8(0))),        // x6*
        RST_vec(ArgVEC::Vec0x30),               // x7
        LD_hl_sp_plus_e8(0),                    // x8*
        LD_sp_hl,                               // x9
        LD_a_mr16(ArgR16MEM::CONST(HexU16(0))), // xA**
        EI,                                     // xB
        INVALID,                                // xC
        INVALID,                                // xD
        CP_a_r8(ArgR8::CONST(HexU8(0))),        // xE*
        RST_vec(ArgVEC::Vec0x38),               // xF
    ],
];

const PREFIX_TABLE: [[Instruction; 16]; 16] = [
    [
        // 0x
        RLC_r8(ArgR8::B),   // x0
        RLC_r8(ArgR8::C),   // x1
        RLC_r8(ArgR8::D),   // x2
        RLC_r8(ArgR8::E),   // x3
        RLC_r8(ArgR8::H),   // x4
        RLC_r8(ArgR8::L),   // x5
        RLC_r8(ArgR8::MHL), // x6
        RLC_r8(ArgR8::A),   // x7
        RRC_r8(ArgR8::B),   // x8
        RRC_r8(ArgR8::C),   // x9
        RRC_r8(ArgR8::D),   // xA
        RRC_r8(ArgR8::E),   // xB
        RRC_r8(ArgR8::H),   // xC
        RRC_r8(ArgR8::L),   // xD
        RRC_r8(ArgR8::MHL), // xE
        RRC_r8(ArgR8::A),   // xF
    ],
    [
        // 1x
        RL_r8(ArgR8::B),   // x0
        RL_r8(ArgR8::C),   // x1
        RL_r8(ArgR8::D),   // x2
        RL_r8(ArgR8::E),   // x3
        RL_r8(ArgR8::H),   // x4
        RL_r8(ArgR8::L),   // x5
        RL_r8(ArgR8::MHL), // x6
        RL_r8(ArgR8::A),   // x7
        RR_r8(ArgR8::B),   // x8
        RR_r8(ArgR8::C),   // x9
        RR_r8(ArgR8::D),   // xA
        RR_r8(ArgR8::E),   // xB
        RR_r8(ArgR8::H),   // xC
        RR_r8(ArgR8::L),   // xD
        RR_r8(ArgR8::MHL), // xE
        RR_r8(ArgR8::A),   // xF
    ],
    [
        // 2x
        SLA_r8(ArgR8::B),   // x0
        SLA_r8(ArgR8::C),   // x1
        SLA_r8(ArgR8::D),   // x2
        SLA_r8(ArgR8::E),   // x3
        SLA_r8(ArgR8::H),   // x4
        SLA_r8(ArgR8::L),   // x5
        SLA_r8(ArgR8::MHL), // x6
        SLA_r8(ArgR8::A),   // x7
        SRA_r8(ArgR8::B),   // x8
        SRA_r8(ArgR8::C),   // x9
        SRA_r8(ArgR8::D),   // xA
        SRA_r8(ArgR8::E),   // xB
        SRA_r8(ArgR8::H),   // xC
        SRA_r8(ArgR8::L),   // xD
        SRA_r8(ArgR8::MHL), // xE
        SRA_r8(ArgR8::A),   // xF
    ],
    [
        // 3x
        SWAP_r8(ArgR8::B),   // x0
        SWAP_r8(ArgR8::C),   // x1
        SWAP_r8(ArgR8::D),   // x2
        SWAP_r8(ArgR8::E),   // x3
        SWAP_r8(ArgR8::H),   // x4
        SWAP_r8(ArgR8::L),   // x5
        SWAP_r8(ArgR8::MHL), // x6
        SWAP_r8(ArgR8::A),   // x7
        SRL_r8(ArgR8::B),    // x8
        SRL_r8(ArgR8::C),    // x9
        SRL_r8(ArgR8::D),    // xA
        SRL_r8(ArgR8::E),    // xB
        SRL_r8(ArgR8::H),    // xC
        SRL_r8(ArgR8::L),    // xD
        SRL_r8(ArgR8::MHL),  // xE
        SRL_r8(ArgR8::A),    // xF
    ],
    [
        // 4x
        BIT_u3_r8(ArgU3::Bit0, ArgR8::B),   // x0
        BIT_u3_r8(ArgU3::Bit0, ArgR8::C),   // x1
        BIT_u3_r8(ArgU3::Bit0, ArgR8::D),   // x2
        BIT_u3_r8(ArgU3::Bit0, ArgR8::E),   // x3
        BIT_u3_r8(ArgU3::Bit0, ArgR8::H),   // x4
        BIT_u3_r8(ArgU3::Bit0, ArgR8::L),   // x5
        BIT_u3_r8(ArgU3::Bit0, ArgR8::MHL), // x6
        BIT_u3_r8(ArgU3::Bit0, ArgR8::A),   // x7
        BIT_u3_r8(ArgU3::Bit1, ArgR8::B),   // x8
        BIT_u3_r8(ArgU3::Bit1, ArgR8::C),   // x9
        BIT_u3_r8(ArgU3::Bit1, ArgR8::D),   // xA
        BIT_u3_r8(ArgU3::Bit1, ArgR8::E),   // xB
        BIT_u3_r8(ArgU3::Bit1, ArgR8::H),   // xC
        BIT_u3_r8(ArgU3::Bit1, ArgR8::L),   // xD
        BIT_u3_r8(ArgU3::Bit1, ArgR8::MHL), // xE
        BIT_u3_r8(ArgU3::Bit1, ArgR8::A),   // xF
    ],
    [
        // 5x
        BIT_u3_r8(ArgU3::Bit2, ArgR8::B),   // x0
        BIT_u3_r8(ArgU3::Bit2, ArgR8::C),   // x1
        BIT_u3_r8(ArgU3::Bit2, ArgR8::D),   // x2
        BIT_u3_r8(ArgU3::Bit2, ArgR8::E),   // x3
        BIT_u3_r8(ArgU3::Bit2, ArgR8::H),   // x4
        BIT_u3_r8(ArgU3::Bit2, ArgR8::L),   // x5
        BIT_u3_r8(ArgU3::Bit2, ArgR8::MHL), // x6
        BIT_u3_r8(ArgU3::Bit2, ArgR8::A),   // x7
        BIT_u3_r8(ArgU3::Bit3, ArgR8::B),   // x8
        BIT_u3_r8(ArgU3::Bit3, ArgR8::C),   // x9
        BIT_u3_r8(ArgU3::Bit3, ArgR8::D),   // xA
        BIT_u3_r8(ArgU3::Bit3, ArgR8::E),   // xB
        BIT_u3_r8(ArgU3::Bit3, ArgR8::H),   // xC
        BIT_u3_r8(ArgU3::Bit3, ArgR8::L),   // xD
        BIT_u3_r8(ArgU3::Bit3, ArgR8::MHL), // xE
        BIT_u3_r8(ArgU3::Bit3, ArgR8::A),   // xF
    ],
    [
        // 6x
        BIT_u3_r8(ArgU3::Bit4, ArgR8::B),   // x0
        BIT_u3_r8(ArgU3::Bit4, ArgR8::C),   // x1
        BIT_u3_r8(ArgU3::Bit4, ArgR8::D),   // x2
        BIT_u3_r8(ArgU3::Bit4, ArgR8::E),   // x3
        BIT_u3_r8(ArgU3::Bit4, ArgR8::H),   // x4
        BIT_u3_r8(ArgU3::Bit4, ArgR8::L),   // x5
        BIT_u3_r8(ArgU3::Bit4, ArgR8::MHL), // x6
        BIT_u3_r8(ArgU3::Bit4, ArgR8::A),   // x7
        BIT_u3_r8(ArgU3::Bit5, ArgR8::B),   // x8
        BIT_u3_r8(ArgU3::Bit5, ArgR8::C),   // x9
        BIT_u3_r8(ArgU3::Bit5, ArgR8::D),   // xA
        BIT_u3_r8(ArgU3::Bit5, ArgR8::E),   // xB
        BIT_u3_r8(ArgU3::Bit5, ArgR8::H),   // xC
        BIT_u3_r8(ArgU3::Bit5, ArgR8::L),   // xD
        BIT_u3_r8(ArgU3::Bit5, ArgR8::MHL), // xE
        BIT_u3_r8(ArgU3::Bit5, ArgR8::A),   // xF
    ],
    [
        // 7x
        BIT_u3_r8(ArgU3::Bit6, ArgR8::B),   // x0
        BIT_u3_r8(ArgU3::Bit6, ArgR8::C),   // x1
        BIT_u3_r8(ArgU3::Bit6, ArgR8::D),   // x2
        BIT_u3_r8(ArgU3::Bit6, ArgR8::E),   // x3
        BIT_u3_r8(ArgU3::Bit6, ArgR8::H),   // x4
        BIT_u3_r8(ArgU3::Bit6, ArgR8::L),   // x5
        BIT_u3_r8(ArgU3::Bit6, ArgR8::MHL), // x6
        BIT_u3_r8(ArgU3::Bit6, ArgR8::A),   // x7
        BIT_u3_r8(ArgU3::Bit7, ArgR8::B),   // x8
        BIT_u3_r8(ArgU3::Bit7, ArgR8::C),   // x9
        BIT_u3_r8(ArgU3::Bit7, ArgR8::D),   // xA
        BIT_u3_r8(ArgU3::Bit7, ArgR8::E),   // xB
        BIT_u3_r8(ArgU3::Bit7, ArgR8::H),   // xC
        BIT_u3_r8(ArgU3::Bit7, ArgR8::L),   // xD
        BIT_u3_r8(ArgU3::Bit7, ArgR8::MHL), // xE
        BIT_u3_r8(ArgU3::Bit7, ArgR8::A),   // xF
    ],
    [
        // 8x
        RES_u3_r8(ArgU3::Bit0, ArgR8::B),   // x0
        RES_u3_r8(ArgU3::Bit0, ArgR8::C),   // x1
        RES_u3_r8(ArgU3::Bit0, ArgR8::D),   // x2
        RES_u3_r8(ArgU3::Bit0, ArgR8::E),   // x3
        RES_u3_r8(ArgU3::Bit0, ArgR8::H),   // x4
        RES_u3_r8(ArgU3::Bit0, ArgR8::L),   // x5
        RES_u3_r8(ArgU3::Bit0, ArgR8::MHL), // x6
        RES_u3_r8(ArgU3::Bit0, ArgR8::A),   // x7
        RES_u3_r8(ArgU3::Bit1, ArgR8::B),   // x8
        RES_u3_r8(ArgU3::Bit1, ArgR8::C),   // x9
        RES_u3_r8(ArgU3::Bit1, ArgR8::D),   // xA
        RES_u3_r8(ArgU3::Bit1, ArgR8::E),   // xB
        RES_u3_r8(ArgU3::Bit1, ArgR8::H),   // xC
        RES_u3_r8(ArgU3::Bit1, ArgR8::L),   // xD
        RES_u3_r8(ArgU3::Bit1, ArgR8::MHL), // xE
        RES_u3_r8(ArgU3::Bit1, ArgR8::A),   // xF
    ],
    [
        // 9x
        RES_u3_r8(ArgU3::Bit2, ArgR8::B),   // x0
        RES_u3_r8(ArgU3::Bit2, ArgR8::C),   // x1
        RES_u3_r8(ArgU3::Bit2, ArgR8::D),   // x2
        RES_u3_r8(ArgU3::Bit2, ArgR8::E),   // x3
        RES_u3_r8(ArgU3::Bit2, ArgR8::H),   // x4
        RES_u3_r8(ArgU3::Bit2, ArgR8::L),   // x5
        RES_u3_r8(ArgU3::Bit2, ArgR8::MHL), // x6
        RES_u3_r8(ArgU3::Bit2, ArgR8::A),   // x7
        RES_u3_r8(ArgU3::Bit3, ArgR8::B),   // x8
        RES_u3_r8(ArgU3::Bit3, ArgR8::C),   // x9
        RES_u3_r8(ArgU3::Bit3, ArgR8::D),   // xA
        RES_u3_r8(ArgU3::Bit3, ArgR8::E),   // xB
        RES_u3_r8(ArgU3::Bit3, ArgR8::H),   // xC
        RES_u3_r8(ArgU3::Bit3, ArgR8::L),   // xD
        RES_u3_r8(ArgU3::Bit3, ArgR8::MHL), // xE
        RES_u3_r8(ArgU3::Bit3, ArgR8::A),   // xF
    ],
    [
        // Ax
        RES_u3_r8(ArgU3::Bit4, ArgR8::B),   // x0
        RES_u3_r8(ArgU3::Bit4, ArgR8::C),   // x1
        RES_u3_r8(ArgU3::Bit4, ArgR8::D),   // x2
        RES_u3_r8(ArgU3::Bit4, ArgR8::E),   // x3
        RES_u3_r8(ArgU3::Bit4, ArgR8::H),   // x4
        RES_u3_r8(ArgU3::Bit4, ArgR8::L),   // x5
        RES_u3_r8(ArgU3::Bit4, ArgR8::MHL), // x6
        RES_u3_r8(ArgU3::Bit4, ArgR8::A),   // x7
        RES_u3_r8(ArgU3::Bit5, ArgR8::B),   // x8
        RES_u3_r8(ArgU3::Bit5, ArgR8::C),   // x9
        RES_u3_r8(ArgU3::Bit5, ArgR8::D),   // xA
        RES_u3_r8(ArgU3::Bit5, ArgR8::E),   // xB
        RES_u3_r8(ArgU3::Bit5, ArgR8::H),   // xC
        RES_u3_r8(ArgU3::Bit5, ArgR8::L),   // xD
        RES_u3_r8(ArgU3::Bit5, ArgR8::MHL), // xE
        RES_u3_r8(ArgU3::Bit5, ArgR8::A),   // xF
    ],
    [
        // Bx
        RES_u3_r8(ArgU3::Bit6, ArgR8::B),   // x0
        RES_u3_r8(ArgU3::Bit6, ArgR8::C),   // x1
        RES_u3_r8(ArgU3::Bit6, ArgR8::D),   // x2
        RES_u3_r8(ArgU3::Bit6, ArgR8::E),   // x3
        RES_u3_r8(ArgU3::Bit6, ArgR8::H),   // x4
        RES_u3_r8(ArgU3::Bit6, ArgR8::L),   // x5
        RES_u3_r8(ArgU3::Bit6, ArgR8::MHL), // x6
        RES_u3_r8(ArgU3::Bit6, ArgR8::A),   // x7
        RES_u3_r8(ArgU3::Bit7, ArgR8::B),   // x8
        RES_u3_r8(ArgU3::Bit7, ArgR8::C),   // x9
        RES_u3_r8(ArgU3::Bit7, ArgR8::D),   // xA
        RES_u3_r8(ArgU3::Bit7, ArgR8::E),   // xB
        RES_u3_r8(ArgU3::Bit7, ArgR8::H),   // xC
        RES_u3_r8(ArgU3::Bit7, ArgR8::L),   // xD
        RES_u3_r8(ArgU3::Bit7, ArgR8::MHL), // xE
        RES_u3_r8(ArgU3::Bit7, ArgR8::A),   // xF
    ],
    [
        // Cx
        SET_u3_r8(ArgU3::Bit0, ArgR8::B),   // x0
        SET_u3_r8(ArgU3::Bit0, ArgR8::C),   // x1
        SET_u3_r8(ArgU3::Bit0, ArgR8::D),   // x2
        SET_u3_r8(ArgU3::Bit0, ArgR8::E),   // x3
        SET_u3_r8(ArgU3::Bit0, ArgR8::H),   // x4
        SET_u3_r8(ArgU3::Bit0, ArgR8::L),   // x5
        SET_u3_r8(ArgU3::Bit0, ArgR8::MHL), // x6
        SET_u3_r8(ArgU3::Bit0, ArgR8::A),   // x7
        SET_u3_r8(ArgU3::Bit1, ArgR8::B),   // x8
        SET_u3_r8(ArgU3::Bit1, ArgR8::C),   // x9
        SET_u3_r8(ArgU3::Bit1, ArgR8::D),   // xA
        SET_u3_r8(ArgU3::Bit1, ArgR8::E),   // xB
        SET_u3_r8(ArgU3::Bit1, ArgR8::H),   // xC
        SET_u3_r8(ArgU3::Bit1, ArgR8::L),   // xD
        SET_u3_r8(ArgU3::Bit1, ArgR8::MHL), // xE
        SET_u3_r8(ArgU3::Bit1, ArgR8::A),   // xF
    ],
    [
        // Dx
        SET_u3_r8(ArgU3::Bit2, ArgR8::B),   // x0
        SET_u3_r8(ArgU3::Bit2, ArgR8::C),   // x1
        SET_u3_r8(ArgU3::Bit2, ArgR8::D),   // x2
        SET_u3_r8(ArgU3::Bit2, ArgR8::E),   // x3
        SET_u3_r8(ArgU3::Bit2, ArgR8::H),   // x4
        SET_u3_r8(ArgU3::Bit2, ArgR8::L),   // x5
        SET_u3_r8(ArgU3::Bit2, ArgR8::MHL), // x6
        SET_u3_r8(ArgU3::Bit2, ArgR8::A),   // x7
        SET_u3_r8(ArgU3::Bit3, ArgR8::B),   // x8
        SET_u3_r8(ArgU3::Bit3, ArgR8::C),   // x9
        SET_u3_r8(ArgU3::Bit3, ArgR8::D),   // xA
        SET_u3_r8(ArgU3::Bit3, ArgR8::E),   // xB
        SET_u3_r8(ArgU3::Bit3, ArgR8::H),   // xC
        SET_u3_r8(ArgU3::Bit3, ArgR8::L),   // xD
        SET_u3_r8(ArgU3::Bit3, ArgR8::MHL), // xE
        SET_u3_r8(ArgU3::Bit3, ArgR8::A),   // xF
    ],
    [
        // Ex
        SET_u3_r8(ArgU3::Bit4, ArgR8::B),   // x0
        SET_u3_r8(ArgU3::Bit4, ArgR8::C),   // x1
        SET_u3_r8(ArgU3::Bit4, ArgR8::D),   // x2
        SET_u3_r8(ArgU3::Bit4, ArgR8::E),   // x3
        SET_u3_r8(ArgU3::Bit4, ArgR8::H),   // x4
        SET_u3_r8(ArgU3::Bit4, ArgR8::L),   // x5
        SET_u3_r8(ArgU3::Bit4, ArgR8::MHL), // x6
        SET_u3_r8(ArgU3::Bit4, ArgR8::A),   // x7
        SET_u3_r8(ArgU3::Bit5, ArgR8::B),   // x8
        SET_u3_r8(ArgU3::Bit5, ArgR8::C),   // x9
        SET_u3_r8(ArgU3::Bit5, ArgR8::D),   // xA
        SET_u3_r8(ArgU3::Bit5, ArgR8::E),   // xB
        SET_u3_r8(ArgU3::Bit5, ArgR8::H),   // xC
        SET_u3_r8(ArgU3::Bit5, ArgR8::L),   // xD
        SET_u3_r8(ArgU3::Bit5, ArgR8::MHL), // xE
        SET_u3_r8(ArgU3::Bit5, ArgR8::A),   // xF
    ],
    [
        // Fx
        SET_u3_r8(ArgU3::Bit6, ArgR8::B),   // x0
        SET_u3_r8(ArgU3::Bit6, ArgR8::C),   // x1
        SET_u3_r8(ArgU3::Bit6, ArgR8::D),   // x2
        SET_u3_r8(ArgU3::Bit6, ArgR8::E),   // x3
        SET_u3_r8(ArgU3::Bit6, ArgR8::H),   // x4
        SET_u3_r8(ArgU3::Bit6, ArgR8::L),   // x5
        SET_u3_r8(ArgU3::Bit6, ArgR8::MHL), // x6
        SET_u3_r8(ArgU3::Bit6, ArgR8::A),   // x7
        SET_u3_r8(ArgU3::Bit7, ArgR8::B),   // x8
        SET_u3_r8(ArgU3::Bit7, ArgR8::C),   // x9
        SET_u3_r8(ArgU3::Bit7, ArgR8::D),   // xA
        SET_u3_r8(ArgU3::Bit7, ArgR8::E),   // xB
        SET_u3_r8(ArgU3::Bit7, ArgR8::H),   // xC
        SET_u3_r8(ArgU3::Bit7, ArgR8::L),   // xD
        SET_u3_r8(ArgU3::Bit7, ArgR8::MHL), // xE
        SET_u3_r8(ArgU3::Bit7, ArgR8::A),   // xF
    ],
];

#[derive(Debug)]
pub struct Decoder<M: MMU> {
    mmu: Rc<RefCell<M>>,

    pc: u16,
    inst_length: u16,
}

impl<M: MMU> Decoder<M> {
    pub fn new(mmu: Rc<RefCell<M>>) -> Self {
        Decoder {
            mmu,
            pc: 0,
            inst_length: 0,
        }
    }

    fn get_instruction(&self, table: &[[Instruction; 16]; 16], code: u8) -> Instruction {
        let upper = ((code & 0xF0) >> 4) as usize;
        let lower = (code & 0xF) as usize;
        table[upper][lower]
    }

    fn get_next_byte(&mut self) -> HexU8 {
        self.inst_length += 1;
        self.mmu.borrow().read_byte(self.pc + 1).into()
    }

    fn get_next_signed_byte(&mut self) -> i8 {
        self.inst_length += 1;
        self.mmu.borrow().read_signed_byte(self.pc + 1)
    }

    fn get_next_word(&mut self) -> HexU16 {
        self.inst_length += 2;
        self.mmu.borrow().read_word(self.pc + 1).into()
    }

    pub fn decode(&mut self, pc: &u16) -> (Instruction, u16) {
        // Save and reset state
        self.pc = *pc;
        self.inst_length = 1;

        let code = self.mmu.borrow().read_byte(self.pc);
        let mut inst = self.get_instruction(&OP_TABLE, code);

        if inst == PREFIX {
            let second = self.mmu.borrow().read_byte(self.pc + 1);
            self.inst_length += 1;
            inst = self.get_instruction(&PREFIX_TABLE, second);
        } else {
            // Fill in constants from following bytes, if applicable
            inst = match inst {
                // 0x
                LD_r16_n16(x, _) => LD_r16_n16(x, self.get_next_word()),
                LD_r8_r8(x, ArgR8::CONST(_)) => LD_r8_r8(x, ArgR8::CONST(self.get_next_byte())),
                LD_mn16_sp(_) => LD_mn16_sp(self.get_next_word()),

                // 1x
                STOP(_) => STOP(self.get_next_byte()),
                JR_e8(_) => JR_e8(self.get_next_signed_byte()),

                // 2x
                JR_cc_e8(x, _) => JR_cc_e8(x, self.get_next_signed_byte()),

                // 3x
                LD_sp_n16(_) => LD_sp_n16(self.get_next_word()),

                // Cx
                JP_cc_n16(x, _) => JP_cc_n16(x, self.get_next_word()),
                JP_n16(_) => JP_n16(self.get_next_word()),
                CALL_cc_n16(x, _) => CALL_cc_n16(x, self.get_next_word()),
                ADD_a_r8(ArgR8::CONST(_)) => ADD_a_r8(ArgR8::CONST(self.get_next_byte())),
                CALL_n16(_) => CALL_n16(self.get_next_word()),
                ADC_a_r8(ArgR8::CONST(_)) => ADC_a_r8(ArgR8::CONST(self.get_next_byte())),

                // Dx
                SUB_a_r8(ArgR8::CONST(_)) => SUB_a_r8(ArgR8::CONST(self.get_next_byte())),
                SBC_a_r8(ArgR8::CONST(_)) => SBC_a_r8(ArgR8::CONST(self.get_next_byte())),

                // Ex
                LDH_mn16_a(_) => LDH_mn16_a(self.get_next_byte()),
                AND_a_r8(ArgR8::CONST(_)) => AND_a_r8(ArgR8::CONST(self.get_next_byte())),
                ADD_sp_e8(_) => ADD_sp_e8(self.get_next_signed_byte()),
                LD_mr16_a(ArgR16MEM::CONST(_)) => LD_mr16_a(ArgR16MEM::CONST(self.get_next_word())),
                XOR_a_r8(ArgR8::CONST(_)) => XOR_a_r8(ArgR8::CONST(self.get_next_byte())),

                // Fx
                LDH_a_mn16(_) => LDH_a_mn16(self.get_next_byte()),
                OR_a_r8(ArgR8::CONST(_)) => OR_a_r8(ArgR8::CONST(self.get_next_byte())),
                LD_hl_sp_plus_e8(_) => LD_hl_sp_plus_e8(self.get_next_signed_byte()),
                LD_a_mr16(ArgR16MEM::CONST(_)) => LD_a_mr16(ArgR16MEM::CONST(self.get_next_word())),
                CP_a_r8(ArgR8::CONST(_)) => CP_a_r8(ArgR8::CONST(self.get_next_byte())),

                // Everything else
                _ => inst,
            };
        }

        (inst, self.inst_length)
    }
}
