pub mod instructions;
mod registers;

use super::MMU;
use instructions::{Instruction::*, *};
use registers::Registers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CPU {
    // Program counter
    pub pc: u16,
    // Stack pointer
    pub sp: u16,
    // The 8 main registers
    pub regs: Registers,
    // Timers (emulation purposes only, not in real hardware)
    pub m_time: u64,
    pub t_time: u64,
}

impl CPU {
    /* #region new() and reset() */
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            regs: Registers::new(),
            m_time: 0,
            t_time: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = 0;
        self.regs = Registers::new();
        self.m_time = 0;
        self.t_time = 0;
    }
    /* #endregion */

    fn panic_no_const() -> ! {
        panic!("Constant not allowed here")
    }

    fn panic_impossible_arguments() -> ! {
        panic!("This combination of arguments is impossible for this instruction");
    }

    fn add_m_time(&mut self, m: u64) {
        self.m_time += m;
        self.t_time = self.m_time * 4;
    }

    fn add_more_mtime_if_const_or_mhl(&mut self, arg: ArgR8, slow: u64, fast: u64) {
        match arg {
            ArgR8::CONST(_) | ArgR8::MHL => self.add_m_time(slow),
            _ => self.add_m_time(fast),
        }
    }

    fn add_more_mtime_if_mhl(&mut self, arg: ArgR8, slow: u64, fast: u64) {
        match arg {
            ArgR8::MHL => self.add_m_time(slow),
            _ => self.add_m_time(fast),
        }
    }

    fn get_value_at_r8(&self, mmu: &MMU, target: &ArgR8) -> u8 {
        match target {
            ArgR8::B => self.regs.b,
            ArgR8::C => self.regs.c,
            ArgR8::D => self.regs.d,
            ArgR8::E => self.regs.e,
            ArgR8::H => self.regs.h,
            ArgR8::L => self.regs.l,
            ArgR8::MHL => mmu.read_byte(self.regs.get_hl()),
            ArgR8::A => self.regs.a,
            ArgR8::CONST(c) => *c,
        }
    }

    fn get_value_at_r16(&self, target: &ArgR16) -> u16 {
        match target {
            ArgR16::BC => self.regs.get_bc(),
            ArgR16::DE => self.regs.get_de(),
            ArgR16::HL => self.regs.get_hl(),
            ArgR16::CONST(c) => *c,
        }
    }

    fn get_value_at_mr16(&mut self, mmu: &MMU, target: &ArgR16MEM) -> u8 {
        let address = match target {
            ArgR16MEM::BC => self.regs.get_bc(),
            ArgR16MEM::DE => self.regs.get_de(),
            ArgR16MEM::HL => self.regs.get_hl(),
            ArgR16MEM::HLI => self.regs.get_hl(),
            ArgR16MEM::HLD => self.regs.get_hl(),
            ArgR16MEM::CONST(c) => *c,
        };

        if matches!(target, ArgR16MEM::HLI) {
            let (v, _) = self.regs.get_hl().overflowing_add(1);
            self.regs.set_hl(v);
        } else if matches!(target, ArgR16MEM::HLD) {
            let (v, _) = self.regs.get_hl().overflowing_sub(1);
            self.regs.set_hl(v);
        }

        mmu.read_byte(address)
    }

    fn set_value_at_mr16(&mut self, mmu: &mut MMU, target: &ArgR16MEM, value: u8) {
        let address = match target {
            ArgR16MEM::BC => self.regs.get_bc(),
            ArgR16MEM::DE => self.regs.get_de(),
            ArgR16MEM::HL => self.regs.get_hl(),
            ArgR16MEM::HLI => {
                let address = self.regs.get_hl();
                self.regs.set_hl(address.overflowing_add(1).0);
                address
            }
            ArgR16MEM::HLD => {
                let address = self.regs.get_hl();
                self.regs.set_hl(address.overflowing_sub(1).0);
                address
            }
            ArgR16MEM::CONST(c) => *c,
        };

        mmu.write_byte(address, value);
    }

    fn set_value_at_r8(&mut self, mmu: &mut MMU, target: &ArgR8, value: u8) {
        match target {
            ArgR8::B => self.regs.b = value,
            ArgR8::C => self.regs.c = value,
            ArgR8::D => self.regs.d = value,
            ArgR8::E => self.regs.e = value,
            ArgR8::H => self.regs.h = value,
            ArgR8::L => self.regs.l = value,
            ArgR8::MHL => mmu.write_byte(self.regs.get_hl(), value),
            ArgR8::A => self.regs.a = value,
            ArgR8::CONST(_) => Self::panic_no_const(),
        }
    }

    fn set_value_at_r16(&mut self, target: &ArgR16, value: u16) {
        match target {
            ArgR16::BC => self.regs.set_bc(value),
            ArgR16::DE => self.regs.set_de(value),
            ArgR16::HL => self.regs.set_hl(value),
            ArgR16::CONST(_) => Self::panic_no_const(),
        }
    }

    fn do_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) -> u8 {
        let value = self.get_value_at_r8(mmu, &operand);
        let cv = (with_carry && self.regs.getf_carry()) as u8;
        let (result, overflow1) = self.regs.a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(cv);
        let nibble_diff = ((self.regs.a & 0xF) as i8) - ((value & 0xF) as i8) - (cv as i8);

        self.regs
            .set_all_flags(result == 0, true, nibble_diff < 0, overflow1 || overflow2);

        result
    }

    fn do_rotate_left(&self, value: u8) -> (u8, bool) {
        // Get value and rotate
        let rotated = value.rotate_left(1);
        // New carry is whatever was rotated
        (rotated, rotated & 1 != 0)
    }

    fn do_rotate_left_carry(&self, value: u8) -> (u8, bool) {
        // Get value as u16
        let value_l = value as u16;
        // Put carry flag at the start, then the value
        let processed = ((self.regs.getf_carry() as u16) << 15) | (value_l << 7);
        // Rotate
        let rotated = processed.rotate_left(1);
        // The top 7 bits of the final value are these bits of rotated
        let new_value_top_7 = ((rotated & 0b0111_1111_0000_0000) >> 7) as u8;
        // The last bit of the final value is the last bit of rotated
        let new_value_last = (rotated & 1) as u8;
        // Put the final value together, and new carry is the first bit of rotated
        (new_value_top_7 | new_value_last, rotated & (1 << 15) != 0)
    }

    fn do_rotate_right(&self, value: u8) -> (u8, bool) {
        // New value is rotated, new carry is what was going to be rotated
        (value.rotate_right(1), value & 1 != 0)
    }

    fn do_rotate_right_carry(&self, value: u8) -> (u8, bool) {
        // Get value as u16
        let value_l = value as u16;
        // Put carry flag at the end, then the value
        let processed = (self.regs.getf_carry() as u16) | (value_l << 1);
        // Rotate
        let rotated = processed.rotate_right(1);
        // The last 7 bits of the final value are these bits of rotated
        let new_value_last_7 = (rotated & 0b0000_0000_1111_1110) as u8;
        // The top bit of the final value is the top bit of rotated
        let new_value_top = ((rotated & (1 << 15)) >> 8) as u8;
        // Put the final value together, and new carry is the last bit of rotated
        (new_value_last_7 | new_value_top, rotated & 1 != 0)
    }
}

