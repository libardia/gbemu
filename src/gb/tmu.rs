use crate::{
    gb::{
        GameBoy,
        hardware_interface::HardwareInterface,
        mmu::io::{IO_DIV, IO_TAC, IO_TIMA, IO_TMA},
    },
    macros::{hex, select},
};

const TAC_CLOCK_4: u8 = 0b01;
const TAC_CLOCK_16: u8 = 0b10;
const TAC_CLOCK_64: u8 = 0b11;
const TAC_CLOCK_256: u8 = 0b00;

#[derive(Debug, Default)]
pub struct TMU {
    pub system_timer: u16,

    pub tac_enable: bool,
    pub tac_clock: u8,
    pub tma: u8,
    pub tima: u8,

    pub last_tick_bit: bool,
}

impl TMU {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tick(ctx: &mut GameBoy) {
        ctx.tmu.inc_systimer();
        // TODO: more stuff to do?
    }

    pub fn inc_systimer(&mut self) {
        self.set_systimer(self.system_timer + 1);
    }

    pub fn set_systimer(&mut self, value: u16) {
        // The system timer is actually only 14 bits
        self.system_timer = select!(value > 0x3FFF; 0, value);
        self.check_tima();
    }

    pub fn check_tima(&mut self) {
        let tick_bit = self.tick_bit();
        if self.last_tick_bit && !tick_bit {
            // TODO: TIMA tick
        }
        self.last_tick_bit = tick_bit;
    }
}

impl HardwareInterface for TMU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read TMU
        match address {
            IO_DIV => self.get_div(),
            IO_TIMA => self.tima,
            IO_TMA => self.tma,
            IO_TAC => self.pack_tac(),

            _ => unimplemented!("can't read {} from TMU", hex!(address, 4)),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write TMU
        match address {
            IO_DIV => self.set_systimer(0),
            IO_TIMA => self.tima = byte,
            IO_TMA => self.tma = byte,
            IO_TAC => self.unpack_tac(byte),

            _ => unimplemented!(
                "can't write {} to {} in TMU",
                hex!(byte, 2),
                hex!(address, 4),
            ),
        };
    }
}

impl TMU {
    pub fn get_div(&self) -> u8 {
        // DIV is the top 8 bits of the system timer,
        // but the system timer is only 14 bits
        ((self.system_timer & 0x3FFF) >> 6) as u8
    }

    pub fn pack_tac(&self) -> u8 {
        0xFF & (self.tac_enable as u8) << 2 & self.tac_clock
    }

    pub fn unpack_tac(&mut self, byte: u8) {
        self.tac_enable = byte & 0b100 != 0;
        self.tac_clock = byte & 0b11;
        self.check_tima();
    }

    pub fn tick_bit(&self) -> bool {
        self.tac_enable
            && match self.tac_clock {
                TAC_CLOCK_4 => self.system_timer & (1 << 3) != 0,
                TAC_CLOCK_16 => self.system_timer & (1 << 5) != 0,
                TAC_CLOCK_64 => self.system_timer & (1 << 7) != 0,
                TAC_CLOCK_256 => self.system_timer & (1 << 9) != 0,
                _ => unimplemented!("bad tac clock value: {}", self.tac_clock),
            }
    }
}
