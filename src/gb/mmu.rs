mod boot_rom;
mod mbc;

use crate::gb::{
    macros::{address_fmt, error_panic, new},
    regions::*,
    types::TTime,
};
use boot_rom::BOOT_ROM;
use log::{error, warn};
use mbc::MBC;

const OPEN_BUS_VALUE: u8 = 0xFF;

#[derive(Debug, Default)]
pub enum AccessMode {
    #[default]
    Full,
    CPU,
    PPU,
    DMA,
}

#[derive(Debug, Default)]
pub struct MMU {
    pub access_mode: AccessMode,
    mbc: Option<Box<dyn MBC>>,
    raw_ram: Vec<u8>,
    pub boot_mode: bool,
    pub dma_block: bool,
    pub ppu_vram_block: bool,
    pub ppu_oam_block: bool,
    system_timer: TTime,
}

impl MMU {
    new!(
        raw_ram = vec![0xFF; ALL_RAM.usize() - ROM_SPACE.usize() - ECHO_RAM.usize()];
        boot_mode = true;
        ...
    );

    pub fn init_io(&mut self) {
        for (reg, def) in IO_DEFAULTS {
            self.raw_ram[adjust_address(reg)] = def;
        }
    }

    pub fn add_time(&mut self, ticks: u16) {
        self.system_timer = self.system_timer.wrapping_add(ticks);
    }

