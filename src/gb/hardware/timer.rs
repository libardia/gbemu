use crate::{
    define_reg_bits,
    gb::{
        GameBoy,
        hardware::{HardwareInit, HardwareInterface},
        registers::{IO_DIV, IO_TAC, IO_TIMA, IO_TMA},
    },
    impossible_address, warn_unimplemented_interface, warn_unimplemented_write,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Default, FromPrimitive, Clone, Copy, PartialEq, Eq)]
enum TACClock {
    #[default]
    Every256 = 0b00,
    Every4 = 0b01,
    Every16 = 0b10,
    Every64 = 0b11,
}

#[derive(Debug, Default)]
pub struct Timer {
    // TODO: Timer
    system_timer: u16,

    tima: u8,
    tma: u8,

    tac_enable: bool,
    tac_clock_select: TACClock,
}

define_reg_bits!(
    for TAC:
        ENABLE:
            width: 0b1;
            pos: 2;
            field: tac_enable: bool;
            to_u8: e => { e as u8 };
            from_u8: e => { e != 0 };
        CLOCK_SELECT:
            width: 0b11;
            pos: 0;
            field: tac_clock_select: TACClock;
            to_u8: c => { c as u8 };
            from_u8: c => { TACClock::from_u8(c).unwrap() };
);

impl HardwareInit for Timer {
    fn init(ctx: &mut GameBoy) {
        warn_unimplemented_interface!("Timer");
        ctx.timer.tac_clock_select = TACClock::Every256;
        ctx.timer.tac_enable = false;

        if ctx.skip_boot {
            // Lower 8 bits that this should be is unknown right now
            ctx.timer.system_timer = 0xAB00;
        }
    }
}

impl HardwareInterface for Timer {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Timer read
        match address {
            IO_DIV => ((ctx.timer.system_timer & 0xFF00) >> 8) as u8, // Top 8 bits of the system timer
            IO_TIMA => ctx.timer.tima,
            IO_TMA => ctx.timer.tma,
            IO_TAC => make_reg_TAC!(ctx.timer),

            _ => impossible_address!("Timer", address),
        }
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Timer write
        warn_unimplemented_write!(ctx, "Timer", address, value);
        match address {
            IO_DIV => ctx.timer.system_timer = 0, // Top 8 bits of the system timer
            IO_TIMA => ctx.timer.tima = value,
            IO_TMA => ctx.timer.tma = value,
            IO_TAC => decomp_reg_TAC!(ctx.timer, value),

            _ => impossible_address!("Timer", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    use test_log::test;

    #[test]
    fn test_read_tac() {
        for i in 0..=0b111u8 {
            let tac_enable = (i & 0b100) != 0;
            let tac_clock_select = TACClock::from_u8(i & 0b011).unwrap();
            let t = Timer {
                system_timer: 0,
                tima: 0,
                tma: 0,
                tac_enable,
                tac_clock_select,
            };

            debug!("{i:0>3b} => {t:?}");

            let expected = ((tac_enable as u8) << TAC_ENABLE_POS)
                | ((tac_clock_select as u8) << TAC_CLOCK_SELECT_POS)
                | TAC_UNUSED_BITS;

            debug!("Expecting: {expected:0>8b}");
            assert_eq!(make_reg_TAC!(t), expected);
        }
    }

    #[test]
    fn test_write_tac() {
        let mut t = Timer {
            system_timer: 0,
            tima: 0,
            tma: 0,
            tac_enable: true,
            tac_clock_select: TACClock::Every64,
        };

        for i in 0..=0b111u8 {
            let b0 = (i & 0b100) >> 2;
            let b12 = i & 0b011;

            let value = (b0 << TAC_ENABLE_POS) | (b12 << TAC_CLOCK_SELECT_POS) | TAC_UNUSED_BITS;

            debug!("Before: {t:?}");
            debug!("Value:  {value:0>8b}");
            decomp_reg_TAC!(t, value);
            debug!("After:  {t:?}");

            assert_eq!(t.tac_enable, b0 != 0);
            assert_eq!(t.tac_clock_select, TACClock::from_u8(b12).unwrap());
        }
    }

    #[test]
    fn test_tacclock_from_primitive() {
        let test_defs = [
            (0b00, TACClock::Every256),
            (0b01, TACClock::Every4),
            (0b10, TACClock::Every16),
            (0b11, TACClock::Every64),
        ];
        for (value, tac_clock) in test_defs {
            let from_u8: TACClock = TACClock::from_u8(value).unwrap();
            log::debug!("{:0>2b} => {:?}, should be {:?}", value, from_u8, tac_clock);
            assert_eq!(from_u8, tac_clock);
        }
    }
}
