use log::warn;

use crate::{
    gb::{
        GameBoy,
        hw::HardwareInterface,
        mmu::{
            io::{IO_IE, IO_IF},
            region::*,
        },
    },
    macros::hex,
};

pub mod io;
pub mod region;

#[derive(Debug, Default)]
pub struct MMU {
    pub rom: MappedMemoryRegion,

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
            rom: MappedMemoryRegion::new(BOOT_ROM),

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
                return ctx.mmu.rom.get(address);
            }
        }

        address_dispatch! {
            on address:
                // ROM and RAM
                #ROM_SPACE => todo!("read from ROM_SPACE"),
                #VRAM      => ctx.mmu.vram.get(address),
                #CART_RAM  => todo!("read from CART_RAM"),
                #WORK_RAM  => ctx.mmu.wram.get(address),
                #ECHO_RAM  => ctx.mmu.wram.get(address - Self::ECHO_RAM_OFFSET),
                #OAM       => ctx.mmu.oam.get(address),
                #HIGH_RAM  => ctx.mmu.hram.get(address),

                // TODO: IO registers read
                // IO_JOYP      => Input::read(ctx, address),
                // #IO_SERIAL   => Serial::read(ctx, address),
                // #IO_TIMER    => Timer::read(ctx, address),
                IO_IF        => ctx.cpu.read(address),
                // #IO_AUDIO    => Audio::read(ctx, address),
                // #IO_GRAPHICS => Graphics::read(ctx, address),
                IO_IE        => ctx.cpu.read(address),

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
                #ROM_SPACE => todo!("write to ROM_SPACE"),
                #VRAM      => ctx.mmu.vram.set(address, byte),
                #CART_RAM  => todo!("write to CART_RAM"),
                #WORK_RAM  => ctx.mmu.wram.set(address, byte),
                #ECHO_RAM  => ctx.mmu.wram.set(address - Self::ECHO_RAM_OFFSET, byte),
                #OAM       => ctx.mmu.oam.set(address, byte),
                #HIGH_RAM  => ctx.mmu.hram.set(address, byte),

                // TODO: IO registers write
                // IO_JOYP      => Input::write(ctx, address, value),
                // #IO_SERIAL   => Serial::write(ctx, address, value),
                // #IO_TIMER    => Timer::write(ctx, address, value),
                IO_IF        => ctx.cpu.write(address, byte),
                // #IO_AUDIO    => Audio::write(ctx, address, value),
                // #IO_GRAPHICS => Graphics::write(ctx, address, value),
                // IO_BANK      => if value != 0 { ctx.mem.boot_mode = false },
                IO_IE        => ctx.cpu.write(address, byte),

                // Anything else is unwritable
                _ => warn!(
                    "ignored write {} to address {}",
                    hex!(byte, 2),
                    hex!(address, 4)
                ),
        }
    }
}
