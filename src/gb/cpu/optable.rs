use super::instruction::{Arg::*, Instruction, Instruction::*};

pub const OP_TABLE: [Instruction; 0x100] = [
    // 0x
    NOP,                // x0
    LD(R16_BC, IMM_16), // x1
    LD(M_BC, R8_A),     // x2
    INC(R16_BC),        // x3
    INC(R8_B),          // x4
    DEC(R8_B),          // x5
    LD(R8_B, IMM_8),    // x6
    RLCA,               // x7
    LD(IMM_16, R16_SP), // x8
    ADD_16(R16_BC),     // x9
    LD(R8_A, M_BC),     // xA
    DEC(R16_BC),        // xB
    INC(R8_C),          // xC
    DEC(R8_C),          // xD
    LD(R8_C, IMM_8),    // xE
    RRCA,               // xF
    // 1x
    STOP(IMM_8),        // x0
    LD(R16_DE, IMM_16), // x1
    LD(M_DE, R8_A),     // x2
    INC(R16_DE),        // x3
    INC(R8_D),          // x4
    DEC(R8_D),          // x5
    LD(R8_D, IMM_8),    // x6
    RLA,                // x7
    JR(C_A, IMM_i8),    // x8
    ADD_16(R16_DE),     // x9
    LD(R8_A, M_DE),     // xA
    DEC(R16_DE),        // xB
    INC(R8_E),          // xC
    DEC(R8_E),          // xD
    LD(R8_E, IMM_8),    // xE
    RRA,                // xF
    // 2x
    JR(C_NZ, IMM_i8),   // x0
    LD(R16_HL, IMM_16), // x1
    LD(M_HLI, R8_A),    // x2
    INC(R16_HL),        // x3
    INC(R8_H),          // x4
    DEC(R8_H),          // x5
    LD(R8_H, IMM_8),    // x6
    DAA,                // x7
    JR(C_Z, IMM_i8),    // x8
    ADD_16(R16_HL),     // x9
    LD(R8_A, M_HLI),    // xA
    DEC(R16_HL),        // xB
    INC(R8_L),          // xC
    DEC(R8_L),          // xD
    LD(R8_L, IMM_8),    // xE
    CPL,                // xF
    // 3x
    JR(C_NC, IMM_i8),   // x0
    LD(R16_SP, IMM_16), // x1
    LD(M_HLD, R8_A),    // x2
    INC(R16_SP),        // x3
    INC(M_HL),          // x4
    DEC(M_HL),          // x5
    LD(M_HL, IMM_8),    // x6
    SCF,                // x7
    JR(C_C, IMM_i8),    // x8
    ADD_16(R16_SP),     // x9
    LD(R8_A, M_HLD),    // xA
    DEC(R16_SP),        // xB
    INC(R8_A),          // xC
    DEC(R8_A),          // xD
    LD(R8_A, IMM_8),    // xE
    CCF,                // xF
    // 4x
    LD(R8_B, R8_B), // x0
    LD(R8_B, R8_C), // x1
    LD(R8_B, R8_D), // x2
    LD(R8_B, R8_E), // x3
    LD(R8_B, R8_H), // x4
    LD(R8_B, R8_L), // x5
    LD(R8_B, M_HL), // x6
    LD(R8_B, R8_A), // x7
    LD(R8_C, R8_B), // x8
    LD(R8_C, R8_C), // x9
    LD(R8_C, R8_D), // xA
    LD(R8_C, R8_E), // xB
    LD(R8_C, R8_H), // xC
    LD(R8_C, R8_L), // xD
    LD(R8_C, M_HL), // xE
    LD(R8_C, R8_A), // xF
    // 5x
    LD(R8_D, R8_B), // x0
    LD(R8_D, R8_C), // x1
    LD(R8_D, R8_D), // x2
    LD(R8_D, R8_E), // x3
    LD(R8_D, R8_H), // x4
    LD(R8_D, R8_L), // x5
    LD(R8_D, M_HL), // x6
    LD(R8_D, R8_A), // x7
    LD(R8_E, R8_B), // x8
    LD(R8_E, R8_C), // x9
    LD(R8_E, R8_D), // xA
    LD(R8_E, R8_E), // xB
    LD(R8_E, R8_H), // xC
    LD(R8_E, R8_L), // xD
    LD(R8_E, M_HL), // xE
    LD(R8_E, R8_A), // xF
    // 6x
    LD(R8_H, R8_B), // x0
    LD(R8_H, R8_C), // x1
    LD(R8_H, R8_D), // x2
    LD(R8_H, R8_E), // x3
    LD(R8_H, R8_H), // x4
    LD(R8_H, R8_L), // x5
    LD(R8_H, M_HL), // x6
    LD(R8_H, R8_A), // x7
    LD(R8_L, R8_B), // x8
    LD(R8_L, R8_C), // x9
    LD(R8_L, R8_D), // xA
    LD(R8_L, R8_E), // xB
    LD(R8_L, R8_H), // xC
    LD(R8_L, R8_L), // xD
    LD(R8_L, M_HL), // xE
    LD(R8_L, R8_A), // xF
    // 7x
    LD(M_HL, R8_B), // x0
    LD(M_HL, R8_C), // x1
    LD(M_HL, R8_D), // x2
    LD(M_HL, R8_E), // x3
    LD(M_HL, R8_H), // x4
    LD(M_HL, R8_L), // x5
    HALT,           // x6
    LD(M_HL, R8_A), // x7
    LD(R8_A, R8_B), // x8
    LD(R8_A, R8_C), // x9
    LD(R8_A, R8_D), // xA
    LD(R8_A, R8_E), // xB
    LD(R8_A, R8_H), // xC
    LD(R8_A, R8_L), // xD
    LD(R8_A, M_HL), // xE
    LD(R8_A, R8_A), // xF
    // 8x
    ADD(R8_B), // x0
    ADD(R8_C), // x1
    ADD(R8_D), // x2
    ADD(R8_E), // x3
    ADD(R8_H), // x4
    ADD(R8_L), // x5
    ADD(M_HL), // x6
    ADD(R8_A), // x7
    ADC(R8_B), // x8
    ADC(R8_C), // x9
    ADC(R8_D), // xA
    ADC(R8_E), // xB
    ADC(R8_H), // xC
    ADC(R8_L), // xD
    ADC(M_HL), // xE
    ADC(R8_A), // xF
    // 9x
    SUB(R8_B), // x0
    SUB(R8_C), // x1
    SUB(R8_D), // x2
    SUB(R8_E), // x3
    SUB(R8_H), // x4
    SUB(R8_L), // x5
    SUB(M_HL), // x6
    SUB(R8_A), // x7
    SBC(R8_B), // x8
    SBC(R8_C), // x9
    SBC(R8_D), // xA
    SBC(R8_E), // xB
    SBC(R8_H), // xC
    SBC(R8_L), // xD
    SBC(M_HL), // xE
    SBC(R8_A), // xF
    // Ax
    AND(R8_B), // x0
    AND(R8_C), // x1
    AND(R8_D), // x2
    AND(R8_E), // x3
    AND(R8_H), // x4
    AND(R8_L), // x5
    AND(M_HL), // x6
    AND(R8_A), // x7
    XOR(R8_B), // x8
    XOR(R8_C), // x9
    XOR(R8_D), // xA
    XOR(R8_E), // xB
    XOR(R8_H), // xC
    XOR(R8_L), // xD
    XOR(M_HL), // xE
    XOR(R8_A), // xF
    // Bx
    OR(R8_B), // x0
    OR(R8_C), // x1
    OR(R8_D), // x2
    OR(R8_E), // x3
    OR(R8_H), // x4
    OR(R8_L), // x5
    OR(M_HL), // x6
    OR(R8_A), // x7
    CP(R8_B), // x8
    CP(R8_C), // x9
    CP(R8_D), // xA
    CP(R8_E), // xB
    CP(R8_H), // xC
    CP(R8_L), // xD
    CP(M_HL), // xE
    CP(R8_A), // xF
    // Cx
    RET(C_NZ),           // x0
    POP(R16_BC),         // x1
    JP(C_NZ, IMM_16),    // x2
    JP(C_A, IMM_16),     // x3
    CALL(C_NZ, IMM_16),  // x4
    PUSH(R16_BC),        // x5
    ADD(IMM_8),          // x6
    RST(CONST_16(0x00)), // x7
    RET(C_Z),            // x8
    RET(C_A),            // x9
    JP(C_Z, IMM_16),     // xA
    PREFIX,              // xB
    CALL(C_Z, IMM_16),   // xC
    CALL(C_A, IMM_16),   // xD
    ADC(IMM_8),          // xE
    RST(CONST_16(0x08)), // xF
    // Dx
    RET(C_NC),           // x0
    POP(R16_DE),         // x1
    JP(C_NC, IMM_16),    // x2
    INVALID,             // x3
    CALL(C_NC, IMM_16),  // x4
    PUSH(R16_DE),        // x5
    SUB(IMM_8),          // x6
    RST(CONST_16(0x10)), // x7
    RET(C_C),            // x8
    RETI,                // x9
    JP(C_C, IMM_16),     // xA
    INVALID,             // xB
    CALL(C_C, IMM_16),   // xC
    INVALID,             // xD
    SBC(IMM_8),          // xE
    RST(CONST_16(0x18)), // xF
    // Ex
    LDH(IMM_8, R8_A),        // x0
    POP(R16_HL),             // x1
    LDH(R8_C, R8_A),         // x2
    INVALID,                 // x3
    INVALID,                 // x4
    PUSH(R16_HL),            // x5
    AND(IMM_8),              // x6
    RST(CONST_16(0x20)),     // x7
    ADD_STK(R16_SP, IMM_i8), // x8
    JP(C_A, M_HL),           // x9
    LD(IMM_16, R8_A),        // xA
    INVALID,                 // xB
    INVALID,                 // xC
    INVALID,                 // xD
    XOR(IMM_8),              // xE
    RST(CONST_16(0x28)),     // xF
    // Fx
    LDH(R8_A, IMM_8),    // x0
    POP(R16_AF),         // x1
    LDH(R8_A, R8_C),     // x2
    DI,                  // x3
    INVALID,             // x4
    PUSH(R16_AF),        // x5
    OR(IMM_8),           // x6
    RST(CONST_16(0x30)), // x7
    LD(R16_HL, IMM_i8),  // x8
    LD(R16_SP, R16_HL),  // x9
    LD(R8_A, IMM_16),    // xA
    EI,                  // xB
    INVALID,             // xC
    INVALID,             // xD
    CP(IMM_8),           // xE
    RST(CONST_16(0x38)), // xF
];

