use crate::util::error_and_panic;

use super::{
    instructions::{
        ArgCOND, ArgR16, ArgR16MEM, ArgR16STK, ArgR8,
        Instruction::{self, *},
    },
    MTime, CPU,
};

impl CPU {
    pub(super) fn execute(&mut self, instruction: Instruction) -> MTime {
        let time = match instruction {
            // Load (LD dest,source)
            LD_r8_r8(r8_dest, r8_src) => self.op_ld_r8_r8(r8_dest, r8_src),
            LD_r16_n16(r16, n16) => self.op_ld_r16_n16(r16, n16.to()),
            LD_mr16_a(mr16) => self.op_ld_a_mr16(mr16, false),
            LDH_mn16_a(n8) => self.op_ldh_a_mn16(n8.to(), false),
            LDH_mc_a => self.op_ldh_a_mc(false),
            LD_a_mr16(mr16) => self.op_ld_a_mr16(mr16, true),
            LDH_a_mn16(n8) => self.op_ldh_a_mn16(n8.to(), true),
            LDH_a_mc => self.op_ldh_a_mc(true),

            // 8-bit arithmetic
            ADC_a_r8(r8) => self.op_add_r8(r8, true),
            ADD_a_r8(r8) => self.op_add_r8(r8, false),
            CP_a_r8(r8) => self.op_cp_r8(r8),
            DEC_r8(r8) => self.op_dec_r8(r8),
            INC_r8(r8) => self.op_inc_r8(r8),
            SBC_a_r8(r8) => self.op_sub_r8(r8, true),
            SUB_a_r8(r8) => self.op_sub_r8(r8, false),

            // 16-bit arithmetic
            ADD_hl_r16(r16) => self.op_add_hl_r16(r16),
            DEC_r16(r16) => self.op_dec_r16(r16),
            INC_r16(r16) => self.op_inc_r16(r16),

            // Bitwise logic
            AND_a_r8(r8) => self.op_and_r8(r8),
            CPL => self.op_cpl(),
            OR_a_r8(r8) => self.op_or_r8(r8),
            XOR_a_r8(r8) => self.op_xor_r8(r8),

            // Bit flags
            BIT_u3_r8(bit, r8) => self.op_bit_u3_r8(bit, r8),
            RES_u3_r8(bit, r8) => self.op_set_u3_r8(bit, r8, false),
            SET_u3_r8(bit, r8) => self.op_set_u3_r8(bit, r8, true),

            // Bit shift
            RL_r8(r8) => self.op_rl_r8(r8, false),
            RLA => self.op_rla(false),
            RLC_r8(r8) => self.op_rl_r8(r8, true),
            RLCA => self.op_rla(true),
            RR_r8(r8) => self.op_rr_r8(r8, false),
            RRA => self.op_rra(false),
            RRC_r8(r8) => self.op_rr_r8(r8, true),
            RRCA => self.op_rra(true),
            SLA_r8(r8) => self.op_sla_r8(r8),
            SRA_r8(r8) => self.op_sra_r8(r8, true),
            SRL_r8(r8) => self.op_sra_r8(r8, false),
            SWAP_r8(r8) => self.op_swap_r8(r8),

            // Jumps and subroutines
            CALL_n16(n16) => self.op_call_cc_n16(ArgCOND::ALWAYS, n16.to()),
            CALL_cc_n16(cond, n16) => self.op_call_cc_n16(cond, n16.to()),
            JP_hl => self.op_jp_hl(),
            JP_n16(n16) => self.op_jp_cc_n16(ArgCOND::ALWAYS, n16.to()),
            JP_cc_n16(cond, n16) => self.op_jp_cc_n16(cond, n16.to()),
            JR_e8(offset) => self.op_jr_cc_n16(ArgCOND::ALWAYS, offset),
            JR_cc_e8(cond, offset) => self.op_jr_cc_n16(cond, offset),
            RET_cc(cond) => self.op_ret_cc(cond),
            RET => self.op_ret(false),
            RETI => self.op_ret(true),
            RST_vec(vec) => self.op_rst(vec),

            // Carry flag
            CCF => self.op_cf(false),
            SCF => self.op_cf(true),

            // Stack manipulation
            ADD_hl_sp => self.op_add_hl_r16(ArgR16::SP),
            ADD_sp_e8(offset) => self.op_add_sp_e8(offset),
            DEC_sp => self.op_inc_r16(ArgR16::SP),
            INC_sp => self.op_inc_r16(ArgR16::SP),
            LD_sp_n16(n16) => self.op_ld_r16_n16(ArgR16::SP, n16.to()),
            LD_mn16_sp(mn16) => self.op_ld_mn16_sp(mn16.to()),
            LD_hl_sp_plus_e8(offset) => self.op_ld_hl_sp_plus_e8(offset),
            LD_sp_hl => self.op_ld_sp_hl(),
            POP_r16(r16) => self.op_pop_r16(r16),
            PUSH_r16(r16) => self.op_push_r16(r16),

            // Interrupt-related
            DI => self.op_di(),
            EI => self.op_ei(),
            HALT => self.op_halt(),

            // Miscellaneous
            DAA => self.op_daa(),
            NOP => self.op_nop(),
            STOP(n8) => self.op_stop(n8.to()),

            // Meta
            PREFIX => self.op_invalid(),
            INVALID => self.op_invalid(),
            TERMINATE => self.op_terminate(),
            DEBUG_PRINT => self.op_debug_print(),
        };

        if self.will_set_ime {
            if self.setting_ime {
                self.setting_ime = false;
                self.will_set_ime = false;
                self.ime = true;
            } else {
                self.setting_ime = true;
            }
        }

        time
    }
}

