use crate::mem_region::io_regs::*;

use super::MMU;

impl MMU {
    pub(super) fn high_ram_get(&self, address: u16) -> u8 {
        const WRITE_ONLY_VALUE: u8 = 0xFF;

        macro_rules! get_bits {
            ($mask:expr) => {
                // Returns 1 in each unreadable bit
                self.hram.get(address) | !$mask
            };
        }

        match address {
            // FF00 = REG_JOYP:         ..WW RRRR
            REG_JOYP => get_bits!(0b0000_1111),
            // FF01 = REG_SB:           XXXX XXXX
            // FF02 = REG_SC:           X... ..XX
            REG_SC => get_bits!(0b1000_0011),
            // FF03
            // FF04 = REG_DIV:          XXXX XXXX*
            // FF05 = REG_TIMA:         XXXX XXXX
            // FF06 = REG_TMA:          XXXX XXXX
            // FF07 = REG_TAC:          .... .XXX
            REG_TAC => get_bits!(0b0000_0111),
            // FF08
            // ...
            // FF0E
            // FF0F = REG_IF:           ...X XXXX
            REG_IF => get_bits!(0b0001_1111),
            // FF10 = REG_NR10:         .XXX XXXX
            REG_NR10 => get_bits!(0b0111_1111),
            // FF11 = REG_NR11:         XXWW WWWW
            REG_NR11 => get_bits!(0b1100_0000),
            // FF12 = REG_NR12:         XXXX XXXX
            // FF13 = REG_NR13:         WWWW WWWW
            REG_NR13 => WRITE_ONLY_VALUE,
            // FF14 = REG_NR14:         WX.. .WWW
            REG_NR14 => get_bits!(0b0100_0000),
            // FF15
            // FF16 = REG_NR21:         XXWW WWWW
            REG_NR21 => get_bits!(0b1100_0000),
            // FF17 = REG_NR22:         XXXX XXXX
            // FF18 = REG_NR23:         WWWW WWWW
            REG_NR23 => WRITE_ONLY_VALUE,
            // FF19 = REG_NR24:         WX.. .WWW
            REG_NR24 => get_bits!(0b0100_0000),
            // FF1A = REG_NR30:         X... ....
            REG_NR30 => get_bits!(0b1000_0000),
            // FF1B = REG_NR31:         WWWW WWWW
            REG_NR31 => WRITE_ONLY_VALUE,
            // FF1C = REG_NR32:         .XX. ....
            REG_NR32 => get_bits!(0b0110_0000),
            // FF1D = REG_NR33:         WWWW WWWW
            REG_NR33 => WRITE_ONLY_VALUE,
            // FF1E = REG_NR34:         WX.. .WWW
            REG_NR34 => get_bits!(0b0100_0000),
            // FF1F
            // FF20 = REG_NR41:         ..WW WWWW
            REG_NR41 => WRITE_ONLY_VALUE,
            // FF21 = REG_NR42:         XXXX XXXX
            // FF22 = REG_NR43:         XXXX XXXX
            // FF23 = REG_NR44:         WX.. ....
            REG_NR44 => get_bits!(0b0100_0000),
            // FF24 = REG_NR50:         XXXX XXXX
            // FF25 = REG_NR51:         XXXX XXXX
            // FF26 = REG_NR52:         X... RRRR
            REG_NR52 => get_bits!(0b1000_1111),
            // FF27
            // ...
            // FF2F
            // FF30 = REG_WAVE_RAM:     XXXX XXXX
            // ...  = REG_WAVE_RAM:     XXXX XXXX
            // FF3F = REG_WAVE_RAM:     XXXX XXXX
            // FF40 = REG_LCDC:         XXXX XXXX
            // FF41 = REG_STAT:         .XXX XXRR
            REG_STAT => get_bits!(0b0111_1111),
            // FF42 = REG_SCY:          XXXX XXXX
            // FF43 = REG_SCX:          XXXX XXXX
            // FF44 = REG_LY:           RRRR RRRR
            // FF45 = REG_LYC:          XXXX XXXX
            // FF46 = REG_DMA:          XXXX XXXX
            // FF47 = REG_BGP:          XXXX XXXX
            // FF48 = REG_OBP0:         XXXX XXXX
            // FF49 = REG_OBP1:         XXXX XXXX
            // FF4A = REG_WY:           XXXX XXXX
            // FF4B = REG_WX:           XXXX XXXX
            // FF4C
            // ...
            // FF4F
            // FF50 = REG_BANK:         WWWW WWWW*
            REG_BANK => WRITE_ONLY_VALUE,
            // FF51
            // ...
            // FFFE
            // FFFF = REG_IE:           ...X XXXX
            REG_IE => get_bits!(0b0001_1111),

            // Fully readable or just normal RAM:
            _ => self.hram.get(address),
        }
    }

