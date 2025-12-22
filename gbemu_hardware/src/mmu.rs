use crate::{
    cart::Cart,
    mmu::{
        regions::{
            CART_RAM, ECHO_RAM, HIGH_RAM, IO_REGS, MappedMemoryRegion, OAM, ROM_SPACE, VRAM,
            WORK_RAM,
        },
        regs::{HardwareRegs, IE},
    },
};

pub mod regions;
pub mod regs;

pub const OPEN_BUS_VALUE: u8 = 0xFF;
pub const UNINIT_VALUE: u8 = 0xFF;

const ECHO_RAM_OFFSET: u16 = 0x2000;

pub struct MMU {
    pub cart: Box<dyn Cart>,

    // RAM areas
    pub vram: MappedMemoryRegion,
    pub wram: MappedMemoryRegion,
    pub oam: MappedMemoryRegion,
    pub hram: MappedMemoryRegion,

    pub io: HardwareRegs,
}

impl MMU {
    pub fn new(cart: Box<dyn Cart>) -> Self {
        Self {
            cart,
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),
            io: HardwareRegs::new(),
        }
    }

    // Return the value of memory at the given address, but without side effects that would
    // otherwise occur if it was a true read.
    pub fn peek(&self, address: u16) -> u8 {
        match address {
            _ if ROM_SPACE.contains(address) => self.cart.peek(address),
            _ if VRAM.contains(address) => self.vram.get(address),
            _ if CART_RAM.contains(address) => self.cart.peek(address),
            _ if WORK_RAM.contains(address) => self.wram.get(address),
            _ if ECHO_RAM.contains(address) => self.wram.get(address - ECHO_RAM_OFFSET),
            _ if OAM.contains(address) => self.oam.get(address),
            _ if IO_REGS.contains(address) => self.io.peek(address),
            _ if HIGH_RAM.contains(address) => self.hram.get(address),
            _ if address == IE => self.io.peek(address),
            _ /* if UNUSABLE.contains(address) */ => OPEN_BUS_VALUE,
        }
    }

    // Write the value to the given address, but without side effects that would otherwise occur if
    // it was a true write.
    pub fn poke(&mut self, address: u16, value: u8) {
        match address {
            _ if ROM_SPACE.contains(address) => self.cart.poke(address, value),
            _ if VRAM.contains(address) => self.vram.set(address, value),
            _ if CART_RAM.contains(address) => self.cart.poke(address, value),
            _ if WORK_RAM.contains(address) => self.wram.set(address, value),
            _ if ECHO_RAM.contains(address) => self.wram.set(address - ECHO_RAM_OFFSET, value),
            _ if OAM.contains(address) => self.oam.set(address, value),
            _ if IO_REGS.contains(address) => self.io.poke(address, value),
            _ if HIGH_RAM.contains(address) => self.hram.set(address, value),
            _ if address == IE => self.io.poke(address, value),
            _ /* if UNUSABLE.contains(address) */ => (), 
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        //TODO: MMU read
        todo!()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        //TODO: MMU write
        todo!()
    }
}
