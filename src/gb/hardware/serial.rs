use crate::{
    define_reg_bits,
    gb::{
        GameBoy,
        hardware::{HardwareInit, HardwareInterface},
        registers::{IO_SB, IO_SC},
    },
    impossible_address, warn_unimplemented_interface, warn_unimplemented_read,
    warn_unimplemented_write,
};

#[derive(Debug, Default)]
pub struct Serial {
    // WARN: This is a dummy Serial implementation
    enabled: bool,
    is_master_clock: bool,

    serial_data: u8,
}

define_reg_bits!(
    for SC:
        ENABLE:
            width: 0b1;
            pos: 7;
            field: enabled: bool;
            to_u8: e => { e as u8 };
            from_u8: e => { e != 0 };
        CLOCK_SELECT:
            width: 0b1;
            pos: 0;
            field: is_master_clock: bool;
            to_u8: m => { m as u8};
            from_u8: m => { m != 0 };
);

impl HardwareInit for Serial {
    fn init(ctx: &mut GameBoy) {
        warn_unimplemented_interface!("Serial data transfer");
        ctx.serial.enabled = false;
        ctx.serial.is_master_clock = false;
        ctx.serial.serial_data = 0;
    }
}

impl HardwareInterface for Serial {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        warn_unimplemented_read!(ctx, "Serial", address);
        match address {
            IO_SB => ctx.serial.serial_data,
            IO_SC => make_reg_SC!(ctx.serial),

            _ => impossible_address!("Serial", address),
        }
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        warn_unimplemented_write!(ctx, "Serial", address, value);
        match address {
            IO_SB => ctx.serial.serial_data = value,
            IO_SC => decomp_reg_SC!(ctx.serial, value),

            _ => impossible_address!("Serial", address),
        }
    }
}
