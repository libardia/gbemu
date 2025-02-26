use super::*;

macro_rules! test_r16_getset {
    ($test_get_name:ident, $get_func:ident, $test_set_name:ident, $set_func:ident, $r1:ident, $r2:ident) => {
        #[test]
        fn $test_get_name() {
            let mut regs = Registers::new();
            regs.$r1 = 0xDE;
            regs.$r2 = 0xAD;
            assert_eq!(regs.$get_func(), 0xDEAD);
        }

        #[test]
        fn $test_set_name() {
            let mut regs = Registers::new();
            regs.$set_func(0xDEAD);
            assert_eq!(regs.$r1, 0xDE);
            assert_eq!(regs.$r2, 0xAD);
        }
    };
}

macro_rules! test_flag_getset {
    ($test_get_name:ident, $get_func:ident, $test_set_name:ident, $set_func:ident, $mask:expr) => {
        #[test]
        fn $test_get_name() {
            let mut regs = Registers::new();
            regs.f = $mask;
            assert!(regs.$get_func());
            regs.f = !$mask;
            assert!(!regs.$get_func())
        }

        #[test]
        fn $test_set_name() {
            let mut regs = Registers::new();
            regs.f = !$mask;
            regs.$set_func(true);
            assert_eq!(regs.f, 0xFF);
            regs.f = $mask;
            regs.$set_func(false);
            assert_eq!(regs.f, 0);
        }
    };
}

test_r16_getset!(test_get_af, get_af, test_set_af, set_af, a, f);
test_r16_getset!(test_get_bc, get_bc, test_set_bc, set_bc, b, c);
test_r16_getset!(test_get_de, get_de, test_set_de, set_de, d, e);
test_r16_getset!(test_get_hl, get_hl, test_set_hl, set_hl, h, l);

test_flag_getset!(test_getf_zero, getf_zero, test_setf_zero, setf_zero, F_ZERO);
test_flag_getset!(test_getf_subtract, getf_subtract, test_setf_subtract, setf_subtract, F_SUB);
test_flag_getset!(test_getf_half_carry, getf_half_carry, test_setf_half_carry, setf_half_carry, F_HCARRY);
test_flag_getset!(test_getf_carry, getf_carry, test_setf_carry, setf_carry, F_CARRY);
