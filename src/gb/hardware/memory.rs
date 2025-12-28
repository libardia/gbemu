use crate::{
    gb::{
        GameBoy,
        hardware::HardwareInterface,
        regions::{
            CART_RAM, ECHO_RAM, HIGH_RAM, MappedMemoryRegion, OAM, ROM_SPACE, VRAM, WORK_RAM,
        },
        registers::{IO_AUDIO, IO_GRAPHICS, IO_IE, IO_IF, IO_JOYP, IO_SERIAL, IO_TIMER},
    },
    get_bits_of, set_bits_of,
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

    // these regs are special
    io_if: u8,
    io_ie: u8,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),
            io_if: UNINIT_VALUE,
            io_ie: UNINIT_VALUE,
        }
    }
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

impl Memory {
    pub fn read(ctx: &GameBoy, address: u16) -> u8 {
        address_dispatch! {
            on address:
                // ROM and RAM
                #ROM_SPACE => ctx.cart.read_rom(address),
                #VRAM      => ctx.mem.vram.get(address),
                #CART_RAM  => ctx.cart.read_ram(address),
                #WORK_RAM  => ctx.mem.wram.get(address),
                #ECHO_RAM  => ctx.mem.wram.get(address - ECHO_RAM_OFFSET),
                #OAM       => ctx.mem.oam.get(address),
                #HIGH_RAM  => ctx.mem.hram.get(address),

                // IO registers
                IO_JOYP      => ctx.input.read(address),
                #IO_SERIAL   => ctx.serial.read(address),
                #IO_TIMER    => ctx.timer.read(address),
                IO_IF        => get_bits_of!(ctx.mem.io_if, 0x1F),
                #IO_AUDIO    => ctx.aud.read(address),
                #IO_GRAPHICS => ctx.gfx.read(address),
                IO_IE        => get_bits_of!(ctx.mem.io_ie, 0x1F),

                // Anything else is unreadable
                _ => OPEN_BUS_VALUE,
        }
    }

    pub fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        address_dispatch! {
            on address:
                // ROM and RAM
                #ROM_SPACE => ctx.cart.write_rom(address, value),
                #VRAM      => ctx.mem.vram.set(address, value),
                #CART_RAM  => ctx.cart.write_ram(address, value),
                #WORK_RAM  => ctx.mem.wram.set(address, value),
                #ECHO_RAM  => ctx.mem.wram.set(address - ECHO_RAM_OFFSET, value),
                #OAM       => ctx.mem.oam.set(address, value),
                #HIGH_RAM  => ctx.mem.hram.set(address, value),

                // IO registers
                IO_JOYP      => ctx.input.write(address, value),
                #IO_SERIAL   => ctx.serial.write(address, value),
                #IO_TIMER    => ctx.timer.write(address, value),
                IO_IF        => ctx.mem.io_if = set_bits_of!(ctx.mem.io_if, value, 0x1F),
                #IO_AUDIO    => ctx.aud.write(address, value),
                #IO_GRAPHICS => ctx.gfx.write(address, value),
                IO_IE        => ctx.mem.io_ie = set_bits_of!(ctx.mem.io_ie, value, 0x1F),

                // Anything else is unwritable
                _ => (),
        }
    }
}
