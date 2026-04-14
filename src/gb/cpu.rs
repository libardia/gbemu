use log::{trace, warn};

use crate::{
    gb::{
        GameBoy,
        cpu::{
            instructions::Instruction,
            interrupts::INT_FLAGS_MASK,
            optables::{OPTABLE, PREFIX_OPTABLE},
        },
        hardware_interface::HardwareInterface,
        mmu::{
            MMU,
            io::{IO_IE, IO_IF},
            region::HIGH_RAM,
        },
    },
    macros::{get_masked, hex, make_word, select, set_masked},
};

pub mod access;
pub mod execute;
pub mod instructions;
pub mod interrupts;
pub mod optables;

#[derive(Debug, Default, Clone)]
pub struct CPU {
    // Registers
    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub a: u8,
    pub f: Flags,

    // Internal
    pub pc: u16,
    pub sp: u16,

    pub ime: bool,
    pub io_if: u8,
    pub io_ie: u8,

    // Misc
    pub ime_timer: u8,
    pub prefix_mode: bool,
    pub halt_bug: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Flags {
    pub fn as_byte(&self) -> u8 {
        let mut b = 0;
        b |= (self.z as u8) << 7;
        b |= (self.n as u8) << 6;
        b |= (self.h as u8) << 5;
        b |= (self.c as u8) << 4;
        b
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            z: (byte & 0x80) != 0,
            n: (byte & 0x40) != 0,
            h: (byte & 0x20) != 0,
            c: (byte & 0x10) != 0,
        }
    }
}

macro_rules! r16 {
    ($r1:ident + $r2:ident) => {
        paste::paste! {
            pub fn [<get_ $r1 $r2>](&self) -> u16 {
                make_word!(self.$r1, self.$r2)
            }

            pub fn [<set_ $r1 $r2>](&mut self, word: u16) {
                self.$r1 = (word >> 8) as u8;
                self.$r2 = (word & 0xFF) as u8;
            }
        }
    };
}

impl CPU {
    pub fn new() -> Self {
        CPU::default()
    }

    pub fn step(ctx: &mut GameBoy) {
        CPU::handle_interrupts(ctx);

        let inst = CPU::decode(ctx);
        CPU::execute(ctx, inst);

        // Special handling for IME flag
        if ctx.cpu.ime_timer > 0 {
            if ctx.cpu.ime_timer == 1 {
                debug_interrupts!(on);
                ctx.cpu.ime = true;
            }

            // Tick down
            ctx.cpu.ime_timer -= 1;
        }
    }

    pub fn decode(ctx: &mut GameBoy) -> Instruction {
        let address = ctx.cpu.pc;
        let byte = CPU::next_byte(ctx) as usize;
        let inst = if ctx.cpu.prefix_mode {
            ctx.cpu.prefix_mode = false;
            PREFIX_OPTABLE[byte]
        } else {
            OPTABLE[byte]
        };
        trace!(
            "{}: decoded byte: {} -> {inst:?}",
            hex!(address, 4),
            hex!(byte, 2),
        );
        inst
    }
}

impl HardwareInterface for CPU {
    fn read(&mut self, address: u16) -> u8 {
        match address {
            IO_IF => get_masked!(self.io_if; INT_FLAGS_MASK),
            IO_IE => self.io_ie,

            _ => unimplemented!("can't read address {} from the CPU", hex!(address, 4)),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            IO_IF => self.io_if = set_masked!(self.io_if, byte; INT_FLAGS_MASK),
            IO_IE => self.io_ie = byte,

            _ => unimplemented!("can't write address {} to the CPU", hex!(address, 4)),
        };
    }
}

impl CPU {
    r16!(b + c);
    r16!(d + e);
    r16!(h + l);

    pub fn get_af(&self) -> u16 {
        make_word!(self.a, self.f.as_byte())
    }

    pub fn set_af(&mut self, word: u16) {
        self.a = (word >> 8) as u8;
        self.f = Flags::from_byte((word & 0xFF) as u8);
    }

    pub fn get_hli(&mut self) -> u16 {
        let hl = self.get_hl();
        self.set_hl(hl.wrapping_add(1));
        hl
    }

    pub fn get_hld(&mut self) -> u16 {
        let hl = self.get_hl();
        self.set_hl(hl.wrapping_sub(1));
        hl
    }

    pub fn read_tick(ctx: &mut GameBoy, address: u16) -> u8 {
        ctx.m_tick(); // Read takes 1 m-cycle
        MMU::read(ctx, address)
    }

    pub fn write_tick(ctx: &mut GameBoy, address: u16, byte: u8) {
        ctx.m_tick(); // Write takes 1 m-cycle
        MMU::write(ctx, address, byte);
    }

    pub fn next_byte(ctx: &mut GameBoy) -> u8 {
        let byte = CPU::read_tick(ctx, ctx.cpu.pc);
        if ctx.cpu.halt_bug {
            // PC doesn't increment, whoops!
            ctx.cpu.halt_bug = false
        } else {
            ctx.cpu.pc = ctx.cpu.pc.wrapping_add(1)
        }
        byte
    }

    pub fn next_signed(ctx: &mut GameBoy) -> i8 {
        CPU::next_byte(ctx) as i8
    }

    pub fn next_word(ctx: &mut GameBoy) -> u16 {
        let lower = CPU::next_byte(ctx);
        let upper = CPU::next_byte(ctx);
        make_word!(upper, lower)
    }

