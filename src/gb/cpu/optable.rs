use super::instruction::{Arg::*, Instruction, Instruction::*, MetaInstruction::*};

pub const OP_TABLE: [Instruction; 0x100] = [
    // 0x
    NOP,                   // x0 - NOP
    LD_16(R16_BC, IMM_16), // x1 - LD BC, n16
    LD(M_BC, R8_A),        // x2 - LD [BC], A
    INC_16(R16_BC),        // x3 - INC BC
    INC(R8_B),             // x4 - INC B
    DEC(R8_B),             // x5 - DEC B
    LD(R8_B, IMM_8),       // x6 - LD B, n8
    RLCA,                  // x7 - RLCA
    LD_16(IMM_16, R16_SP), // x8 - LD [a16], SP
    ADD_16(R16_BC),        // x9 - ADD HL, BC
    LD(R8_A, M_BC),        // xA - LD A, [BC]
    DEC_16(R16_BC),        // xB - DEC BC
    INC(R8_C),             // xC - INC C
    DEC(R8_C),             // xD - DEC C
    LD(R8_C, IMM_8),       // xE - LD C, n8
    RRCA,                  // xF - RRCA
    // 1x
    STOP(IMM_8),           // x0 - STOP n8
    LD_16(R16_DE, IMM_16), // x1 - LD DE, n16
    LD(M_DE, R8_A),        // x2 - LD [DE], A
    INC_16(R16_DE),        // x3 - INC DE
    INC(R8_D),             // x4 - INC D
    DEC(R8_D),             // x5 - DEC D
    LD(R8_D, IMM_8),       // x6 - LD D, n8
    RLA,                   // x7 - RLA
    JR(C_A, IMM_i8),       // x8 - JR e8
    ADD_16(R16_DE),        // x9 - ADD HL, DE
    LD(R8_A, M_DE),        // xA - LD A, [DE]
    DEC_16(R16_DE),        // xB - DEC DE
    INC(R8_E),             // xC - INC E
    DEC(R8_E),             // xD - DEC E
    LD(R8_E, IMM_8),       // xE - LD E, n8
    RRA,                   // xF - RRA
    // 2x
    JR(C_NZ, IMM_i8),      // x0 - JR NZ, e8
    LD_16(R16_HL, IMM_16), // x1 - LD HL, n16
    LD(M_HLI, R8_A),       // x2 - LD [HL+], A
    INC_16(R16_HL),        // x3 - INC HL
    INC(R8_H),             // x4 - INC H
    DEC(R8_H),             // x5 - DEC H
    LD(R8_H, IMM_8),       // x6 - LD H, n8
    DAA,                   // x7 - DAA
    JR(C_Z, IMM_i8),       // x8 - JR Z, e8
    ADD_16(R16_HL),        // x9 - ADD HL, HL
    LD(R8_A, M_HLI),       // xA - LD A, [HL+]
    DEC_16(R16_HL),        // xB - DEC HL
    INC(R8_L),             // xC - INC L
    DEC(R8_L),             // xD - DEC L
    LD(R8_L, IMM_8),       // xE - LD L, n8
    CPL,                   // xF - CPL
    // 3x
    JR(C_NC, IMM_i8),      // x0 - JR NC, e8
    LD_16(R16_SP, IMM_16), // x1 - LD SP, n16
    LD(M_HLD, R8_A),       // x2 - LD [HL-], A
    INC_16(R16_SP),        // x3 - INC SP
    INC(M_HL),             // x4 - INC [HL]
    DEC(M_HL),             // x5 - DEC [HL]
    LD(M_HL, IMM_8),       // x6 - LD [HL], n8
    SCF,                   // x7 - SCF
    JR(C_C, IMM_i8),       // x8 - JR C, e8
    ADD_16(R16_SP),        // x9 - ADD HL, SP
    LD(R8_A, M_HLD),       // xA - LD A, [HL-]
    DEC_16(R16_SP),        // xB - DEC SP
    INC(R8_A),             // xC - INC A
    DEC(R8_A),             // xD - DEC A
    LD(R8_A, IMM_8),       // xE - LD A, n8
    CCF,                   // xF - CCF
    // 4x
    LD(R8_B, R8_B), // x0 - LD B, B
    LD(R8_B, R8_C), // x1 - LD B, C
    LD(R8_B, R8_D), // x2 - LD B, D
    LD(R8_B, R8_E), // x3 - LD B, E
    LD(R8_B, R8_H), // x4 - LD B, H
    LD(R8_B, R8_L), // x5 - LD B, L
    LD(R8_B, M_HL), // x6 - LD B, [HL]
    LD(R8_B, R8_A), // x7 - LD B, A
    LD(R8_C, R8_B), // x8 - LD C, B
    LD(R8_C, R8_C), // x9 - LD C, C
    LD(R8_C, R8_D), // xA - LD C, D
    LD(R8_C, R8_E), // xB - LD C, E
    LD(R8_C, R8_H), // xC - LD C, H
    LD(R8_C, R8_L), // xD - LD C, L
    LD(R8_C, M_HL), // xE - LD C, [HL]
    LD(R8_C, R8_A), // xF - LD C, A
    // 5x
    LD(R8_D, R8_B), // x0 - LD D, B
    LD(R8_D, R8_C), // x1 - LD D, C
    LD(R8_D, R8_D), // x2 - LD D, D
    LD(R8_D, R8_E), // x3 - LD D, E
    LD(R8_D, R8_H), // x4 - LD D, H
    LD(R8_D, R8_L), // x5 - LD D, L
    LD(R8_D, M_HL), // x6 - LD D, [HL]
    LD(R8_D, R8_A), // x7 - LD D, A
    LD(R8_E, R8_B), // x8 - LD E, B
    LD(R8_E, R8_C), // x9 - LD E, C
    LD(R8_E, R8_D), // xA - LD E, D
    LD(R8_E, R8_E), // xB - LD E, E
    LD(R8_E, R8_H), // xC - LD E, H
    LD(R8_E, R8_L), // xD - LD E, L
    LD(R8_E, M_HL), // xE - LD E, [HL]
    LD(R8_E, R8_A), // xF - LD E, A
    // 6x
    LD(R8_H, R8_B), // x0 - LD H, B
    LD(R8_H, R8_C), // x1 - LD H, C
    LD(R8_H, R8_D), // x2 - LD H, D
    LD(R8_H, R8_E), // x3 - LD H, E
    LD(R8_H, R8_H), // x4 - LD H, H
    LD(R8_H, R8_L), // x5 - LD H, L
    LD(R8_H, M_HL), // x6 - LD H, [HL]
    LD(R8_H, R8_A), // x7 - LD H, A
    LD(R8_L, R8_B), // x8 - LD L, B
    LD(R8_L, R8_C), // x9 - LD L, C
    LD(R8_L, R8_D), // xA - LD L, D
    LD(R8_L, R8_E), // xB - LD L, E
    LD(R8_L, R8_H), // xC - LD L, H
    LD(R8_L, R8_L), // xD - LD L, L
    LD(R8_L, M_HL), // xE - LD L, [HL]
    LD(R8_L, R8_A), // xF - LD L, A
    // 7x
    LD(M_HL, R8_B), // x0 - LD [HL], B
    LD(M_HL, R8_C), // x1 - LD [HL], C
    LD(M_HL, R8_D), // x2 - LD [HL], D
    LD(M_HL, R8_E), // x3 - LD [HL], E
    LD(M_HL, R8_H), // x4 - LD [HL], H
    LD(M_HL, R8_L), // x5 - LD [HL], L
    HALT,           // x6 - HALT
    LD(M_HL, R8_A), // x7 - LD [HL], A
    LD(R8_A, R8_B), // x8 - LD A, B
    LD(R8_A, R8_C), // x9 - LD A, C
    LD(R8_A, R8_D), // xA - LD A, D
    LD(R8_A, R8_E), // xB - LD A, E
    LD(R8_A, R8_H), // xC - LD A, H
    LD(R8_A, R8_L), // xD - LD A, L
    LD(R8_A, M_HL), // xE - LD A, [HL]
    LD(R8_A, R8_A), // xF - LD A, A
    // 8x
    ADD(R8_B), // x0 - ADD A, B
    ADD(R8_C), // x1 - ADD A, C
    ADD(R8_D), // x2 - ADD A, D
    ADD(R8_E), // x3 - ADD A, E
    ADD(R8_H), // x4 - ADD A, H
    ADD(R8_L), // x5 - ADD A, L
    ADD(M_HL), // x6 - ADD A, [HL]
    ADD(R8_A), // x7 - ADD A, A
    ADC(R8_B), // x8 - ADC A, B
    ADC(R8_C), // x9 - ADC A, C
    ADC(R8_D), // xA - ADC A, D
    ADC(R8_E), // xB - ADC A, E
    ADC(R8_H), // xC - ADC A, H
    ADC(R8_L), // xD - ADC A, L
    ADC(M_HL), // xE - ADC A, [HL]
    ADC(R8_A), // xF - ADC A, A
    // 9x
    SUB(R8_B), // x0 - SUB A, B
    SUB(R8_C), // x1 - SUB A, C
    SUB(R8_D), // x2 - SUB A, D
    SUB(R8_E), // x3 - SUB A, E
    SUB(R8_H), // x4 - SUB A, H
    SUB(R8_L), // x5 - SUB A, L
    SUB(M_HL), // x6 - SUB A, [HL]
    SUB(R8_A), // x7 - SUB A, A
    SBC(R8_B), // x8 - SBC A, B
    SBC(R8_C), // x9 - SBC A, C
    SBC(R8_D), // xA - SBC A, D
    SBC(R8_E), // xB - SBC A, E
    SBC(R8_H), // xC - SBC A, H
    SBC(R8_L), // xD - SBC A, L
    SBC(M_HL), // xE - SBC A, [HL]
    SBC(R8_A), // xF - SBC A, A
    // Ax
    AND(R8_B), // x0 - AND A, B
    AND(R8_C), // x1 - AND A, C
    AND(R8_D), // x2 - AND A, D
    AND(R8_E), // x3 - AND A, E
    AND(R8_H), // x4 - AND A, H
    AND(R8_L), // x5 - AND A, L
    AND(M_HL), // x6 - AND A, [HL]
    AND(R8_A), // x7 - AND A, A
    XOR(R8_B), // x8 - XOR A, B
    XOR(R8_C), // x9 - XOR A, C
    XOR(R8_D), // xA - XOR A, D
    XOR(R8_E), // xB - XOR A, E
    XOR(R8_H), // xC - XOR A, H
    XOR(R8_L), // xD - XOR A, L
    XOR(M_HL), // xE - XOR A, [HL]
    XOR(R8_A), // xF - XOR A, A
    // Bx
    OR(R8_B), // x0 - OR A, B
    OR(R8_C), // x1 - OR A, C
    OR(R8_D), // x2 - OR A, D
    OR(R8_E), // x3 - OR A, E
    OR(R8_H), // x4 - OR A, H
    OR(R8_L), // x5 - OR A, L
    OR(M_HL), // x6 - OR A, [HL]
    OR(R8_A), // x7 - OR A, A
    CP(R8_B), // x8 - CP A, B
    CP(R8_C), // x9 - CP A, C
    CP(R8_D), // xA - CP A, D
    CP(R8_E), // xB - CP A, E
    CP(R8_H), // xC - CP A, H
    CP(R8_L), // xD - CP A, L
    CP(M_HL), // xE - CP A, [HL]
    CP(R8_A), // xF - CP A, A
    // Cx
    RET(C_NZ),           // x0 - RET NZ
    POP(R16_BC),         // x1 - POP BC
    JP(C_NZ, IMM_16),    // x2 - JP NZ, a16
    JP(C_A, IMM_16),     // x3 - JP a16
    CALL(C_NZ, IMM_16),  // x4 - CALL NZ, a16
    PUSH(R16_BC),        // x5 - PUSH BC
    ADD(IMM_8),          // x6 - ADD A, n8
    RST(CONST_16(0x00)), // x7 - RST $00
    RET(C_Z),            // x8 - RET Z
    RET(C_A),            // x9 - RET
    JP(C_Z, IMM_16),     // xA - JP Z, a16
    PREFIX,              // xB - PREFIX
    CALL(C_Z, IMM_16),   // xC - CALL Z, a16
    CALL(C_A, IMM_16),   // xD - CALL a16
    ADC(IMM_8),          // xE - ADC A, n8
    RST(CONST_16(0x08)), // xF - RST $08
    // Dx
    RET(C_NC),           // x0 - RET NC
    POP(R16_DE),         // x1 - POP DE
    JP(C_NC, IMM_16),    // x2 - JP NC, a16
    INVALID(SHOW_CPU),   // x3 - INVALID (Meta-instruction: Print state of the CPU)
    CALL(C_NC, IMM_16),  // x4 - CALL NC, a16
    PUSH(R16_DE),        // x5 - PUSH DE
    SUB(IMM_8),          // x6 - SUB A, n8
    RST(CONST_16(0x10)), // x7 - RST $10
    RET(C_C),            // x8 - RET C
    RETI,                // x9 - RETI
    JP(C_C, IMM_16),     // xA - JP C, a16
    INVALID(TERMINATE),  // xB - INVALID (Meta-instruction: Terminate the emulator)
    CALL(C_C, IMM_16),   // xC - CALL C, a16
    INVALID(DUMP),       // xD - INVALID (Meta-instruction: Full state dump to file)
    SBC(IMM_8),          // xE - SBC A, n8
    RST(CONST_16(0x18)), // xF - RST $18
    // Ex
    LDH(IMM_8, R8_A),        // x0 - LDH [a8], A
    POP(R16_HL),             // x1 - POP HL
    LDH(R8_C, R8_A),         // x2 - LDH [C], A
    INVALID(NONE),           // x3 - INVALID
    INVALID(NONE),           // x4 - INVALID
    PUSH(R16_HL),            // x5 - PUSH HL
    AND(IMM_8),              // x6 - AND A, n8
    RST(CONST_16(0x20)),     // x7 - RST $20
    ADD_STK(R16_SP, IMM_i8), // x8 - ADD SP, e8
    JP(C_A, M_HL),           // x9 - JP HL
    LD(IMM_16, R8_A),        // xA - LD [a16], A
    INVALID(NONE),           // xB - INVALID
    INVALID(NONE),           // xC - INVALID
    INVALID(NONE),           // xD - INVALID
    XOR(IMM_8),              // xE - XOR A, n8
    RST(CONST_16(0x28)),     // xF - RST $28
    // Fx
    LDH(R8_A, IMM_8),            // x0 - LDH A, [a8]
    POP(R16_AF),                 // x1 - POP AF
    LDH(R8_A, R8_C),             // x2 - LDH A, [C]
    DI,                          // x3 - DI
    INVALID(NONE),               // x4 - INVALID
    PUSH(R16_AF),                // x5 - PUSH AF
    OR(IMM_8),                   // x6 - OR A, n8
    RST(CONST_16(0x30)),         // x7 - RST $30
    LD_HL_SP_E8(R16_HL, IMM_i8), // x8 - LD HL, SP + e8
    LD_16(R16_SP, R16_HL),       // x9 - LD SP, HL
    LD(R8_A, IMM_16),            // xA - LD A, [a16]
    EI,                          // xB - EI
    INVALID(NONE),               // xC - INVALID
    INVALID(NONE),               // xD - INVALID
    CP(IMM_8),                   // xE - CP A, n8
    RST(CONST_16(0x38)),         // xF - RST $38
];

