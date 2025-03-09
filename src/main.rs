use cpu_basic::BasicCPU;
use gb::GB;
use mmu_basic::BasicMMU;
use ppu_basic::BasicPPU;

mod cpu;
mod cpu_basic;
mod gb;
mod hex;
mod mem_region;
mod mmu;
mod mmu_basic;
mod ppu;
mod ppu_basic;

fn make_dummy_header() -> [u8; 0x150] {
    const LOGO: [u8; 48] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ];

    let mut header = [0; 0x150];

    header[0x100] = 0x00;
    header[0x101] = 0xC3;
    header[0x102] = 0x50;
    header[0x103] = 0x01;

    for (i, b) in LOGO.iter().enumerate() {
        header[0x104 + i] = *b;
    }

    let mut checksum = 0u8;
    for a in 0x134..=0x14C {
        checksum = checksum.wrapping_sub(header[a]).wrapping_sub(1);
    }
    header[0x14D] = checksum;

    header
}

#[allow(unused)]
fn main() {
    type MMU = BasicMMU;
    type CPU = BasicCPU<MMU>;
    type PPU = BasicPPU<MMU>;

    let mut gb: GB<CPU, PPU, MMU> = GB::new(3);

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
        0xED, // Print
        0xEC, // Terminate
    ];

    let infinite_loop = [0x20, -2i8 as u8];

    let breakpoints = [0x150 + 16];

    gb.set_debug_mode(true);
    // gb.set_breakpoints(&breakpoints);

    // gb.load_rom_file(
    //     r"C:\Users\libar\Projects\rust\gbemu\test-roms\blargg\cpu_instrs\cpu_instrs.gb",
    // );
    // gb.load_rom(0x100, &infinite_loop);

    gb.load_rom(0, &make_dummy_header());
    gb.load_rom(0x150, &binary_coded_decimal);

    gb.execute_at(0);
}
