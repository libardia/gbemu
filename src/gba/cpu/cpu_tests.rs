use super::*;
use super::registers::flag_masks::*;

macro_rules! test_add {
    ($test_name:ident, $target_enum:ident, $target:ident) => {
        #[test]
        fn $test_name() {
            // Everything initialized to zeroes
            let mut mmu = MMU::new();
            let mut cpu = CPU::new();
            cpu.regs.a = 5;
            let target_is_a = cpu.regs.a == cpu.regs.$target;
            if !target_is_a {
                cpu.regs.$target = 3;
            }
            cpu.execute(&mut mmu, Instruction::ADD(ArgR8::$target_enum));
            // Result should be 10 if the target was A, else 8
            assert_eq!(cpu.regs.a, if target_is_a { 10 } else { 8 });
            // Flags should be 0: result was not zero, was not subtraction, no carrying
            assert_eq!(cpu.regs.f, 0);
            
            cpu.reset();
            cpu.regs.a = 0b0000_1111; // = 0x0F = 15
            if !target_is_a {
                cpu.regs.$target = 0b1;
            }
            cpu.execute(&mut mmu, Instruction::ADD(ArgR8::$target_enum));
            // Result should be 30 if the target was A, else 16
            assert_eq!(cpu.regs.a, if target_is_a { 30 } else { 16 });
            // Flags should all be 0, except half carry
            assert_eq!(cpu.regs.f, F_HCARRY);

            // Reset cpu
            cpu.reset();
            cpu.regs.a = 0xFF; // = 255
            if !target_is_a {
                cpu.regs.$target = 5;
            }
            cpu.execute(&mut mmu, Instruction::ADD(ArgR8::$target_enum));
            // Because of overflow, result should be 254 (0xFE) if target was A, else 4
            assert_eq!(cpu.regs.a, if target_is_a { 0xFE } else { 4 });
            // Half carry and carry flags both set
            assert_eq!(cpu.regs.f, F_CARRY | F_HCARRY);

            // Reset cpu
            cpu.reset();
            cpu.regs.a = 0xC0; // = 192
            cpu.regs.$target = 0xC0; // = 192; if the target is A doesn't matter here
            cpu.execute(&mut mmu, Instruction::ADD(ArgR8::$target_enum));
            // Because of overflow, result should be 128 (0x80)
            assert_eq!(cpu.regs.a, 0x80);
            // Carry set, half carry NOT set, everything else 0
            assert_eq!(cpu.regs.f, F_CARRY);

            // Reset cpu
            cpu.reset();
            cpu.regs.a = 0x80; // = 128
            cpu.regs.$target = 0x80; // = 128; if the target is A doesn't matter here
            cpu.execute(&mut mmu, Instruction::ADD(ArgR8::$target_enum));
            // Because of overflow, result should be 0
            assert_eq!(cpu.regs.a, 0);
            // Carry set, half carry NOT set, zero set
            assert_eq!(cpu.regs.f, F_CARRY | F_ZERO);
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
