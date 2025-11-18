#[allow(dead_code)]
mod gb;

use gb::GB;

fn main() {
    let mut gb = GB::default();
    println!("{:X?}", gb.test_decode(&[0xCB, 0xDE, 0xAD, 0xBE, 0xEF,]))
}
