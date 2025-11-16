use super::instruction::{Arg::*, Instruction, Instruction::*};

pub const OP_TABLE: [Instruction; 0x100] = [
    // 0x
    NOP,                // x0
    LD(R16_BC, IMM_16), // x1*
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

pub const PREFIX_TABLE: [Instruction; 256] = [
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
];
