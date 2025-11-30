#[derive(Debug, Default)]
pub struct MemRegion {
    pub begin: u16,
    pub end: u16,
}

impl MemRegion {
    pub fn contains(&self, address: u16) -> bool {
        address >= self.begin && address <= self.end
    }

    pub fn size(&self) -> u16 {
        self.end - self.begin + 1
    }

    pub fn usize(&self) -> usize {
        (self.end - self.begin) as usize + 1
    }
}

/* #region Predefined memory regions */

macro_rules! const_regions {
    ($($name:ident: $begin:expr, $end:expr)*) => {
        $(
            pub const $name: MemRegion = MemRegion { begin: $begin,  end: $end };
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

    DMA_USABLE:    0xFF80, 0xFFFE
);

macro_rules! io_regs {
    ($type:ty, $($name:ident: $address:expr; $default:expr)*) => {
        $(
            pub const $name: u16 = $address;
            paste::paste! {
                pub const [<$name _DEFAULT>]: u16 = $default;
            }
        )*

        pub const IO_DEFAULTS: $type = [
            $(
                ($name, $default)
            ),*
        ];
    };
}

io_regs!(
    [(u16, u8); 43],
    // P1 $FF00 $CF
    IO_JOYP: 0xFF00; 0xCF
    // SB $FF01 $00
    IO_SB:   0xFF01; 0x00
    // SC $FF02 $7E
    IO_SC:   0xFF02; 0x7E
    // DIV $FF04 $18
    IO_DIV:  0xFF04; 0x18
    // TIMA $FF05 $00
    IO_TIMA: 0xFF05; 0x00
    // TMA $FF06 $00
    IO_TMA:  0xFF06; 0x00
    // TAC $FF07 $F8
    IO_TAC:  0xFF07; 0xF8
    // IF $FF0F $E1
    IO_IF:   0xFF0F; 0xE1
    // NR10 $FF10 $80
    IO_NR10: 0xFF10; 0x80
    // NR11 $FF11 $BF
    IO_NR11: 0xFF11; 0xBF
    // NR12 $FF12 $F3
    IO_NR12: 0xFF12; 0xF3
    // NR13 $FF13 $FF
    IO_NR13: 0xFF13; 0xFF
    // NR14 $FF14 $BF
    IO_NR14: 0xFF14; 0xBF
    // NR21 $FF16 $3F
    IO_NR21: 0xFF16; 0x3F
    // NR22 $FF17 $00
    IO_NR22: 0xFF17; 0x00
    // NR23 $FF18 $FF
    IO_NR23: 0xFF18; 0xFF
    // NR24 $FF19 $BF
    IO_NR24: 0xFF19; 0xBF
    // NR30 $FF1A $7F
    IO_NR30: 0xFF1A; 0x7F
    // NR31 $FF1B $FF
    IO_NR31: 0xFF1B; 0xFF
    // NR32 $FF1C $9F
    IO_NR32: 0xFF1C; 0x9F
    // NR33 $FF1D $FF
    IO_NR33: 0xFF1D; 0xFF
    // NR34 $FF1E $BF
    IO_NR34: 0xFF1E; 0xBF
    // NR41 $FF20 $FF
    IO_NR41: 0xFF20; 0xFF
    // NR42 $FF21 $00
    IO_NR42: 0xFF21; 0x00
    // NR43 $FF22 $00
    IO_NR43: 0xFF22; 0x00
    // NR44 $FF23 $BF
    IO_NR44: 0xFF23; 0xBF
    // NR50 $FF24 $77
    IO_NR50: 0xFF24; 0x77
    // NR51 $FF25 $F3
    IO_NR51: 0xFF25; 0xF3
    // NR52 $FF26 $F1
    IO_NR52: 0xFF26; 0xF1
    // LCDC $FF40 $91
    IO_LCDC: 0xFF40; 0x91
    // STAT $FF41 $81
    IO_STAT: 0xFF41; 0x81
    // SCY $FF42 $00
    IO_SCY:  0xFF42; 0x00
    // SCX $FF43 $00
    IO_SCX:  0xFF43; 0x00
    // LY $FF44 $91
    IO_LY:   0xFF44; 0x91
    // LYC $FF45 $00
    IO_LYC:  0xFF45; 0x00
    // DMA $FF46 $FF
    IO_DMA:  0xFF46; 0xFF
    // BGP $FF47 $FC
    IO_BGP:  0xFF47; 0xFC
    // OBP0 $FF48 $??
    IO_OBP0: 0xFF48; 0xFF
    // OBP1 $FF49 $??
    IO_OBP1: 0xFF49; 0xFF
    // WY $FF4A $00
    IO_WY:   0xFF4A; 0x00
    // WX $FF4B $00
    IO_WX:   0xFF4B; 0x00
    // BANK $0xFF50 $01
    IO_BANK: 0xFF50; 0x01
    // IE $FFFF $00
    IO_IE:   0xFFFF; 0x00
);

/* #endregion */
