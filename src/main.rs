use crate::gb::cpu::optable::OP_TABLE;

mod gb;

fn main() {
    let b = 0x1A;
    println!("OP[{b:#X}]:\n\n{:X?}", OP_TABLE[b]);
}
