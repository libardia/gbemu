use crate::{
    cpu_log,
    gb::{
        GameBoy,
        hardware::{
            memory::Memory,
            processor::{
                Processor,
                instructions::{
                    Cond,
                    Instruction::{self, *},
                    Mem,
                    MetaInstruction::*,
                    R8, R16,
                },
            },
        },
    },
};

mod op_arith16;
mod op_arith8;
mod op_bit;
mod op_jump;
mod op_load;
mod op_logic;
mod op_misc;
mod op_shift;
mod op_stack;

impl Processor {
    pub fn execute(ctx: &mut GameBoy, inst: Instruction) -> u16 {
        cpu_log!(trace, ctx, "Execute");
        match inst {
            // Load
            LD_r8_r8(dest, src) => op_load::r8_r8(ctx, dest, src),
            LD_r8_mem(dest, src) => op_load::r8_mem(ctx, dest, src),
            LD_mem_r8(dest, src) => op_load::mem_r8(ctx, dest, src),
            LD_r16_r16(dest, src) => op_load::r16_r16(ctx, dest, src),

            // Load high
            LDH_A_mem(src) => op_load::high_a_mem(ctx, src),
            LDH_mem_A(dest) => op_load::high_mem_a(ctx, dest),

            // 8-bit arithmetic
            ADD_r8(op) => op_arith8::add(ctx, op, false),
            ADC_r8(op) => op_arith8::add(ctx, op, true),
            SUB_r8(op) => op_arith8::sub(ctx, op, false),
            SBC_r8(op) => op_arith8::sub(ctx, op, true),
            INC_r8(target) => op_arith8::inc(ctx, target),
            DEC_r8(target) => op_arith8::dec(ctx, target),
            CP_r8(op) => op_arith8::cp(ctx, op),

            // 16-bit arithmetic
            ADD_r16(op) => op_arith16::add(ctx, op),
            INC_r16(target) => op_arith16::inc(ctx, target),
            DEC_r16(target) => op_arith16::dec(ctx, target),

            // Logic
            AND(op) => op_logic::and(ctx, op),
            OR(op) => op_logic::or(ctx, op),
            XOR(op) => op_logic::xor(ctx, op),
            CPL => op_logic::cpl(ctx),

            // Bit flags
            BIT(bit, target) => op_bit::bit(ctx, bit, target),
            SET(bit, target) => op_bit::set(ctx, bit, target),
            RES(bit, target) => op_bit::res(ctx, bit, target),

            // Bit shifts
            RL(target) => op_shift::rl(ctx, target, true, false),
            RLA => op_shift::rl(ctx, R8::A, true, true),
            RLC(target) => op_shift::rl(ctx, target, false, false),
            RLCA => op_shift::rl(ctx, R8::A, false, true),
            RR(target) => op_shift::rr(ctx, target, true, false),
            RRA => op_shift::rr(ctx, R8::A, true, true),
            RRC(target) => op_shift::rr(ctx, target, false, false),
            RRCA => op_shift::rr(ctx, R8::A, false, true),
            SLA(target) => op_shift::sl(ctx, target),
            SRA(target) => op_shift::sr(ctx, target, true),
            SRL(target) => op_shift::sr(ctx, target, false),
            SWAP(target) => op_shift::swap(ctx, target),

            // Jumps and subroutines
            CALL(cond, address) => op_jump::call(ctx, cond, address.0),
            JP(cond, address) => op_jump::jump(ctx, cond, address),
            JR(cond, off) => op_jump::jump_rel(ctx, cond, off.0),
            RET(cond) => op_jump::ret(ctx, cond, false),
            RETI => op_jump::ret(ctx, Cond::ALWAYS, true),
            RST(address) => op_jump::rst(ctx, address.0),

            // Carry flag
            CCF => op_misc::ccf(ctx),
            SCF => op_misc::scf(ctx),

            // Stack manipulation
            ADD_SP_e8(off) => op_stack::offset_sp(ctx, off.0),
            LD_a16_SP(address) => op_stack::save_sp(ctx, address.0),
            LD_HL_SPe8(off) => op_stack::offset_sp_to_hl(ctx, off.0),
            POP(target) => op_stack::pop(ctx, target),
            PUSH(target) => op_stack::push(ctx, target),

            // Interrupts
            DI => op_misc::di(ctx),
            EI => op_misc::ei(ctx),
            HALT => op_misc::halt(ctx),

            // Misc
            DAA => op_misc::daa(ctx),
            NOP => 1, // Do nothing for 1 MTime
            STOP(_) => op_misc::stop(ctx),
            PREFIX => cpu_log!(
                error_panic,
                ctx,
                "Tried to execute {inst:?}, which is only used as a marker."
            ),

            // Meta
            INVALID(meta) => match meta {
                SHOW_CPU if ctx.cpu.meta_inst => todo!(),
                TERMINATE if ctx.cpu.meta_inst => todo!(),
                DUMP if ctx.cpu.meta_inst => todo!(),

                _ => cpu_log!(error_panic, ctx, "Tried to execute an invalid instruction."),
            },
            UNKNOWN => cpu_log!(
                error_panic,
                ctx,
                "Tried to execute the \"{inst:?}\" placeholder instruction."
            ),
        }
    }