    pub(super) fn high_ram_set(&mut self, address: u16, value: u8) {
        macro_rules! set_bits {
            ($mask:expr) => {{
                let bits_to_write = value & $mask;
                let current_compl = self.hram.get(address) & !$mask;
                self.hram.set(address, current_compl | bits_to_write);
            }};
        }
        match address {
            // FF00 = REG_JOYP:         ..WW RRRR
            REG_JOYP => set_bits!(0b0011_0000),
            // FF01 = REG_SB:           XXXX XXXX
            // FF02 = REG_SC:           X... ..XX
            REG_SC => set_bits!(0b1000_0011),
            // FF03
            // FF04 = REG_DIV:          XXXX XXXX*
            REG_DIV => self.reset_div = true,
            // FF05 = REG_TIMA:         XXXX XXXX
            // FF06 = REG_TMA:          XXXX XXXX
            // FF07 = REG_TAC:          .... .XXX
            REG_TAC => set_bits!(0b0000_0111),
            // FF08
            // ...
            // FF0E
            // FF0F = REG_IF:           ...X XXXX
            REG_IF => set_bits!(0b0001_1111),
            // FF10 = REG_NR10:         .XXX XXXX
            REG_NR10 => set_bits!(0b0111_1111),
            // FF11 = REG_NR11:         XXWW WWWW
            // FF12 = REG_NR12:         XXXX XXXX
            // FF13 = REG_NR13:         WWWW WWWW
            // FF14 = REG_NR14:         WX.. .WWW
            REG_NR14 => set_bits!(0b1100_0111),
            // FF15
            // FF16 = REG_NR21:         XXWW WWWW
            // FF17 = REG_NR22:         XXXX XXXX
            // FF18 = REG_NR23:         WWWW WWWW
            // FF19 = REG_NR24:         WX.. .WWW
            REG_NR24 => set_bits!(0b1100_0111),
            // FF1A = REG_NR30:         X... ....
            REG_NR30 => set_bits!(0b1000_0000),
            // FF1B = REG_NR31:         WWWW WWWW
            // FF1C = REG_NR32:         .XX. ....
            REG_NR32 => set_bits!(0b0110_0000),
            // FF1D = REG_NR33:         WWWW WWWW
            // FF1E = REG_NR34:         WX.. .WWW
            REG_NR34 => set_bits!(0b1100_0111),
            // FF1F
            // FF20 = REG_NR41:         ..WW WWWW
            REG_NR41 => set_bits!(0b0011_1111),
            // FF21 = REG_NR42:         XXXX XXXX
            // FF22 = REG_NR43:         XXXX XXXX
            // FF23 = REG_NR44:         WX.. ....
            REG_NR44 => set_bits!(0b1100_0000),
            // FF24 = REG_NR50:         XXXX XXXX
            // FF25 = REG_NR51:         XXXX XXXX
            // FF26 = REG_NR52:         X... RRRR
            REG_NR52 => set_bits!(0b1000_0000),
            // FF27
            // ...
            // FF2F
            // FF30 = REG_WAVE_RAM:     XXXX XXXX
            // ...  = REG_WAVE_RAM:     XXXX XXXX
            // FF3F = REG_WAVE_RAM:     XXXX XXXX
            // FF40 = REG_LCDC:         XXXX XXXX
            // FF41 = REG_STAT:         .XXX XXRR
            REG_STAT => set_bits!(0b0111_1100),
            // FF42 = REG_SCY:          XXXX XXXX
            // FF43 = REG_SCX:          XXXX XXXX
            // FF44 = REG_LY:           RRRR RRRR
            REG_LY => (/* Do nothing */),
            // FF45 = REG_LYC:          XXXX XXXX
            // FF46 = REG_DMA:          XXXX XXXX
            REG_DMA => {
                self.hram.set(address, value);
                self.execute_dma = true;
            }
            // FF47 = REG_BGP:          XXXX XXXX
            // FF48 = REG_OBP0:         XXXX XXXX
            // FF49 = REG_OBP1:         XXXX XXXX
            // FF4A = REG_WY:           XXXX XXXX
            // FF4B = REG_WX:           XXXX XXXX
            // FF4C
            // ...
            // FF4F
            // FF50 = REG_BANK:         WWWW WWWW*
            REG_BANK => {
                if self.boot_mode && value != 0 {
                    self.boot_mode = false;
                    self.hram.set(address, value);
                }
            }
            // FF51
            // ...
            // FFFE
            // FFFF = REG_IE:           ...X XXXX
            REG_IE => set_bits!(0b0001_1111),

            // Fully writable or just normal RAM:
            _ => self.hram.set(address, value),
        }
    }
}
