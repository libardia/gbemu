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
    // Needed to handle the delay of the IME flag
    will_set_ime: bool,
    setting_ime: bool,
    // Nonstandard operations
    pub terminate: bool,
    pub debug_print: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            regs: Registers::new(),
            m_time: 0,
            t_time: 0,
            will_set_ime: false,
            setting_ime: false,
            terminate: false,
            debug_print: false,
        }
    }

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

    fn get_value_at_r8(&self, mmu: &MMU, target: ArgR8) -> u8 {
        match target {
            ArgR8::B => self.regs.b,
            ArgR8::C => self.regs.c,
            ArgR8::D => self.regs.d,
            ArgR8::E => self.regs.e,
            ArgR8::H => self.regs.h,
            ArgR8::L => self.regs.l,
            ArgR8::MHL => mmu.read_byte(self.regs.get_hl()),
            ArgR8::A => self.regs.a,
            ArgR8::CONST(c) => c.0,
        }
    }

    fn get_value_at_r16(&self, target: ArgR16) -> u16 {
        match target {
            ArgR16::BC => self.regs.get_bc(),
            ArgR16::DE => self.regs.get_de(),
            ArgR16::HL => self.regs.get_hl(),
            ArgR16::SP => self.sp,
        }
    }

    fn get_value_at_mr16(&mut self, mmu: &MMU, target: ArgR16MEM) -> u8 {
        let address = match target {
            ArgR16MEM::BC => self.regs.get_bc(),
            ArgR16MEM::DE => self.regs.get_de(),
            ArgR16MEM::HLI => self.regs.get_hl(),
            ArgR16MEM::HLD => self.regs.get_hl(),
            ArgR16MEM::CONST(c) => c.0,
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

    fn get_value_at_r16stk(&self, target: ArgR16STK) -> u16 {
        match target {
            ArgR16STK::BC => self.regs.get_bc(),
            ArgR16STK::DE => self.regs.get_de(),
            ArgR16STK::HL => self.regs.get_hl(),
            ArgR16STK::AF => self.regs.get_af() & 0xFFF0,
        }
    }

    fn set_value_at_r16stk(&mut self, target: ArgR16STK, value: u16) {
        match target {
            ArgR16STK::BC => self.regs.set_bc(value),
            ArgR16STK::DE => self.regs.set_de(value),
            ArgR16STK::HL => self.regs.set_hl(value),
            ArgR16STK::AF => self.regs.set_af(value & 0xFFF0),
        }
    }

    fn set_value_at_mr16(&mut self, mmu: &mut MMU, target: ArgR16MEM, value: u8) {
        let address = match target {
            ArgR16MEM::BC => self.regs.get_bc(),
            ArgR16MEM::DE => self.regs.get_de(),
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
            ArgR16MEM::CONST(c) => c.0,
        };

        mmu.write_byte(address, value);
    }

    fn set_value_at_r8(&mut self, mmu: &mut MMU, target: ArgR8, value: u8) {
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

    fn set_value_at_r16(&mut self, target: ArgR16, value: u16) {
        match target {
            ArgR16::BC => self.regs.set_bc(value),
            ArgR16::DE => self.regs.set_de(value),
            ArgR16::HL => self.regs.set_hl(value),
            ArgR16::SP => self.sp = value,
        }
    }

    fn do_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) -> u8 {
        let value = self.get_value_at_r8(mmu, operand);
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

    fn eval_condition(&self, condition: ArgCOND) -> bool {
        match condition {
            ArgCOND::NZ => !self.regs.getf_zero(),
            ArgCOND::Z => self.regs.getf_zero(),
            ArgCOND::NC => !self.regs.getf_carry(),
            ArgCOND::C => self.regs.getf_carry(),
            ArgCOND::ALWAYS => true,
        }
    }

    fn push_word(&mut self, mmu: &mut MMU, value: u16) {
        self.sp -= 2;
        mmu.write_word(self.sp, value);
    }

    fn pop_word(&mut self, mmu: &MMU) -> u16 {
        let word = mmu.read_word(self.sp);
        self.sp += 2;
        word
    }
}

// Instruction functions
impl CPU {
    /* #region Load instructions =============================================================== */

    // LD r8,r8   (m: 1)
    // LD r8,n8   (m: 2)
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

        let value = self.get_value_at_r8(mmu, src);

        self.set_value_at_r8(mmu, dest, value);

        self.add_m_time(match (dest, src) {
            (ArgR8::MHL, ArgR8::CONST(_)) => 3,
            (_, ArgR8::CONST(_) | ArgR8::MHL) => 2,
            (_, _) => 1,
        });
    }

    // LD r16,n16 (m: 3)
    // LD SP,n16  (m: 3)
    fn op_load_const_to_r16(&mut self, dest: ArgR16, value: u16) {
        self.set_value_at_r16(dest, value);

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
            self.regs.a = self.get_value_at_mr16(mmu, address);
        } else {
            self.set_value_at_mr16(mmu, address, self.regs.a);
        }

        self.add_m_time(match address {
            ArgR16MEM::CONST(_) => 4,
            _ => 2,
        });
    }

    // LDH [n16],A (m: 3)
    // LDH A,[n16] (m: 3)
    fn op_loadhigh_between_a_mn16(&mut self, mmu: &mut MMU, half_address: u8, a_is_dest: bool) {
        let address = 0xFF00 + (half_address as u16);

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

    /* #region 8-bit arithmetic ================================================================ */

    // ADC A,r8   (m: 1)
    // ADC A,[HL] (m: 2)
    // ADC A,n8   (m: 2)
    // ADD A,r8   (m: 1)
    // ADD A,[HL] (m: 2)
    // ADD A,n8   (m: 2)
    fn op_add8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        let value = self.get_value_at_r8(mmu, operand);
        let cv = (with_carry && self.regs.getf_carry()) as u8;
        let (result, overflow1) = self.regs.a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(cv);
        let nibble_sum = (self.regs.a & 0xF) + (value & 0xF) + cv;

        self.regs
            .set_all_flags(result == 0, false, nibble_sum > 0xF, overflow1 || overflow2);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // CP A,r8   (m: 1)
    // CP A,[HL] (m: 2)
    // CP A,n8   (m: 2)
    fn op_compare8(&mut self, mmu: &MMU, operand: ArgR8) {
        self.do_sub8(mmu, operand, false);

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // DEC r8   (m: 1)
    // DEC [HL] (m: 3)
    fn op_dec8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, target);
        let new_value = value.overflowing_sub(1).0;

        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(true);
        self.regs.setf_half_carry(value & 0xF == 0);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 3, 1);
    }

    // INC r8   (m: 1)
    // INC [HL] (m: 3)
    fn op_inc8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, target);
        let new_value = value.overflowing_add(1).0;

        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(value & 0xF == 0xF);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 3, 1);
    }

    // SBC A,r8   (m: 1)
    // SBC A,[HL] (m: 2)
    // SBC A,n8   (m: 2)
    // SUB A,r8   (m: 1)
    // SUB A,[HL] (m: 2)
    // SUB A,n8   (m: 2)
    fn op_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        self.regs.a = self.do_sub8(mmu, operand, with_carry);

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    /* #endregion */

    /* #region 16-bit arithmetic =============================================================== */

    // ADD HL,r16 (m: 2)
    // ADD HL,SP  (m: 2)
    fn op_add_r16_to_hl(&mut self, operand: ArgR16) {
        let lhs = self.regs.get_hl();
        let rhs = self.get_value_at_r16(operand);
        let (result, overflow) = lhs.overflowing_add(rhs);
        let half_carry = (lhs & 0x0FFF) + (rhs & 0x0FFF) > 0x0FFF;

        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(half_carry);
        self.regs.setf_carry(overflow);

        self.regs.set_hl(result);

        self.add_m_time(2);
    }

    // DEC r16 (m: 2)
    // DEC SP  (m: 2)
    fn op_dec16(&mut self, target: ArgR16) {
        let value = self.get_value_at_r16(target);
        self.set_value_at_r16(target, value.overflowing_sub(1).0);

        self.add_m_time(2);
    }

    // INC r16 (m: 2)
    // INC SP  (m: 2)
    fn op_inc16(&mut self, target: ArgR16) {
        let value = self.get_value_at_r16(target);
        self.set_value_at_r16(target, value.overflowing_add(1).0);

        self.add_m_time(2);
    }

    /* #endregion */

    /* #region Bitwise logic =================================================================== */

    // AND A,r8   (m: 1)
    // AND A,[HL] (m: 2)
    // AND A,n8   (m: 2)
    fn op_bitwise_and_r8(&mut self, mmu: &MMU, operand: ArgR8) {
        let value = self.get_value_at_r8(mmu, operand);
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

    // OR A,r8   (m: 1)
    // OR A,[HL] (m: 2)
    // OR A,n8   (m: 2)
    fn op_bitwise_or_r8(&mut self, mmu: &MMU, operand: ArgR8) {
        let value = self.get_value_at_r8(mmu, operand);
        let result = self.regs.a | value;

        self.regs.set_all_flags(result == 0, false, false, false);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    // XOR A,r8   (m: 1)
    // XOR A,[HL] (m: 2)
    // XOR A,n8   (m: 2)
    fn op_bitwise_xor_r8(&mut self, mmu: &MMU, operand: ArgR8) {
        let value = self.get_value_at_r8(mmu, operand);
        let result = self.regs.a ^ value;

        self.regs.set_all_flags(result == 0, false, false, false);

        self.regs.a = result;

        self.add_more_mtime_if_const_or_mhl(operand, 2, 1);
    }

    /* #endregion */

    /* #region Bit flags ======================================================================= */

    // BIT u3,r8   (m: 2)
    // BIT u3,[HL] (m: 3)
    fn op_bit_test_r8(&mut self, mmu: &MMU, operand: ArgR8, bit_index: ArgU3) {
        if matches!(operand, ArgR8::CONST(_)) {
            Self::panic_no_const();
        }

        let value = self.get_value_at_r8(mmu, operand);

        self.regs.setf_zero(value & (bit_index as u8) == 0);
        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(true);

        self.add_more_mtime_if_mhl(operand, 3, 2);
    }

    // RES u3,r8   (m: 2)
    // RES u3,[HL] (m: 4)
    // SET u3,r8   (m: 2)
    // SET u3,[HL] (m: 4)
    fn op_set_bit_r8(&mut self, mmu: &mut MMU, operand: ArgR8, bit_index: ArgU3, set: bool) {
        if matches!(operand, ArgR8::CONST(_)) {
            Self::panic_no_const();
        }

        let value = self.get_value_at_r8(mmu, operand);

        let new_value = if set {
            value | (bit_index as u8)
        } else {
            value & !(bit_index as u8)
        };

        self.set_value_at_r8(mmu, operand, new_value);

        self.add_more_mtime_if_mhl(operand, 4, 2);
    }

    /* #endregion */

    /* #region Bit shift ======================================================================= */

    // RL r8    (m: 2)
    // RL [HL]  (m: 4)
    // RLC r8   (m: 2)
    // RLC [HL] (m: 4)
    fn op_rotate_r8_left(&mut self, mmu: &mut MMU, target: ArgR8, through_carry: bool) {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_left(self.get_value_at_r8(mmu, target))
        } else {
            self.do_rotate_left_carry(self.get_value_at_r8(mmu, target))
        };

        self.regs
            .set_all_flags(new_value == 0, false, false, new_carry);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // RLA  (m: 1)
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

    // RR r8    (m: 2)
    // RR [HL]  (m: 4)
    // RRC r8   (m: 2)
    // RRC [HL] (m: 4)
    fn op_rotate_r8_right(&mut self, mmu: &mut MMU, target: ArgR8, through_carry: bool) {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_right(self.get_value_at_r8(mmu, target))
        } else {
            self.do_rotate_right_carry(self.get_value_at_r8(mmu, target))
        };

        self.regs
            .set_all_flags(new_value == 0, false, false, new_carry);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // RRA  (m: 1)
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

    // SLA r8   (m: 2)
    // SLA [HL] (m: 4)
    fn op_shift_left_arithmetic(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, target);
        let original_top_bit = value & 0x80;
        let new_value = value << 1;

        self.regs
            .set_all_flags(new_value == 0, false, false, original_top_bit != 0);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // SRA r8   (m: 2)
    // SRA [HL] (m: 4)
    // SRL r8   (m: 2)
    // SRL [HL] (m: 4)
    fn op_shift_right(&mut self, mmu: &mut MMU, target: ArgR8, is_arithmetic: bool) {
        let value = self.get_value_at_r8(mmu, target);
        let original_bottom_bit = value & 1;
        let shifted_value = value >> 1;

        let new_value = if is_arithmetic {
            (value & 0x80) | shifted_value
        } else {
            shifted_value
        };

        self.regs
            .set_all_flags(new_value == 0, false, false, original_bottom_bit != 0);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    // SWAP r8   (m: 2)
    // SWAP [HL] (m: 4)
    fn op_swap(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, target);

        let new_value = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);

        self.regs.set_all_flags(new_value == 0, false, false, false);

        self.set_value_at_r8(mmu, target, new_value);

        self.add_more_mtime_if_mhl(target, 4, 2);
    }

    /* #endregion */

    /* #region Jumps and subroutines =========================================================== */

    // CALL n16    (m:   6)
    // CALL cc,n16 (m: 6/3)
    fn op_call(&mut self, mmu: &mut MMU, condition: ArgCOND, address: u16) {
        if self.eval_condition(condition) {
            self.push_word(mmu, self.pc);
            self.pc = address;
            self.add_m_time(6);
        } else {
            self.add_m_time(3);
        }
    }

    // JP HL (m: 1)
    fn op_jump_hl(&mut self) {
        self.pc = self.regs.get_hl();
        self.add_m_time(1);
    }

    // JP n16    (m:   4)
    // JP cc,n16 (m: 4/3)
    fn op_jump_cond(&mut self, condition: ArgCOND, address: u16) {
        if self.eval_condition(condition) {
            self.pc = address;
            self.add_m_time(4);
        } else {
            self.add_m_time(3);
        }
    }

    // JR n16    (m:   3)
    // JR cc,n16 (m: 3/2)
    fn op_jump_relative(&mut self, condition: ArgCOND, offset: i8) {
        if self.eval_condition(condition) {
            // When offset is converted to u16, it will be filled with the same bits as an i16.
            // Because of two's complement, adding the reults (allowing for overflow) is exactly
            // the same as subtracting, if offset was negative.
            self.pc = self.pc.wrapping_add(offset as u16);
            self.add_m_time(3);
        } else {
            self.add_m_time(2);
        }
    }

    // RET cc (m: 5/2)
    fn op_return_condition(&mut self, mmu: &MMU, condition: ArgCOND) {
        if self.eval_condition(condition) {
            self.pc = self.pop_word(mmu);
            self.add_m_time(5);
        } else {
            self.add_m_time(2);
        }
    }

    // RET  (m: 4)
    // RETI (m: 4)
    fn op_return(&mut self, mmu: &MMU, enable_interrupts: bool) {
        if enable_interrupts {
            // Because this is equivalent to EI then RET, the IME flag is actually set at the end
            // of this insctruction
            self.regs.ime = true;
        }
        self.pc = self.pop_word(mmu);
        self.add_m_time(4);
    }

    // RST vec (m: 4)
    fn op_call_vector(&mut self, mmu: &mut MMU, vec_address: ArgVEC) {
        self.push_word(mmu, self.pc);
        self.pc = vec_address as u16;
        self.add_m_time(4);
    }

    /* #endregion */

    /* #region Carry flag ====================================================================== */

    // CCF (m: 1)
    // SCF (m: 1)
    fn op_carry_flag(&mut self, is_set: bool) {
        self.regs.setf_carry(if is_set {
            true
        } else {
            !self.regs.getf_carry()
        });
        self.regs.setf_half_carry(false);
        self.regs.setf_subtract(false);
        self.add_m_time(1);
    }

    /* #endregion */

    /* #region Stack manipulation ============================================================== */

    // ADD SP,e8 (m: 4)
    fn op_add_e8_to_sp(&mut self, offset: i8) {
        let osp = self.sp;
        let asu16 = offset as u16;

        // I really don't understand how the flags are supposed to work here. This is the best I
        // could figure from the docs I could find. Hopefully Blaarg's test ROMs will make this
        // clear.
        let (nhc, nc) = if offset > 0 {
            let nibble_sum = (asu16 & 0xF) + (osp & 0xF);
            let byte_sum = (asu16 & 0xFF) + (osp & 0xFF);
            (nibble_sum > 0xF, byte_sum > 0xFF)
        } else {
            (false, false)
        };

        self.regs.set_all_flags(false, false, nhc, nc);

        self.sp = osp.wrapping_add(asu16);

        self.add_m_time(4);
    }

    // LD [n16],SP (m: 5)
    fn op_load_sp_to_mn16(&mut self, mmu: &mut MMU, address: u16) {
        mmu.write_word(address, self.sp);
        self.add_m_time(5);
    }

    // LD HL,SP+e8 (m: 3)
    fn op_load_sp_plus_e8_to_hl(&mut self, offset: i8) {
        let osp = self.sp;
        let asu16 = offset as u16;

        // I really don't understand how the flags are supposed to work here. This is the best I
        // could figure from the docs I could find. Hopefully Blaarg's test ROMs will make this
        // clear.
        let (nhc, nc) = if offset > 0 {
            let nibble_sum = (asu16 & 0xF) + (osp & 0xF);
            let byte_sum = (asu16 & 0xFF) + (osp & 0xFF);
            (nibble_sum > 0xF, byte_sum > 0xFF)
        } else {
            (false, false)
        };

        self.regs.set_all_flags(false, false, nhc, nc);

        self.regs.set_hl(osp.wrapping_add(asu16));

        self.add_m_time(3);
    }

    // LD SP,HL (m: 2)
    fn op_load_hl_to_sp(&mut self) {
        self.sp = self.regs.get_hl();
        self.add_m_time(2);
    }

    // POP AF      (m: 3)
    // POP r16     (m: 3)
    fn op_pop_r16(&mut self, mmu: &MMU, target: ArgR16STK) {
        let value = self.pop_word(mmu);
        self.set_value_at_r16stk(target, value);
        self.add_m_time(3);
    }

    // PUSH AF  (m: 4)
    // PUSH r16 (m: 4)
    fn op_push_r16(&mut self, mmu: &mut MMU, target: ArgR16STK) {
        self.push_word(mmu, self.get_value_at_r16stk(target));
        self.add_m_time(4);
    }

    /* #endregion */

    /* #region Interrupt-related =============================================================== */

    // DI (m: 1)
    fn op_disable_interrupts(&mut self) {
        self.regs.ime = false;
        self.add_m_time(1);
    }

    // EI (m: 1)
    fn op_enable_interrupts_delayed(&mut self) {
        self.will_set_ime = true;
        self.add_m_time(1);
    }

    // TODO: HALT (m: --)
    fn op_halt(&mut self) {
        todo!()
    }

    /* #endregion */

    /* #region Miscellaneous =================================================================== */

    // TODO: DAA (m: 1)
    fn op_daa(&mut self) {
        let mut adj = 0u8;
        if self.regs.getf_subtract() {
            if self.regs.getf_half_carry() {
                adj += 0x6;
            }
            if self.regs.getf_carry() {
                adj += 0x60;
            }
            self.regs.a = self.regs.a.wrapping_sub(adj);
        } else {
            if self.regs.getf_half_carry() || (self.regs.a & 0xF) > 0x9 {
                adj += 0x6;
            }
            if self.regs.getf_carry() || self.regs.a > 0x99 {
                adj += 0x60;
                self.regs.setf_carry(true);
            }
            self.regs.a = self.regs.a.wrapping_add(adj);
        }

        self.regs.setf_zero(self.regs.a == 0);
        self.regs.setf_half_carry(false);

        self.add_m_time(1);
    }

    // NOP (m: 1)
    fn op_nop(&mut self) {
        self.add_m_time(1);
    }

    // TODO: STOP (m: --)
    fn op_stop(&mut self, next: u8) {
        todo!();
    }

    /* #endregion */
}

