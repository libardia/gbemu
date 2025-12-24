use crate::gb::hardware::processor::instructions::{
    Byte, Cond,
    Instruction::{self, *},
    Mem,
    MetaInstruction::*,
    Offset, R8, R16, Word,
};

pub const OP_TABLE: [Instruction; 0x100] = [
    // 0x
    NOP,                                    // x0 - NOP
    LD_r16_r16(R16::BC, R16::IMM(Word(0))), // x1 - LD BC, n16
    LD_mem_r8(Mem::BC, R8::A),              // x2 - LD [BC], A
    INC_r16(R16::BC),                       // x3 - INC BC
    INC_r8(R8::B),                          // x4 - INC B
    DEC_r8(R8::B),                          // x5 - DEC B
    LD_r8_r8(R8::B, R8::IMM(Byte(0))),      // x6 - LD B, n8
    RLCA,                                   // x7 - RLCA
    LD_a16_SP(Word(0)),                     // x8 - LD [a16], SP
    ADD_r16(R16::BC),                       // x9 - ADD HL, BC
    LD_r8_mem(R8::A, Mem::BC),              // xA - LD A, [BC]
    DEC_r16(R16::BC),                       // xB - DEC BC
    INC_r8(R8::C),                          // xC - INC C
    DEC_r8(R8::C),                          // xD - DEC C
    LD_r8_r8(R8::C, R8::IMM(Byte(0))),      // xE - LD C, n8
    RRCA,                                   // xF - RRCA
    // 1x
    STOP(Byte(0)),                          // x0 - STOP n8
    LD_r16_r16(R16::DE, R16::IMM(Word(0))), // x1 - LD DE, n16
    LD_mem_r8(Mem::DE, R8::A),              // x2 - LD [DE], A
    INC_r16(R16::DE),                       // x3 - INC DE
    INC_r8(R8::D),                          // x4 - INC D
    DEC_r8(R8::D),                          // x5 - DEC D
    LD_r8_r8(R8::D, R8::IMM(Byte(0))),      // x6 - LD D, n8
    RLA,                                    // x7 - RLA
    JR(Cond::ALWAYS, Offset(0)),            // x8 - JR e8
    ADD_r16(R16::DE),                       // x9 - ADD HL, DE
    LD_r8_mem(R8::A, Mem::DE),              // xA - LD A, [DE]
    DEC_r16(R16::DE),                       // xB - DEC DE
    INC_r8(R8::E),                          // xC - INC E
    DEC_r8(R8::E),                          // xD - DEC E
    LD_r8_r8(R8::E, R8::IMM(Byte(0))),      // xE - LD E, n8
    RRA,                                    // xF - RRA
    // 2x
    JR(Cond::NZ, Offset(0)),                // x0 - JR NZ, e8
    LD_r16_r16(R16::HL, R16::IMM(Word(0))), // x1 - LD HL, n16
    LD_mem_r8(Mem::HLI, R8::A),             // x2 - LD [HL+], A
    INC_r16(R16::HL),                       // x3 - INC HL
    INC_r8(R8::H),                          // x4 - INC H
    DEC_r8(R8::H),                          // x5 - DEC H
    LD_r8_r8(R8::H, R8::IMM(Byte(0))),      // x6 - LD H, n8
    DAA,                                    // x7 - DAA
    JR(Cond::Z, Offset(0)),                 // x8 - JR Z, e8
    ADD_r16(R16::HL),                       // x9 - ADD HL, HL
    LD_r8_mem(R8::A, Mem::HLI),             // xA - LD A, [HL+]
    DEC_r16(R16::HL),                       // xB - DEC HL
    INC_r8(R8::L),                          // xC - INC L
    DEC_r8(R8::L),                          // xD - DEC L
    LD_r8_r8(R8::L, R8::IMM(Byte(0))),      // xE - LD L, n8
    CPL,                                    // xF - CPL
    // 3x
    JR(Cond::NC, Offset(0)),                // x0 - JR NC, e8
    LD_r16_r16(R16::SP, R16::IMM(Word(0))), // x1 - LD SP, n16
    LD_mem_r8(Mem::HLD, R8::A),             // x2 - LD [HL-], A
    INC_r16(R16::SP),                       // x3 - INC SP
    INC_r8(R8::MHL),                        // x4 - INC [HL]
    DEC_r8(R8::MHL),                        // x5 - DEC [HL]
    LD_r8_r8(R8::MHL, R8::IMM(Byte(0))),    // x6 - LD [HL], n8
    SCF,                                    // x7 - SCF
    JR(Cond::C, Offset(0)),                 // x8 - JR C, e8
    ADD_r16(R16::SP),                       // x9 - ADD HL, SP
    LD_r8_mem(R8::A, Mem::HLD),             // xA - LD A, [HL-]
    DEC_r16(R16::SP),                       // xB - DEC SP
    INC_r8(R8::A),                          // xC - INC A
    DEC_r8(R8::A),                          // xD - DEC A
    LD_r8_r8(R8::A, R8::IMM(Byte(0))),      // xE - LD A, n8
    CCF,                                    // xF - CCF
    // 4x
    LD_r8_r8(R8::B, R8::B),   // x0 - LD B, B
    LD_r8_r8(R8::B, R8::C),   // x1 - LD B, C
    LD_r8_r8(R8::B, R8::D),   // x2 - LD B, D
    LD_r8_r8(R8::B, R8::E),   // x3 - LD B, E
    LD_r8_r8(R8::B, R8::H),   // x4 - LD B, H
    LD_r8_r8(R8::B, R8::L),   // x5 - LD B, L
    LD_r8_r8(R8::B, R8::MHL), // x6 - LD B, [HL]
    LD_r8_r8(R8::B, R8::A),   // x7 - LD B, A
    LD_r8_r8(R8::C, R8::B),   // x8 - LD C, B
    LD_r8_r8(R8::C, R8::C),   // x9 - LD C, C
    LD_r8_r8(R8::C, R8::D),   // xA - LD C, D
    LD_r8_r8(R8::C, R8::E),   // xB - LD C, E
    LD_r8_r8(R8::C, R8::H),   // xC - LD C, H
    LD_r8_r8(R8::C, R8::L),   // xD - LD C, L
    LD_r8_r8(R8::C, R8::MHL), // xE - LD C, [HL]
    LD_r8_r8(R8::C, R8::A),   // xF - LD C, A
    // 5x
    LD_r8_r8(R8::D, R8::B),   // x0 - LD D, B
    LD_r8_r8(R8::D, R8::C),   // x1 - LD D, C
    LD_r8_r8(R8::D, R8::D),   // x2 - LD D, D
    LD_r8_r8(R8::D, R8::E),   // x3 - LD D, E
    LD_r8_r8(R8::D, R8::H),   // x4 - LD D, H
    LD_r8_r8(R8::D, R8::L),   // x5 - LD D, L
    LD_r8_r8(R8::D, R8::MHL), // x6 - LD D, [HL]
    LD_r8_r8(R8::D, R8::A),   // x7 - LD D, A
    LD_r8_r8(R8::E, R8::B),   // x8 - LD E, B
    LD_r8_r8(R8::E, R8::C),   // x9 - LD E, C
    LD_r8_r8(R8::E, R8::D),   // xA - LD E, D
    LD_r8_r8(R8::E, R8::E),   // xB - LD E, E
    LD_r8_r8(R8::E, R8::H),   // xC - LD E, H
    LD_r8_r8(R8::E, R8::L),   // xD - LD E, L
    LD_r8_r8(R8::E, R8::MHL), // xE - LD E, [HL]
    LD_r8_r8(R8::E, R8::A),   // xF - LD E, A
    // 6x
    LD_r8_r8(R8::H, R8::B),   // x0 - LD H, B
    LD_r8_r8(R8::H, R8::C),   // x1 - LD H, C
    LD_r8_r8(R8::H, R8::D),   // x2 - LD H, D
    LD_r8_r8(R8::H, R8::E),   // x3 - LD H, E
    LD_r8_r8(R8::H, R8::H),   // x4 - LD H, H
    LD_r8_r8(R8::H, R8::L),   // x5 - LD H, L
    LD_r8_r8(R8::H, R8::MHL), // x6 - LD H, [HL]
    LD_r8_r8(R8::H, R8::A),   // x7 - LD H, A
    LD_r8_r8(R8::L, R8::B),   // x8 - LD L, B
    LD_r8_r8(R8::L, R8::C),   // x9 - LD L, C
    LD_r8_r8(R8::L, R8::D),   // xA - LD L, D
    LD_r8_r8(R8::L, R8::E),   // xB - LD L, E
    LD_r8_r8(R8::L, R8::H),   // xC - LD L, H
    LD_r8_r8(R8::L, R8::L),   // xD - LD L, L
    LD_r8_r8(R8::L, R8::MHL), // xE - LD L, [HL]
    LD_r8_r8(R8::L, R8::A),   // xF - LD L, A
    // 7x
    LD_r8_r8(R8::MHL, R8::B), // x0 - LD [HL], B
    LD_r8_r8(R8::MHL, R8::C), // x1 - LD [HL], C
    LD_r8_r8(R8::MHL, R8::D), // x2 - LD [HL], D
    LD_r8_r8(R8::MHL, R8::E), // x3 - LD [HL], E
    LD_r8_r8(R8::MHL, R8::H), // x4 - LD [HL], H
    LD_r8_r8(R8::MHL, R8::L), // x5 - LD [HL], L
    HALT,                     // x6 - HALT
    LD_r8_r8(R8::MHL, R8::A), // x7 - LD [HL], A
    LD_r8_r8(R8::A, R8::B),   // x8 - LD A, B
    LD_r8_r8(R8::A, R8::C),   // x9 - LD A, C
    LD_r8_r8(R8::A, R8::D),   // xA - LD A, D
    LD_r8_r8(R8::A, R8::E),   // xB - LD A, E
    LD_r8_r8(R8::A, R8::H),   // xC - LD A, H
    LD_r8_r8(R8::A, R8::L),   // xD - LD A, L
    LD_r8_r8(R8::A, R8::MHL), // xE - LD A, [HL]
    LD_r8_r8(R8::A, R8::A),   // xF - LD A, A
    // 8x
    ADD_r8(R8::B),   // x0 - ADD A, B
    ADD_r8(R8::C),   // x1 - ADD A, C
    ADD_r8(R8::D),   // x2 - ADD A, D
    ADD_r8(R8::E),   // x3 - ADD A, E
    ADD_r8(R8::H),   // x4 - ADD A, H
    ADD_r8(R8::L),   // x5 - ADD A, L
    ADD_r8(R8::MHL), // x6 - ADD A, [HL]
    ADD_r8(R8::A),   // x7 - ADD A, A
    ADC_r8(R8::B),   // x8 - ADC A, B
    ADC_r8(R8::C),   // x9 - ADC A, C
    ADC_r8(R8::D),   // xA - ADC A, D
    ADC_r8(R8::E),   // xB - ADC A, E
    ADC_r8(R8::H),   // xC - ADC A, H
    ADC_r8(R8::L),   // xD - ADC A, L
    ADC_r8(R8::MHL), // xE - ADC A, [HL]
    ADC_r8(R8::A),   // xF - ADC A, A
    // 9x
    SUB_r8(R8::B),   // x0 - SUB A, B
    SUB_r8(R8::C),   // x1 - SUB A, C
    SUB_r8(R8::D),   // x2 - SUB A, D
    SUB_r8(R8::E),   // x3 - SUB A, E
    SUB_r8(R8::H),   // x4 - SUB A, H
    SUB_r8(R8::L),   // x5 - SUB A, L
    SUB_r8(R8::MHL), // x6 - SUB A, [HL]
    SUB_r8(R8::A),   // x7 - SUB A, A
    SBC_r8(R8::B),   // x8 - SBC A, B
    SBC_r8(R8::C),   // x9 - SBC A, C
    SBC_r8(R8::D),   // xA - SBC A, D
    SBC_r8(R8::E),   // xB - SBC A, E
    SBC_r8(R8::H),   // xC - SBC A, H
    SBC_r8(R8::L),   // xD - SBC A, L
    SBC_r8(R8::MHL), // xE - SBC A, [HL]
    SBC_r8(R8::A),   // xF - SBC A, A
    // Ax
    AND(R8::B),   // x0 - AND A, B
    AND(R8::C),   // x1 - AND A, C
    AND(R8::D),   // x2 - AND A, D
    AND(R8::E),   // x3 - AND A, E
    AND(R8::H),   // x4 - AND A, H
    AND(R8::L),   // x5 - AND A, L
    AND(R8::MHL), // x6 - AND A, [HL]
    AND(R8::A),   // x7 - AND A, A
    XOR(R8::B),   // x8 - XOR A, B
    XOR(R8::C),   // x9 - XOR A, C
    XOR(R8::D),   // xA - XOR A, D
    XOR(R8::E),   // xB - XOR A, E
    XOR(R8::H),   // xC - XOR A, H
    XOR(R8::L),   // xD - XOR A, L
    XOR(R8::MHL), // xE - XOR A, [HL]
    XOR(R8::A),   // xF - XOR A, A
    // Bx
    OR(R8::B),      // x0 - OR A, B
    OR(R8::C),      // x1 - OR A, C
    OR(R8::D),      // x2 - OR A, D
    OR(R8::E),      // x3 - OR A, E
    OR(R8::H),      // x4 - OR A, H
    OR(R8::L),      // x5 - OR A, L
    OR(R8::MHL),    // x6 - OR A, [HL]
    OR(R8::A),      // x7 - OR A, A
    CP_r8(R8::B),   // x8 - CP A, B
    CP_r8(R8::C),   // x9 - CP A, C
    CP_r8(R8::D),   // xA - CP A, D
    CP_r8(R8::E),   // xB - CP A, E
    CP_r8(R8::H),   // xC - CP A, H
    CP_r8(R8::L),   // xD - CP A, L
    CP_r8(R8::MHL), // xE - CP A, [HL]
    CP_r8(R8::A),   // xF - CP A, A
    // Cx
    RET(Cond::NZ),                       // x0 - RET NZ
    POP(R16::BC),                        // x1 - POP BC
    JP(Cond::NZ, Mem::IMM(Word(0))),     // x2 - JP NZ, a16
    JP(Cond::ALWAYS, Mem::IMM(Word(0))), // x3 - JP a16
    CALL(Cond::NZ, Word(0)),             // x4 - CALL NZ, a16
    PUSH(R16::BC),                       // x5 - PUSH BC
    ADD_r8(R8::IMM(Byte(0))),            // x6 - ADD A, n8
    RST(Word(0x00)),                     // x7 - RST $00
    RET(Cond::Z),                        // x8 - RET Z
    RET(Cond::ALWAYS),                   // x9 - RET
    JP(Cond::Z, Mem::IMM(Word(0))),      // xA - JP Z, a16
    PREFIX,                              // xB - PREFIX
    CALL(Cond::Z, Word(0)),              // xC - CALL Z, a16
    CALL(Cond::ALWAYS, Word(0)),         // xD - CALL a16
    ADC_r8(R8::IMM(Byte(0))),            // xE - ADC A, n8
    RST(Word(0x08)),                     // xF - RST $08
    // Dx
    RET(Cond::NC),                   // x0 - RET NC
    POP(R16::DE),                    // x1 - POP DE
    JP(Cond::NC, Mem::IMM(Word(0))), // x2 - JP NC, a16
    INVALID(SHOW_CPU),               // x3 - INVALID (Meta-instruction: Print state of the CPU)
    CALL(Cond::NC, Word(0)),         // x4 - CALL NC, a16
    PUSH(R16::DE),                   // x5 - PUSH DE
    SUB_r8(R8::IMM(Byte(0))),        // x6 - SUB A, n8
    RST(Word(0x10)),                 // x7 - RST $10
    RET(Cond::C),                    // x8 - RET C
    RETI,                            // x9 - RETI
    JP(Cond::C, Mem::IMM(Word(0))),  // xA - JP C, a16
    INVALID(TERMINATE),              // xB - INVALID (Meta-instruction: Terminate the emulator)
    CALL(Cond::C, Word(0)),          // xC - CALL C, a16
    INVALID(DUMP),                   // xD - INVALID (Meta-instruction: Full state dump to file)
    SBC_r8(R8::IMM(Byte(0))),        // xE - SBC A, n8
    RST(Word(0x18)),                 // xF - RST $18
    // Ex
    LDH_mem_A(Mem::HIGH_IMM(Byte(0))),   // x0 - LDH [a8], A
    POP(R16::HL),                        // x1 - POP HL
    LDH_mem_A(Mem::HIGH_C),              // x2 - LDH [C], A
    INVALID(NONE),                       // x3 - INVALID
    INVALID(NONE),                       // x4 - INVALID
    PUSH(R16::HL),                       // x5 - PUSH HL
    AND(R8::IMM(Byte(0))),               // x6 - AND A, n8
    RST(Word(0x20)),                     // x7 - RST $20
    ADD_SP_e8(Offset(0)),                // x8 - ADD SP, e8
    JP(Cond::ALWAYS, Mem::HL),           // x9 - JP HL
    LD_mem_r8(Mem::IMM(Word(0)), R8::A), // xA - LD [a16], A
    INVALID(NONE),                       // xB - INVALID
    INVALID(NONE),                       // xC - INVALID
    INVALID(NONE),                       // xD - INVALID
    XOR(R8::IMM(Byte(0))),               // xE - XOR A, n8
    RST(Word(0x28)),                     // xF - RST $28
    // Fx
    LDH_A_mem(Mem::HIGH_IMM(Byte(0))),   // x0 - LDH A, [a8]
    POP(R16::AF),                        // x1 - POP AF
    LDH_A_mem(Mem::HIGH_C),              // x2 - LDH A, [C]
    DI,                                  // x3 - DI
    INVALID(NONE),                       // x4 - INVALID
    PUSH(R16::AF),                       // x5 - PUSH AF
    OR(R8::IMM(Byte(0))),                // x6 - OR A, n8
    RST(Word(0x30)),                     // x7 - RST $30
    LD_HL_SPe8(Offset(0)),               // x8 - LD HL, SP + e8
    LD_r16_r16(R16::SP, R16::HL),        // x9 - LD SP, HL
    LD_r8_mem(R8::A, Mem::IMM(Word(0))), // xA - LD A, [a16]
    EI,                                  // xB - EI
    INVALID(NONE),                       // xC - INVALID
    INVALID(NONE),                       // xD - INVALID
    CP_r8(R8::IMM(Byte(0))),             // xE - CP A, n8
    RST(Word(0x38)),                     // xF - RST $38
];

