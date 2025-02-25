mod cpu;
use cpu::*;

fn main() {
    let mut cpu = CPU::default();
    cpu.registers.set_af(0xDEAD);
    cpu.registers.set_bc(0xBEEF);
    cpu.registers.set_de(0xCAFE);
    cpu.registers.set_hl(0xF00D);
    println!("{cpu}")
}
