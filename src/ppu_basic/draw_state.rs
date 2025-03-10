use core::slice;
use std::{collections::VecDeque, ops::Index};

use crate::{mem_region::regions::OAM, mmu::MMU};

use super::{get_bit_flag, PPUMode};

const SELECTED_OBJS_RESERVED: usize = 10 + 5;
const FIFO_RESERVED: usize = 16 + 8;

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

pub const GRAYSCALE_PALETTE: MetaPalette = MetaPalette(WHITE, LIGHT_GRAY, DARK_GRAY, BLACK);
pub const GAMEBOY_PALETTE: MetaPalette =
    MetaPalette(GB_GREEN_0, GB_GREEN_1, GB_GREEN_2, GB_GREEN_3);

pub const NUM_OBJECTS: u16 = (OAM.end - OAM.begin + 1) / OBJECT_BYTE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaPalette(u32, u32, u32, u32);
impl Index<ColorID> for MetaPalette {
    type Output = u32;

    fn index(&self, index: ColorID) -> &Self::Output {
        match index {
            ColorID::Color0 => &self.0,
            ColorID::Color1 => &self.1,
            ColorID::Color2 => &self.2,
            ColorID::Color3 => &self.3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorID {
    Color0,
    Color1,
    Color2,
    Color3,
}
impl From<u8> for ColorID {
    fn from(value: u8) -> Self {
        assert!(
            value < 4,
            "Tried to make a color from a value larger than 3"
        );
        match value {
            0 => ColorID::Color0,
            1 => ColorID::Color1,
            2 => ColorID::Color2,
            3 => ColorID::Color3,
            _ => unreachable!("Tried to make a color from a value larger than 3"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette(u8);
impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        Palette(value)
    }
}
impl Index<u8> for Palette {
    type Output = ColorID;

    fn index(&self, index: u8) -> &Self::Output {
        assert!(index <= 3, "Palette only indexes values 0-3");
        let shift = index * 2;
        let mask = 0b11 << shift;
        let bits = self.0 & mask;
        let code = bits >> shift;
        assert!(code < 4);
        match code {
            0 => &ColorID::Color0,
            1 => &ColorID::Color1,
            2 => &ColorID::Color2,
            3 => &ColorID::Color3,
            _ => unreachable!("Invalid color in palette"),
        }
    }
}

const PIXELS_PER_TILE: usize = 8 * 8;
#[derive(Debug)]
pub struct Tile {
    pixels: [ColorID; PIXELS_PER_TILE],
}
impl Tile {
    pub fn new<M: MMU>(mmu: &M, start_address: u16) -> Self {
        let mut pixels = [ColorID::Color0; PIXELS_PER_TILE];

        for row in 0..8 {
            let a = start_address + (row * 2) as u16;
            let tile_ls = mmu.read_blocked_byte(a);
            let tile_ms = mmu.read_blocked_byte(a + 1);
            for bit in 0..8 {
                let m = 1 << (7 - bit);
                let color = match (tile_ms & m != 0, tile_ls & m != 0) {
                    (false, false) => ColorID::Color0,
                    (false, true) => ColorID::Color1,
                    (true, false) => ColorID::Color2,
                    (true, true) => ColorID::Color3,
                };
                pixels[(row * 8) + bit] = color;
            }
        }

        Self { pixels }
    }

    // TODO: decide what type y makes most sense to be after draw is done
    pub fn row_at(&self, y: usize) -> &[ColorID] {
        &self.pixels[(y * 8)..(y * 8 + 8)]
    }

    // TODO: Delete this one if not necessary
    pub fn pixel_at(&self, x: u8, y: u8) -> &ColorID {
        &self.pixels[(y * 8 + x) as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FIFOPixel {
    color: ColorID,
    pub pixels: [ColorID; 64],
}

pub const OBJECT_BYTE_SIZE: u16 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Object {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub flags: u8,
}
impl Object {
    pub fn get_8x16_tile_indexes(&self) -> (u8, u8) {
        let tile1 = self.tile_index & 0xFE;
        let tile2 = self.tile_index & 0x01;
        (tile1, tile2)
    }

    get_bit_flag!(get_priority, flags, 7);
    get_bit_flag!(get_y_flip, flags, 6);
    get_bit_flag!(get_x_flip, flags, 5);
    get_bit_flag!(get_palette, flags, 4);
}

#[derive(Debug)]
pub struct DrawState {
    pub mode: PPUMode,
    pub next_mode: PPUMode,
    pub current_line: u8,
    pub dots_this_line: u16,
    pub dots_this_mode: u16,
    pub selected_objects: Vec<Object>,
    pub bg_fifo: VecDeque<FIFOPixel>,
    pub obj_fifo: VecDeque<FIFOPixel>,
    pub draw_mode_length: u16,
}
impl DrawState {
    pub fn new() -> Self {
        let mut selected_objects = Vec::new();
        selected_objects.reserve(SELECTED_OBJS_RESERVED);
        let mut bg_fifo = VecDeque::new();
        bg_fifo.reserve(FIFO_RESERVED);
        let mut obj_fifo = VecDeque::new();
        obj_fifo.reserve(FIFO_RESERVED);

        Self {
            mode: PPUMode::OamScan,
            next_mode: PPUMode::OamScan,
            current_line: 0,
            dots_this_line: 0,
            dots_this_mode: 0,
            selected_objects,
            bg_fifo,
            obj_fifo,
            draw_mode_length: 0,
        }
    }
}
