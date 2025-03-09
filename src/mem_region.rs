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

    pub const JOYP: u16 = 0xFF00;
    pub const SB: u16 = 0xFF01;
    pub const SC: u16 = 0xFF02;
    pub const DIV: u16 = 0xFF04;
    pub const TIMA: u16 = 0xFF05;
    pub const TMA: u16 = 0xFF06;
    pub const TAC: u16 = 0xFF07;
    pub const IF: u16 = 0xFF0F;
    pub const NR10: u16 = 0xFF10;
    pub const NR11: u16 = 0xFF11;
    pub const NR12: u16 = 0xFF12;
    pub const NR13: u16 = 0xFF13;
    pub const NR14: u16 = 0xFF14;
    pub const NR21: u16 = 0xFF16;
    pub const NR22: u16 = 0xFF17;
    pub const NR23: u16 = 0xFF18;
    pub const NR24: u16 = 0xFF19;
    pub const NR30: u16 = 0xFF1A;
    pub const NR31: u16 = 0xFF1B;
    pub const NR32: u16 = 0xFF1C;
    pub const NR33: u16 = 0xFF1D;
    pub const NR34: u16 = 0xFF1E;
    pub const NR41: u16 = 0xFF20;
    pub const NR42: u16 = 0xFF21;
    pub const NR43: u16 = 0xFF22;
    pub const NR44: u16 = 0xFF23;
    pub const NR50: u16 = 0xFF24;
    pub const NR51: u16 = 0xFF25;
    pub const NR52: u16 = 0xFF26;
    pub const WAVE_RAM: MemoryRegion = MemoryRegion::new(0xFF30, 0xFF40);
    pub const LCDC: u16 = 0xFF40;
    pub const STAT: u16 = 0xFF41;
    pub const SCY: u16 = 0xFF42;
    pub const SCX: u16 = 0xFF43;
    pub const LY: u16 = 0xFF44;
    pub const LYC: u16 = 0xFF45;
    pub const DMA: u16 = 0xFF46;
    pub const BGP: u16 = 0xFF47;
    pub const OBP0: u16 = 0xFF48;
    pub const OBP1: u16 = 0xFF49;
    pub const BANK: u16 = 0xFF50;
    pub const WY: u16 = 0xFF4A;
    pub const WX: u16 = 0xFF4B;
    pub const IE: u16 = 0xFFFF;
}
