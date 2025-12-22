use crate::regions::MemoryRegion;

// Input
pub const IO_JOYP: u16 = 0xFF00;

// Serial
pub const IO_SB: u16 = 0xFF01;
pub const IO_SC: u16 = 0xFF02;
pub const IO_SERIAL: MemoryRegion = MemoryRegion::new(IO_SB, IO_SC);

// Timer
pub const IO_DIV: u16 = 0xFF04;
pub const IO_TIMA: u16 = 0xFF05;
pub const IO_TMA: u16 = 0xFF06;
pub const IO_TAC: u16 = 0xFF07;
pub const IO_TIMER: MemoryRegion = MemoryRegion::new(IO_DIV, IO_TAC);

// Interruptions
pub const IO_IF: u16 = 0xFF0F;

// Audio
pub const IO_NR10: u16 = 0xFF10;
pub const IO_NR11: u16 = 0xFF11;
pub const IO_NR12: u16 = 0xFF12;
pub const IO_NR13: u16 = 0xFF13;
pub const IO_NR14: u16 = 0xFF14;
pub const IO_NR21: u16 = 0xFF16;
pub const IO_NR22: u16 = 0xFF17;
pub const IO_NR23: u16 = 0xFF18;
pub const IO_NR24: u16 = 0xFF19;
pub const IO_NR30: u16 = 0xFF1A;
pub const IO_NR31: u16 = 0xFF1B;
pub const IO_NR32: u16 = 0xFF1C;
pub const IO_NR33: u16 = 0xFF1D;
pub const IO_NR34: u16 = 0xFF1E;
pub const IO_NR41: u16 = 0xFF20;
pub const IO_NR42: u16 = 0xFF21;
pub const IO_NR43: u16 = 0xFF22;
pub const IO_NR44: u16 = 0xFF23;
pub const IO_NR50: u16 = 0xFF24;
pub const IO_NR51: u16 = 0xFF25;
pub const IO_NR52: u16 = 0xFF26;
pub const IO_WAVE: MemoryRegion = MemoryRegion::new(0xFF30, 0xFF3F);
pub const IO_AUDIO: MemoryRegion = MemoryRegion::new(IO_NR10, IO_WAVE.end);

// Graphics
pub const IO_LCDC: u16 = 0xFF40;
pub const IO_STAT: u16 = 0xFF41;
pub const IO_SCY: u16 = 0xFF42;
pub const IO_SCX: u16 = 0xFF43;
pub const IO_LY: u16 = 0xFF44;
pub const IO_LYC: u16 = 0xFF45;
pub const IO_DMA: u16 = 0xFF46;
pub const IO_BGP: u16 = 0xFF47;
pub const IO_OBP0: u16 = 0xFF48;
pub const IO_OBP1: u16 = 0xFF49;
pub const IO_WY: u16 = 0xFF4A;
pub const IO_WX: u16 = 0xFF4B;
pub const IO_GRAPHICS: MemoryRegion = MemoryRegion::new(IO_LCDC, IO_WX);

// System
pub const IO_BANK: u16 = 0xFF50;

// Way off in normal HRAM
pub const IO_IE: u16 = 0xFFFF;