// Instruction functions
impl CPU {
    /* #region Load instructions */

    // LD r8,r8 (m: 1)
    // LD r8,n8 (m: 2)
    // LD [HL],r8 (m: 2)
    // LD [HL],n8 (m: 3)
    // LD r8,[HL] (m: 2)
    fn op_load8(&mut self, mmu: &mut MMU, dest: ArgR8, src: ArgR8) {
        if matches!((dest, src), (ArgR8::MHL, ArgR8::MHL)) {
            Self::panic_impossible_arguments();
        }

        if dest == src {
            // No op if src == dest
            self.op_nop();
            return;
        }

        let value = self.get_value_at_r8(mmu, &src);

        self.set_value_at_r8(mmu, &dest, value);

        self.add_m_time(match (dest, src) {
            (ArgR8::MHL, ArgR8::CONST(_)) => 3,
            (_, ArgR8::CONST(_) | ArgR8::MHL) => 2,
            (_, _) => 1,
        });
    }

    // LD r16,n16 (m: 3)
    fn op_load_const_to_r16(&mut self, dest: ArgR16, value: u16) {
        self.set_value_at_r16(&dest, value);

        self.add_m_time(3);
    }

    // LD [r16],A (m: 2)
    // LD [n16],A (m: 4)
    // LD A,[r16] (m: 2)
    // LD A,[n16] (m: 4)
    // LD [HLI],A (m: 2)
    // LD [HLD],A (m: 2)
    // LD A,[HLI] (m: 2)
    // LD A,[HLD] (m: 2)
    fn op_load_between_a_mr16(&mut self, mmu: &mut MMU, address: ArgR16MEM, a_is_dest: bool) {
        if a_is_dest {
            self.regs.a = self.get_value_at_mr16(mmu, &address);
        } else {
            self.set_value_at_mr16(mmu, &address, self.regs.a);
        }

        self.add_m_time(match address {
            ArgR16MEM::CONST(_) => 4,
            _ => 2,
        });
    }