// Instruction helpers
impl CPU {
    /* #region MMU convenience ================================================================= */

    fn read_mhl(&self) -> u8 {
        self.mmu_read(self.get_hl())
    }

    fn write_mhl(&mut self, value: u8) {
        self.mmu_write(self.get_hl(), value);
    }

    /* #endregion */

    /* #region Get and set based on instruction arguments ====================================== */

    fn get_r8(&self, r8: ArgR8) -> u8 {
        match r8 {
            ArgR8::B => self.b,
            ArgR8::C => self.c,
            ArgR8::D => self.d,
            ArgR8::E => self.e,
            ArgR8::H => self.h,
            ArgR8::L => self.l,
            ArgR8::MHL => self.read_mhl(),
            ArgR8::A => self.a,
            ArgR8::CONST(c) => c.to(),
        }
    }

    fn set_r8(&mut self, r8: ArgR8, value: u8) {
        match r8 {
            ArgR8::B => self.b = value,
            ArgR8::C => self.c = value,
            ArgR8::D => self.d = value,
            ArgR8::E => self.e = value,
            ArgR8::H => self.h = value,
            ArgR8::L => self.l = value,
            ArgR8::MHL => self.write_mhl(value),
            ArgR8::A => self.a = value,
            ArgR8::CONST(_) => self.panic_no_const(),
        }
    }

    fn get_r16(&self, r16: ArgR16) -> u16 {
        match r16 {
            ArgR16::BC => self.get_bc(),
            ArgR16::DE => self.get_de(),
            ArgR16::HL => self.get_hl(),
            ArgR16::SP => self.sp,
        }
    }

    fn set_r16(&mut self, r16: ArgR16, value: u16) {
        match r16 {
            ArgR16::BC => self.set_bc(value),
            ArgR16::DE => self.set_de(value),
            ArgR16::HL => self.set_hl(value),
            ArgR16::SP => self.sp = value,
        }
    }

    fn get_mr16_as_address(&mut self, mr16: ArgR16MEM) -> u16 {
        match mr16 {
            ArgR16MEM::BC => self.get_bc(),
            ArgR16MEM::DE => self.get_de(),
            ArgR16MEM::HLI => self.get_hl_then_inc(),
            ArgR16MEM::HLD => self.get_hl_then_dec(),
            ArgR16MEM::CONST(c) => c.to(),
        }
    }

    fn get_mr16(&mut self, mr16: ArgR16MEM) -> u8 {
        let address = self.get_mr16_as_address(mr16);
        self.mmu_read(address)
    }

    fn set_mr16(&mut self, mr16: ArgR16MEM, value: u8) {
        let address = self.get_mr16_as_address(mr16);
        self.mmu_write(address, value);
    }

    fn get_r16stk(&self, r16stk: ArgR16STK) -> u16 {
        match r16stk {
            ArgR16STK::BC => self.get_bc(),
            ArgR16STK::DE => self.get_de(),
            ArgR16STK::HL => self.get_hl(),
            ArgR16STK::AF => self.get_af(),
        }
    }

    fn set_r16stk(&mut self, r16stk: ArgR16STK, value: u16) {
        match r16stk {
            ArgR16STK::BC => self.set_bc(value),
            ArgR16STK::DE => self.set_de(value),
            ArgR16STK::HL => self.set_hl(value),
            ArgR16STK::AF => self.set_af(value),
        }
    }

    /* #endregion */

    /* #region Predefined panics =============================================================== */

    fn panic_no_const(&self) -> ! {
        error_and_panic!(
            "[PC {:?}] Constant value not allowed here! {:?}",
            self.this_instruction_pc,
            self.this_instruction
        );
    }

    fn panic_impossible_arguments(&self) -> ! {
        error_and_panic!(
            "[PC {:?}] Impossible arguments for instruction! {:?}",
            self.this_instruction_pc,
            self.this_instruction
        );
    }

    /* #endregion */
}

macro_rules! decide_time_for_r8 {
    ($r8:expr; $fast:expr, $slow:expr) => {
        match $r8 {
            ArgR8::CONST(_) | ArgR8::MHL => $slow,
            _ => $fast,
        }
        .into()
    };
}

mod cpu_execute_bitflags;
mod cpu_execute_bitshift;
mod cpu_execute_bitwise;
mod cpu_execute_carry;
mod cpu_execute_interrupt;
mod cpu_execute_jumps;
mod cpu_execute_load;
mod cpu_execute_math16;
mod cpu_execute_math8;
mod cpu_execute_meta;
mod cpu_execute_misc;
mod cpu_execute_stack;
