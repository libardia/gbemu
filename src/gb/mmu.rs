mod boot_rom;
mod mbc;
mod regions;

use crate::macros::{address_fmt, error_panic, new};
use boot_rom::BOOT_ROM;
use log::warn;
use mbc::MBC;
use regions::*;
use std::ops::{Index, IndexMut};

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
}

impl MMU {
    new!(
        raw_ram = vec![0xFF; ALL_RAM.usize() - ROM_SPACE.usize() - ECHO_RAM.usize()];
        boot_mode = true;
        ...
    );

    pub fn get(&self, address: u16) -> u8 {
        match self.access_mode {
            AccessMode::Full => self[address],
            AccessMode::CPU => self.get_cpu(address),
            AccessMode::PPU => self.get_ppu(address),
            AccessMode::DMA => self.get_dma(address),
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        match self.access_mode {
            AccessMode::Full => self[address] = value,
            AccessMode::CPU => self.set_cpu(address, value),
            AccessMode::PPU => self.set_ppu(address, value),
            AccessMode::DMA => self.set_dma(address, value),
        }
    }

    pub fn get_word(&self, address: u16) -> u16 {
        let low = self.get(address) as u16;
        let high = self.get(address + 1) as u16;
        (high << 8) | low
    }

    /* #region CPU */

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
                    error_panic!(
                        "CPU tried to read from {} but no MBC is defined! Is there a cartridge?",
                        address_fmt!(address)
                    );
                }
            }
        } else {
            // Any other address
            self[address]
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
            self[address] = value;
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
            self[address]
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
            self[address] = value;
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
            self[address]
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
            self[address] = value;
        }
    }

    /* #endregion */
}

/* #region Raw indexing */

impl MMU {
    fn adjust_address(address: u16) -> u16 {
        let folded = if address >= ECHO_RAM.begin {
            if address <= ECHO_RAM.end {
                address - (ECHO_RAM.begin - WORK_RAM.begin)
            } else {
                address - ECHO_RAM.size()
            }
        } else {
            address
        };
        folded - ROM_SPACE.size()
    }
}

impl Index<u16> for MMU {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.raw_ram[Self::adjust_address(index) as usize]
    }
}

impl IndexMut<u16> for MMU {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.raw_ram[Self::adjust_address(index) as usize]
    }
}

/* #endregion */
