mod gba;

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

    gba.load(&prog);
    gba.run(true);
}
