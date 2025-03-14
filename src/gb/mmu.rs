use std::array;

use crate::{
    gb::gpu::color_id::ColorID,
    mem_region::{
        io_regs::REG_BANK,
        regions::{
            BOOT_ROM_BANK, ECHO_RAM, EXTERNAL_RAM, HIGH_RAM, OAM, ROM_BANK_N, TILE_DATA, TILE_MAPS,
            UNUSABLE_MEM, WORK_RAM,
        },
    },
    util::new,
};

use boot_rom::BOOT_ROM;
use log::{error, warn};
use mapped_region::MappedRegion;

use super::{gpu::tile::Tile, mbc::MBC};

pub mod boot_rom;
pub mod mapped_region;
pub mod nombc;

const NUM_TILES: usize = TILE_DATA.usize() / 16;
const ECHO_RAM_OFFSET: u16 = 0x2000;

#[derive(Debug)]
pub struct MMU {
    mbc: Option<Box<dyn MBC>>,
    vram_tile_raw: MappedRegion,
    vram_tiles: [Tile; NUM_TILES],
    vram_tilemaps: MappedRegion,
    wram: MappedRegion,
    oam: MappedRegion,
    hram: MappedRegion,
    boot_mode: bool,
}
impl Default for MMU {
    fn default() -> Self {
        Self {
            mbc: None,
            vram_tile_raw: MappedRegion::new(TILE_DATA),
            vram_tiles: array::from_fn(|i| {
                let a = (i as u16) * 16 + TILE_DATA.begin();
                Tile::new(a)
            }),
            vram_tilemaps: MappedRegion::new(TILE_MAPS),
            wram: MappedRegion::new(WORK_RAM),
            oam: MappedRegion::new(OAM),
            hram: MappedRegion::new(HIGH_RAM),
            boot_mode: true,
        }
    }
}
impl MMU {
    new!();

    /* #region Public ========================================================================== */

    pub fn get(&self, address: u16) -> u8 {
        // In boot mode, the boot ROM "overrides" this address range
        if self.boot_mode {
            if BOOT_ROM_BANK.contains(address) {
                return BOOT_ROM[address as usize];
            }
        }

        // MBC-controlled ranges
        if address <= ROM_BANK_N.end() || EXTERNAL_RAM.contains(address) {
            return match self.mbc.as_ref() {
                Some(mbc) => mbc.read_byte(address),
                None => warn_read_open_bus!(address, "No MBC present."),
            };
        }

        // VRAM tile data range
        get_or_continue!(address, self.vram_tile_raw);

        // VRAM tilesets range
        get_or_continue!(address, self.vram_tilemaps);

        // Work RAM
        get_or_continue!(address, self.wram);

        // Echo RAM
        if ECHO_RAM.contains(address) {
            warn!("Write to Echo RAM at 0x{address:0>4X}. Echo RAM is not intended to be used.");
            return self.wram.get(address - ECHO_RAM_OFFSET);
        }

        // OAM
        get_or_continue!(address, self.oam);

        // Unusable space
        if UNUSABLE_MEM.contains(address) {
            return warn_read_open_bus!(address, "Tried to read from unusable address space.");
        }

        // High RAM is the only address space left
        self.hram.get(address)
    }

    pub fn set(&mut self, address: u16, value: u8) {
        // In boot mode, the boot ROM "overrides" this address range
        if self.boot_mode {
            if BOOT_ROM_BANK.contains(address) {
                error!("Tried to write to read-only boot ROM in boot mode at address 0x{address:0>4X}! Something has gone very wrong.");
                return;
            } else if address == REG_BANK {
                // If the value is anything other than 0, disable boot mode
                self.boot_mode = value == 0;
                self.hram.set(address, value);
                return;
            }
        }

        // MBC-controlled ranges
        if address <= ROM_BANK_N.end() || EXTERNAL_RAM.contains(address) {
            match self.mbc.as_mut() {
                Some(mbc) => mbc.write_byte(address, value),
                None => warn_write_rom!(address, "No MBC present."),
            }
            return;
        }

        // Special handling for vram tiles
        if self.vram_tile_raw.contains(address) {
            self.vram_tile_raw.set(address, value);
            self.update_tile(address);
            return;
        }

        // VRAM tilemaps
        set_or_continue!(address, value, self.vram_tilemaps);

        // Work RAM
        set_or_continue!(address, value, self.wram);

        // Echo RAM
        if ECHO_RAM.contains(address) {
            warn!("Read from Echo RAM at 0x{address:0>4X}. Echo RAM is not intended to be used.");
            self.wram.set(address - ECHO_RAM_OFFSET, value);
            return;
        }

        // OAM
        set_or_continue!(address, value, self.oam);

        // Unusable space
        if UNUSABLE_MEM.contains(address) {
            warn_write_open_bus!(address, "Tried to write to unusable address space.");
            return;
        }

        // High RAM is the only address space left
        self.hram.set(address, value);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.get(address)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.set(address, value);
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let lsb = self.read_byte(address) as u16;
        let msb = self.read_byte(address + 1) as u16;
        (msb << 8) | lsb
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let lsb = (value & 0xFF) as u8;
        let msb = ((value & 0xFF00) >> 8) as u8;
        self.write_byte(address, lsb);
        self.write_byte(address + 1, msb);
    }

    /* #endregion */

    /* #region Helpers ========================================================================= */

    fn update_tile(&mut self, address: u16) {
        let index = TILE_DATA.uoffset(address) / 16;
        let tile = &mut self.vram_tiles[index];
        for row in 0..8 {
            let a = tile.address + (row * 2) as u16;
            let tile_ls = self.vram_tile_raw.get(a);
            let tile_ms = self.vram_tile_raw.get(a + 1);
            for bit in 0..8 {
                let m = 1 << (7 - bit);
                let color = match (tile_ms & m != 0, tile_ls & m != 0) {
                    (false, false) => ColorID::Color0,
                    (false, true) => ColorID::Color1,
                    (true, false) => ColorID::Color2,
                    (true, true) => ColorID::Color3,
                };
                tile.pixels[(row * 8) + bit] = color;
            }
        }
    }

    /* #endregion */
}

macro_rules! warn_read_open_bus {
    ($address:expr) => {{
        log::warn!(
            "Memory address 0x{:0>4X} invalid, returning open bus value.",
            $address,
        );
        0xFF
    }};
    ($address:expr, $($format_arg:expr),+) => {{
        log::warn!(
            "Memory address 0x{:0>4X} invalid, returning open bus value. {}",
            $address,
            format!($($format_arg),+)
        );
        0xFF
    }};
}
pub(self) use warn_read_open_bus;

macro_rules! warn_write_open_bus {
    ($address:expr, $($format_arg:expr),+) => {
        log::warn!("Memory address 0x{:0>4X} invalid, ignoring write. {}", $address, format!($($format_arg),+))
    };
}
pub(self) use warn_write_open_bus;

macro_rules! warn_write_rom {
    ($address:expr, $($format_arg:expr),+) => {
        log::warn!("Tried to write to read only memory at 0x{:0>4X}. {}", $address, format!($($format_arg),+))
    };
}
pub(self) use warn_write_rom;

macro_rules! get_or_continue {
    ($address:expr, $mapped_region:expr) => {
        if $mapped_region.contains($address) {
            return $mapped_region.get($address);
        }
    };
}
pub(self) use get_or_continue;

macro_rules! set_or_continue {
    ($address:expr, $value:expr, $mapped_region:expr) => {
        if $mapped_region.contains($address) {
            $mapped_region.set($address, $value);
            return;
        }
    };
}
pub(self) use set_or_continue;

mod mmu_set_mbc;