pub const PREFIX_TABLE: [Instruction; 0x100] = [
    // 0x
    RLC(R8::B),   // x0 - RLC B
    RLC(R8::C),   // x1 - RLC C
    RLC(R8::D),   // x2 - RLC D
    RLC(R8::E),   // x3 - RLC E
    RLC(R8::H),   // x4 - RLC H
    RLC(R8::L),   // x5 - RLC L
    RLC(R8::MHL), // x6 - RLC [HL]
    RLC(R8::A),   // x7 - RLC A
    RRC(R8::B),   // x8 - RRC B
    RRC(R8::C),   // x9 - RRC C
    RRC(R8::D),   // xA - RRC D
    RRC(R8::E),   // xB - RRC E
    RRC(R8::H),   // xC - RRC H
    RRC(R8::L),   // xD - RRC L
    RRC(R8::MHL), // xE - RRC [HL]
    RRC(R8::A),   // xF - RRC A
    // 1x
    RL(R8::B),   // x0 - RL B
    RL(R8::C),   // x1 - RL C
    RL(R8::D),   // x2 - RL D
    RL(R8::E),   // x3 - RL E
    RL(R8::H),   // x4 - RL H
    RL(R8::L),   // x5 - RL L
    RL(R8::MHL), // x6 - RL [HL]
    RL(R8::A),   // x7 - RL A
    RR(R8::B),   // x8 - RR B
    RR(R8::C),   // x9 - RR C
    RR(R8::D),   // xA - RR D
    RR(R8::E),   // xB - RR E
    RR(R8::H),   // xC - RR H
    RR(R8::L),   // xD - RR L
    RR(R8::MHL), // xE - RR [HL]
    RR(R8::A),   // xF - RR A
    // 2x
    SLA(R8::B),   // x0 - SLA B
    SLA(R8::C),   // x1 - SLA C
    SLA(R8::D),   // x2 - SLA D
    SLA(R8::E),   // x3 - SLA E
    SLA(R8::H),   // x4 - SLA H
    SLA(R8::L),   // x5 - SLA L
    SLA(R8::MHL), // x6 - SLA [HL]
    SLA(R8::A),   // x7 - SLA A
    SRA(R8::B),   // x8 - SRA B
    SRA(R8::C),   // x9 - SRA C
    SRA(R8::D),   // xA - SRA D
    SRA(R8::E),   // xB - SRA E
    SRA(R8::H),   // xC - SRA H
    SRA(R8::L),   // xD - SRA L
    SRA(R8::MHL), // xE - SRA [HL]
    SRA(R8::A),   // xF - SRA A
    // 3x
    SWAP(R8::B),   // x0 - SWAP B
    SWAP(R8::C),   // x1 - SWAP C
    SWAP(R8::D),   // x2 - SWAP D
    SWAP(R8::E),   // x3 - SWAP E
    SWAP(R8::H),   // x4 - SWAP H
    SWAP(R8::L),   // x5 - SWAP L
    SWAP(R8::MHL), // x6 - SWAP [HL]
    SWAP(R8::A),   // x7 - SWAP A
    SRL(R8::B),    // x8 - SRL B
    SRL(R8::C),    // x9 - SRL C
    SRL(R8::D),    // xA - SRL D
    SRL(R8::E),    // xB - SRL E
    SRL(R8::H),    // xC - SRL H
    SRL(R8::L),    // xD - SRL L
    SRL(R8::MHL),  // xE - SRL [HL]
    SRL(R8::A),    // xF - SRL A
    // 4x
    BIT(0, R8::B),   // x0 - BIT 0, B
    BIT(0, R8::C),   // x1 - BIT 0, C
    BIT(0, R8::D),   // x2 - BIT 0, D
    BIT(0, R8::E),   // x3 - BIT 0, E
    BIT(0, R8::H),   // x4 - BIT 0, H
    BIT(0, R8::L),   // x5 - BIT 0, L
    BIT(0, R8::MHL), // x6 - BIT 0, [HL]
    BIT(0, R8::A),   // x7 - BIT 0, A
    BIT(1, R8::B),   // x8 - BIT 1, B
    BIT(1, R8::C),   // x9 - BIT 1, C
    BIT(1, R8::D),   // xA - BIT 1, D
    BIT(1, R8::E),   // xB - BIT 1, E
    BIT(1, R8::H),   // xC - BIT 1, H
    BIT(1, R8::L),   // xD - BIT 1, L
    BIT(1, R8::MHL), // xE - BIT 1, [HL]
    BIT(1, R8::A),   // xF - BIT 1, A
    // 5x
    BIT(2, R8::B),   // x0 - BIT 2, B
    BIT(2, R8::C),   // x1 - BIT 2, C
    BIT(2, R8::D),   // x2 - BIT 2, D
    BIT(2, R8::E),   // x3 - BIT 2, E
    BIT(2, R8::H),   // x4 - BIT 2, H
    BIT(2, R8::L),   // x5 - BIT 2, L
    BIT(2, R8::MHL), // x6 - BIT 2, [HL]
    BIT(2, R8::A),   // x7 - BIT 2, A
    BIT(3, R8::B),   // x8 - BIT 3, B
    BIT(3, R8::C),   // x9 - BIT 3, C
    BIT(3, R8::D),   // xA - BIT 3, D
    BIT(3, R8::E),   // xB - BIT 3, E
    BIT(3, R8::H),   // xC - BIT 3, H
    BIT(3, R8::L),   // xD - BIT 3, L
    BIT(3, R8::MHL), // xE - BIT 3, [HL]
    BIT(3, R8::A),   // xF - BIT 3, A
    // 6x
    BIT(4, R8::B),   // x0 - BIT 4, B
    BIT(4, R8::C),   // x1 - BIT 4, C
    BIT(4, R8::D),   // x2 - BIT 4, D
    BIT(4, R8::E),   // x3 - BIT 4, E
    BIT(4, R8::H),   // x4 - BIT 4, H
    BIT(4, R8::L),   // x5 - BIT 4, L
    BIT(4, R8::MHL), // x6 - BIT 4, [HL]
    BIT(4, R8::A),   // x7 - BIT 4, A
    BIT(5, R8::B),   // x8 - BIT 5, B
    BIT(5, R8::C),   // x9 - BIT 5, C
    BIT(5, R8::D),   // xA - BIT 5, D
    BIT(5, R8::E),   // xB - BIT 5, E
    BIT(5, R8::H),   // xC - BIT 5, H
    BIT(5, R8::L),   // xD - BIT 5, L
    BIT(5, R8::MHL), // xE - BIT 5, [HL]
    BIT(5, R8::A),   // xF - BIT 5, A
    // 7x
    BIT(6, R8::B),   // x0 - BIT 6, B
    BIT(6, R8::C),   // x1 - BIT 6, C
    BIT(6, R8::D),   // x2 - BIT 6, D
    BIT(6, R8::E),   // x3 - BIT 6, E
    BIT(6, R8::H),   // x4 - BIT 6, H
    BIT(6, R8::L),   // x5 - BIT 6, L
    BIT(6, R8::MHL), // x6 - BIT 6, [HL]
    BIT(6, R8::A),   // x7 - BIT 6, A
    BIT(7, R8::B),   // x8 - BIT 7, B
    BIT(7, R8::C),   // x9 - BIT 7, C
    BIT(7, R8::D),   // xA - BIT 7, D
    BIT(7, R8::E),   // xB - BIT 7, E
    BIT(7, R8::H),   // xC - BIT 7, H
    BIT(7, R8::L),   // xD - BIT 7, L
    BIT(7, R8::MHL), // xE - BIT 7, [HL]
    BIT(7, R8::A),   // xF - BIT 7, A
    // 8x
    RES(0, R8::B),   // x0 - RES 0, B
    RES(0, R8::C),   // x1 - RES 0, C
    RES(0, R8::D),   // x2 - RES 0, D
    RES(0, R8::E),   // x3 - RES 0, E
    RES(0, R8::H),   // x4 - RES 0, H
    RES(0, R8::L),   // x5 - RES 0, L
    RES(0, R8::MHL), // x6 - RES 0, [HL]
    RES(0, R8::A),   // x7 - RES 0, A
    RES(1, R8::B),   // x8 - RES 1, B
    RES(1, R8::C),   // x9 - RES 1, C
    RES(1, R8::D),   // xA - RES 1, D
    RES(1, R8::E),   // xB - RES 1, E
    RES(1, R8::H),   // xC - RES 1, H
    RES(1, R8::L),   // xD - RES 1, L
    RES(1, R8::MHL), // xE - RES 1, [HL]
    RES(1, R8::A),   // xF - RES 1, A
    // 9x
    RES(2, R8::B),   // x0 - RES 2, B
    RES(2, R8::C),   // x1 - RES 2, C
    RES(2, R8::D),   // x2 - RES 2, D
    RES(2, R8::E),   // x3 - RES 2, E
    RES(2, R8::H),   // x4 - RES 2, H
    RES(2, R8::L),   // x5 - RES 2, L
    RES(2, R8::MHL), // x6 - RES 2, [HL]
    RES(2, R8::A),   // x7 - RES 2, A
    RES(3, R8::B),   // x8 - RES 3, B
    RES(3, R8::C),   // x9 - RES 3, C
    RES(3, R8::D),   // xA - RES 3, D
    RES(3, R8::E),   // xB - RES 3, E
    RES(3, R8::H),   // xC - RES 3, H
    RES(3, R8::L),   // xD - RES 3, L
    RES(3, R8::MHL), // xE - RES 3, [HL]
    RES(3, R8::A),   // xF - RES 3, A
    // Ax
    RES(4, R8::B),   // x0 - RES 4, B
    RES(4, R8::C),   // x1 - RES 4, C
    RES(4, R8::D),   // x2 - RES 4, D
    RES(4, R8::E),   // x3 - RES 4, E
    RES(4, R8::H),   // x4 - RES 4, H
    RES(4, R8::L),   // x5 - RES 4, L
    RES(4, R8::MHL), // x6 - RES 4, [HL]
    RES(4, R8::A),   // x7 - RES 4, A
    RES(5, R8::B),   // x8 - RES 5, B
    RES(5, R8::C),   // x9 - RES 5, C
    RES(5, R8::D),   // xA - RES 5, D
    RES(5, R8::E),   // xB - RES 5, E
    RES(5, R8::H),   // xC - RES 5, H
    RES(5, R8::L),   // xD - RES 5, L
    RES(5, R8::MHL), // xE - RES 5, [HL]
    RES(5, R8::A),   // xF - RES 5, A
    // Bx
    RES(6, R8::B),   // x0 - RES 6, B
    RES(6, R8::C),   // x1 - RES 6, C
    RES(6, R8::D),   // x2 - RES 6, D
    RES(6, R8::E),   // x3 - RES 6, E
    RES(6, R8::H),   // x4 - RES 6, H
    RES(6, R8::L),   // x5 - RES 6, L
    RES(6, R8::MHL), // x6 - RES 6, [HL]
    RES(6, R8::A),   // x7 - RES 6, A
    RES(7, R8::B),   // x8 - RES 7, B
    RES(7, R8::C),   // x9 - RES 7, C
    RES(7, R8::D),   // xA - RES 7, D
    RES(7, R8::E),   // xB - RES 7, E
    RES(7, R8::H),   // xC - RES 7, H
    RES(7, R8::L),   // xD - RES 7, L
    RES(7, R8::MHL), // xE - RES 7, [HL]
    RES(7, R8::A),   // xF - RES 7, A
    // Cx
    SET(0, R8::B),   // x0 - SET 0, B
    SET(0, R8::C),   // x1 - SET 0, C
    SET(0, R8::D),   // x2 - SET 0, D
    SET(0, R8::E),   // x3 - SET 0, E
    SET(0, R8::H),   // x4 - SET 0, H
    SET(0, R8::L),   // x5 - SET 0, L
    SET(0, R8::MHL), // x6 - SET 0, [HL]
    SET(0, R8::A),   // x7 - SET 0, A
    SET(1, R8::B),   // x8 - SET 1, B
    SET(1, R8::C),   // x9 - SET 1, C
    SET(1, R8::D),   // xA - SET 1, D
    SET(1, R8::E),   // xB - SET 1, E
    SET(1, R8::H),   // xC - SET 1, H
    SET(1, R8::L),   // xD - SET 1, L
    SET(1, R8::MHL), // xE - SET 1, [HL]
    SET(1, R8::A),   // xF - SET 1, A
    // Dx
    SET(2, R8::B),   // x0 - SET 2, B
    SET(2, R8::C),   // x1 - SET 2, C
    SET(2, R8::D),   // x2 - SET 2, D
    SET(2, R8::E),   // x3 - SET 2, E
    SET(2, R8::H),   // x4 - SET 2, H
    SET(2, R8::L),   // x5 - SET 2, L
    SET(2, R8::MHL), // x6 - SET 2, [HL]
    SET(2, R8::A),   // x7 - SET 2, A
    SET(3, R8::B),   // x8 - SET 3, B
    SET(3, R8::C),   // x9 - SET 3, C
    SET(3, R8::D),   // xA - SET 3, D
    SET(3, R8::E),   // xB - SET 3, E
    SET(3, R8::H),   // xC - SET 3, H
    SET(3, R8::L),   // xD - SET 3, L
    SET(3, R8::MHL), // xE - SET 3, [HL]
    SET(3, R8::A),   // xF - SET 3, A
    // Ex
    SET(4, R8::B),   // x0 - SET 4, B
    SET(4, R8::C),   // x1 - SET 4, C
    SET(4, R8::D),   // x2 - SET 4, D
    SET(4, R8::E),   // x3 - SET 4, E
    SET(4, R8::H),   // x4 - SET 4, H
    SET(4, R8::L),   // x5 - SET 4, L
    SET(4, R8::MHL), // x6 - SET 4, [HL]
    SET(4, R8::A),   // x7 - SET 4, A
    SET(5, R8::B),   // x8 - SET 5, B
    SET(5, R8::C),   // x9 - SET 5, C
    SET(5, R8::D),   // xA - SET 5, D
    SET(5, R8::E),   // xB - SET 5, E
    SET(5, R8::H),   // xC - SET 5, H
    SET(5, R8::L),   // xD - SET 5, L
    SET(5, R8::MHL), // xE - SET 5, [HL]
    SET(5, R8::A),   // xF - SET 5, A
    // Fx
    SET(6, R8::B),   // x0 - SET 6, B
    SET(6, R8::C),   // x1 - SET 6, C
    SET(6, R8::D),   // x2 - SET 6, D
    SET(6, R8::E),   // x3 - SET 6, E
    SET(6, R8::H),   // x4 - SET 6, H
    SET(6, R8::L),   // x5 - SET 6, L
    SET(6, R8::MHL), // x6 - SET 6, [HL]
    SET(6, R8::A),   // x7 - SET 6, A
    SET(7, R8::B),   // x8 - SET 7, B
    SET(7, R8::C),   // x9 - SET 7, C
    SET(7, R8::D),   // xA - SET 7, D
    SET(7, R8::E),   // xB - SET 7, E
    SET(7, R8::H),   // xC - SET 7, H
    SET(7, R8::L),   // xD - SET 7, L
    SET(7, R8::MHL), // xE - SET 7, [HL]
    SET(7, R8::A),   // xF - SET 7, A
];
