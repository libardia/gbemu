#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    begin: u16,
    end: u16,
}

impl MemoryRegion {
    pub const fn new(begin: u16, end: u16) -> Self {
        Self { begin, end }
    }

    pub const fn contains(&self, address: u16) -> bool {
        address >= self.begin && address <= self.end
    }

    pub const fn begin(&self) -> u16 {
        self.begin
    }

    pub const fn end(&self) -> u16 {
        self.end
    }

    pub const fn size(&self) -> u16 {
        self.end - self.begin + 1
    }

    pub const fn offset(&self, address: u16) -> u16 {
        address - self.begin
    }

    pub const fn ubegin(&self) -> usize {
        self.begin as usize
    }

    pub const fn uend(&self) -> usize {
        self.end as usize
    }

    pub const fn usize(&self) -> usize {
        self.size() as usize
    }

    pub const fn uoffset(&self, address: u16) -> usize {
        self.offset(address) as usize
    }
}

// impl From<(u16, u16)> for MemoryRegion {
//     fn from(value: (u16, u16)) -> Self {
//         MemoryRegion::new(value.0, value.1)
//     }
// }

pub mod regions {
    use super::*;

    pub const BOOT_ROM_BANK: MemoryRegion = MemoryRegion::new(0x0000, 0x00FF);
    pub const CART_HEADER: MemoryRegion = MemoryRegion::new(0x0100, 0x014F);

    pub const ROM_SPACE: MemoryRegion = MemoryRegion::new(0x0000, 0x7FFF);
    pub const ROM_BANK_0: MemoryRegion = MemoryRegion::new(0x0000, 0x3FFF);
    pub const ROM_BANK_N: MemoryRegion = MemoryRegion::new(0x4000, 0x7FFF);

    pub const VRAM: MemoryRegion = MemoryRegion::new(0x8000, 0x9FFF);
    pub const TILE_DATA: MemoryRegion = MemoryRegion::new(0x8000, 0x97FF);
    pub const TILE_MAPS: MemoryRegion = MemoryRegion::new(0x9800, 0x9FFF);
    pub const TILE_MAP_0: MemoryRegion = MemoryRegion::new(0x9800, 0x9BFF);
    pub const TILE_MAP_1: MemoryRegion = MemoryRegion::new(0x9C00, 0x9FFF);

    pub const EXTERNAL_RAM: MemoryRegion = MemoryRegion::new(0xA000, 0xBFFF);

    pub const WORK_RAM: MemoryRegion = MemoryRegion::new(0xC000, 0xDFFF);
    pub const ECHO_RAM: MemoryRegion = MemoryRegion::new(0xE000, 0xFDFF);

    pub const OAM: MemoryRegion = MemoryRegion::new(0xFE00, 0xFE9F);

    pub const UNUSABLE_MEM: MemoryRegion = MemoryRegion::new(0xFEA0, 0xFEFF);

    pub const HIGH_RAM: MemoryRegion = MemoryRegion::new(0xFF00, 0xFFFF);
    pub const IO_REGS: MemoryRegion = MemoryRegion::new(0xFF00, 0xFF7F);
}

pub mod io_regs {
    use super::MemoryRegion;

    pub const REG_JOYP: u16 = 0xFF00;
    pub const REG_SB: u16 = 0xFF01;
    pub const REG_SC: u16 = 0xFF02;
    pub const REG_DIV: u16 = 0xFF04;
    pub const REG_TIMA: u16 = 0xFF05;
    pub const REG_TMA: u16 = 0xFF06;
    pub const REG_TAC: u16 = 0xFF07;
    pub const REG_IF: u16 = 0xFF0F;
    pub const REG_NR10: u16 = 0xFF10;
    pub const REG_NR11: u16 = 0xFF11;
    pub const REG_NR12: u16 = 0xFF12;
    pub const REG_NR13: u16 = 0xFF13;
    pub const REG_NR14: u16 = 0xFF14;
    pub const REG_NR21: u16 = 0xFF16;
    pub const REG_NR22: u16 = 0xFF17;
    pub const REG_NR23: u16 = 0xFF18;
    pub const REG_NR24: u16 = 0xFF19;
    pub const REG_NR30: u16 = 0xFF1A;
    pub const REG_NR31: u16 = 0xFF1B;
    pub const REG_NR32: u16 = 0xFF1C;
    pub const REG_NR33: u16 = 0xFF1D;
    pub const REG_NR34: u16 = 0xFF1E;
    pub const REG_NR41: u16 = 0xFF20;
    pub const REG_NR42: u16 = 0xFF21;
    pub const REG_NR43: u16 = 0xFF22;
    pub const REG_NR44: u16 = 0xFF23;
    pub const REG_NR50: u16 = 0xFF24;
    pub const REG_NR51: u16 = 0xFF25;
    pub const REG_NR52: u16 = 0xFF26;
    pub const REG_WAVE_RAM: MemoryRegion = MemoryRegion::new(0xFF30, 0xFF40);
    pub const REG_LCDC: u16 = 0xFF40;
    pub const REG_STAT: u16 = 0xFF41;
    pub const REG_SCY: u16 = 0xFF42;
    pub const REG_SCX: u16 = 0xFF43;
    pub const REG_LY: u16 = 0xFF44;
    pub const REG_LYC: u16 = 0xFF45;
    pub const REG_DMA: u16 = 0xFF46;
    pub const REG_BGP: u16 = 0xFF47;
    pub const REG_OBP0: u16 = 0xFF48;
    pub const REG_OBP1: u16 = 0xFF49;
    pub const REG_BANK: u16 = 0xFF50;
    pub const REG_WY: u16 = 0xFF4A;
    pub const REG_WX: u16 = 0xFF4B;
    pub const REG_IE: u16 = 0xFFFF;
}

pub mod header_data {
    // These are the only parts relevant to emulation
    pub const CART_TYPE: u16 = 0x0147;
    pub const ROM_SIZE: u16 = 0x0148;
    pub const RAM_SIZE: u16 = 0x0149;
}
