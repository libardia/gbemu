use crate::{
    GameBoy,
    memory::{
        hardware_regs::{HardwareRegs, IE},
        regions::{
            CART_RAM, ECHO_RAM, HIGH_RAM, IO_REGS, MappedMemoryRegion, OAM, ROM_SPACE, VRAM,
            WORK_RAM,
        },
    },
};

pub mod hardware_regs;
pub mod regions;

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

    io: HardwareRegs,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),
            io: Default::default(),
        }
    }
}

impl Memory {
    // Return the value of memory at the given address, but without side effects that would
    // otherwise occur if it was a true read.
    pub fn peek(ctx: &GameBoy, address: u16) -> u8 {
        match address {
            _ if ROM_SPACE.contains(address) => ctx.cart.peek(address),
            _ if VRAM.contains(address) => ctx.mmu.vram.get(address),
            _ if CART_RAM.contains(address) => ctx.cart.peek(address),
            _ if WORK_RAM.contains(address) => ctx.mmu.wram.get(address),
            _ if ECHO_RAM.contains(address) => ctx.mmu.wram.get(address - ECHO_RAM_OFFSET),
            _ if OAM.contains(address) => ctx.mmu.oam.get(address),
            _ if IO_REGS.contains(address) => ctx.mmu.io.peek(address),
            _ if HIGH_RAM.contains(address) => ctx.mmu.hram.get(address),
            _ if address == IE => ctx.mmu.io.peek(address),
            _ /* if UNUSABLE.contains(address) */ => OPEN_BUS_VALUE,
        }
    }

    // Write the value to the given address, but without side effects that would otherwise occur if
    // it was a true write.
    pub fn poke(ctx: &mut GameBoy, address: u16, value: u8) {
        match address {
            _ if ROM_SPACE.contains(address) => ctx.cart.poke(address, value),
            _ if VRAM.contains(address) => ctx.mmu.vram.set(address, value),
            _ if CART_RAM.contains(address) => ctx.cart.poke(address, value),
            _ if WORK_RAM.contains(address) => ctx.mmu.wram.set(address, value),
            _ if ECHO_RAM.contains(address) => ctx.mmu.wram.set(address - ECHO_RAM_OFFSET, value),
            _ if OAM.contains(address) => ctx.mmu.oam.set(address, value),
            _ if IO_REGS.contains(address) => ctx.mmu.io.poke(address, value),
            _ if HIGH_RAM.contains(address) => ctx.mmu.hram.set(address, value),
            _ if address == IE => ctx.mmu.io.poke(address, value),
            _ /* if UNUSABLE.contains(address) */ => (),
        }
    }

    pub fn read(ctx: &GameBoy, address: u16) -> u8 {
        //TODO: MMU read
        todo!()
    }

    pub fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        //TODO: MMU write
        todo!()
    }
}
