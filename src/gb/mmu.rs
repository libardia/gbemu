use crate::{
    gb::{GameBoy, mmu::region::*},
    hex,
};

pub mod region;

#[derive(Debug, Default)]
pub struct MMU {
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
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),

            boot_mode: false,
        }
    }

    pub fn read(ctx: &GameBoy, address: u16) -> u8 {
        if ctx.mmu.boot_mode {
            if BOOT_ROM.contains(address) {
                // TODO Return early (BOOT ROM "maps over" everything else)
                todo!("boot ROM doesn't exist yet");
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

                // TODO IO registers
                // IO_JOYP      => Input::read(ctx, address),
                // #IO_SERIAL   => Serial::read(ctx, address),
                // #IO_TIMER    => Timer::read(ctx, address),
                // IO_IF        => get_bits_of!(ctx.mem.io_if, 0x1F),
                // #IO_AUDIO    => Audio::read(ctx, address),
                // #IO_GRAPHICS => Graphics::read(ctx, address),
                // IO_IE        => ctx.mem.io_ie,

                // Anything else is unreadable
                _ => 0xFF,
        }
    }

    pub fn write(ctx: &mut GameBoy, address: u16, byte: u8) {
        if ctx.mmu.boot_mode {
            if BOOT_ROM.contains(address) {
                panic!(
                    "Something tried to write {} to address {} in the boot rom. Something has gone very wrong!",
                    hex!(byte, 2),
                    hex!(address, 4)
                );
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

                // TODO IO registers
                // IO_JOYP      => Input::write(ctx, address, value),
                // #IO_SERIAL   => Serial::write(ctx, address, value),
                // #IO_TIMER    => Timer::write(ctx, address, value),
                // IO_IF        => ctx.mem.io_if = set_bits_of!(ctx.mem.io_if, value, 0x1F),
                // #IO_AUDIO    => Audio::write(ctx, address, value),
                // #IO_GRAPHICS => Graphics::write(ctx, address, value),
                // IO_BANK      => if value != 0 { ctx.mem.boot_mode = false },
                // IO_IE        => ctx.mem.io_ie = value,

                // Anything else is unwritable
                _ => (),
        }
    }
}
