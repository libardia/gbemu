use std::ops::Index;

use super::color_id::ColorID;

const fn color_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

const WHITE: u32 = color_from_rgb(0xFF, 0xFF, 0xFF);
const LIGHT_GRAY: u32 = color_from_rgb(0xAA, 0xAA, 0xAA);
const DARK_GRAY: u32 = color_from_rgb(0x55, 0x55, 0x55);
const BLACK: u32 = color_from_rgb(0x00, 0x00, 0x00);

const GB_GREEN_0: u32 = color_from_rgb(0x9B, 0xBC, 0x0F);
const GB_GREEN_1: u32 = color_from_rgb(0x8B, 0xAC, 0x0F);
const GB_GREEN_2: u32 = color_from_rgb(0x30, 0x62, 0x30);
const GB_GREEN_3: u32 = color_from_rgb(0x0F, 0x38, 0x0F);

pub const GRAYSCALE_MPALETTE: MetaPalette = MetaPalette([WHITE, LIGHT_GRAY, DARK_GRAY, BLACK]);
pub const GAMEBOY_MPALETTE: MetaPalette =
    MetaPalette([GB_GREEN_0, GB_GREEN_1, GB_GREEN_2, GB_GREEN_3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaPalette([u32; 4]);
impl Index<ColorID> for MetaPalette {
    type Output = u32;

    fn index(&self, index: ColorID) -> &Self::Output {
        &self.0[index as usize]
    }
}
