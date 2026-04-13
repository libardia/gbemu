use std::{fs::File, path::Path};

use crate::{
    gb::cart::{
        header::{CartType, read_header},
        romonly::CartRomOnly,
    },
    macros::{hex, unwrap_or_error},
};

pub mod header;
pub mod romonly;

pub trait Cart {
    fn load(&mut self, f: &mut File);
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}

pub fn load_cart(file_path: &str) -> Box<dyn Cart> {
    let p = Path::new(file_path);
    let f = &mut unwrap_or_error!(File::open(p), "failed to open cart file");
    let header = unwrap_or_error!(read_header(f), "error reading header");

    let mut cart: Box<dyn Cart> = match header.cart_type {
        CartType::RomOnly => Box::new(CartRomOnly::new()),
        // CartType::MBC1 => Box::new(todo!()),
        // CartType::MBC1Ram => Box::new(todo!()),
        // CartType::MBC1RamBattery => Box::new(todo!()),
        // CartType::MBC2 => Box::new(todo!()),
        // CartType::MBC2Battery => Box::new(todo!()),
        // CartType::RomRam => Box::new(todo!()),
        // CartType::RomRamBattery => Box::new(todo!()),
        // CartType::MMM01 => Box::new(todo!()),
        // CartType::MMM01Ram => Box::new(todo!()),
        // CartType::MMM01RamBattery => Box::new(todo!()),
        // CartType::MBC3TimerBattery => Box::new(todo!()),
        // CartType::MBC3TimerRamBattery => Box::new(todo!()),
        // CartType::MBC3 => Box::new(todo!()),
        // CartType::MBC3Ram => Box::new(todo!()),
        // CartType::MBC3RamBattery => Box::new(todo!()),
        // CartType::MBC5 => Box::new(todo!()),
        // CartType::MBC5Ram => Box::new(todo!()),
        // CartType::MBC5RamBattery => Box::new(todo!()),
        // CartType::MBC5Rumble => Box::new(todo!()),
        // CartType::MBC5RumbleRam => Box::new(todo!()),
        // CartType::MBC5RumbleRamBattery => Box::new(todo!()),
        // CartType::MBC6 => Box::new(todo!()),
        // CartType::MBC7SensorRumbleRamBattery => Box::new(todo!()),
        // CartType::PocketCamera => Box::new(todo!()),
        // CartType::BandaiTama5 => Box::new(todo!()),
        // CartType::HuC3 => Box::new(todo!()),
        // CartType::HuC1RamBattery => Box::new(todo!()),
        CartType::Unknown(byte) => panic!("unknown cartridge type: {}", hex!(byte, 2)),
        cart_type => todo!("cart type {cart_type:?} not yet supported"),
    };

    cart.load(f);

    cart
}
