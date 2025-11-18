#[allow(dead_code)]
mod gb;
mod macros;

use gb::GB;

fn main() {
    let mut gb = GB::new();
    println!("{:X?}", gb.test_decode(&[0xCB, 0xDE, 0xAD, 0xBE, 0xEF,]))
}
