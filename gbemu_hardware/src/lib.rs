use crate::{cpu::CPU, mmu::MMU, regions::HEADER};
use std::{
    fs::File,
    io::{BufReader, Read, Result},
    path::Path,
};

mod cart;
mod cart_types;
mod cpu;
mod mmu;
mod regions;

struct GameBoy {
    mmu: MMU,
    cpu: CPU,
}

fn run(rom_path: &str) -> Result<()> {
    let binfile = File::open(Path::new(&rom_path));
    let mut reader = BufReader::new(binfile?);

    // Seek forward to the header
    reader.seek_relative(0x100);

    let mut header = vec![0; HEADER.size()];
    reader.read_exact(&mut header);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
