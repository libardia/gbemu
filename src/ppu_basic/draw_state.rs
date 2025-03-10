use std::collections::VecDeque;

use super::{get_bit_flag, PPUMode};

const SELECTED_OBJS_RESERVED: usize = 10 + 5;
const FIFO_RESERVED: usize = 16 + 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {}

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
    pub bg_fifo: VecDeque<Pixel>,
    pub obj_fifo: VecDeque<Pixel>,
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
        }
    }
}
