use crate::{
    gb::gpu::color_id::ColorID,
    util::{error_and_panic, new},
};

#[derive(Debug)]
pub struct Tile {
    pub address: u16,
    pub pixels: [ColorID; 8 * 8],
}
impl Tile {
    new!(address: u16; {
        Self {
            address,
            pixels: [ColorID::Color0; 8 * 8],
        }
    });

    pub fn pix_at(&self, x: usize, y: usize) -> ColorID {
        if x > 7 || y > 7 {
            error_and_panic!("Tried to get out of bounds pixel in tile: ({x}, {y})");
        }

        self.pixels[y * 8 + x]
    }
}
