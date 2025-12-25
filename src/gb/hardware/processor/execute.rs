use crate::{
    error_panic,
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

mod op_arith8;
mod op_load;

impl Processor {
    pub fn execute(ctx: &mut GameBoy, inst: Instruction) -> u16 {
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
            ADD_r16(op) => todo!(),
            INC_r16(target) => todo!(),
            DEC_r16(target) => todo!(),

            // Logic
            AND(op) => todo!(),
            OR(op) => todo!(),
            XOR(op) => todo!(),
            CPL => todo!(),

            // Bit flags
            BIT(bit, target) => todo!(),
            SET(bit, target) => todo!(),
            RES(bit, target) => todo!(),

            // Bit shifts
            RL(target) => todo!(),
            RLA => todo!(),
            RLC(target) => todo!(),
            RLCA => todo!(),
            RR(target) => todo!(),
            RRA => todo!(),
            RRC(target) => todo!(),
            RRCA => todo!(),
            SLA(target) => todo!(),
            SRA(target) => todo!(),
            SRL(target) => todo!(),
            SWAP(target) => todo!(),

            // Jumps and subroutines
            CALL(cond, address) => todo!(),
            JP(cond, address) => todo!(),
            JR(cond, off) => todo!(),
            RET(cond) => todo!(),
            RETI => todo!(),
            RST(address) => todo!(),

            // Carry flag
            CCF => todo!(),
            SCF => todo!(),

            // Stack manipulation
            ADD_SP_e8(off) => todo!(),
            LD_a16_SP(address) => todo!(),
            LD_HL_SPe8(off) => todo!(),
            POP(target) => todo!(),
            PUSH(target) => todo!(),

            // Interrupts
            DI => todo!(),
            EI => todo!(),
            HALT => todo!(),

            // Misc
            DAA => todo!(),
            NOP => 1, // Do nothing for 1 MTime
            STOP(_) => todo!(),
            PREFIX => error_panic!("Tried to execute PREFIX, which is only used as a marker."),

            // Meta
            INVALID(meta) => match meta {
                SHOW_CPU if ctx.cpu.meta_inst => todo!(),
                TERMINATE if ctx.cpu.meta_inst => todo!(),
                DUMP if ctx.cpu.meta_inst => todo!(),

                _ => error_panic!("Tried to execute an invalid instruction."),
            },
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
            R8::IMM(value) => error_panic!(
                "Tried to set a value into the constant {value:?}, which doesn't make sense.",
            ),
        }
    }

    fn get_r16(ctx: &GameBoy, src: R16) -> u16 {
        match src {
            R16::BC => ctx.cpu.r.get_bc(),
            R16::DE => ctx.cpu.r.get_de(),
            R16::HL => ctx.cpu.r.get_hl(),
            R16::SP => ctx.cpu.sp,
            R16::AF => ctx.cpu.r.get_af(),
            R16::IMM(word) => word.0,
        }
    }

    fn set_r16(ctx: &mut GameBoy, dest: R16, value: u16) {
        match dest {
            R16::BC => ctx.cpu.r.set_bc(value),
            R16::DE => ctx.cpu.r.set_de(value),
            R16::HL => ctx.cpu.r.set_hl(value),
            R16::SP => ctx.cpu.sp = value,
            R16::AF => ctx.cpu.r.set_af(value),
            R16::IMM(value) => error_panic!(
                "Tried to set a value into the constant {value:?}, which doesn't make sense.",
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
