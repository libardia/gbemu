use log::warn;

use crate::{
    gb::{
        GameBoy,
        hardware_interface::HardwareInterface,
        mmu::{io::*, region::*},
    },
    macros::hex,
};

pub mod header_data;
pub mod io;
pub mod region;

#[derive(Debug, Default)]
pub struct MMU {
    pub boot_rom: MappedMemoryRegion,

    pub vram: MappedMemoryRegion,
    pub wram: MappedMemoryRegion,
    pub oam: MappedMemoryRegion,
    pub hram: MappedMemoryRegion,

    pub boot_mode: bool,
}

macro_rules! address_dispatch {
    {
        on $address:ident:
            $($(#$reg:ident)?$($value:ident)? => $op:expr,)+
            $(_ => $op_other:expr,)?
    } => {
        match $address {
            $($(_ if $reg.contains($address))? $($value)? => $op,)+
            $(_ => $op_other,)?
        }
    };
}

impl MMU {
    pub const ECHO_RAM_OFFSET: u16 = ECHO_RAM_BEGIN - WORK_RAM_BEGIN;

    pub fn new() -> Self {
        Self {
            boot_rom: MappedMemoryRegion::new(BOOT_ROM),

            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),

            boot_mode: true,
        }
    }

    pub fn read(ctx: &mut GameBoy, address: u16) -> u8 {
        if ctx.mmu.boot_mode {
            if BOOT_ROM.contains(address) {
                // Return early (boot rom "maps over" everything else)
                return ctx.mmu.boot_rom.get(address);
            }
        }

        address_dispatch! {
            on address:
                // ROM and RAM
                #ROM_SPACE => ctx.cart.read(address),
                #VRAM      => ctx.mmu.vram.get(address),
                #CART_RAM  => ctx.cart.read(address),
                #WORK_RAM  => ctx.mmu.wram.get(address),
                #ECHO_RAM  => ctx.mmu.wram.get(address - Self::ECHO_RAM_OFFSET),
                #OAM       => ctx.mmu.oam.get(address),
                #HIGH_RAM  => ctx.mmu.hram.get(address),

                IO_JOYP => ctx.inu.read(address),
                #IO_SBU => ctx.sdu.read(address),
                #IO_TMU => ctx.tmu.read(address),
                IO_IF   => ctx.cpu.read(address),
                #IO_APU => ctx.apu.read(address),
                #IO_PPU => ctx.ppu.read(address),
                IO_IE   => ctx.cpu.read(address),

                // Anything else is unreadable
                _ => {
                    warn!(
                        "read from invalid address {}, $FF will be returned",
                        hex!(address, 4),
                    );
                    0xFF
                },
        }
    }

    pub fn write(ctx: &mut GameBoy, address: u16, byte: u8) {
        if ctx.mmu.boot_mode {
            if BOOT_ROM.contains(address) {
                warn!(
                    "ignored write {} to address {} in the boot rom",
                    hex!(byte, 2),
                    hex!(address, 4),
                );
                return;
            }
        }

        address_dispatch! {
            on address:
                // ROM and RAM
                #ROM_SPACE => ctx.cart.write(address, byte),
                #VRAM      => ctx.mmu.vram.set(address, byte),
                #CART_RAM  => ctx.cart.write(address, byte),
                #WORK_RAM  => ctx.mmu.wram.set(address, byte),
                #ECHO_RAM  => ctx.mmu.wram.set(address - Self::ECHO_RAM_OFFSET, byte),
                #OAM       => ctx.mmu.oam.set(address, byte),
                #HIGH_RAM  => ctx.mmu.hram.set(address, byte),

                IO_JOYP => ctx.inu.write(address, byte),
                #IO_SBU => ctx.sdu.write(address, byte),
                #IO_TMU => ctx.tmu.write(address, byte),
                IO_IF   => ctx.cpu.write(address, byte),
                #IO_APU => ctx.apu.write(address, byte),
                #IO_PPU => ctx.ppu.write(address, byte),
                IO_BANK => if byte != 0 { ctx.mmu.boot_mode = false },
                IO_IE   => ctx.cpu.write(address, byte),

                // Anything else is unwritable
                _ => warn!(
                    "ignored write {} to address {}",
                    hex!(byte, 2),
                    hex!(address, 4)
                ),
        }
    }
}