    // LDH [n16],A (m: 3)
    // LDH A,[n16] (m: 3)
    fn op_loadhigh_between_a_mn16(&mut self, mmu: &mut MMU, half_address: u8, a_is_dest: bool) {
        let address = 0xFF + (half_address as u16);

        if a_is_dest {
            self.regs.a = mmu.read_byte(address);
        } else {
            mmu.write_byte(address, self.regs.a);
        }

        self.add_m_time(3);
    }

    // LDH [C],A (m: 2)
    // LDH A,[C] (m: 2)
    fn op_loadhigh_between_a_mc(&mut self, mmu: &mut MMU, a_is_dest: bool) {
        let address = 0xFF00 + (self.regs.c as u16);

        if a_is_dest {
            self.regs.a = mmu.read_byte(address);
        } else {
            mmu.write_byte(address, self.regs.a);
        }

        self.add_m_time(2);
    }

    /* #endregion */

    /* #region 8-bit arithmetic */

    // ADC A,r8 (m: 1)
    // ADC A,[HL] (m: 2)
    // ADC A,n8 (m: 2)
    // ADD A,r8 (m: 1)
    // ADD A,[HL] (m: 2)
    // ADD A,n8 (m: 2)
    fn op_add8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        let value = self.get_value_at_r8(mmu, &operand);
        let cv = (with_carry && self.regs.getf_carry()) as u8;
        let (result, overflow1) = self.regs.a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(cv);
        let nibble_sum = (self.regs.a & 0xF) + (value & 0xF) + cv;

        self.regs
            .set_all_flags(result == 0, false, nibble_sum > 0xF, overflow1 || overflow2);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // CP A,r8 (m: 1)
    // CP A,[HL] (m: 2)
    // CP A,n8 (m: 2)
    fn op_compare8(&mut self, mmu: &MMU, operand: ArgR8) {
        self.do_sub8(mmu, operand, false);

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // DEC r8 (m: 1)
    // DEC [HL] (m: 3)
    fn op_dec8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let new_value = value.overflowing_sub(1).0;

        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(true);
        self.regs.setf_half_carry(value & 0xF == 0);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_more_mtime_if_mhl(target, 3, 1);
    }

