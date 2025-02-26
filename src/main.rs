mod gba;

use gba::*;

fn main() {
    let gba = GBA::default();
    println!("{gba:?}")
}
