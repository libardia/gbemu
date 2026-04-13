use std::{fs::File, path::Path};

use crate::{
    gb::cart::{other::CartOther, romonly::CartRomOnly},
    macros::unwrap_or_error,
};

pub mod header;
pub mod other;
pub mod romonly;

pub trait Cart {
    fn init(&mut self);
}

pub fn load_cart(file_path: &str) -> Box<dyn Cart> {
    let _f = unwrap_or_error!(File::open(Path::new(file_path)), "Failed to open cart file");

    if file_path.is_empty() {
        Box::new(CartRomOnly {})
    } else {
        Box::new(CartOther {})
    }
}
