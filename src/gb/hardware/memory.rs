use crate::{
    byte_fmt, error_panic,
    gb::{
        GameBoy,
        hardware::{
            HardwareInterface, audio::Audio, graphics::Graphics, input::Input, serial::Serial,
            timer::Timer,
        },
        regions::{
            BOOT_ROM_AREA, CART_RAM, ECHO_RAM, HIGH_RAM, MappedMemoryRegion, OAM, ROM_SPACE, VRAM,
            WORK_RAM,
        },
        registers::{IO_AUDIO, IO_BANK, IO_GRAPHICS, IO_IE, IO_IF, IO_JOYP, IO_SERIAL, IO_TIMER},
    },
    get_bits_of, set_bits_of, word_fmt,
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

    // State
    boot_mode: bool,
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
            boot_mode: false,
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
    pub fn init(ctx: &mut GameBoy) {
        // TODO: init memory
        // TODO: randomize ROM maybe?
        ctx.mem.boot_mode = !ctx.skip_boot;

        if ctx.skip_boot {
            ctx.mem.io_if = 0xE1;
            ctx.mem.io_ie = 0x00;
        }
    }

    pub fn read(ctx: &GameBoy, address: u16) -> u8 {
        if ctx.mem.boot_mode {
            if BOOT_ROM_AREA.contains(address) {
                // Return early (BOOT ROM "maps over" everything else)
                return BOOT_ROM[address as usize];
            }
        }

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
                IO_JOYP      => Input::read(ctx, address),
                #IO_SERIAL   => Serial::read(ctx, address),
                #IO_TIMER    => Timer::read(ctx, address),
                IO_IF        => get_bits_of!(ctx.mem.io_if, 0x1F),
                #IO_AUDIO    => Audio::read(ctx, address),
                #IO_GRAPHICS => Graphics::read(ctx, address),
                IO_IE        => ctx.mem.io_ie,

                // Anything else is unreadable
                _ => OPEN_BUS_VALUE,
        }
    }

    pub fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        if ctx.mem.boot_mode {
            if BOOT_ROM_AREA.contains(address) {
                error_panic!(
                    "Something tried to write {} to address {} in the boot rom. Something has gone very wrong!",
                    byte_fmt!(value),
                    word_fmt!(address)
                );
            }
        }

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
                IO_JOYP      => Input::write(ctx, address, value),
                #IO_SERIAL   => Serial::write(ctx, address, value),
                #IO_TIMER    => Timer::write(ctx, address, value),
                IO_IF        => ctx.mem.io_if = set_bits_of!(ctx.mem.io_if, value, 0x1F),
                #IO_AUDIO    => Audio::write(ctx, address, value),
                #IO_GRAPHICS => Graphics::write(ctx, address, value),
                IO_BANK      => if value != 0 { ctx.mem.boot_mode = false },
                IO_IE        => ctx.mem.io_ie = value,

                // Anything else is unwritable
                _ => (),
        }
    }

    pub fn write_masked(ctx: &mut GameBoy, address: u16, value: u8, mask: u8) {
        Memory::write(
            ctx,
            address,
            set_bits_of!(Memory::read(ctx, address), value, mask),
        );
    }
}

const BOOT_ROM: [u8; BOOT_ROM_AREA.usize()] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];
