use crate::gb::cpu::optable::OP_TABLE;
use std::fs;

mod gb;

fn main() {
    test_opcodes();
}

fn test_opcodes() {
    let mut codes_from_file: [&str; 0x100] = [""; 0x100];
    let contents = fs::read_to_string("res/opcodes-base.txt").unwrap();
    let lines = contents.split("\n");
    for line in lines {
        let mut parts = line.split("\t");
        let code =
            usize::from_str_radix(parts.next().unwrap().strip_prefix("0x").unwrap(), 16).unwrap();
        let mnem = parts.next().unwrap();
        codes_from_file[code] = mnem.trim();
    }

    for i in 0..=0xFF {
        println!(
            "{i:#X?}:\n\tMine:    {:X?}\n\tCorrect: {}",
            OP_TABLE[i], codes_from_file[i]
        );
    }
}
