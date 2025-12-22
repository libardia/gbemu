use crate::memory::{
    OPEN_BUS_VALUE, UNINIT_VALUE,
    regions::{IO_REGS, MappedMemoryRegion, MemoryRegion},
};

pub const JOYP: u16 = 0xFF00;
pub const SB: u16 = 0xFF01;
pub const SC: u16 = 0xFF02;
pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;
pub const IF: u16 = 0xFF0F;
pub const NR10: u16 = 0xFF10;
pub const NR11: u16 = 0xFF11;
pub const NR12: u16 = 0xFF12;
pub const NR13: u16 = 0xFF13;
pub const NR14: u16 = 0xFF14;
pub const NR21: u16 = 0xFF16;
pub const NR22: u16 = 0xFF17;
pub const NR23: u16 = 0xFF18;
pub const NR24: u16 = 0xFF19;
pub const NR30: u16 = 0xFF1A;
pub const NR31: u16 = 0xFF1B;
pub const NR32: u16 = 0xFF1C;
pub const NR33: u16 = 0xFF1D;
pub const NR34: u16 = 0xFF1E;
pub const NR41: u16 = 0xFF20;
pub const NR42: u16 = 0xFF21;
pub const NR43: u16 = 0xFF22;
pub const NR44: u16 = 0xFF23;
pub const NR50: u16 = 0xFF24;
pub const NR51: u16 = 0xFF25;
pub const NR52: u16 = 0xFF26;
pub const WAVE_RAM: MemoryRegion = MemoryRegion::new(0xFF30, 0xFF3F);
pub const LCDC: u16 = 0xFF40;
pub const STAT: u16 = 0xFF41;
pub const SCY: u16 = 0xFF42;
pub const SCX: u16 = 0xFF43;
pub const LY: u16 = 0xFF44;
pub const LYC: u16 = 0xFF45;
pub const DMA: u16 = 0xFF46;
pub const BGP: u16 = 0xFF47;
pub const OBP0: u16 = 0xFF48;
pub const OBP1: u16 = 0xFF49;
pub const WY: u16 = 0xFF4A;
pub const WX: u16 = 0xFF4B;
pub const BANK: u16 = 0xFF50;
pub const IE: u16 = 0xFFFF;

#[derive(Debug)]
pub struct HardwareRegs {
    io_raw: MappedMemoryRegion,
    // This is actually at the end of the mem at 0xFFFF, outside the IO range
    ie: u8,
}

impl Default for HardwareRegs {
    fn default() -> Self {
        Self {
            io_raw: MappedMemoryRegion::new(IO_REGS),
            ie: UNINIT_VALUE,
        }
    }
}

impl HardwareRegs {
    // Return the value of the register, unmasked and without side effects.
    pub fn peek(&self, address: u16) -> u8 {
        if address == IE {
            self.ie
        } else {
            self.io_raw.get(address)
        }
    }

    // Set the value of the register, unmasked and without side effects.
    pub fn poke(&mut self, address: u16, value: u8) {
        if address == IE {
            self.ie = value;
        } else {
            self.io_raw.set(address, value);
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        macro_rules! get_bits {
            ($mask:expr) => {
                // Get only the bits which are 1 in the mask
                // Returns 1 in each unreadable bit
                self.peek(address) | !$mask
            };
            () => {
                // Fully readable
                self.peek(address)
            };
        }

        match address {
            JOYP => get_bits!(0b00111111),
            SB => get_bits!(),
            SC => get_bits!(0b10000001),
            DIV => get_bits!(),
            TIMA => get_bits!(),
            TMA => get_bits!(),
            TAC => get_bits!(0b00000111),
            IF => get_bits!(0b00011111),
            NR10 => get_bits!(0b01111111),
            NR11 => get_bits!(0b11000000),
            NR12 => get_bits!(),
            // NR13 is not readable!
            NR14 => get_bits!(0b01000000),
            NR21 => get_bits!(0b11000000),
            NR22 => get_bits!(),
            // NR23 is not readable!
            NR24 => get_bits!(0b01000000),
            NR30 => get_bits!(0b10000000),
            // NR31 is not readable!
            NR32 => get_bits!(0b01100000),
            // NR33 is not readable!
            NR34 => get_bits!(0b01000000),
            // NR41 is not readable!
            NR42 => get_bits!(),
            NR43 => get_bits!(),
            NR44 => get_bits!(0b01000000),
            NR50 => get_bits!(),
            NR51 => get_bits!(),
            NR52 => get_bits!(0b10001111),
            _ if WAVE_RAM.contains(address) => get_bits!(),
            LCDC => get_bits!(),
            STAT => get_bits!(0b01111111),
            SCY => get_bits!(),
            SCX => get_bits!(),
            LY => get_bits!(),
            LYC => get_bits!(),
            DMA => get_bits!(),
            BGP => get_bits!(),
            OBP0 => get_bits!(),
            OBP1 => get_bits!(),
            WY => get_bits!(),
            WX => get_bits!(),
            // BANK is not readable!
            IE => get_bits!(0b00011111),

            // Either not a register or not readable
            _ => OPEN_BUS_VALUE,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        macro_rules! set_bits {
            ($mask:expr) => {{
                // Current byte: Bits to be set are reset to 0, all other bits are unaffected
                let current_masked = self.peek(address) & !$mask;
                // New byte: Bits NOT to be set are reset to 0, other bits are unaffected
                let value_masked = value & $mask;
                self.poke(address, current_masked | value_masked);
            }};
            () => {
                // Fully writable
                self.poke(address, value)
            };
        }

        match address {
            //TODO: IO regs write
            JOYP => set_bits!(0b00110000),
            SB => set_bits!(),
            SC => set_bits!(0b10000001),
            DIV => set_bits!(), // SIDE EFFECT: When any value is set, DIV resets to 0
            TIMA => set_bits!(),
            TMA => set_bits!(),
            TAC => set_bits!(0b00000111), // SIDE EFFECT: Potentially causes a timer tick
            IF => set_bits!(0b00011111),
            NR10 => set_bits!(0b01111111),
            NR11 => set_bits!(),
            NR12 => todo!(),
            NR13 => todo!(),
            NR14 => todo!(),
            NR21 => todo!(),
            NR22 => todo!(),
            NR23 => todo!(),
            NR24 => todo!(),
            NR30 => todo!(),
            NR31 => todo!(),
            NR32 => todo!(),
            NR33 => todo!(),
            NR34 => todo!(),
            NR41 => todo!(),
            NR42 => todo!(),
            NR43 => todo!(),
            NR44 => todo!(),
            NR50 => todo!(),
            NR51 => todo!(),
            NR52 => todo!(),
            _ if WAVE_RAM.contains(address) => todo!(),
            LCDC => todo!(),
            STAT => todo!(),
            SCY => todo!(),
            SCX => todo!(),
            LY => todo!(),
            LYC => todo!(),
            DMA => todo!(),
            BGP => todo!(),
            OBP0 => todo!(),
            OBP1 => todo!(),
            WY => todo!(),
            WX => todo!(),
            BANK => todo!(),
            IE => todo!(),

            // Either not a register or not writable
            _ => (),
        }
    }
}
