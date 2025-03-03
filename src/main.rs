mod gba;
mod hex;

use gba::*;

fn main() {
    let mut gba = GBA::new();

    let prog = [
        0x01, 0xAD, 0xDE, // Write 0xDEAD into BC
        0x80, // A += B (0xDE)
        0x81, // A += C (0x8B)
        0xEA, 0xAD, 0xDE, // Write A to [0xDEAD]
        0xEC, // Terminate
    ];

    gba.load(0x0000, &mmu::BOOT_ROM);
    gba.load(0x0100, &make_dummy_header());
    gba.load(0x0150, &prog);
    gba.run(true);
    gba.translate(&mmu::BOOT_ROM);
}

const HEADER_BEGIN: usize = 0x0100;
const HEADER_END: usize = 0x014F;
const HEADER_SIZE: usize = HEADER_END - HEADER_BEGIN + 1;
fn make_dummy_header() -> [u8; HEADER_SIZE] {
    const LOGO: [u8; 48] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ];

    const ENTRY: [u8; 4] = [
        // NOP; JMP 0x0150
        0x00, 0xC3, 0x50, 0x01,
    ];

    let mut header: [u8; HEADER_SIZE] = [0u8; HEADER_SIZE];
    for (i, b) in ENTRY.iter().enumerate() {
        let address = (i + 0x100) - HEADER_BEGIN;
        header[address] = *b;
    }

    for (i, b) in LOGO.iter().enumerate() {
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
