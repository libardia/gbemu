use crate::{
    GameBoy,
    regions::{HIGH_RAM, MappedMemoryRegion, OAM, VRAM, WORK_RAM},
};

pub const OPEN_BUS_VALUE: u8 = 0xFF;
pub const UNINIT_VALUE: u8 = 0xFF;

const ECHO_RAM_OFFSET: u16 = 0x2000;

#[derive(Debug)]
pub struct Memory {
    // RAM areas
    vram: MappedMemoryRegion,
    wram: MappedMemoryRegion,
    oam: MappedMemoryRegion,
    hram: MappedMemoryRegion,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),
        }
    }
}

impl Memory {
    pub fn read(ctx: &GameBoy, address: u16) -> u8 {
        //TODO: MMU read
        todo!()
    }

    pub fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        //TODO: MMU write
        todo!()
    }
}
