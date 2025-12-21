use std::{
    fs::File,
    io::{Read, Result, Seek, SeekFrom},
    path::Path,
};

const CART_INFO_START: u64 = 0x0147;

pub trait Cart {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, value: u8);
}

fn load_cart(cart_path: &Path) -> Result<Box<dyn Cart>> {
    let cart_file = File::open(cart_path)?;

    let cart_info = [0; 3];
    cart_file.seek(SeekFrom::Start(CART_INFO_START))?;
    cart_file.read_exact(&mut cart_info)?;
    cart_file.rewind()?;

    let cart_type = cart_info[0];
    let cart_rom_size = cart_info[1];
    let cart_ram_size = cart_info[2];
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