    // INC r8 (m: 1)
    // INC [HL] (m: 3)
    fn op_inc8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let new_value = value.overflowing_add(1).0;

        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(value & 0xF == 0xF);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_more_mtime_if_mhl(target, 3, 1);
    }

    // SBC A,r8 (m: 1)
    // SBC A,[HL] (m: 2)
    // SBC A,n8 (m: 2)
    // SUB A,r8 (m: 1)
    // SUB A,[HL] (m: 2)
    // SUB A,n8 (m: 2)
    fn op_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        self.regs.a = self.do_sub8(mmu, operand, with_carry);

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    /* #endregion */

    /* #region 16-bit arithmetic */

    // ADD HL,r16 (m: 2)
    fn op_add_r16_to_hl(&mut self, operand: ArgR16) {
        if matches!(operand, ArgR16::CONST(_)) {
            Self::panic_no_const();
        }

        let lhs = self.regs.get_hl();
        let rhs = self.get_value_at_r16(&operand);
        let (result, overflow) = lhs.overflowing_add(rhs);
        let half_carry = (lhs & 0x0FFF) + (rhs & 0x0FFF) > 0x0FFF;

        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(half_carry);
        self.regs.setf_carry(overflow);

        self.regs.set_hl(result);

        self.add_m_time(2);
    }

    // DEC r16 (m: 2)
    fn op_dec16(&mut self, target: ArgR16) {
        let value = self.get_value_at_r16(&target);
        self.set_value_at_r16(&target, value.overflowing_sub(1).0);

        self.add_m_time(2);
    }

    // INC r16 (m: 2)
    fn op_inc16(&mut self, target: ArgR16) {
        let value = self.get_value_at_r16(&target);
        self.set_value_at_r16(&target, value.overflowing_add(1).0);

        self.add_m_time(2);
    }

    /* #endregion */

    /* #region Bitwise logic */

    // AND A,r8 (m: 1)
    // AND A,[HL] (m: 2)
    // AND A,n8 (m: 2)
    fn op_bitwise_and_r8(&mut self, mmu: &MMU, operand: ArgR8) {
        let value = self.get_value_at_r8(mmu, &operand);
        let result = self.regs.a & value;

        self.regs.set_all_flags(result == 0, false, true, false);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // CPL (m: 1)
    fn op_bitwise_complement(&mut self) {
        self.regs.a = !self.regs.a;

        self.regs.setf_subtract(true);
        self.regs.setf_half_carry(true);

        self.add_m_time(1);
    }

    // OR A,r8 (m: 1)
    // OR A,[HL] (m: 2)
    // OR A,n8 (m: 2)
    fn op_bitwise_or_r8(&mut self, mmu: &MMU, operand: ArgR8) {
        let value = self.get_value_at_r8(mmu, &operand);
        let result = self.regs.a | value;

        self.regs.set_all_flags(result == 0, false, false, false);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // XOR A,r8 (m: 1)
    // XOR A,[HL] (m: 2)
    // XOR A,n8 (m: 2)
    fn op_bitwise_xor_r8(&mut self, mmu: &MMU, operand: ArgR8) {
        let value = self.get_value_at_r8(mmu, &operand);
        let result = self.regs.a ^ value;

        self.regs.set_all_flags(result == 0, false, false, false);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    /* #endregion */

    /* #region Bit flags */

    // BIT u3,r8 (m: 2)
    // BIT u3,[HL] (m: 3)
    fn op_bit_test_r8(&mut self, mmu: &MMU, operand: ArgR8, bit_index: ArgU3) {
        if matches!(operand, ArgR8::CONST(_)) {
            Self::panic_no_const();
        }

        let value = self.get_value_at_r8(mmu, &operand);

        self.regs.setf_zero(value & (bit_index as u8) == 0);
        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(true);

        self.add_more_mtime_if_mhl(operand, 3, 2);
    }

    // RES u3,r8 (m: 2)
    // RES u3,[HL] (m: 4)
    // SET u3,r8 (m: 2)
    // SET u3,[HL] (m: 4)
    fn op_set_bit_r8(&mut self, mmu: &mut MMU, operand: ArgR8, bit_index: ArgU3, set: bool) {
        if matches!(operand, ArgR8::CONST(_)) {
            Self::panic_no_const();
        }

        let value = self.get_value_at_r8(mmu, &operand);

        let new_value = if set {
            value | (bit_index as u8)
        } else {
            value & !(bit_index as u8)
        };

        self.set_value_at_r8(mmu, &operand, new_value);

        self.add_more_mtime_if_mhl(operand, 4, 2);
    }

    /* #endregion */

    /* #region Bit shift */

    // RL r8 (m: 2)
    // RL [HL] (m: 4)
    // RLC r8 (m: 2)
    // RLC [HL] (m: 4)
    fn op_rotate_r8_left(&mut self, mmu: &mut MMU, target: ArgR8, through_carry: bool) {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_left(self.get_value_at_r8(mmu, &target))
        } else {
            self.do_rotate_left_carry(self.get_value_at_r8(mmu, &target))
        };

        self.regs
            .set_all_flags(new_value == 0, false, false, new_carry);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // RLA (m: 1)
    // RLCA (m: 1)
    fn op_rotate_a_left(&mut self, through_carry: bool) {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_left(self.regs.a)
        } else {
            self.do_rotate_left_carry(self.regs.a)
        };

        self.regs.set_all_flags(false, false, false, new_carry);

        self.regs.a = new_value;

        self.add_m_time(1);
    }

    // RR r8 (m: 2)
    // RR [HL] (m: 4)
    // RRC r8 (m: 2)
    // RRC [HL] (m: 4)
    fn op_rotate_r8_right(&mut self, mmu: &mut MMU, target: ArgR8, through_carry: bool) {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_right(self.get_value_at_r8(mmu, &target))
        } else {
            self.do_rotate_right_carry(self.get_value_at_r8(mmu, &target))
        };

        self.regs
            .set_all_flags(new_value == 0, false, false, new_carry);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // RRA (m: 1)
    // RRCA (m: 1)
    fn op_rotate_a_right(&mut self, through_carry: bool) {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_right(self.regs.a)
        } else {
            self.do_rotate_right_carry(self.regs.a)
        };

        self.regs.set_all_flags(false, false, false, new_carry);

        self.regs.a = new_value;

        self.add_m_time(1);
    }

    // SLA r8 (m: 2)
    // SLA [HL] (m: 4)
    fn op_shift_left_arithmetic(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let original_top_bit = value & 0x80;
        let new_value = value << 1;

        self.regs
            .set_all_flags(new_value == 0, false, false, original_top_bit != 0);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // SRA r8 (m: 2)
    // SRA [HL] (m: 4)
    // SRL r8 (m: 2)
    // SRL [HL] (m: 4)
    fn op_shift_right(&mut self, mmu: &mut MMU, target: ArgR8, is_arithmetic: bool) {
        let value = self.get_value_at_r8(mmu, &target);
        let original_bottom_bit = value & 1;
        let shifted_value = value >> 1;

        let new_value = if is_arithmetic {
            (value & 0x80) | shifted_value
        } else {
            shifted_value
        };

        self.regs
            .set_all_flags(new_value == 0, false, false, original_bottom_bit != 0);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // TODO: SWAP r8 (m: 2)
    // TODO: SWAP [HL] (m: 4)

    /* #endregion */

    /* #region Jumps and subroutines */

    // TODO: CALL n16 (m: 6)
    // TODO: CALL cc,n16 (m: 6/3)
    // TODO: JP HL (m: 1)
    // TODO: JP n16 (m: 4)
    // TODO: JP cc,n16 (m: 4/3)
    // TODO: JR n16 (m: 3)
    // TODO: JR cc,n16 (m: 3/2)
    // TODO: RET cc (m: 5/2)
    // TODO: RET (m: 4)
    // TODO: RETI (m: 4)
    // TODO: RST vec (m: 4)

    /* #endregion */

    /* #region Carry flag */

    // TODO: CCF (m: 1)
    // TODO: SCF (m: 1)

    /* #endregion */

    /* #region Stack manipulation */

    // TODO: ADD HL,SP (m: 2)
    // TODO: ADD SP,e8 (m: 4)
    // TODO: DEC SP (m: 2)
    // TODO: INC SP (m: 2)
    // TODO: LD SP,n16 (m: 3)
    // TODO: LD [n16],SP (m: 5)
    // TODO: LD HL,SP+e8 (m: 3)
    // TODO: LD SP,HL (m: 2)
    // TODO: POP AF (m: 3)
    // TODO: POP r16 (m: 3)
    // TODO: PUSH AF (m: 4)
    // TODO: PUSH r16 (m: 4)

    /* #endregion */

    /* #region Interrupt-related */

    // TODO: DI (m: 1)
    // TODO: EI (m: 1)
    // TODO: HALT (m: --)

    /* #endregion */

    /* #region Miscellaneous */

    // TODO: DAA (m: 1)

    // NOP (m: 1)
    fn op_nop(&mut self) {
        self.add_m_time(1);
    }

    // TODO: STOP (m: --)

    /* #endregion */
}

// Execute
impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instruction) {
        match instruction {
            // Load (LD_dest_source)
            LD_r8_r8(dest, src) => self.op_load8(mmu, dest, src),
            LD_r16_n16(dest, value) => self.op_load_const_to_r16(dest, value),
            LD_mr16_a(address) => self.op_load_between_a_mr16(mmu, address, false),
            LDH_mn16_a(half_address) => self.op_loadhigh_between_a_mn16(mmu, half_address, false),
            LDH_mc_a => self.op_loadhigh_between_a_mc(mmu, false),
            LD_a_mr16(address) => self.op_load_between_a_mr16(mmu, address, true),
            LDH_a_mn16(half_address) => self.op_loadhigh_between_a_mn16(mmu, half_address, true),
            LDH_a_mc => self.op_loadhigh_between_a_mc(mmu, true),

            // 8-bit arithmetic
            ADC_a_r8(operand) => self.op_add8(mmu, operand, true),
            ADD_a_r8(operand) => self.op_add8(mmu, operand, false),
            CP_a_r8(operand) => self.op_compare8(mmu, operand),
            DEC_r8(target) => self.op_dec8(mmu, target),
            INC_r8(target) => self.op_inc8(mmu, target),
            SBC_a_r8(operand) => self.op_sub8(mmu, operand, true),
            SUB_a_r8(operand) => self.op_sub8(mmu, operand, false),

            // 16-bit arithmetic
            ADD_hl_r16(operand) => self.op_add_r16_to_hl(operand),
            DEC_r16(target) => self.op_dec16(target),
            INC_r16(target) => self.op_inc16(target),

            // Bitwise logic
            AND_a_r8(operand) => self.op_bitwise_and_r8(mmu, operand),
            CPL => self.op_bitwise_complement(),
            OR_a_r8(operand) => self.op_bitwise_or_r8(mmu, operand),
            XOR_a_r8(operand) => self.op_bitwise_xor_r8(mmu, operand),

            // Bit flags
            BIT_u3_r8(bit_index, operand) => self.op_bit_test_r8(mmu, operand, bit_index),
            RES_u3_r8(bit_index, operand) => self.op_set_bit_r8(mmu, operand, bit_index, false),
            SET_u3_r8(bit_index, operand) => self.op_set_bit_r8(mmu, operand, bit_index, true),

            // Bit shift
            RL_r8(target) => self.op_rotate_r8_left(mmu, target, false),
            RLA => self.op_rotate_a_left(false),
            RLC_r8(target) => self.op_rotate_r8_left(mmu, target, true),
            RLCA => self.op_rotate_a_left(true),
            RR_r8(target) => self.op_rotate_r8_right(mmu, target, false),
            RRA => self.op_rotate_a_right(false),
            RRC_r8(target) => self.op_rotate_r8_right(mmu, target, false),
            RRCA => self.op_rotate_a_right(true),
            SLA_r8(target) => todo!(),
            SRA_r8(target) => todo!(),
            SRL_r8(target) => todo!(),
            SWAP_r8(target) => todo!(),

            // Jumps and subroutines
            CALL_n16(address) => todo!(),
            CALL_cc_n16(condition, address) => todo!(),
            JP_hl => todo!(),
            JP_n16(address) => todo!(),
            JP_cc_n16(condition, address) => todo!(),
            JR_e8(offset) => todo!(),
            JR_cc_e8(condition, offset) => todo!(),
            RET_cc(condition) => todo!(),
            RET => todo!(),
            RETI => todo!(),
            RST_vec(vec_address) => todo!(),

            // Carry flag
            CCF => todo!(),
            SCF => todo!(),

            // Stack manipulation
            ADD_hl_sp => todo!(),
            ADD_sp_e8(operand) => todo!(),
            DEC_sp => todo!(),
            INC_sp => todo!(),
            LD_sp_n16(value) => todo!(),
            LD_mn16_sp(address) => todo!(),
            LD_hl_sp_plus_e8(offset) => todo!(),
            LD_sp_hl => todo!(),
            POP_r16(target) => todo!(),
            PUSH_r16(target) => todo!(),

            // Interrupt-related
            DI => todo!(),
            EI => todo!(),
            HALT => todo!(),

            // Miscellaneous
            DAA => todo!(),
            NOP => self.op_nop(),
            STOP => todo!(),
        }
    }
}

/* #region Display and tests */
// Display ========================================================================================

impl Display for CPU {
    #[rustfmt::skip]
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let Registers { a, f, b, c, d, e, h, l } = &self.regs;
        write!(formatter, "+--------------------------+\n")?;
        write!(formatter, "| PC: 0x{:0>4X}    SP: 0x{:0>4X} | M-time: {}\n", self.pc, self.sp, self.m_time)?;
        write!(formatter, "| A:  0x{:0>2X}      F:  {:0>4b}   | T-time: {}\n", a, f >> 4, self.t_time)?;
        write!(formatter, "| B:  0x{b:0>2X}      C:  0x{c:0>2X}   |\n")?;
        write!(formatter, "| D:  0x{d:0>2X}      E:  0x{e:0>2X}   |\n")?;
        write!(formatter, "| H:  0x{h:0>2X}      L:  0x{l:0>2X}   |\n")?;
        write!(formatter, "+--------------------------+\n")
    }
}

// Tests ==========================================================================================
#[cfg(test)]
mod cpu_tests;

/* #endregion */
