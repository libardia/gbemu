use crate::{cart::load_cart, cpu::CPU, mmu::MMU};
use std::{io::Result, path::Path};

mod cart;
mod cart_types;
mod cpu;
mod macros;
mod mmu;

struct GameBoy {
    mmu: MMU,
    cpu: CPU,
}

fn run(rom_path: &str) -> Result<()> {
    let cart = load_cart(Path::new(rom_path))?;

    //TODO: create the rest of the GameBoy object and run it

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
