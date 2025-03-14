use crate::{gb::gpu::color_id::ColorID, util::new};

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
}
