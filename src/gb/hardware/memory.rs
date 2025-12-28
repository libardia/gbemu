use crate::gb::{
    GameBoy,
    hardware::HardwareInterface,
    regions::{CART_RAM, ECHO_RAM, HIGH_RAM, MappedMemoryRegion, OAM, ROM_SPACE, VRAM, WORK_RAM},
    registers::{IO_AUDIO, IO_GRAPHICS, IO_IF, IO_JOYP, IO_SERIAL, IO_TIMER},
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

    // an IO register that acts like normal RAM
    if_reg: u8,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            vram: MappedMemoryRegion::new(VRAM),
            wram: MappedMemoryRegion::new(WORK_RAM),
            oam: MappedMemoryRegion::new(OAM),
            hram: MappedMemoryRegion::new(HIGH_RAM),
            // TODO: IF register needs to be masked
            if_reg: UNINIT_VALUE,
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

                // Unusable ram here; $FEA0 - $FEFF

                // IO registers
                IO_JOYP      => ctx.input.read(address),
                #IO_SERIAL   => ctx.serial.read(address),
                #IO_TIMER    => ctx.timer.read(address),
                IO_IF        => ctx.mem.if_reg,
                #IO_AUDIO    => ctx.aud.read(address),
                #IO_GRAPHICS => ctx.gfx.read(address),

                // High RAM
                #HIGH_RAM  => ctx.mem.hram.get(address),

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

                // Unusable ram here; $FEA0 - $FEFF

                // IO registers
                IO_JOYP      => ctx.input.write(address, value),
                #IO_SERIAL   => ctx.serial.write(address, value),
                #IO_TIMER    => ctx.timer.write(address, value),
                IO_IF        => ctx.mem.if_reg = value,
                #IO_AUDIO    => ctx.aud.write(address, value),
                #IO_GRAPHICS => ctx.gfx.write(address, value),

                // High RAM
                #HIGH_RAM  => ctx.mem.hram.set(address, value),

                // Anything else is unwritable
                _ => (),
        }
    }
}
