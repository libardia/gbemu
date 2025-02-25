mod cpu;
use cpu::*;

fn main() {
    let mut cpu = CPU::default();
    cpu.pc = 0xDEAD;
    cpu.sp = 0xBEEF;
    cpu.reg.set_af(0xDEAD);
    cpu.reg.set_bc(0xBEEF);
    cpu.reg.set_de(0xF00D);
    cpu.reg.set_hl(0xCAFE);
    println!("{cpu}");
}
