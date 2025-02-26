use super::*;

macro_rules! test_add {
    ($test_name:ident, $add_target:ident, $target:ident) => {
        #[test]
        fn $test_name() {
            // Everything initialized to zeroes
            let mut cpu = CPU::default();
            cpu.regs.a = 5;
            let target_is_a = cpu.regs.a == cpu.regs.$target;
            if !target_is_a {
                cpu.regs.$target = 3;
            }
            cpu.execute(Instruction::ADD(Target::$add_target));
            // Result should be 10 if the target was A, else 8
            assert_eq!(cpu.regs.a, if target_is_a { 10 } else { 8 });
            // Flags should be 0: result was not zero, was not subtraction, no carrying
            assert_eq!(cpu.regs.f, 0);
            
            // Reset cpu
            cpu = CPU::default();
            cpu.regs.a = 0b0000_1111; // = 0x0F = 15
            if !target_is_a {
                cpu.regs.$target = 0b1;
            }
            cpu.execute(Instruction::ADD(Target::$add_target));
            // Result should be 30 if the target was A, else 16
            assert_eq!(cpu.regs.a, if target_is_a { 30 } else { 16 });
            // Flags should all be 0, except half carry
            assert_eq!(cpu.regs.f, HALF_CARRY_FLAG_MASK);
        }
    };
}

test_add!(test_add_a, A, a);
test_add!(test_add_b, B, b);
test_add!(test_add_c, C, c);
test_add!(test_add_d, D, d);
test_add!(test_add_e, E, e);
test_add!(test_add_h, H, h);
test_add!(test_add_l, L, l);
