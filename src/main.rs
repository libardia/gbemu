use crate::gb::cpu::optable::OP_TABLE;

mod gb;

fn main() {
    println!("Hello world! {:#X?}", OP_TABLE[0xFF]);
}
