use cpu::CPU;
use cpu_basic::BasicCPU;
use gb::GB;
use mmu::MMU;
use mmu_basic::BasicMMU;

mod cpu;
mod cpu_basic;
mod gb;
mod hex;
mod mmu;
mod mmu_basic;

fn main() {
    type MMU = BasicMMU;
    type CPU = BasicCPU<MMU>;

    let mut gb: GB<CPU, MMU> = GB::new();

    const JR_1: u8 = -5i8 as u8;
    const JR_2: u8 = -8i8 as u8;
    let prog = [
        0x01, 0xAD, 0xDE, // Write 0xDEAD into BC
        0x80, // A += B (0xDE)
        0x81, // A += C (0x8B)
        0xEA, 0xAD, 0xDE, // Write A to [0xDEAD]
        0xED, // Print
        0x21, 0xFF, 0xFF, // Write 0xFFFF into HL
        0xAF, // A = A xor A; sets A to 0
        0x06, 0xFF, // Load 0xFF into B
        0x70, // Write B into [HL]
        0x2B, // Decrement HL
        0xBD, // Compare A & L
        0x20, JR_1, // Jump if zero flag is set, back 5
        0xBC, // Compare A & H
        0x20, JR_2, // Jump if zero flag is set, back 8
        0xED, // Print
        0xEC, // Terminate
    ];

    let breakpoints = [0x100];

    gb.set_debug_mode(true);
    // gb.set_breakpoints(&breakpoints);

    gb.load_rom(0x100, &prog);
    gb.execute_at(0x100);
}
