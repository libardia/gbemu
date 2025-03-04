mod gba;
mod hex;

use gba::*;

fn main() {
    let mut gba = GBA::new();

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

    gba.debug_mode = true;
    gba.set_breakpoints(&breakpoints);

    gba.load(0x0000, &mmu::BOOT_ROM);
    gba.load(0x0100, &make_dummy_header());
    gba.load(0x0150, &prog);
    // Skip the boot rom... it won't work without I/O registers
    gba.run_at(0x100);
}

const HEADER_BEGIN: usize = 0x0100;
const HEADER_END: usize = 0x014F;
const HEADER_SIZE: usize = HEADER_END - HEADER_BEGIN + 1;
fn make_dummy_header() -> [u8; HEADER_SIZE] {
    let logo = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ];

    let entry = [
        // NOP; JMP 0x0150
        0x00, 0xC3, 0x50, 0x01,
    ];

    let mut header: [u8; HEADER_SIZE] = [0u8; HEADER_SIZE];
    for (i, b) in entry.iter().enumerate() {
        let address = (i + 0x100) - HEADER_BEGIN;
        header[address] = *b;
    }

    for (i, b) in logo.iter().enumerate() {
        let address = (i + 0x104) - HEADER_BEGIN;
        header[address] = *b;
    }

    let checksum_begin: usize = 0x0134 - HEADER_BEGIN;
    let checksum_end: usize = 0x014C - HEADER_BEGIN + 1;
    let checksum_address: usize = 0x014D - HEADER_BEGIN;

    let mut checksum = 0u8;
    for i in checksum_begin..checksum_end {
        checksum = checksum.wrapping_sub(header[i].wrapping_sub(1))
    }
    header[checksum_address] = checksum;

    header
}
