pub struct MemRegion {
    pub begin: u16,
    pub end: u16,
}

impl MemRegion {
    pub const fn new(begin: u16, end: u16) -> Self {
        Self { begin, end }
    }

    pub const fn contains(&self, address: u16) -> bool {
        address >= self.begin && address <= self.end
    }

    pub const fn size(&self) -> u16 {
        self.end - self.begin + 1
    }

    pub const fn usize(&self) -> usize {
        (self.end - self.begin) as usize + 1
    }
}

/* #region Predefined memory regions */

pub const ALL_RAM: MemRegion = MemRegion::new(0x0000, 0xFFFF);

pub const BOOT_ROM_BANK: MemRegion = MemRegion::new(0x0000, 0x00FF);
pub const CART_HEADER: MemRegion = MemRegion::new(0x0100, 0x014F);

pub const ROM_SPACE: MemRegion = MemRegion::new(0x0000, 0x7FFF);
pub const ROM_BANK_0: MemRegion = MemRegion::new(0x0000, 0x3FFF);
pub const ROM_BANK_N: MemRegion = MemRegion::new(0x4000, 0x7FFF);

pub const VRAM: MemRegion = MemRegion::new(0x8000, 0x9FFF);
pub const TILE_DATA: MemRegion = MemRegion::new(0x8000, 0x97FF);
pub const TILE_MAPS: MemRegion = MemRegion::new(0x9800, 0x9FFF);
pub const TILE_MAP_0: MemRegion = MemRegion::new(0x9800, 0x9BFF);
pub const TILE_MAP_1: MemRegion = MemRegion::new(0x9C00, 0x9FFF);

pub const EXTERNAL_RAM: MemRegion = MemRegion::new(0xA000, 0xBFFF);

pub const WORK_RAM: MemRegion = MemRegion::new(0xC000, 0xDFFF);
pub const ECHO_RAM: MemRegion = MemRegion::new(0xE000, 0xFDFF);

pub const OAM: MemRegion = MemRegion::new(0xFE00, 0xFE9F);

pub const UNUSABLE_MEM: MemRegion = MemRegion::new(0xFEA0, 0xFEFF);

pub const HIGH_RAM: MemRegion = MemRegion::new(0xFF00, 0xFFFF);
pub const IO_REGS: MemRegion = MemRegion::new(0xFF00, 0xFF7F);

/* #endregion */