pub const PREFIX_TABLE: [Instruction; 0x100] = [
    // 0x
    NOP, // x0 - RLC B
    NOP, // x1 - RLC C
    NOP, // x2 - RLC D
    NOP, // x3 - RLC E
    NOP, // x4 - RLC H
    NOP, // x5 - RLC L
    NOP, // x6 - RLC [HL]
    NOP, // x7 - RLC A
    NOP, // x8 - RRC B
    NOP, // x9 - RRC C
    NOP, // xA - RRC D
    NOP, // xB - RRC E
    NOP, // xC - RRC H
    NOP, // xD - RRC L
    NOP, // xE - RRC [HL]
    NOP, // xF - RRC A
    // 1x
    NOP, // x0 - RL B
    NOP, // x1 - RL C
    NOP, // x2 - RL D
    NOP, // x3 - RL E
    NOP, // x4 - RL H
    NOP, // x5 - RL L
    NOP, // x6 - RL [HL]
    NOP, // x7 - RL A
    NOP, // x8 - RR B
    NOP, // x9 - RR C
    NOP, // xA - RR D
    NOP, // xB - RR E
    NOP, // xC - RR H
    NOP, // xD - RR L
    NOP, // xE - RR [HL]
    NOP, // xF - RR A
    // 2x
    NOP, // x0 - SLA B
    NOP, // x1 - SLA C
    NOP, // x2 - SLA D
    NOP, // x3 - SLA E
    NOP, // x4 - SLA H
    NOP, // x5 - SLA L
    NOP, // x6 - SLA [HL]
    NOP, // x7 - SLA A
    NOP, // x8 - SRA B
    NOP, // x9 - SRA C
    NOP, // xA - SRA D
    NOP, // xB - SRA E
    NOP, // xC - SRA H
    NOP, // xD - SRA L
    NOP, // xE - SRA [HL]
    NOP, // xF - SRA A
    // 3x
    NOP, // x0 - SWAP B
    NOP, // x1 - SWAP C
    NOP, // x2 - SWAP D
    NOP, // x3 - SWAP E
    NOP, // x4 - SWAP H
    NOP, // x5 - SWAP L
    NOP, // x6 - SWAP [HL]
    NOP, // x7 - SWAP A
    NOP, // x8 - SRL B
    NOP, // x9 - SRL C
    NOP, // xA - SRL D
    NOP, // xB - SRL E
    NOP, // xC - SRL H
    NOP, // xD - SRL L
    NOP, // xE - SRL [HL]
    NOP, // xF - SRL A
    // 4x
    NOP, // x0 - BIT 0, B
    NOP, // x1 - BIT 0, C
    NOP, // x2 - BIT 0, D
    NOP, // x3 - BIT 0, E
    NOP, // x4 - BIT 0, H
    NOP, // x5 - BIT 0, L
    NOP, // x6 - BIT 0, [HL]
    NOP, // x7 - BIT 0, A
    NOP, // x8 - BIT 1, B
    NOP, // x9 - BIT 1, C
    NOP, // xA - BIT 1, D
    NOP, // xB - BIT 1, E
    NOP, // xC - BIT 1, H
    NOP, // xD - BIT 1, L
    NOP, // xE - BIT 1, [HL]
    NOP, // xF - BIT 1, A
    // 5x
    NOP, // x0 - BIT 2, B
    NOP, // x1 - BIT 2, C
    NOP, // x2 - BIT 2, D
    NOP, // x3 - BIT 2, E
    NOP, // x4 - BIT 2, H
    NOP, // x5 - BIT 2, L
    NOP, // x6 - BIT 2, [HL]
    NOP, // x7 - BIT 2, A
    NOP, // x8 - BIT 3, B
    NOP, // x9 - BIT 3, C
    NOP, // xA - BIT 3, D
    NOP, // xB - BIT 3, E
    NOP, // xC - BIT 3, H
    NOP, // xD - BIT 3, L
    NOP, // xE - BIT 3, [HL]
    NOP, // xF - BIT 3, A
    // 6x
    NOP, // x0 - BIT 4, B
    NOP, // x1 - BIT 4, C
    NOP, // x2 - BIT 4, D
    NOP, // x3 - BIT 4, E
    NOP, // x4 - BIT 4, H
    NOP, // x5 - BIT 4, L
    NOP, // x6 - BIT 4, [HL]
    NOP, // x7 - BIT 4, A
    NOP, // x8 - BIT 5, B
    NOP, // x9 - BIT 5, C
    NOP, // xA - BIT 5, D
    NOP, // xB - BIT 5, E
    NOP, // xC - BIT 5, H
    NOP, // xD - BIT 5, L
    NOP, // xE - BIT 5, [HL]
    NOP, // xF - BIT 5, A
    // 7x
    NOP, // x0 - BIT 6, B
    NOP, // x1 - BIT 6, C
    NOP, // x2 - BIT 6, D
    NOP, // x3 - BIT 6, E
    NOP, // x4 - BIT 6, H
    NOP, // x5 - BIT 6, L
    NOP, // x6 - BIT 6, [HL]
    NOP, // x7 - BIT 6, A
    NOP, // x8 - BIT 7, B
    NOP, // x9 - BIT 7, C
    NOP, // xA - BIT 7, D
    NOP, // xB - BIT 7, E
    NOP, // xC - BIT 7, H
    NOP, // xD - BIT 7, L
    NOP, // xE - BIT 7, [HL]
    NOP, // xF - BIT 7, A
    // 8x
    NOP, // x0 - RES 0, B
    NOP, // x1 - RES 0, C
    NOP, // x2 - RES 0, D
    NOP, // x3 - RES 0, E
    NOP, // x4 - RES 0, H
    NOP, // x5 - RES 0, L
    NOP, // x6 - RES 0, [HL]
    NOP, // x7 - RES 0, A
    NOP, // x8 - RES 1, B
    NOP, // x9 - RES 1, C
    NOP, // xA - RES 1, D
    NOP, // xB - RES 1, E
    NOP, // xC - RES 1, H
    NOP, // xD - RES 1, L
    NOP, // xE - RES 1, [HL]
    NOP, // xF - RES 1, A
    // 9x
    NOP, // x0 - RES 2, B
    NOP, // x1 - RES 2, C
    NOP, // x2 - RES 2, D
    NOP, // x3 - RES 2, E
    NOP, // x4 - RES 2, H
    NOP, // x5 - RES 2, L
    NOP, // x6 - RES 2, [HL]
    NOP, // x7 - RES 2, A
    NOP, // x8 - RES 3, B
    NOP, // x9 - RES 3, C
    NOP, // xA - RES 3, D
    NOP, // xB - RES 3, E
    NOP, // xC - RES 3, H
    NOP, // xD - RES 3, L
    NOP, // xE - RES 3, [HL]
    NOP, // xF - RES 3, A
    // Ax
    NOP, // x0 - RES 4, B
    NOP, // x1 - RES 4, C
    NOP, // x2 - RES 4, D
    NOP, // x3 - RES 4, E
    NOP, // x4 - RES 4, H
    NOP, // x5 - RES 4, L
    NOP, // x6 - RES 4, [HL]
    NOP, // x7 - RES 4, A
    NOP, // x8 - RES 5, B
    NOP, // x9 - RES 5, C
    NOP, // xA - RES 5, D
    NOP, // xB - RES 5, E
    NOP, // xC - RES 5, H
    NOP, // xD - RES 5, L
    NOP, // xE - RES 5, [HL]
    NOP, // xF - RES 5, A
    // Bx
    NOP, // x0 - RES 6, B
    NOP, // x1 - RES 6, C
    NOP, // x2 - RES 6, D
    NOP, // x3 - RES 6, E
    NOP, // x4 - RES 6, H
    NOP, // x5 - RES 6, L
    NOP, // x6 - RES 6, [HL]
    NOP, // x7 - RES 6, A
    NOP, // x8 - RES 7, B
    NOP, // x9 - RES 7, C
    NOP, // xA - RES 7, D
    NOP, // xB - RES 7, E
    NOP, // xC - RES 7, H
    NOP, // xD - RES 7, L
    NOP, // xE - RES 7, [HL]
    NOP, // xF - RES 7, A
    // Cx
    NOP, // x0 - SET 0, B
    NOP, // x1 - SET 0, C
    NOP, // x2 - SET 0, D
    NOP, // x3 - SET 0, E
    NOP, // x4 - SET 0, H
    NOP, // x5 - SET 0, L
    NOP, // x6 - SET 0, [HL]
    NOP, // x7 - SET 0, A
    NOP, // x8 - SET 1, B
    NOP, // x9 - SET 1, C
    NOP, // xA - SET 1, D
    NOP, // xB - SET 1, E
    NOP, // xC - SET 1, H
    NOP, // xD - SET 1, L
    NOP, // xE - SET 1, [HL]
    NOP, // xF - SET 1, A
    // Dx
    NOP, // x0 - SET 2, B
    NOP, // x1 - SET 2, C
    NOP, // x2 - SET 2, D
    NOP, // x3 - SET 2, E
    NOP, // x4 - SET 2, H
    NOP, // x5 - SET 2, L
    NOP, // x6 - SET 2, [HL]
    NOP, // x7 - SET 2, A
    NOP, // x8 - SET 3, B
    NOP, // x9 - SET 3, C
    NOP, // xA - SET 3, D
    NOP, // xB - SET 3, E
    NOP, // xC - SET 3, H
    NOP, // xD - SET 3, L
    NOP, // xE - SET 3, [HL]
    NOP, // xF - SET 3, A
    // Ex
    NOP, // x0 - SET 4, B
    NOP, // x1 - SET 4, C
    NOP, // x2 - SET 4, D
    NOP, // x3 - SET 4, E
    NOP, // x4 - SET 4, H
    NOP, // x5 - SET 4, L
    NOP, // x6 - SET 4, [HL]
    NOP, // x7 - SET 4, A
    NOP, // x8 - SET 5, B
    NOP, // x9 - SET 5, C
    NOP, // xA - SET 5, D
    NOP, // xB - SET 5, E
    NOP, // xC - SET 5, H
    NOP, // xD - SET 5, L
    NOP, // xE - SET 5, [HL]
    NOP, // xF - SET 5, A
    // Fx
    NOP, // x0 - SET 6, B
    NOP, // x1 - SET 6, C
    NOP, // x2 - SET 6, D
    NOP, // x3 - SET 6, E
    NOP, // x4 - SET 6, H
    NOP, // x5 - SET 6, L
    NOP, // x6 - SET 6, [HL]
    NOP, // x7 - SET 6, A
    NOP, // x8 - SET 7, B
    NOP, // x9 - SET 7, C
    NOP, // xA - SET 7, D
    NOP, // xB - SET 7, E
    NOP, // xC - SET 7, H
    NOP, // xD - SET 7, L
    NOP, // xE - SET 7, [HL]
    NOP, // xF - SET 7, A
];