    pub fn push_stack(ctx: &mut GameBoy, word: u16) {
        ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
        CPU::write_tick(ctx, ctx.cpu.sp, (word >> 8) as u8);
        ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
        CPU::write_tick(ctx, ctx.cpu.sp, (word & 0xFF) as u8);

        if !HIGH_RAM.contains(ctx.cpu.sp) {
            warn!("stack pointer outside HRAM, at {}!", hex!(ctx.cpu.sp, 4));
        }
    }

    pub fn pop_stack(ctx: &mut GameBoy) -> u16 {
        let low = CPU::read_tick(ctx, ctx.cpu.sp);
        ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);
        let high = CPU::read_tick(ctx, ctx.cpu.sp);
        ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

        if !HIGH_RAM.contains(ctx.cpu.sp) {
            warn!("stack pointer outside HRAM, at {}!", hex!(ctx.cpu.sp, 4));
        }

        make_word!(high, low)
    }

    pub fn debug_str(&self) -> String {
        return format!(
            concat!(
                "\n== CPU ==================",
                "\n| BC: {:>02X} {:>02X} | F:   {}{}{}{} |",
                "\n| DE: {:>02X} {:>02X} | IME: {:<4} |",
                "\n| HL: {:>02X} {:>02X} | PC:  {:>04X} |",
                "\n| AF: {:>02X} {:>02X} | SP:  {:>04X} |",
                "\n=========================",
            ),
            self.b,
            self.c,
            select!(self.f.z; "Z", "-"),
            select!(self.f.n; "N", "-"),
            select!(self.f.h; "H", "-"),
            select!(self.f.c; "C", "-"),
            self.d,
            self.e,
            select!(self.ime; "on", "off"),
            self.h,
            self.l,
            self.pc,
            self.a,
            self.f.as_byte(),
            self.sp
        );
    }
}

macro_rules! debug_interrupts {
    (on) => {
        log::debug!("interrupts enabled");
    };
    (..on) => {
        log::debug!("interrupts will be enabled");
    };
    (off) => {
        log::debug!("interrupts disabled");
    };
}
pub(crate) use debug_interrupts;

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::testutil::dummy_ctx;

    use super::*;

    macro_rules! r16_test {
        ($r1:ident + $r2:ident) => {
            paste::paste! {
                #[test]
                fn [<r16_ $r1 $r2 _test>]() {
                    let mut cpu = CPU::default();

                    assert_eq!(cpu.$r1, 0);
                    assert_eq!(cpu.$r2, 0);
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0);

                    cpu.[<set_ $r1 $r2>](0xDEAD);
                    assert_eq!(cpu.$r1, 0xDE);
                    assert_eq!(cpu.$r2, 0xAD);
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0xDEAD);

                    cpu.$r1 = 0xBE;
                    cpu.$r2 = 0xEF;
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0xBEEF);
                }
            }
        };
    }

    r16_test!(b + c);
    r16_test!(d + e);
    r16_test!(h + l);

    #[test]
    pub fn test_af() {
        let mut cpu = CPU::default();
        assert_eq!(cpu.get_af(), 0);

        cpu.set_af(0xDEAD);
        assert_eq!(cpu.a, 0xDE);
        assert_eq!(cpu.f.as_byte(), 0xA0);
        assert_eq!(cpu.get_af(), 0xDEA0);

        cpu.a = 0xBE;
        cpu.f = Flags {
            z: true,
            n: true,
            h: true,
            c: true,
        };
        assert_eq!(cpu.get_af(), 0xBEF0);
    }

    #[test]
    pub fn decode_test() {
        let ctx = &mut dummy_ctx();

        let address = 0xC0CA; // Put instruction in work ram
        let byte = 0xFE; // Instruction CP_A_n8

        ctx.cpu.pc = address;
        MMU::write(ctx, address, byte);

        assert_eq!(ctx.cpu.pc, address);
        let inst = CPU::decode(ctx);
        assert_eq!(ctx.cpu.pc, address + 1);
        assert_eq!(inst, Instruction::CP_A_n8)
    }

    #[test]
    pub fn decode_prefix_test() {
        let ctx = &mut dummy_ctx();

        let address = 0xC0BE; // Put instruction in work ram
        let prefix = 0xCB; // Instruction prefix
        let byte = 0xEF; // Instruction SET_5_A

        ctx.cpu.pc = address;
        MMU::write(ctx, address, prefix);
        MMU::write(ctx, address + 1, byte);

        assert!(!ctx.cpu.prefix_mode);
        assert_eq!(ctx.cpu.pc, address);
        let inst = CPU::decode(ctx);
        assert_eq!(ctx.cpu.pc, address + 1);
        assert_eq!(inst, Instruction::PREFIX);

        // "Execution" of the prefix instruction really is just a NOP and setting prefix mode
        ctx.cpu.prefix_mode = true;

        let inst = CPU::decode(ctx);
        assert_eq!(ctx.cpu.pc, address + 2);
        assert_eq!(inst, Instruction::SET_5_A);
        assert!(!ctx.cpu.prefix_mode);
    }

    #[test]
    fn test_halt_bug() {
        const ADDRESS: u16 = 0xCF00;

        let ctx = &mut dummy_ctx();

        ctx.cpu.pc = ADDRESS;
        for i in 0..3 {
            MMU::write(ctx, ADDRESS + i, i as u8);
        }

        assert_eq!(CPU::next_byte(ctx), 0);
        ctx.cpu.halt_bug = true;
        assert_eq!(CPU::next_byte(ctx), 1);
        assert_eq!(CPU::next_byte(ctx), 1);
        assert_eq!(CPU::next_byte(ctx), 2);
    }
}
