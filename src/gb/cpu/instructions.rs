#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    /* #region Load instructions */
    LD_B_B,    // $40: LD B, B
    LD_B_C,    // $41: LD B, C
    LD_B_D,    // $42: LD B, D
    LD_B_E,    // $43: LD B, E
    LD_B_H,    // $44: LD B, H
    LD_B_L,    // $45: LD B, L
    LD_B_mHL,  // $46: LD B, [HL]
    LD_B_A,    // $47: LD B, A
    LD_B_n8,   // $06: LD B, n8
    LD_C_B,    // $48: LD C, B
    LD_C_C,    // $49: LD C, C
    LD_C_D,    // $4A: LD C, D
    LD_C_E,    // $4B: LD C, E
    LD_C_H,    // $4C: LD C, H
    LD_C_L,    // $4D: LD C, L
    LD_C_mHL,  // $4E: LD C, [HL]
    LD_C_A,    // $4F: LD C, A
    LD_C_n8,   // $0E: LD C, n8
    LD_D_B,    // $50: LD D, B
    LD_D_C,    // $51: LD D, C
    LD_D_D,    // $52: LD D, D
    LD_D_E,    // $53: LD D, E
    LD_D_H,    // $54: LD D, H
    LD_D_L,    // $55: LD D, L
    LD_D_mHL,  // $56: LD D, [HL]
    LD_D_A,    // $57: LD D, A
    LD_D_n8,   // $16: LD D, n8
    LD_E_B,    // $58: LD E, B
    LD_E_C,    // $59: LD E, C
    LD_E_D,    // $5A: LD E, D
    LD_E_E,    // $5B: LD E, E
    LD_E_H,    // $5C: LD E, H
    LD_E_L,    // $5D: LD E, L
    LD_E_mHL,  // $5E: LD E, [HL]
    LD_E_A,    // $5F: LD E, A
    LD_E_n8,   // $1E: LD E, n8
    LD_H_B,    // $60: LD H, B
    LD_H_C,    // $61: LD H, C
    LD_H_D,    // $62: LD H, D
    LD_H_E,    // $63: LD H, E
    LD_H_H,    // $64: LD H, H
    LD_H_L,    // $65: LD H, L
    LD_H_mHL,  // $66: LD H, [HL]
    LD_H_A,    // $67: LD H, A
    LD_H_n8,   // $26: LD H, n8
    LD_L_B,    // $68: LD L, B
    LD_L_C,    // $69: LD L, C
    LD_L_D,    // $6A: LD L, D
    LD_L_E,    // $6B: LD L, E
    LD_L_H,    // $6C: LD L, H
    LD_L_L,    // $6D: LD L, L
    LD_L_mHL,  // $6E: LD L, [HL]
    LD_L_A,    // $6F: LD L, A
    LD_L_n8,   // $2E: LD L, n8
    LD_mHL_B,  // $70: LD [HL], B
    LD_mHL_C,  // $71: LD [HL], C
    LD_mHL_D,  // $72: LD [HL], D
    LD_mHL_E,  // $73: LD [HL], E
    LD_mHL_H,  // $74: LD [HL], H
    LD_mHL_L,  // $75: LD [HL], L
    LD_mHL_A,  // $77: LD [HL], A
    LD_mHL_n8, // $36: LD [HL], n8
    LD_mHLi_A, // $22: LD [HL+], A
    LD_mHLd_A, // $32: LD [HL-], A
    LD_A_B,    // $78: LD A, B
    LD_A_C,    // $79: LD A, C
    LD_A_D,    // $7A: LD A, D
    LD_A_E,    // $7B: LD A, E
    LD_A_H,    // $7C: LD A, H
    LD_A_L,    // $7D: LD A, L
    LD_A_mHL,  // $7E: LD A, [HL]
    LD_A_mBC,  // $0A: LD A, [BC]
    LD_A_mDE,  // $1A: LD A, [DE]
    LD_A_mHLi, // $2A: LD A, [HL+]
    LD_A_mHLd, // $3A: LD A, [HL-]
    LD_A_A,    // $7F: LD A, A
    LD_A_n8,   // $3E: LD A, n8
    LD_mBC_A,  // $02: LD [BC], A
    LD_mDE_A,  // $12: LD [DE], A
    LD_ma16_A, // $EA: LD [a16], A
    LD_A_ma16, // $FA: LD A, [a16]
    LDH_ma8_A, // $E0: LDH [a8], A
    LDH_mC_A,  // $E2: LDH [C], A
    LDH_A_ma8, // $F0: LDH A, [a8]
    LDH_A_mC,  // $F2: LDH A, [C]
    LD_BC_n16, // $01: LD BC, n16
    LD_DE_n16, // $11: LD DE, n16
    LD_HL_n16, // $21: LD HL, n16
    /* #endregion */

    /* #region 8-bit arithmetic */
    ADD_A_B,   // $80: ADD A, B
    ADD_A_C,   // $81: ADD A, C
    ADD_A_D,   // $82: ADD A, D
    ADD_A_E,   // $83: ADD A, E
    ADD_A_H,   // $84: ADD A, H
    ADD_A_L,   // $85: ADD A, L
    ADD_A_mHL, // $86: ADD A, [HL]
    ADD_A_A,   // $87: ADD A, A
    ADD_A_n8,  // $C6: ADD A, n8
    ADC_A_B,   // $88: ADC A, B
    ADC_A_C,   // $89: ADC A, C
    ADC_A_D,   // $8A: ADC A, D
    ADC_A_E,   // $8B: ADC A, E
    ADC_A_H,   // $8C: ADC A, H
    ADC_A_L,   // $8D: ADC A, L
    ADC_A_mHL, // $8E: ADC A, [HL]
    ADC_A_A,   // $8F: ADC A, A
    ADC_A_n8,  // $CE: ADC A, n8
    SUB_A_B,   // $90: SUB A, B
    SUB_A_C,   // $91: SUB A, C
    SUB_A_D,   // $92: SUB A, D
    SUB_A_E,   // $93: SUB A, E
    SUB_A_H,   // $94: SUB A, H
    SUB_A_L,   // $95: SUB A, L
    SUB_A_mHL, // $96: SUB A, [HL]
    SUB_A_A,   // $97: SUB A, A
    SUB_A_n8,  // $D6: SUB A, n8
    SBC_A_B,   // $98: SBC A, B
    SBC_A_C,   // $99: SBC A, C
    SBC_A_D,   // $9A: SBC A, D
    SBC_A_E,   // $9B: SBC A, E
    SBC_A_H,   // $9C: SBC A, H
    SBC_A_L,   // $9D: SBC A, L
    SBC_A_mHL, // $9E: SBC A, [HL]
    SBC_A_A,   // $9F: SBC A, A
    SBC_A_n8,  // $DE: SBC A, n8
    INC_B,     // $04: INC B
    INC_C,     // $0C: INC C
    INC_D,     // $14: INC D
    INC_E,     // $1C: INC E
    INC_H,     // $24: INC H
    INC_L,     // $2C: INC L
    INC_mHL,   // $34: INC [HL]
    INC_A,     // $3C: INC A
    DEC_B,     // $05: DEC B
    DEC_C,     // $0D: DEC C
    DEC_D,     // $15: DEC D
    DEC_E,     // $1D: DEC E
    DEC_H,     // $25: DEC H
    DEC_L,     // $2D: DEC L
    DEC_mHL,   // $35: DEC [HL]
    DEC_A,     // $3D: DEC A
    CP_A_B,    // $B8: CP A, B
    CP_A_C,    // $B9: CP A, C
    CP_A_D,    // $BA: CP A, D
    CP_A_E,    // $BB: CP A, E
    CP_A_H,    // $BC: CP A, H
    CP_A_L,    // $BD: CP A, L
    CP_A_mHL,  // $BE: CP A, [HL]
    CP_A_A,    // $BF: CP A, A
    CP_A_n8,   // $FE: CP A, n8
    /* #endregion */

    /* #region 16-bit arithmetic */
    ADD_HL_BC, // $09: ADD HL, BC
    ADD_HL_DE, // $19: ADD HL, DE
    ADD_HL_HL, // $29: ADD HL, HL
    ADD_HL_SP, // $39: ADD HL, SP
    INC_BC,    // $03: INC BC
    INC_DE,    // $13: INC DE
    INC_HL,    // $23: INC HL
    INC_SP,    // $33: INC SP
    DEC_BC,    // $0B: DEC BC
    DEC_DE,    // $1B: DEC DE
    DEC_HL,    // $2B: DEC HL
    DEC_SP,    // $3B: DEC SP
    /* #endregion */

    /* #region Bitwise logic */
    AND_A_B,   // $A0: AND A, B
    AND_A_C,   // $A1: AND A, C
    AND_A_D,   // $A2: AND A, D
    AND_A_E,   // $A3: AND A, E
    AND_A_H,   // $A4: AND A, H
    AND_A_L,   // $A5: AND A, L
    AND_A_mHL, // $A6: AND A, [HL]
    AND_A_A,   // $A7: AND A, A
    AND_A_n8,  // $E6: AND A, n8
    OR_A_B,    // $B0: OR A, B
    OR_A_C,    // $B1: OR A, C
    OR_A_D,    // $B2: OR A, D
    OR_A_E,    // $B3: OR A, E
    OR_A_H,    // $B4: OR A, H
    OR_A_L,    // $B5: OR A, L
    OR_A_mHL,  // $B6: OR A, [HL]
    OR_A_A,    // $B7: OR A, A
    OR_A_n8,   // $F6: OR A, n8
    XOR_A_B,   // $A8: XOR A, B
    XOR_A_C,   // $A9: XOR A, C
    XOR_A_D,   // $AA: XOR A, D
    XOR_A_E,   // $AB: XOR A, E
    XOR_A_H,   // $AC: XOR A, H
    XOR_A_L,   // $AD: XOR A, L
    XOR_A_mHL, // $AE: XOR A, [HL]
    XOR_A_A,   // $AF: XOR A, A
    XOR_A_n8,  // $EE: XOR A, n8
    CPL,       // $2F: CPL
    /* #endregion */

    /* #region Bit flag */
    BIT_0_B,   // $CB40: BIT 0, B
    BIT_0_C,   // $CB41: BIT 0, C
    BIT_0_D,   // $CB42: BIT 0, D
    BIT_0_E,   // $CB43: BIT 0, E
    BIT_0_H,   // $CB44: BIT 0, H
    BIT_0_L,   // $CB45: BIT 0, L
    BIT_0_mHL, // $CB46: BIT 0, [HL]
    BIT_0_A,   // $CB47: BIT 0, A
    BIT_1_B,   // $CB48: BIT 1, B
    BIT_1_C,   // $CB49: BIT 1, C
    BIT_1_D,   // $CB4A: BIT 1, D
    BIT_1_E,   // $CB4B: BIT 1, E
    BIT_1_H,   // $CB4C: BIT 1, H
    BIT_1_L,   // $CB4D: BIT 1, L
    BIT_1_mHL, // $CB4E: BIT 1, [HL]
    BIT_1_A,   // $CB4F: BIT 1, A
    BIT_2_B,   // $CB50: BIT 2, B
    BIT_2_C,   // $CB51: BIT 2, C
    BIT_2_D,   // $CB52: BIT 2, D
    BIT_2_E,   // $CB53: BIT 2, E
    BIT_2_H,   // $CB54: BIT 2, H
    BIT_2_L,   // $CB55: BIT 2, L
    BIT_2_mHL, // $CB56: BIT 2, [HL]
    BIT_2_A,   // $CB57: BIT 2, A
    BIT_3_B,   // $CB58: BIT 3, B
    BIT_3_C,   // $CB59: BIT 3, C
    BIT_3_D,   // $CB5A: BIT 3, D
    BIT_3_E,   // $CB5B: BIT 3, E
    BIT_3_H,   // $CB5C: BIT 3, H
    BIT_3_L,   // $CB5D: BIT 3, L
    BIT_3_mHL, // $CB5E: BIT 3, [HL]
    BIT_3_A,   // $CB5F: BIT 3, A
    BIT_4_B,   // $CB60: BIT 4, B
    BIT_4_C,   // $CB61: BIT 4, C
    BIT_4_D,   // $CB62: BIT 4, D
    BIT_4_E,   // $CB63: BIT 4, E
    BIT_4_H,   // $CB64: BIT 4, H
    BIT_4_L,   // $CB65: BIT 4, L
    BIT_4_mHL, // $CB66: BIT 4, [HL]
    BIT_4_A,   // $CB67: BIT 4, A
    BIT_5_B,   // $CB68: BIT 5, B
    BIT_5_C,   // $CB69: BIT 5, C
    BIT_5_D,   // $CB6A: BIT 5, D
    BIT_5_E,   // $CB6B: BIT 5, E
    BIT_5_H,   // $CB6C: BIT 5, H
    BIT_5_L,   // $CB6D: BIT 5, L
    BIT_5_mHL, // $CB6E: BIT 5, [HL]
    BIT_5_A,   // $CB6F: BIT 5, A
    BIT_6_B,   // $CB70: BIT 6, B
    BIT_6_C,   // $CB71: BIT 6, C
    BIT_6_D,   // $CB72: BIT 6, D
    BIT_6_E,   // $CB73: BIT 6, E
    BIT_6_H,   // $CB74: BIT 6, H
    BIT_6_L,   // $CB75: BIT 6, L
    BIT_6_mHL, // $CB76: BIT 6, [HL]
    BIT_6_A,   // $CB77: BIT 6, A
    BIT_7_B,   // $CB78: BIT 7, B
    BIT_7_C,   // $CB79: BIT 7, C
    BIT_7_D,   // $CB7A: BIT 7, D
    BIT_7_E,   // $CB7B: BIT 7, E
    BIT_7_H,   // $CB7C: BIT 7, H
    BIT_7_L,   // $CB7D: BIT 7, L
    BIT_7_mHL, // $CB7E: BIT 7, [HL]
    BIT_7_A,   // $CB7F: BIT 7, A
    RES_0_B,   // $CB80: RES 0, B
    RES_0_C,   // $CB81: RES 0, C
    RES_0_D,   // $CB82: RES 0, D
    RES_0_E,   // $CB83: RES 0, E
    RES_0_H,   // $CB84: RES 0, H
    RES_0_L,   // $CB85: RES 0, L
    RES_0_mHL, // $CB86: RES 0, [HL]
    RES_0_A,   // $CB87: RES 0, A
    RES_1_B,   // $CB88: RES 1, B
    RES_1_C,   // $CB89: RES 1, C
    RES_1_D,   // $CB8A: RES 1, D
    RES_1_E,   // $CB8B: RES 1, E
    RES_1_H,   // $CB8C: RES 1, H
    RES_1_L,   // $CB8D: RES 1, L
    RES_1_mHL, // $CB8E: RES 1, [HL]
    RES_1_A,   // $CB8F: RES 1, A
    RES_2_B,   // $CB90: RES 2, B
    RES_2_C,   // $CB91: RES 2, C
    RES_2_D,   // $CB92: RES 2, D
    RES_2_E,   // $CB93: RES 2, E
    RES_2_H,   // $CB94: RES 2, H
    RES_2_L,   // $CB95: RES 2, L
    RES_2_mHL, // $CB96: RES 2, [HL]
    RES_2_A,   // $CB97: RES 2, A
    RES_3_B,   // $CB98: RES 3, B
    RES_3_C,   // $CB99: RES 3, C
    RES_3_D,   // $CB9A: RES 3, D
    RES_3_E,   // $CB9B: RES 3, E
    RES_3_H,   // $CB9C: RES 3, H
    RES_3_L,   // $CB9D: RES 3, L
    RES_3_mHL, // $CB9E: RES 3, [HL]
    RES_3_A,   // $CB9F: RES 3, A
    RES_4_B,   // $CBA0: RES 4, B
    RES_4_C,   // $CBA1: RES 4, C
    RES_4_D,   // $CBA2: RES 4, D
    RES_4_E,   // $CBA3: RES 4, E
    RES_4_H,   // $CBA4: RES 4, H
    RES_4_L,   // $CBA5: RES 4, L
    RES_4_mHL, // $CBA6: RES 4, [HL]
    RES_4_A,   // $CBA7: RES 4, A
    RES_5_B,   // $CBA8: RES 5, B
    RES_5_C,   // $CBA9: RES 5, C
    RES_5_D,   // $CBAA: RES 5, D
    RES_5_E,   // $CBAB: RES 5, E
    RES_5_H,   // $CBAC: RES 5, H
    RES_5_L,   // $CBAD: RES 5, L
    RES_5_mHL, // $CBAE: RES 5, [HL]
    RES_5_A,   // $CBAF: RES 5, A
    RES_6_B,   // $CBB0: RES 6, B
    RES_6_C,   // $CBB1: RES 6, C
    RES_6_D,   // $CBB2: RES 6, D
    RES_6_E,   // $CBB3: RES 6, E
    RES_6_H,   // $CBB4: RES 6, H
    RES_6_L,   // $CBB5: RES 6, L
    RES_6_mHL, // $CBB6: RES 6, [HL]
    RES_6_A,   // $CBB7: RES 6, A
    RES_7_B,   // $CBB8: RES 7, B
    RES_7_C,   // $CBB9: RES 7, C
    RES_7_D,   // $CBBA: RES 7, D
    RES_7_E,   // $CBBB: RES 7, E
    RES_7_H,   // $CBBC: RES 7, H
    RES_7_L,   // $CBBD: RES 7, L
    RES_7_mHL, // $CBBE: RES 7, [HL]
    RES_7_A,   // $CBBF: RES 7, A
    SET_0_B,   // $CBC0: SET 0, B
    SET_0_C,   // $CBC1: SET 0, C
    SET_0_D,   // $CBC2: SET 0, D
    SET_0_E,   // $CBC3: SET 0, E
    SET_0_H,   // $CBC4: SET 0, H
    SET_0_L,   // $CBC5: SET 0, L
    SET_0_mHL, // $CBC6: SET 0, [HL]
    SET_0_A,   // $CBC7: SET 0, A
    SET_1_B,   // $CBC8: SET 1, B
    SET_1_C,   // $CBC9: SET 1, C
    SET_1_D,   // $CBCA: SET 1, D
    SET_1_E,   // $CBCB: SET 1, E
    SET_1_H,   // $CBCC: SET 1, H
    SET_1_L,   // $CBCD: SET 1, L
    SET_1_mHL, // $CBCE: SET 1, [HL]
    SET_1_A,   // $CBCF: SET 1, A
    SET_2_B,   // $CBD0: SET 2, B
    SET_2_C,   // $CBD1: SET 2, C
    SET_2_D,   // $CBD2: SET 2, D
    SET_2_E,   // $CBD3: SET 2, E
    SET_2_H,   // $CBD4: SET 2, H
    SET_2_L,   // $CBD5: SET 2, L
    SET_2_mHL, // $CBD6: SET 2, [HL]
    SET_2_A,   // $CBD7: SET 2, A
    SET_3_B,   // $CBD8: SET 3, B
    SET_3_C,   // $CBD9: SET 3, C
    SET_3_D,   // $CBDA: SET 3, D
    SET_3_E,   // $CBDB: SET 3, E
    SET_3_H,   // $CBDC: SET 3, H
    SET_3_L,   // $CBDD: SET 3, L
    SET_3_mHL, // $CBDE: SET 3, [HL]
    SET_3_A,   // $CBDF: SET 3, A
    SET_4_B,   // $CBE0: SET 4, B
    SET_4_C,   // $CBE1: SET 4, C
    SET_4_D,   // $CBE2: SET 4, D
    SET_4_E,   // $CBE3: SET 4, E
    SET_4_H,   // $CBE4: SET 4, H
    SET_4_L,   // $CBE5: SET 4, L
    SET_4_mHL, // $CBE6: SET 4, [HL]
    SET_4_A,   // $CBE7: SET 4, A
    SET_5_B,   // $CBE8: SET 5, B
    SET_5_C,   // $CBE9: SET 5, C
    SET_5_D,   // $CBEA: SET 5, D
    SET_5_E,   // $CBEB: SET 5, E
    SET_5_H,   // $CBEC: SET 5, H
    SET_5_L,   // $CBED: SET 5, L
    SET_5_mHL, // $CBEE: SET 5, [HL]
    SET_5_A,   // $CBEF: SET 5, A
    SET_6_B,   // $CBF0: SET 6, B
    SET_6_C,   // $CBF1: SET 6, C
    SET_6_D,   // $CBF2: SET 6, D
    SET_6_E,   // $CBF3: SET 6, E
    SET_6_H,   // $CBF4: SET 6, H
    SET_6_L,   // $CBF5: SET 6, L
    SET_6_mHL, // $CBF6: SET 6, [HL]
    SET_6_A,   // $CBF7: SET 6, A
    SET_7_B,   // $CBF8: SET 7, B
    SET_7_C,   // $CBF9: SET 7, C
    SET_7_D,   // $CBFA: SET 7, D
    SET_7_E,   // $CBFB: SET 7, E
    SET_7_H,   // $CBFC: SET 7, H
    SET_7_L,   // $CBFD: SET 7, L
    SET_7_mHL, // $CBFE: SET 7, [HL]
    SET_7_A,   // $CBFF: SET 7, A
    /* #endregion */

    /* #region Bit shift */
    RRA,      // $1F: RRA
    RRCA,     // $0F: RRCA
    RLA,      // $17: RLA
    RLCA,     // $07: RLCA
    RL_B,     // $CB10: RL B
    RL_C,     // $CB11: RL C
    RL_D,     // $CB12: RL D
    RL_E,     // $CB13: RL E
    RL_H,     // $CB14: RL H
    RL_L,     // $CB15: RL L
    RL_mHL,   // $CB16: RL [HL]
    RL_A,     // $CB17: RL A
    RLC_B,    // $CB00: RLC B
    RLC_C,    // $CB01: RLC C
    RLC_D,    // $CB02: RLC D
    RLC_E,    // $CB03: RLC E
    RLC_H,    // $CB04: RLC H
    RLC_L,    // $CB05: RLC L
    RLC_mHL,  // $CB06: RLC [HL]
    RLC_A,    // $CB07: RLC A
    SLA_B,    // $CB20: SLA B
    SLA_C,    // $CB21: SLA C
    SLA_D,    // $CB22: SLA D
    SLA_E,    // $CB23: SLA E
    SLA_H,    // $CB24: SLA H
    SLA_L,    // $CB25: SLA L
    SLA_mHL,  // $CB26: SLA [HL]
    SLA_A,    // $CB27: SLA A
    RR_B,     // $CB18: RR B
    RR_C,     // $CB19: RR C
    RR_D,     // $CB1A: RR D
    RR_E,     // $CB1B: RR E
    RR_H,     // $CB1C: RR H
    RR_L,     // $CB1D: RR L
    RR_mHL,   // $CB1E: RR [HL]
    RR_A,     // $CB1F: RR A
    RRC_B,    // $CB08: RRC B
    RRC_C,    // $CB09: RRC C
    RRC_D,    // $CB0A: RRC D
    RRC_E,    // $CB0B: RRC E
    RRC_H,    // $CB0C: RRC H
    RRC_L,    // $CB0D: RRC L
    RRC_mHL,  // $CB0E: RRC [HL]
    RRC_A,    // $CB0F: RRC A
    SRA_B,    // $CB28: SRA B
    SRA_C,    // $CB29: SRA C
    SRA_D,    // $CB2A: SRA D
    SRA_E,    // $CB2B: SRA E
    SRA_H,    // $CB2C: SRA H
    SRA_L,    // $CB2D: SRA L
    SRA_mHL,  // $CB2E: SRA [HL]
    SRA_A,    // $CB2F: SRA A
    SRL_B,    // $CB38: SRL B
    SRL_C,    // $CB39: SRL C
    SRL_D,    // $CB3A: SRL D
    SRL_E,    // $CB3B: SRL E
    SRL_H,    // $CB3C: SRL H
    SRL_L,    // $CB3D: SRL L
    SRL_mHL,  // $CB3E: SRL [HL]
    SRL_A,    // $CB3F: SRL A
    SWAP_B,   // $CB30: SWAP B
    SWAP_C,   // $CB31: SWAP C
    SWAP_D,   // $CB32: SWAP D
    SWAP_E,   // $CB33: SWAP E
    SWAP_H,   // $CB34: SWAP H
    SWAP_L,   // $CB35: SWAP L
    SWAP_mHL, // $CB36: SWAP [HL]
    SWAP_A,   // $CB37: SWAP A
    /* #endregion */

    /* #region Jumps */
    CALL_a16,    // $CD: CALL a16
    CALL_NZ_a16, // $C4: CALL NZ, a16
    CALL_Z_a16,  // $CC: CALL Z, a16
    CALL_NC_a16, // $D4: CALL NC, a16
    CALL_C_a16,  // $DC: CALL C, a16
    JR_e8,       // $18: JR e8
    JR_NZ_e8,    // $20: JR NZ, e8
    JR_Z_e8,     // $28: JR Z, e8
    JR_NC_e8,    // $30: JR NC, e8
    JR_C_e8,     // $38: JR C, e8
    JP_a16,      // $C3: JP a16
    JP_HL,       // $E9: JP HL
    JP_NZ_a16,   // $C2: JP NZ, a16
    JP_Z_a16,    // $CA: JP Z, a16
    JP_NC_a16,   // $D2: JP NC, a16
    JP_C_a16,    // $DA: JP C, a16
    RET,         // $C9: RET
    RETI,        // $D9: RETI
    RET_NZ,      // $C0: RET NZ
    RET_Z,       // $C8: RET Z
    RET_NC,      // $D0: RET NC
    RET_C,       // $D8: RET C
    RST_v00,     // $C7: RST $00
    RST_v08,     // $CF: RST $08
    RST_v10,     // $D7: RST $10
    RST_v18,     // $DF: RST $18
    RST_v20,     // $E7: RST $20
    RST_v28,     // $EF: RST $28
    RST_v30,     // $F7: RST $30
    RST_v38,     // $FF: RST $38
    /* #endregion */

    /* #region Carry flag */
    SCF, // $37: SCF
    CCF, // $3F: CCF
    /* #endregion */

    /* #region Stack */
    LD_SP_n16,  // $31: LD SP, n16
    LD_SP_HL,   // $F9: LD SP, HL
    LD_HL_SPe8, // $F8: LD HL, SP+e8
    LD_ma16_SP, // $08: LD [a16], SP
    ADD_SP_e8,  // $E8: ADD SP, e8
    POP_BC,     // $C1: POP BC
    POP_DE,     // $D1: POP DE
    POP_HL,     // $E1: POP HL
    POP_AF,     // $F1: POP AF
    PUSH_BC,    // $C5: PUSH BC
    PUSH_DE,    // $D5: PUSH DE
    PUSH_HL,    // $E5: PUSH HL
    PUSH_AF,    // $F5: PUSH AF
    /* #endregion */

    /* #region Interrupts */
    DI,   // $F3: DI
    EI,   // $FB: EI
    HALT, // $76: HALT
    /* #endregion */

    /* #region Misc */
    DAA,     // $27: DAA
    NOP,     // $00: NOP
    STOP_n8, // $10: STOP n8
    PREFIX,  // $CB: PREFIX
    INVALID,
    /* #endregion */
}
