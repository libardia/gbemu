mod cpu;

use cpu::*;

fn main() {
    let mut cpu = CPU::default();
    cpu.pc = 0xDEAD;
    cpu.sp = 0xBEEF;
    cpu.regs.set_af(0xDEAD);
    cpu.regs.set_bc(0xBEEF);
    cpu.regs.set_de(0xF00D);
    cpu.regs.set_hl(0xCAFE);
    println!("{cpu}");
}
