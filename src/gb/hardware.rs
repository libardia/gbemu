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
                    from_u8: $param_from:ident => $from_u8:block;
            )*
        )*
    ) => {
        paste::paste! {
            $(
                $(
                    const [<$for _ $name _POS>]: u8 = $pos;
                    const [<$for _ $name _MASK>]: u8 = $width << $pos;
                    #[allow(non_snake_case)]
                    fn [<$for _ $field _to_u8>]($param_to: $ftype) -> u8 $to_u8
                    #[allow(unused_braces, non_snake_case)]
                    fn [<$for _ $field _from_u8>](internal_param_from_u8: u8) -> $ftype {
                        let $param_from = internal_param_from_u8 >> $pos;
                        $from_u8
                    }
                )*
                const [<$for _UNUSED_BITS>]: u8 = $((![<$for _ $name _MASK>]))&*;

                macro_rules! [<make_reg_ $for>] {
                    ($self_ref:expr) => {
                        $((([<$for _ $field _to_u8>]($self_ref.$field)) << [<$for _ $name _POS>]))|* | [<$for _UNUSED_BITS>]
                    }
                }

                macro_rules! [<decomp_reg_ $for>] {
                    ($self_ref:expr, $value:expr) => {{
                        $(
                            $self_ref.$field = [<$for _ $field _from_u8>]($value & [<$for _ $name _MASK>]);
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

#[cfg(test)]
mod tests {
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;
    use test_log::test;

    #[derive(Debug, Default, FromPrimitive, Clone, Copy, PartialEq, Eq)]
    enum TestEnum {
        #[default]
        A, // = 0 = b00
        B, // = 1 = b01
        C, // = 2 = b10
        D, // = 3 = b11
    }

    struct TestStruct {
        reg1: bool,
        reg2: bool,
        test_enum: TestEnum,
    }

    #[test]
    fn test_sequential_enum() {
        macro_rules! do_test {
            ($enum:expr, $value:expr) => {
                assert_eq!($enum as u8, $value);
                assert_eq!(TestEnum::from_u8($value).unwrap(), $enum);
            };
        }

        do_test!(TestEnum::A, 0b00);
        do_test!(TestEnum::B, 0b01);
        do_test!(TestEnum::C, 0b10);
        do_test!(TestEnum::D, 0b11);
    }

    #[test]
    fn test_define_reg_bits() {
        define_reg_bits!(
            for TEST:
                REG1:
                    width: 0b1;
                    pos: 0;
                    field: reg1: bool;
                    to_u8: r => { r as u8 };
                    from_u8: r => { r != 0 };
                REG2:
                    width: 0b1;
                    pos: 2;
                    field: reg2: bool;
                    to_u8: r => { r as u8 };
                    from_u8: r => { r != 0 };
                TESTENUM:
                    width: 0b11;
                    pos: 4;
                    field: test_enum: TestEnum;
                    to_u8: t => { t as u8 };
                    from_u8: t => { TestEnum::from_u8(t).unwrap() };
        );

        assert_eq!(TEST_REG1_POS, 0);
        assert_eq!(TEST_REG1_MASK, 0b00000001);
        assert_eq!(TEST_reg1_to_u8(true), 1);
        assert_eq!(TEST_reg1_from_u8(1), true);

        assert_eq!(TEST_REG2_POS, 2);
        assert_eq!(TEST_REG2_MASK, 0b00000100);
        assert_eq!(TEST_reg2_to_u8(false), 0);
        assert_eq!(TEST_reg2_from_u8(0), false);

        assert_eq!(TEST_TESTENUM_POS, 4);
        assert_eq!(TEST_TESTENUM_MASK, 0b00110000);
        assert_eq!(TEST_test_enum_to_u8(TestEnum::C), 0b10);
        assert_eq!(TEST_test_enum_from_u8(0b00100000), TestEnum::C);

        assert_eq!(TEST_UNUSED_BITS, 0b11001010);

        let mut t = TestStruct {
            reg1: false,
            reg2: true,
            test_enum: TestEnum::D,
        };

        assert_eq!(make_reg_TEST!(t), 0b11111110);
        decomp_reg_TEST!(t, 0b11101011);
        assert_eq!(t.reg1, true);
        assert_eq!(t.reg2, false);
        assert_eq!(t.test_enum, TestEnum::C);
    }
}
