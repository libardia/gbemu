use crate::mem_region::{io_regs::*, regions::HIGH_RAM};

use super::MMU;

impl MMU {
    pub(super) fn high_ram_get(&self, address: u16) -> u8 {
        // TODO: high ram get
        match address {
            REG_JOYP => self.hram.get(REG_JOYP) & 0xF,
            // REG_JOYP:        ..WW RRRR
            // REG_SB:          XXXX XXXX
            // REG_SC:          X... ..XX
            // REG_DIV:         XXXX XXXX*
            // REG_TIMA:        XXXX XXXX
            // REG_TMA:         XXXX XXXX
            // REG_TAC:         .... .XXX
            // REG_IF:          XXXX XXXX
            // REG_NR10:        .XXX XXXX
            // REG_NR11:        XXRR RRRR
            // REG_NR12:        XXXX XXXX
            // REG_NR13:        WWWW WWWW
            // REG_NR14:        WX.. .WWW
            // REG_NR21:        XXRR RRRR
            // REG_NR22:        XXXX XXXX
            // REG_NR23:        WWWW WWWW
            // REG_NR24:        WX.. .WWW
            // REG_NR30:        X... ....
            // REG_NR31:        WWWW WWWW
            // REG_NR32:        .XX. ....
            // REG_NR33:        WWWW WWWW
            // REG_NR34:        WX.. .WWW
            // REG_NR41:        ..WW WWWW
            // REG_NR42:        XXXX XXXX
            // REG_NR43:        XXXX XXXX
            // REG_NR44:        WX.. ....
            // REG_NR50:        XXXX XXXX
            // REG_NR51:        XXXX XXXX
            // REG_NR52:        X... RRRR
            // REG_WAVE_RAM:    ALL FULLY R/W*
            // REG_LCDC:        XXXX XXXX
            // REG_STAT:        .XXX XXRR
            // REG_SCY:         XXXX XXXX
            // REG_SCX:         XXXX XXXX
            // REG_LY:          RRRR RRRR
            // REG_LYC:
            // REG_DMA:
            // REG_BGP:
            // REG_OBP0:
            // REG_OBP1:
            // REG_BANK:
            // REG_WY:
            // REG_WX:
            // REG_IE:
            // Fully R or just normal RAM:
            _ => self.hram.get(address),
        }
    }

    pub(super) fn high_ram_set(&mut self, address: u16, value: u8) {
        // TODO: high ram set
    }
}
