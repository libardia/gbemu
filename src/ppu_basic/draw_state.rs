use std::collections::VecDeque;

use super::PPUMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pixel {}

#[derive(Debug)]
pub struct DrawState {
    pub mode: PPUMode,
    pub next_mode: PPUMode,
    pub current_line: u8,
    pub dots_this_line: u16,
    pub dots_this_mode: u16,
    pub bg_fifo: VecDeque<Pixel>,
    pub obj_fifo: VecDeque<Pixel>,
}

impl DrawState {
    pub fn new() -> Self {
        Self {
            mode: PPUMode::OamScan,
            next_mode: PPUMode::OamScan,
            current_line: 0,
            dots_this_line: 0,
            dots_this_mode: 0,
            bg_fifo: VecDeque::new(),
            obj_fifo: VecDeque::new(),
        }
    }
}
