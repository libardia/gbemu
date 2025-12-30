use crate::gb::GameBoy;

pub mod audio;
pub mod cartridge;
pub mod graphics;
pub mod input;
pub mod memory;
pub mod processor;
pub mod serial;
pub mod timer;

pub trait HardwareInit {
    fn init(ctx: &mut GameBoy);
}

pub trait HardwareInterface {
    fn read(ctx: &GameBoy, address: u16) -> u8;
    fn write(ctx: &mut GameBoy, address: u16, value: u8);
}

#[macro_export]
macro_rules! define_reg_bits {
    ($(
        for $for:ident:
            $(
                $name:ident:
                    width: $width:expr;
                    pos: $pos:expr;
                    field: $field:ident: $ftype:ty;
                    to_u8: $param_to:ident => $to_u8:block;
                    from_u8: $param_from:ident => $from_u8:expr;
            )*
        )*
    ) => {
        paste::paste! {
            $(
                $(
                    const [<$for _ $name _POS>]: u8 = $pos;
                    const [<$for _ $name _MASK>]: u8 = $width << $pos;
                    fn [<$field _to_u8>]($param_to: $ftype) -> u8 $to_u8
                    fn [<$field _from_u8>]($param_from: u8) -> $ftype $from_u8
                )*
                const [<$for _UNUSED_BITS>]: u8 = $((![<$for _ $name _MASK>]))&*;


                macro_rules! [<make_reg_ $for>] {
                    ($self_ref:expr) => {
                        $((([<$field _to_u8>]($self_ref.$field)) << [<$for _ $name _POS>]))|* | [<$for _UNUSED_BITS>]
                    }
                }

                macro_rules! [<decomp_reg_ $for>] {
                    ($self_ref:expr, $value:expr) => {{
                        $(
                            $self_ref.$field = [<$field _from_u8>]($value & [<$for _ $name _MASK>]);
                        )*
                    }}
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! warn_unimplemented_interface {
    ($interface:expr) => {
        log::warn!("{} is not currently implemented.", $interface)
    };
}

#[macro_export]
macro_rules! warn_unimplemented_read {
    ($ctx:expr, $interface:expr, $address:expr) => {
        crate::cpu_log!(
            warn,
            $ctx,
            "Read from {0} register at {1}. {0} is not implemented yet, so {2} was returned!",
            $interface,
            crate::word_fmt!($address),
            crate::byte_fmt!(crate::gb::hardware::memory::OPEN_BUS_VALUE),
        )
    };
}

#[macro_export]
macro_rules! warn_unimplemented_write {
    ($ctx:expr, $interface:expr, $address:expr, $value:expr) => {
        crate::cpu_log!(
            warn,
            $ctx,
            "Wrote {1} to {0} register at {2}. {0} is not implemented yet!",
            $interface,
            crate::byte_fmt!($value),
            crate::word_fmt!($address),
        )
    };
}
