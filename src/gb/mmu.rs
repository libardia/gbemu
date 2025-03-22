use std::array;

use crate::{
    gb::gpu::color_id::ColorID,
    mem_region::{
        regions::{
            BOOT_ROM_BANK, ECHO_RAM, EXTERNAL_RAM, HIGH_RAM, OAM, ROM_SPACE, TILE_DATA, TILE_MAPS,
            TILE_MAP_0, TILE_MAP_1, UNUSABLE_MEM, WORK_RAM,
        },
        MemoryRegion,
    },
    util::{error_and_panic, new, Hex16, Hex8},
};

use boot_rom::BOOT_ROM;
use log::warn;
use mapped_region::MappedRegion;

use super::{gpu::tile::Tile, mbc::MBC};

pub mod boot_rom;
pub mod mapped_region;
pub mod mbc_rom_only;
pub mod nintendo_logo;

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
    blocked_ranges: Vec<MemoryRegion>,
    reset_div: bool,
    execute_dma: bool,
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
            blocked_ranges: Vec::new(),
            reset_div: false,
            execute_dma: false,
        }
    }
}
impl MMU {
    new!();

    /* #region Public ========================================================================== */

    pub fn block_region(&mut self, region: MemoryRegion) {
        self.blocked_ranges.push(region);
    }

    pub fn unblock_region(&mut self, region: MemoryRegion) {
        self.blocked_ranges.retain(|e| *e != region);
    }

    pub fn should_reset_div(&mut self) -> bool {
        let reset = self.reset_div;
        self.reset_div = false;
        reset
    }

    pub fn should_start_dma(&mut self) -> bool {
        let dma = self.execute_dma;
        self.execute_dma = false;
        dma
    }

    pub fn get(&self, address: u16) -> u8 {
        let hex_address = Hex16::make(address);

        // MBC-controlled ranges
        if ROM_SPACE.contains(address) || EXTERNAL_RAM.contains(address) {
            return match self.mbc.as_ref() {
                Some(mbc) => mbc.read_byte(address),
                None => warn_read_open_bus!(hex_address, "No MBC present."),
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
            warn!("Write to Echo RAM at {hex_address:?}. Echo RAM is not intended to be used.");
            return self.wram.get(address - ECHO_RAM_OFFSET);
        }

        // OAM
        get_or_continue!(address, self.oam);

        // Unusable space
        if UNUSABLE_MEM.contains(address) {
            return warn_read_open_bus!(hex_address, "Tried to read from unusable address space.");
        }

        // High RAM is the only address space left
        self.hram.get(address)
    }

    pub fn get_tile_index_at(&self, map: bool, tile_x: usize, tile_y: usize) -> u8 {
        let index = tile_y * 32 + tile_x;
        if !map {
            self.get(TILE_MAP_0.begin() + index as u16)
        } else {
            self.get(TILE_MAP_1.begin() + index as u16)
        }
    }

    pub fn get_tile(&self, unsigned: bool, tile_index: u8) -> &Tile {
        match (tile_index, unsigned) {
            // Tile index is in range 128-255 OR we're using unsigned addressing
            (128..=255, _) | (_, true) => &self.vram_tiles[tile_index as usize],
            // Tile index is in range 0-127 AND we're using signed addressing
            _ => &self.vram_tiles[0x100 + tile_index as usize],
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        let hex_address = Hex16::make(address);

        // MBC-controlled ranges
        if ROM_SPACE.contains(address) || EXTERNAL_RAM.contains(address) {
            match self.mbc.as_mut() {
                Some(mbc) => mbc.write_byte(address, value),
                None => warn_write_rom!(hex_address, Hex8::make(value), "No MBC present."),
            }
            return;
        }

        // VRAM tiles
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
            warn!("Read from Echo RAM at {hex_address:?}. Echo RAM is not intended to be used.");
            self.wram.set(address - ECHO_RAM_OFFSET, value);
            return;
        }

        // OAM
        set_or_continue!(address, value, self.oam);

        // Unusable space
        if UNUSABLE_MEM.contains(address) {
            warn_write_open_bus!(hex_address, "Tried to write to unusable address space.");
            return;
        }

        // High RAM is the only address space left
        self.hram.set(address, value);
    }

    pub fn cpu_read(&self, address: u16) -> u8 {
        let hex_address = Hex16::make(address);

        // In boot mode, the boot ROM "overrides" this address range
        if self.boot_mode {
            if BOOT_ROM_BANK.contains(address) {
                return BOOT_ROM[address as usize];
            }
        }

        for blocked in self.blocked_ranges.iter() {
            if blocked.contains(address) {
                return warn_read_open_bus!(hex_address, "Address is blocked.");
            }
        }

        if HIGH_RAM.contains(address) {
            // Enforce register readability
            self.high_ram_get(address)
        } else {
            self.get(address)
        }
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        let hex_address = Hex16::make(address);

        // In boot mode, the boot ROM "overrides" this address range
        if self.boot_mode && BOOT_ROM_BANK.contains(address) {
            // The boot ROM shouldn't ever try to write to this area, and by the time the main
            // program gets control, boot mode should be disabled. So if something tries to write
            // here, something has broken somewhere.
            error_and_panic!("Tried to write to read-only boot ROM in boot mode at address {hex_address:?}! Something has gone very wrong.");
        }

        for blocked in self.blocked_ranges.iter() {
            if blocked.contains(address) {
                warn_write_open_bus!(hex_address, "Address is blocked.");
                return;
            }
        }

        if HIGH_RAM.contains(address) {
            // Enforce register writeability
            self.high_ram_set(address, value)
        } else {
            self.set(address, value)
        }
    }

    pub fn cpu_read_word(&self, address: u16) -> u16 {
        let lsb = self.cpu_read(address) as u16;
        let msb = self.cpu_read(address + 1) as u16;
        (msb << 8) | lsb
    }

    pub fn cpu_write_word(&mut self, address: u16, value: u16) {
        let lsb = (value & 0xFF) as u8;
        let msb = ((value & 0xFF00) >> 8) as u8;
        self.cpu_write(address, lsb);
        self.cpu_write(address + 1, msb);
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
            "Memory address {:?} invalid, returning open bus value.",
            $address,
        );
        0xFF
    }};
    ($address:expr, $($format_arg:expr),+) => {{
        log::warn!(
            "Memory address {:?} invalid, returning open bus value. {}",
            $address,
            format!($($format_arg),+)
        );
        0xFF
    }};
}
pub(self) use warn_read_open_bus;

macro_rules! warn_write_open_bus {
    ($address:expr, $($format_arg:expr),+) => {
        log::warn!("Memory address {:?} invalid, ignoring write. {}", $address, format!($($format_arg),+))
    };
}
pub(self) use warn_write_open_bus;

macro_rules! warn_write_rom {
    ($address:expr, $value:expr, $($format_arg:expr),+) => {
        log::warn!("Tried to write {:?} to read only memory at {:?}. {}", $value, $address, format!($($format_arg),+))
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

mod high_ram_handler;
mod mmu_load;
