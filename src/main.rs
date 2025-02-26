mod gba;

use gba::*;

fn main() {
    let gba = GBA::new();
    println!("{gba:?}")
}