pub const PREFIX_TABLE: [Instruction; 0x100] = [
    // 0x
    RLC(R8_B), // x0 - RLC B
    RLC(R8_C), // x1 - RLC C
    RLC(R8_D), // x2 - RLC D
    RLC(R8_E), // x3 - RLC E
    RLC(R8_H), // x4 - RLC H
    RLC(R8_L), // x5 - RLC L
    RLC(M_HL), // x6 - RLC [HL]
    RLC(R8_A), // x7 - RLC A
    RRC(R8_B), // x8 - RRC B
    RRC(R8_C), // x9 - RRC C
    RRC(R8_D), // xA - RRC D
    RRC(R8_E), // xB - RRC E
    RRC(R8_H), // xC - RRC H
    RRC(R8_L), // xD - RRC L
    RRC(M_HL), // xE - RRC [HL]
    RRC(R8_A), // xF - RRC A
    // 1x
    RL(R8_B), // x0 - RL B
    RL(R8_C), // x1 - RL C
    RL(R8_D), // x2 - RL D
    RL(R8_E), // x3 - RL E
    RL(R8_H), // x4 - RL H
    RL(R8_L), // x5 - RL L
    RL(M_HL), // x6 - RL [HL]
    RL(R8_A), // x7 - RL A
    RR(R8_B), // x8 - RR B
    RR(R8_C), // x9 - RR C
    RR(R8_D), // xA - RR D
    RR(R8_E), // xB - RR E
    RR(R8_H), // xC - RR H
    RR(R8_L), // xD - RR L
    RR(M_HL), // xE - RR [HL]
    RR(R8_A), // xF - RR A
    // 2x
    SLA(R8_B), // x0 - SLA B
    SLA(R8_C), // x1 - SLA C
    SLA(R8_D), // x2 - SLA D
    SLA(R8_E), // x3 - SLA E
    SLA(R8_H), // x4 - SLA H
    SLA(R8_L), // x5 - SLA L
    SLA(M_HL), // x6 - SLA [HL]
    SLA(R8_A), // x7 - SLA A
    SRA(R8_B), // x8 - SRA B
    SRA(R8_C), // x9 - SRA C
    SRA(R8_D), // xA - SRA D
    SRA(R8_E), // xB - SRA E
    SRA(R8_H), // xC - SRA H
    SRA(R8_L), // xD - SRA L
    SRA(M_HL), // xE - SRA [HL]
    SRA(R8_A), // xF - SRA A
    // 3x
    SWAP(R8_B), // x0 - SWAP B
    SWAP(R8_C), // x1 - SWAP C
    SWAP(R8_D), // x2 - SWAP D
    SWAP(R8_E), // x3 - SWAP E
    SWAP(R8_H), // x4 - SWAP H
    SWAP(R8_L), // x5 - SWAP L
    SWAP(M_HL), // x6 - SWAP [HL]
    SWAP(R8_A), // x7 - SWAP A
    SRL(R8_B),  // x8 - SRL B
    SRL(R8_C),  // x9 - SRL C
    SRL(R8_D),  // xA - SRL D
    SRL(R8_E),  // xB - SRL E
    SRL(R8_H),  // xC - SRL H
    SRL(R8_L),  // xD - SRL L
    SRL(M_HL),  // xE - SRL [HL]
    SRL(R8_A),  // xF - SRL A
    // 4x
    BIT(CONST_8(0), R8_B), // x0 - BIT 0, B
    BIT(CONST_8(0), R8_C), // x1 - BIT 0, C
    BIT(CONST_8(0), R8_D), // x2 - BIT 0, D
    BIT(CONST_8(0), R8_E), // x3 - BIT 0, E
    BIT(CONST_8(0), R8_H), // x4 - BIT 0, H
    BIT(CONST_8(0), R8_L), // x5 - BIT 0, L
    BIT(CONST_8(0), M_HL), // x6 - BIT 0, [HL]
    BIT(CONST_8(0), R8_A), // x7 - BIT 0, A
    BIT(CONST_8(1), R8_B), // x8 - BIT 1, B
    BIT(CONST_8(1), R8_C), // x9 - BIT 1, C
    BIT(CONST_8(1), R8_D), // xA - BIT 1, D
    BIT(CONST_8(1), R8_E), // xB - BIT 1, E
    BIT(CONST_8(1), R8_H), // xC - BIT 1, H
    BIT(CONST_8(1), R8_L), // xD - BIT 1, L
    BIT(CONST_8(1), M_HL), // xE - BIT 1, [HL]
    BIT(CONST_8(1), R8_A), // xF - BIT 1, A
    // 5x
    BIT(CONST_8(2), R8_B), // x0 - BIT 2, B
    BIT(CONST_8(2), R8_C), // x1 - BIT 2, C
    BIT(CONST_8(2), R8_D), // x2 - BIT 2, D
    BIT(CONST_8(2), R8_E), // x3 - BIT 2, E
    BIT(CONST_8(2), R8_H), // x4 - BIT 2, H
    BIT(CONST_8(2), R8_L), // x5 - BIT 2, L
    BIT(CONST_8(2), M_HL), // x6 - BIT 2, [HL]
    BIT(CONST_8(2), R8_A), // x7 - BIT 2, A
    BIT(CONST_8(3), R8_B), // x8 - BIT 3, B
    BIT(CONST_8(3), R8_C), // x9 - BIT 3, C
    BIT(CONST_8(3), R8_D), // xA - BIT 3, D
    BIT(CONST_8(3), R8_E), // xB - BIT 3, E
    BIT(CONST_8(3), R8_H), // xC - BIT 3, H
    BIT(CONST_8(3), R8_L), // xD - BIT 3, L
    BIT(CONST_8(3), M_HL), // xE - BIT 3, [HL]
    BIT(CONST_8(3), R8_A), // xF - BIT 3, A
    // 6x
    BIT(CONST_8(4), R8_B), // x0 - BIT 4, B
    BIT(CONST_8(4), R8_C), // x1 - BIT 4, C
    BIT(CONST_8(4), R8_D), // x2 - BIT 4, D
    BIT(CONST_8(4), R8_E), // x3 - BIT 4, E
    BIT(CONST_8(4), R8_H), // x4 - BIT 4, H
    BIT(CONST_8(4), R8_L), // x5 - BIT 4, L
    BIT(CONST_8(4), M_HL), // x6 - BIT 4, [HL]
    BIT(CONST_8(4), R8_A), // x7 - BIT 4, A
    BIT(CONST_8(5), R8_B), // x8 - BIT 5, B
    BIT(CONST_8(5), R8_C), // x9 - BIT 5, C
    BIT(CONST_8(5), R8_D), // xA - BIT 5, D
    BIT(CONST_8(5), R8_E), // xB - BIT 5, E
    BIT(CONST_8(5), R8_H), // xC - BIT 5, H
    BIT(CONST_8(5), R8_L), // xD - BIT 5, L
    BIT(CONST_8(5), M_HL), // xE - BIT 5, [HL]
    BIT(CONST_8(5), R8_A), // xF - BIT 5, A
    // 7x
    BIT(CONST_8(6), R8_B), // x0 - BIT 6, B
    BIT(CONST_8(6), R8_C), // x1 - BIT 6, C
    BIT(CONST_8(6), R8_D), // x2 - BIT 6, D
    BIT(CONST_8(6), R8_E), // x3 - BIT 6, E
    BIT(CONST_8(6), R8_H), // x4 - BIT 6, H
    BIT(CONST_8(6), R8_L), // x5 - BIT 6, L
    BIT(CONST_8(6), M_HL), // x6 - BIT 6, [HL]
    BIT(CONST_8(6), R8_A), // x7 - BIT 6, A
    BIT(CONST_8(7), R8_B), // x8 - BIT 7, B
    BIT(CONST_8(7), R8_C), // x9 - BIT 7, C
    BIT(CONST_8(7), R8_D), // xA - BIT 7, D
    BIT(CONST_8(7), R8_E), // xB - BIT 7, E
    BIT(CONST_8(7), R8_H), // xC - BIT 7, H
    BIT(CONST_8(7), R8_L), // xD - BIT 7, L
    BIT(CONST_8(7), M_HL), // xE - BIT 7, [HL]
    BIT(CONST_8(7), R8_A), // xF - BIT 7, A
    // 8x
    RES(CONST_8(0), R8_B), // x0 - RES 0, B
    RES(CONST_8(0), R8_C), // x1 - RES 0, C
    RES(CONST_8(0), R8_D), // x2 - RES 0, D
    RES(CONST_8(0), R8_E), // x3 - RES 0, E
    RES(CONST_8(0), R8_H), // x4 - RES 0, H
    RES(CONST_8(0), R8_L), // x5 - RES 0, L
    RES(CONST_8(0), M_HL), // x6 - RES 0, [HL]
    RES(CONST_8(0), R8_A), // x7 - RES 0, A
    RES(CONST_8(1), R8_B), // x8 - RES 1, B
    RES(CONST_8(1), R8_C), // x9 - RES 1, C
    RES(CONST_8(1), R8_D), // xA - RES 1, D
    RES(CONST_8(1), R8_E), // xB - RES 1, E
    RES(CONST_8(1), R8_H), // xC - RES 1, H
    RES(CONST_8(1), R8_L), // xD - RES 1, L
    RES(CONST_8(1), M_HL), // xE - RES 1, [HL]
    RES(CONST_8(1), R8_A), // xF - RES 1, A
    // 9x
    RES(CONST_8(2), R8_B), // x0 - RES 2, B
    RES(CONST_8(2), R8_C), // x1 - RES 2, C
    RES(CONST_8(2), R8_D), // x2 - RES 2, D
    RES(CONST_8(2), R8_E), // x3 - RES 2, E
    RES(CONST_8(2), R8_H), // x4 - RES 2, H
    RES(CONST_8(2), R8_L), // x5 - RES 2, L
    RES(CONST_8(2), M_HL), // x6 - RES 2, [HL]
    RES(CONST_8(2), R8_A), // x7 - RES 2, A
    RES(CONST_8(3), R8_B), // x8 - RES 3, B
    RES(CONST_8(3), R8_C), // x9 - RES 3, C
    RES(CONST_8(3), R8_D), // xA - RES 3, D
    RES(CONST_8(3), R8_E), // xB - RES 3, E
    RES(CONST_8(3), R8_H), // xC - RES 3, H
    RES(CONST_8(3), R8_L), // xD - RES 3, L
    RES(CONST_8(3), M_HL), // xE - RES 3, [HL]
    RES(CONST_8(3), R8_A), // xF - RES 3, A
    // Ax
    RES(CONST_8(4), R8_B), // x0 - RES 4, B
    RES(CONST_8(4), R8_C), // x1 - RES 4, C
    RES(CONST_8(4), R8_D), // x2 - RES 4, D
    RES(CONST_8(4), R8_E), // x3 - RES 4, E
    RES(CONST_8(4), R8_H), // x4 - RES 4, H
    RES(CONST_8(4), R8_L), // x5 - RES 4, L
    RES(CONST_8(4), M_HL), // x6 - RES 4, [HL]
    RES(CONST_8(4), R8_A), // x7 - RES 4, A
    RES(CONST_8(5), R8_B), // x8 - RES 5, B
    RES(CONST_8(5), R8_C), // x9 - RES 5, C
    RES(CONST_8(5), R8_D), // xA - RES 5, D
    RES(CONST_8(5), R8_E), // xB - RES 5, E
    RES(CONST_8(5), R8_H), // xC - RES 5, H
    RES(CONST_8(5), R8_L), // xD - RES 5, L
    RES(CONST_8(5), M_HL), // xE - RES 5, [HL]
    RES(CONST_8(5), R8_A), // xF - RES 5, A
    // Bx
    RES(CONST_8(6), R8_B), // x0 - RES 6, B
    RES(CONST_8(6), R8_C), // x1 - RES 6, C
    RES(CONST_8(6), R8_D), // x2 - RES 6, D
    RES(CONST_8(6), R8_E), // x3 - RES 6, E
    RES(CONST_8(6), R8_H), // x4 - RES 6, H
    RES(CONST_8(6), R8_L), // x5 - RES 6, L
    RES(CONST_8(6), M_HL), // x6 - RES 6, [HL]
    RES(CONST_8(6), R8_A), // x7 - RES 6, A
    RES(CONST_8(7), R8_B), // x8 - RES 7, B
    RES(CONST_8(7), R8_C), // x9 - RES 7, C
    RES(CONST_8(7), R8_D), // xA - RES 7, D
    RES(CONST_8(7), R8_E), // xB - RES 7, E
    RES(CONST_8(7), R8_H), // xC - RES 7, H
    RES(CONST_8(7), R8_L), // xD - RES 7, L
    RES(CONST_8(7), M_HL), // xE - RES 7, [HL]
    RES(CONST_8(7), R8_A), // xF - RES 7, A
    // Cx
    SET(CONST_8(0), R8_B), // x0 - SET 0, B
    SET(CONST_8(0), R8_C), // x1 - SET 0, C
    SET(CONST_8(0), R8_D), // x2 - SET 0, D
    SET(CONST_8(0), R8_E), // x3 - SET 0, E
    SET(CONST_8(0), R8_H), // x4 - SET 0, H
    SET(CONST_8(0), R8_L), // x5 - SET 0, L
    SET(CONST_8(0), M_HL), // x6 - SET 0, [HL]
    SET(CONST_8(0), R8_A), // x7 - SET 0, A
    SET(CONST_8(1), R8_B), // x8 - SET 1, B
    SET(CONST_8(1), R8_C), // x9 - SET 1, C
    SET(CONST_8(1), R8_D), // xA - SET 1, D
    SET(CONST_8(1), R8_E), // xB - SET 1, E
    SET(CONST_8(1), R8_H), // xC - SET 1, H
    SET(CONST_8(1), R8_L), // xD - SET 1, L
    SET(CONST_8(1), M_HL), // xE - SET 1, [HL]
    SET(CONST_8(1), R8_A), // xF - SET 1, A
    // Dx
    SET(CONST_8(2), R8_B), // x0 - SET 2, B
    SET(CONST_8(2), R8_C), // x1 - SET 2, C
    SET(CONST_8(2), R8_D), // x2 - SET 2, D
    SET(CONST_8(2), R8_E), // x3 - SET 2, E
    SET(CONST_8(2), R8_H), // x4 - SET 2, H
    SET(CONST_8(2), R8_L), // x5 - SET 2, L
    SET(CONST_8(2), M_HL), // x6 - SET 2, [HL]
    SET(CONST_8(2), R8_A), // x7 - SET 2, A
    SET(CONST_8(3), R8_B), // x8 - SET 3, B
    SET(CONST_8(3), R8_C), // x9 - SET 3, C
    SET(CONST_8(3), R8_D), // xA - SET 3, D
    SET(CONST_8(3), R8_E), // xB - SET 3, E
    SET(CONST_8(3), R8_H), // xC - SET 3, H
    SET(CONST_8(3), R8_L), // xD - SET 3, L
    SET(CONST_8(3), M_HL), // xE - SET 3, [HL]
    SET(CONST_8(3), R8_A), // xF - SET 3, A
    // Ex
    SET(CONST_8(4), R8_B), // x0 - SET 4, B
    SET(CONST_8(4), R8_C), // x1 - SET 4, C
    SET(CONST_8(4), R8_D), // x2 - SET 4, D
    SET(CONST_8(4), R8_E), // x3 - SET 4, E
    SET(CONST_8(4), R8_H), // x4 - SET 4, H
    SET(CONST_8(4), R8_L), // x5 - SET 4, L
    SET(CONST_8(4), M_HL), // x6 - SET 4, [HL]
    SET(CONST_8(4), R8_A), // x7 - SET 4, A
    SET(CONST_8(5), R8_B), // x8 - SET 5, B
    SET(CONST_8(5), R8_C), // x9 - SET 5, C
    SET(CONST_8(5), R8_D), // xA - SET 5, D
    SET(CONST_8(5), R8_E), // xB - SET 5, E
    SET(CONST_8(5), R8_H), // xC - SET 5, H
    SET(CONST_8(5), R8_L), // xD - SET 5, L
    SET(CONST_8(5), M_HL), // xE - SET 5, [HL]
    SET(CONST_8(5), R8_A), // xF - SET 5, A
    // Fx
    SET(CONST_8(6), R8_B), // x0 - SET 6, B
    SET(CONST_8(6), R8_C), // x1 - SET 6, C
    SET(CONST_8(6), R8_D), // x2 - SET 6, D
    SET(CONST_8(6), R8_E), // x3 - SET 6, E
    SET(CONST_8(6), R8_H), // x4 - SET 6, H
    SET(CONST_8(6), R8_L), // x5 - SET 6, L
    SET(CONST_8(6), M_HL), // x6 - SET 6, [HL]
    SET(CONST_8(6), R8_A), // x7 - SET 6, A
    SET(CONST_8(7), R8_B), // x8 - SET 7, B
    SET(CONST_8(7), R8_C), // x9 - SET 7, C
    SET(CONST_8(7), R8_D), // xA - SET 7, D
    SET(CONST_8(7), R8_E), // xB - SET 7, E
    SET(CONST_8(7), R8_H), // xC - SET 7, H
    SET(CONST_8(7), R8_L), // xD - SET 7, L
    SET(CONST_8(7), M_HL), // xE - SET 7, [HL]
    SET(CONST_8(7), R8_A), // xF - SET 7, A
];
