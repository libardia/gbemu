use crate::gb::GameBoy;

mod gb;

fn main() {
    let mut gb = GameBoy::new();
    gb.run();
}