// Execute
impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, inst: Instruction, inst_length: u16) {
        // Advance PC first
        // TODO: emulate HALT bug
        self.pc += inst_length;

        match inst {
            // Load (LD_dest_source)
            LD_r8_r8(dest, src) => self.op_load8(mmu, dest, src),
            LD_r16_n16(dest, value) => self.op_load_const_to_r16(dest, value.0),
            LD_mr16_a(address) => self.op_load_between_a_mr16(mmu, address, false),
            LDH_mn16_a(half_address) => self.op_loadhigh_between_a_mn16(mmu, half_address.0, false),
            LDH_mc_a => self.op_loadhigh_between_a_mc(mmu, false),
            LD_a_mr16(address) => self.op_load_between_a_mr16(mmu, address, true),
            LDH_a_mn16(half_address) => self.op_loadhigh_between_a_mn16(mmu, half_address.0, true),
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
            SLA_r8(target) => self.op_shift_left_arithmetic(mmu, target),
            SRA_r8(target) => self.op_shift_right(mmu, target, true),
            SRL_r8(target) => self.op_shift_right(mmu, target, false),
            SWAP_r8(target) => self.op_swap(mmu, target),

            // Jumps and subroutines
            CALL_n16(address) => self.op_call(mmu, ArgCOND::ALWAYS, address.0),
            CALL_cc_n16(condition, address) => self.op_call(mmu, condition, address.0),
            JP_hl => self.op_jump_hl(),
            JP_n16(address) => self.op_jump_cond(ArgCOND::ALWAYS, address.0),
            JP_cc_n16(condition, address) => self.op_jump_cond(condition, address.0),
            JR_e8(offset) => self.op_jump_relative(ArgCOND::ALWAYS, offset.0),
            JR_cc_e8(condition, offset) => self.op_jump_relative(condition, offset.0),
            RET_cc(condition) => self.op_return_condition(mmu, condition),
            RET => self.op_return(mmu, false),
            RETI => self.op_return(mmu, true),
            RST_vec(vec_address) => self.op_call_vector(mmu, vec_address),

            // Carry flag
            CCF => self.op_carry_flag(false),
            SCF => self.op_carry_flag(true),

            // Stack manipulation
            ADD_hl_sp => self.op_add_r16_to_hl(ArgR16::SP),
            ADD_sp_e8(offset) => self.op_add_e8_to_sp(offset.0),
            DEC_sp => self.op_dec16(ArgR16::SP),
            INC_sp => self.op_inc16(ArgR16::SP),
            LD_sp_n16(value) => self.op_load_const_to_r16(ArgR16::SP, value.0),
            LD_mn16_sp(address) => self.op_load_sp_to_mn16(mmu, address.0),
            LD_hl_sp_plus_e8(offset) => self.op_load_sp_plus_e8_to_hl(offset.0),
            LD_sp_hl => self.op_load_hl_to_sp(),
            POP_r16(target) => self.op_pop_r16(mmu, target),
            PUSH_r16(target) => self.op_push_r16(mmu, target),

            // Interrupt-related
            DI => self.op_disable_interrupts(),
            EI => self.op_enable_interrupts_delayed(),
            HALT => self.op_halt(),

            // Miscellaneous
            DAA => self.op_daa(),
            NOP => self.op_nop(),
            STOP(next) => self.op_stop(next.0),

            // Meta
            PREFIX => panic!("Attempted to execute the PREFIX meta-instruction!"),
            INVALID => panic!("Attempted to execute an invalid instruction!"),
            TERMINATE => self.terminate = true,
            DEBUG_PRINT => self.debug_print = true,
        }

        // Special handling for delaying changing IME
        if self.will_set_ime {
            if self.setting_ime {
                self.regs.ime = true;
                self.will_set_ime = false;
                self.setting_ime = false;
            } else {
                self.setting_ime = true;
            }
        }
    }
}

/* #region Display ============================================================================= */
impl Display for CPU {
    #[rustfmt::skip]
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let Registers {
            a,
            f,
            b,
            c,
            d,
            e,
            h,
            l,
            ime,
        } = &self.regs;

        write!(formatter, "+--------------------------+\n")?;
        write!(formatter, "| PC: 0x{:0>4X}    SP: 0x{:0>4X} | M-time: {}\n", self.pc, self.sp, self.m_time)?;
        write!(formatter, "| A:  0x{:0>2X}      F:  {:0>4b}   | T-time: {}\n", a, f >> 4, self.t_time)?;
        write!(formatter, "| B:  0x{b:0>2X}      C:  0x{c:0>2X}   | IME: {}\n", ime)?;
        write!(formatter, "| D:  0x{d:0>2X}      E:  0x{e:0>2X}   |\n")?;
        write!(formatter, "| H:  0x{h:0>2X}      L:  0x{l:0>2X}   |\n")?;
        write!(formatter, "+--------------------------+")
    }
}
/* #endregion */
