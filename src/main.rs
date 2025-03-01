mod gba;

use gba::*;

fn main() {
    let mut gba = GBA::new();
    gba.mainloop();

    // println!("{:0>8b}", 0b1111_0000u8.swap_bytes())
}
