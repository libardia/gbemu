#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    pub begin: u16,
    pub end: u16,
}

impl MemoryRegion {
    pub const fn new(begin: u16, end: u16) -> Self {
        Self { begin, end }
    }

    pub fn contains(&self, address: u16) -> bool {
        address >= self.begin && address < self.end
    }

    pub const fn size(&self) -> u16 {
        self.end - self.begin
    }
}

impl From<(u16, u16)> for MemoryRegion {
    fn from(value: (u16, u16)) -> Self {
        MemoryRegion::new(value.0, value.0)
    }
}

#[allow(dead_code)]
pub mod regions {
    use super::*;

    pub const BOOT_ROM_BANK: MemoryRegion = MemoryRegion::new(0x0000, 0x0100);
    pub const ROM_BANK_0: MemoryRegion = MemoryRegion::new(0x0000, 0x4000);
    pub const ROM_BANK_N: MemoryRegion = MemoryRegion::new(0x4000, 0x8000);
    pub const VRAM: MemoryRegion = MemoryRegion::new(0x8000, 0xA000);
    pub const EXTERNAL_RAM: MemoryRegion = MemoryRegion::new(0xA000, 0xC000);
    pub const WORK_RAM: MemoryRegion = MemoryRegion::new(0xC000, 0xE000);
    pub const ECHO_RAM: MemoryRegion = MemoryRegion::new(0xE000, 0xFE00);
    pub const OAM: MemoryRegion = MemoryRegion::new(0xFE00, 0xFEA0);
    pub const UNUSABLE_MEM: MemoryRegion = MemoryRegion::new(0xFEA0, 0xFF00);
    pub const IO_REGS: MemoryRegion = MemoryRegion::new(0xFF00, 0xFF80);
    pub const HIGH_RAM: MemoryRegion = MemoryRegion::new(0xFF80, 0xFFFF);
    // NOTE: The address 0xFFFF itself is not covered here, because it is special.
}

#[allow(dead_code)]
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
