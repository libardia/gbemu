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
    IO_WAVE_RAM:   0xFF30, 0xFF3F
);

macro_rules! io_regs {
    ($($name:ident: $address:expr)*) => {
        $(
            pub const $name: u16 = $address;
        )*
    };
}

io_regs!(
    IO_JOYP: 0xFF00
    IO_SB:   0xFF01
    IO_SC:   0xFF02
    IO_DIV:  0xFF04
    IO_TIMA: 0xFF05
    IO_TMA:  0xFF06
    IO_TAC:  0xFF07
    IO_IF:   0xFF0F
    IO_NR10: 0xFF10
    IO_NR11: 0xFF11
    IO_NR12: 0xFF12
    IO_NR13: 0xFF13
    IO_NR14: 0xFF14
    IO_NR21: 0xFF16
    IO_NR22: 0xFF17
    IO_NR23: 0xFF18
    IO_NR24: 0xFF19
    IO_NR30: 0xFF1A
    IO_NR31: 0xFF1B
    IO_NR32: 0xFF1C
    IO_NR33: 0xFF1D
    IO_NR34: 0xFF1E
    IO_NR41: 0xFF20
    IO_NR42: 0xFF21
    IO_NR43: 0xFF22
    IO_NR44: 0xFF23
    IO_NR50: 0xFF24
    IO_NR51: 0xFF25
    IO_NR52: 0xFF26
    IO_LCDC: 0xFF40
    IO_STAT: 0xFF41
    IO_SCY:  0xFF42
    IO_SCX:  0xFF43
    IO_LY:   0xFF44
    IO_LYC:  0xFF45
    IO_DMA:  0xFF46
    IO_BGP:  0xFF47
    IO_OBP0: 0xFF48
    IO_OBP1: 0xFF49
    IO_WY:   0xFF4A
    IO_WX:   0xFF4B
    IO_BANK: 0xFF50
    IO_IE:   0xFFFF
);

/* #endregion */
