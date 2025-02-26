mod gba;

use gba::*;

fn main() {
    let mut gba = GBA::new();
    gba.mainloop();
    println!("{gba:?}")
}