    pub fn get(&self, address: u16) -> u8 {
        match self.access_mode {
            AccessMode::Full => self.get_full(address),
            AccessMode::CPU => self.get_cpu(address),
            AccessMode::PPU => self.get_ppu(address),
            AccessMode::DMA => self.get_dma(address),
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        match self.access_mode {
            AccessMode::Full => self.set_full(address, value),
            AccessMode::CPU => self.set_cpu(address, value),
            AccessMode::PPU => self.set_ppu(address, value),
            AccessMode::DMA => self.set_dma(address, value),
        }
    }

    /* #region CPU */
    fn get_raw(&self, address: u16) -> u8 {
        self.raw_ram[adjust_address(address)]
    }

    fn set_raw(&mut self, address: u16, value: u8) {
        self.raw_ram[adjust_address(address)] = value;
    }

    fn get_full(&self, address: u16) -> u8 {
        if HIGH_RAM.contains(address) {
            self.hram_get(address)
        } else {
            self.get_raw(address)
        }
    }

    fn set_full(&mut self, address: u16, value: u8) {
        if HIGH_RAM.contains(address) {
            self.hram_set(address, value);
        } else {
            self.set_raw(address, value);
        }
    }

    fn get_cpu(&self, address: u16) -> u8 {
        // CPU range:
        //      Everything
        // CPU can't read:
        //      Anything but some of high ram during DMA
        //      VRAM, during PPU's mode 3
        //      OAM, during PPU's mode 2 or 3
        if self.dma_block && !DMA_USABLE.contains(address) {
            // Everything except a part of HRAM is unavailable during DMA transfer
            warn!(
                "Blocked CPU read from {} during DMA; all but {} - {} is blocked during DMA.",
                address_fmt!(address),
                address_fmt!(DMA_USABLE.begin),
                address_fmt!(DMA_USABLE.end),
            );
            OPEN_BUS_VALUE
        } else if self.ppu_oam_block && OAM.contains(address) {
            // Can't read from OAM while PPU is on mode 2 or 3
            warn!(
                "Blocked CPU read from OAM at {} while PPU was busy.",
                address_fmt!(address)
            );
            OPEN_BUS_VALUE
        } else if self.ppu_vram_block && VRAM.contains(address) {
            // Can't read from VRAM while PPU is on mode 3
            warn!(
                "Blocked CPU read from VRAM at {} while PPU was busy.",
                address_fmt!(address)
            );
            OPEN_BUS_VALUE
        } else if self.boot_mode && BOOT_ROM_BANK.contains(address) {
            // Map the boot ROM over cart ROM when in boot mode
            BOOT_ROM[address as usize]
        } else if ROM_SPACE.contains(address) {
            match self.mbc.as_ref() {
                Some(mbc) => mbc.get(address),
                None => {
                    error!(
                        "CPU tried to read from {} but no MBC is defined! Is there a cartridge?",
                        address_fmt!(address)
                    );
                    OPEN_BUS_VALUE
                }
            }
        } else {
            // Any other address
            self.get_full(address)
        }
    }

    fn set_cpu(&mut self, address: u16, value: u8) {
        // CPU range:
        //      Everything
        // CPU can't write:
        //      Anything but some of high ram during DMA
        //      VRAM, during PPU's mode 3
        //      OAM, during PPU's mode 2 or 3
        if self.dma_block && !DMA_USABLE.contains(address) {
            // Everything except a part of HRAM is unavailable during DMA transfer
            warn!(
                "Blocked CPU write to {} during DMA; all but {} - {} is blocked during DMA.",
                address_fmt!(address),
                address_fmt!(DMA_USABLE.begin),
                address_fmt!(DMA_USABLE.end),
            );
        } else if self.ppu_oam_block && OAM.contains(address) {
            // Can't write to OAM while PPU is on mode 2 or 3
            warn!(
                "Blocked CPU write to OAM at {} while PPU was busy.",
                address_fmt!(address)
            );
        } else if self.ppu_vram_block && VRAM.contains(address) {
            // Can't write to VRAM while PPU is on mode 3
            warn!(
                "Blocked CPU write to VRAM at {} while PPU was busy.",
                address_fmt!(address)
            );
        } else if ROM_SPACE.contains(address) {
            match self.mbc.as_mut() {
                Some(mbc) => mbc.set(address, value),
                None => {
                    error_panic!(
                        "CPU tried to write to {} but no MBC is defined! Is there a cartridge?",
                        address_fmt!(address)
                    );
                }
            }
        } else {
            // Any other address
            self.set_full(address, value);
        };
    }

    /* #endregion */

    /* #region PPU */

    fn get_ppu(&self, address: u16) -> u8 {
        // PPU range:
        //      VRAM and OAM
        // PPU can't read:
        //      Anything during DMA
        if self.dma_block {
            warn!("Blocked PPU read at {} during DMA.", address_fmt!(address));
            OPEN_BUS_VALUE
        } else {
            self.get_full(address)
        }
    }

    fn set_ppu(&mut self, address: u16, value: u8) {
        // PPU range:
        //      VRAM and OAM
        // PPU shouldn't write:
        //      Anything, except some registers in high ram
        if self.dma_block {
            warn!("Blocked PPU write at {} during DMA.", address_fmt!(address));
        } else {
            self.set_full(address, value);
        }
    }

    /* #endregion */

    /* #region DMA */

    fn get_dma(&self, address: u16) -> u8 {
        // DMA range:
        //      Start addresses: XX00, XX from 00 to DF (so, any ROM or RAM)
        //        160 (A0) byte length from the start
        //      Destination: OAM
        // DMA can't read:
        //      VRAM, during PPU's mode 3
        if self.ppu_vram_block && VRAM.contains(address) {
            warn!(
                "Blocked DMA read at {} while PPU was busy.",
                address_fmt!(address)
            );
            OPEN_BUS_VALUE
        } else if ROM_SPACE.contains(address) {
            match self.mbc.as_ref() {
                Some(mbc) => mbc.get(address),
                None => {
                    error_panic!(
                        "DMA tried to read from {} but no MBC is defined! Is there a cartridge?",
                        address_fmt!(address)
                    );
                }
            }
        } else {
            self.get_full(address)
        }
    }

    fn set_dma(&mut self, address: u16, value: u8) {
        // DMA range:
        //      Start addresses: XX00, XX from 00 to DF (so, any ROM or RAM)
        //        160 (A0) byte length from the start
        //      Destination: OAM
        // DMA should't write:
        //      Anywhere except OAM and some registers in HRAM
        if self.ppu_vram_block && VRAM.contains(address) {
            warn!(
                "Blocked DMA write at {} while PPU was busy.",
                address_fmt!(address)
            );
        } else {
            self.set_full(address, value);
        }
    }

    /* #endregion */

    /* #region High RAM */
    fn hram_get(&self, address: u16) -> u8 {
        const WRITE_ONLY_VALUE: u8 = 0xFF;

        macro_rules! get_bits {
            ($mask:expr) => {
                // Returns 1 in each unreadable bit
                self.get_raw(address) | !$mask
            };
        }

        match address {
            // FF00 = IO_JOYP:         ..XX RRRR
            IO_JOYP => self.get_joyp(),
            // FF01 = IO_SB:           XXXX XXXX
            // FF02 = IO_SC:           X... ...X
            IO_SC => get_bits!(0b1000_0001),
            // FF03
            // FF04 = IO_DIV:          XXXX XXXX*
            IO_DIV => (self.system_timer >> 8) as u8,
            // FF05 = IO_TIMA:         XXXX XXXX
            // FF06 = IO_TMA:          XXXX XXXX
            // FF07 = IO_TAC:          .... .XXX
            IO_TAC => get_bits!(0b0000_0111),
            // FF08
            // ...
            // FF0E
            // FF0F = IO_IF:           ...X XXXX
            IO_IF => get_bits!(0b0001_1111),
            // FF10 = IO_NR10:         .XXX XXXX
            IO_NR10 => get_bits!(0b0111_1111),
            // FF11 = IO_NR11:         XXWW WWWW
            IO_NR11 => get_bits!(0b1100_0000),
            // FF12 = IO_NR12:         XXXX XXXX
            // FF13 = IO_NR13:         WWWW WWWW
            IO_NR13 => WRITE_ONLY_VALUE,
            // FF14 = IO_NR14:         WX.. .WWW
            IO_NR14 => get_bits!(0b0100_0000),
            // FF15
            // FF16 = IO_NR21:         XXWW WWWW
            IO_NR21 => get_bits!(0b1100_0000),
            // FF17 = IO_NR22:         XXXX XXXX
            // FF18 = IO_NR23:         WWWW WWWW
            IO_NR23 => WRITE_ONLY_VALUE,
            // FF19 = IO_NR24:         WX.. .WWW
            IO_NR24 => get_bits!(0b0100_0000),
            // FF1A = IO_NR30:         X... ....
            IO_NR30 => get_bits!(0b1000_0000),
            // FF1B = IO_NR31:         WWWW WWWW
            IO_NR31 => WRITE_ONLY_VALUE,
            // FF1C = IO_NR32:         .XX. ....
            IO_NR32 => get_bits!(0b0110_0000),
            // FF1D = IO_NR33:         WWWW WWWW
            IO_NR33 => WRITE_ONLY_VALUE,
            // FF1E = IO_NR34:         WX.. .WWW
            IO_NR34 => get_bits!(0b0100_0000),
            // FF1F
            // FF20 = IO_NR41:         ..WW WWWW
            IO_NR41 => WRITE_ONLY_VALUE,
            // FF21 = IO_NR42:         XXXX XXXX
            // FF22 = IO_NR43:         XXXX XXXX
            // FF23 = IO_NR44:         WX.. ....
            IO_NR44 => get_bits!(0b0100_0000),
            // FF24 = IO_NR50:         XXXX XXXX
            // FF25 = IO_NR51:         XXXX XXXX
            // FF26 = IO_NR52:         X... RRRR
            IO_NR52 => get_bits!(0b1000_1111),
            // FF27
            // ...
            // FF2F
            // FF30 = IO_WAVE_RAM:     XXXX XXXX
            // ...  = IO_WAVE_RAM:     XXXX XXXX
            // FF3F = IO_WAVE_RAM:     XXXX XXXX
            // FF40 = IO_LCDC:         XXXX XXXX
            // FF41 = IO_STAT:         .XXX XXRR
            IO_STAT => get_bits!(0b0111_1111),
            // FF42 = IO_SCY:          XXXX XXXX
            // FF43 = IO_SCX:          XXXX XXXX
            // FF44 = IO_LY:           RRRR RRRR
            // FF45 = IO_LYC:          XXXX XXXX
            // FF46 = IO_DMA:          XXXX XXXX
            // FF47 = IO_BGP:          XXXX XXXX
            // FF48 = IO_OBP0:         XXXX XXXX
            // FF49 = IO_OBP1:         XXXX XXXX
            // FF4A = IO_WY:           XXXX XXXX
            // FF4B = IO_WX:           XXXX XXXX
            // FF4C
            // ...
            // FF4F
            // FF50 = IO_BANK:         WWWW WWWW*
            IO_BANK => WRITE_ONLY_VALUE,
            // FF51
            // ...
            // FFFE
            // FFFF = IO_IE:           ...X XXXX
            IO_IE => get_bits!(0b0001_1111),

            // Fully readable or just normal RAM:
            _ => self.get_raw(address),
        }
    }

    fn hram_set(&mut self, address: u16, value: u8) {
        macro_rules! set_bits {
            ($mask:expr) => {{
                let bits_to_write = value & $mask;
                let current_compl = self.get_raw(address) & !$mask;
                self.set_raw(address, current_compl | bits_to_write);
            }};
        }
        match address {
            // FF00 = IO_JOYP:         ..XX RRRR
            IO_JOYP => set_bits!(0b0011_0000),
            // FF01 = IO_SB:           XXXX XXXX
            // FF02 = IO_SC:           X... ...X
            IO_SC => set_bits!(0b1000_0001),
            // FF03
            // FF04 = IO_DIV:          XXXX XXXX*
            IO_DIV => self.system_timer = 0,
            // FF05 = IO_TIMA:         XXXX XXXX
            // FF06 = IO_TMA:          XXXX XXXX
            // FF07 = IO_TAC:          .... .XXX
            IO_TAC => set_bits!(0b0000_0111),
            // FF08
            // ...
            // FF0E
            // FF0F = IO_IF:           ...X XXXX
            IO_IF => set_bits!(0b0001_1111),
            // FF10 = IO_NR10:         .XXX XXXX
            IO_NR10 => set_bits!(0b0111_1111),
            // FF11 = IO_NR11:         XXWW WWWW
            // FF12 = IO_NR12:         XXXX XXXX
            // FF13 = IO_NR13:         WWWW WWWW
            // FF14 = IO_NR14:         WX.. .WWW
            IO_NR14 => set_bits!(0b1100_0111),
            // FF15
            // FF16 = IO_NR21:         XXWW WWWW
            // FF17 = IO_NR22:         XXXX XXXX
            // FF18 = IO_NR23:         WWWW WWWW
            // FF19 = IO_NR24:         WX.. .WWW
            IO_NR24 => set_bits!(0b1100_0111),
            // FF1A = IO_NR30:         X... ....
            IO_NR30 => set_bits!(0b1000_0000),
            // FF1B = IO_NR31:         WWWW WWWW
            // FF1C = IO_NR32:         .XX. ....
            IO_NR32 => set_bits!(0b0110_0000),
            // FF1D = IO_NR33:         WWWW WWWW
            // FF1E = IO_NR34:         WX.. .WWW
            IO_NR34 => set_bits!(0b1100_0111),
            // FF1F
            // FF20 = IO_NR41:         ..WW WWWW
            IO_NR41 => set_bits!(0b0011_1111),
            // FF21 = IO_NR42:         XXXX XXXX
            // FF22 = IO_NR43:         XXXX XXXX
            // FF23 = IO_NR44:         WX.. ....
            IO_NR44 => set_bits!(0b1100_0000),
            // FF24 = IO_NR50:         XXXX XXXX
            // FF25 = IO_NR51:         XXXX XXXX
            // FF26 = IO_NR52:         X... RRRR
            IO_NR52 => set_bits!(0b1000_0000),
            // FF27
            // ...
            // FF2F
            // FF30 = IO_WAVE_RAM:     XXXX XXXX
            // ...  = IO_WAVE_RAM:     XXXX XXXX
            // FF3F = IO_WAVE_RAM:     XXXX XXXX
            // FF40 = IO_LCDC:         XXXX XXXX
            // FF41 = IO_STAT:         .XXX XXRR
            IO_STAT => set_bits!(0b0111_1100),
            // FF42 = IO_SCY:          XXXX XXXX
            // FF43 = IO_SCX:          XXXX XXXX
            // FF44 = IO_LY:           RRRR RRRR
            IO_LY => (/* Do nothing */),
            // FF45 = IO_LYC:          XXXX XXXX
            // FF46 = IO_DMA:          XXXX XXXX
            IO_DMA => {
                // TODO: self.execute_dma = true;
            }
            // FF47 = IO_BGP:          XXXX XXXX
            // FF48 = IO_OBP0:         XXXX XXXX
            // FF49 = IO_OBP1:         XXXX XXXX
            // FF4A = IO_WY:           XXXX XXXX
            // FF4B = IO_WX:           XXXX XXXX
            // FF4C
            // ...
            // FF4F
            // FF50 = IO_BANK:         WWWW WWWW*
            IO_BANK => {
                if self.boot_mode && value != 0 {
                    self.boot_mode = false;
                }
            }
            // FF51
            // ...
            // FFFE
            // FFFF = IO_IE:           ...X XXXX
            IO_IE => set_bits!(0b0001_1111),

            // Fully writable or just normal RAM:
            _ => self.set_raw(address, value),
        }
    }
    /* #endregion */
}

/* #region Raw indexing */

impl MMU {
    fn get_joyp(&self) -> u8 {
        // TODO: Return currently held AND SELECTED buttons
        // Returns only the last 4 bits: 0 means button is held, 1 is not held
        let joyp = self.get_raw(IO_JOYP);
        let butt = (joyp & 0b0010_0000) != 0;
        let dpad = (joyp & 0b0001_0000) != 0;
        0b1100_1111 & ((butt as u8) << 5) & ((dpad as u8) << 4)
    }
}

fn adjust_address(address: u16) -> usize {
    let folded = if address >= ECHO_RAM.begin {
        if address <= ECHO_RAM.end {
            address - (ECHO_RAM.begin - WORK_RAM.begin)
        } else {
            address - ECHO_RAM.size()
        }
    } else {
        address
    };
    (folded - ROM_SPACE.size()) as usize
}

/* #endregion */