    fn get_r8(ctx: &GameBoy, src: R8) -> u8 {
        match src {
            R8::B => ctx.cpu.r.b,
            R8::C => ctx.cpu.r.c,
            R8::D => ctx.cpu.r.d,
            R8::E => ctx.cpu.r.e,
            R8::H => ctx.cpu.r.h,
            R8::L => ctx.cpu.r.l,
            R8::MHL => Memory::read(ctx, ctx.cpu.r.get_hl()),
            R8::A => ctx.cpu.r.a,
            R8::IMM(byte) => byte.into(),
        }
    }

    fn set_r8(ctx: &mut GameBoy, dest: R8, value: u8) {
        match dest {
            R8::B => ctx.cpu.r.b = value,
            R8::C => ctx.cpu.r.c = value,
            R8::D => ctx.cpu.r.d = value,
            R8::E => ctx.cpu.r.e = value,
            R8::H => ctx.cpu.r.h = value,
            R8::L => ctx.cpu.r.l = value,
            R8::MHL => Memory::write(ctx, ctx.cpu.r.get_hl(), value),
            R8::A => ctx.cpu.r.a = value,
            R8::IMM(inner_value) => cpu_log!(
                error_panic,
                ctx,
                "Tried to set a value into the constant {inner_value:?}, which doesn't make sense."
            ),
        }
    }

    fn get_r16(ctx: &GameBoy, src: R16) -> u16 {
        match src {
            R16::BC => ctx.cpu.r.get_bc(),
            R16::DE => ctx.cpu.r.get_de(),
            R16::HL => ctx.cpu.r.get_hl(),
            R16::SP => ctx.cpu.sp,
            R16::AF => Processor::get_af(ctx),
            R16::IMM(word) => word.0,
        }
    }

    fn set_r16(ctx: &mut GameBoy, dest: R16, value: u16) {
        match dest {
            R16::BC => ctx.cpu.r.set_bc(value),
            R16::DE => ctx.cpu.r.set_de(value),
            R16::HL => ctx.cpu.r.set_hl(value),
            R16::SP => ctx.cpu.sp = value,
            R16::AF => Processor::set_af(ctx, value),
            R16::IMM(inner_value) => cpu_log!(
                error_panic,
                ctx,
                "Tried to set a value into the constant {inner_value:?}, which doesn't make sense."
            ),
        }
    }

    fn mem_to_address(ctx: &mut GameBoy, mem: Mem) -> u16 {
        match mem {
            Mem::BC => ctx.cpu.r.get_bc(),
            Mem::DE => ctx.cpu.r.get_de(),
            Mem::HL => ctx.cpu.r.get_hl(),
            Mem::HLI => ctx.cpu.r.get_hli(),
            Mem::HLD => ctx.cpu.r.get_hld(),
            Mem::IMM(address) => address.0,
            Mem::HIGH_C => ctx.cpu.r.c as u16 + 0xFF00,
            Mem::HIGH_IMM(half_address) => half_address.0 as u16 + 0xFF00,
        }
    }

    fn get_mem(ctx: &mut GameBoy, src: Mem) -> u8 {
        let address = Self::mem_to_address(ctx, src);
        Memory::read(ctx, address)
    }

    fn set_mem(ctx: &mut GameBoy, dest: Mem, value: u8) {
        let address = Self::mem_to_address(ctx, dest);
        Memory::write(ctx, address, value);
    }

    fn test_condition(ctx: &GameBoy, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !ctx.cpu.f.z,
            Cond::Z => ctx.cpu.f.z,
            Cond::NC => !ctx.cpu.f.c,
            Cond::C => ctx.cpu.f.c,
            Cond::ALWAYS => true,
        }
    }
}
