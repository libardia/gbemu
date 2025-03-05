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

    let simple_add = [
        0x01, 0xAD, 0xDE, // Write 0xDEAD into BC
        0x80, // A += B (0xDE)
        0x81, // A += C (0x8B)
        0xEA, 0xAD, 0xDE, // Write A to [0xDEAD]
        0xED, // Print
    ];

    let jr_1 = -5i8 as u8;
    let jr_2 = -8i8 as u8;
    let fill_mem = [
        0x21, 0xFF, 0xFF, // Write 0xFFFF into HL
        0xAF, // A = A xor A; sets A to 0
        0x06, 0xFF, // Load 0xFF into B
        0x70, // Write B into [HL]
        0x2B, // Decrement HL
        0xBD, // Compare A & L
        0x20, jr_1, // Jump if zero flag is set, back 5
        0xBC, // Compare A & H
        0x20, jr_2, // Jump if zero flag is set, back 8
        0xED, // Print
        0xEC, // Terminate
    ];

    let binary_coded_decimal = [
        0x3E, 0x94, // Load 0x94 into A; 94 in BCD
        0x06, 0x29, // Load 0x29 into B; 29 in BCD
        0x80, // A += B (0x94 + 0x29 = 0xBD, in BCD: 0x94 + 0x29 = 123 -> 0x23)
        0x27, // DAA (convert A into BCD)
        0x3E, 0x05, // Load 0x05 into A; 5 in BCD
        0x87, // A += A (0x05 + 0x05 = 0x0A, in BCD: 0x05 + 0x05 = 0x10)
        0x27, // DAA (convert A into BCD)
        0x3E, 0x10, // Load 0x10 into A; 10 in BCD
        0x06, 0x05, // Load 0x05 into B; 5 in BCD
        0x90, // A -= B (0x10 - 0x05 = 0x0B, in BCD: 0x10 - 0x05 = 0x05)
        0x27, // DAA (convert A into BCD)
        0xEC, // Terminate
    ];

    let breakpoints = [0x100];

    gb.set_debug_mode(true);
    gb.set_breakpoints(&breakpoints);

    gb.load_rom_file(
        r"C:\Users\libar\Projects\rust\gbemu\test-roms\blargg\cpu_instrs\cpu_instrs.gb",
    );
    gb.execute_at(0x100);
}
