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

macro_rules! const_regions {
    ($($name:ident: $begin:expr, $end:expr)*) => {
        $(
            pub const $name: MemRegion = MemRegion::new($begin, $end);
        )*
    };
}

const_regions!(
    ALL_RAM:       0x0000, 0xFFFF

    BOOT_ROM_BANK: 0x0000, 0x00FF
    CART_HEADER:   0x0100, 0x014F

    ROM_SPACE:     0x0000, 0x7FFF
    ROM_BANK_0:    0x0000, 0x3FFF
    ROM_BANK_N:    0x4000, 0x7FFF

    VRAM:          0x8000, 0x9FFF
    TILE_DATA:     0x8000, 0x97FF
    TILE_MAPS:     0x9800, 0x9FFF
    TILE_MAP_0:    0x9800, 0x9BFF
    TILE_MAP_1:    0x9C00, 0x9FFF

    EXTERNAL_RAM:  0xA000, 0xBFFF

    WORK_RAM:      0xC000, 0xDFFF
    ECHO_RAM:      0xE000, 0xFDFF

    OAM:           0xFE00, 0xFE9F

    UNUSABLE_MEM:  0xFEA0, 0xFEFF

    HIGH_RAM:      0xFF00, 0xFFFF
    IO_REGS:       0xFF00, 0xFF7F
);

/* #endregion */
