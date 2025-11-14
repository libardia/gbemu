use crate::optable::OP_TABLE;

mod instruction;
mod optable;

fn main() {
    println!("Hello world! {:#X?}", OP_TABLE[0xFF]);
}
