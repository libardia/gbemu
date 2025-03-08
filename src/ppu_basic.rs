use std::{cell::RefCell, collections::VecDeque, fmt::Display, ops::Index, rc::Rc};

use minifb::{Window, WindowOptions};

use crate::{
    cpu::MTime,
    mem_region::{
        io_regs::*,
        regions::{OAM, VRAM},
    },
    mmu::MMU,
    ppu::PPU,
};

const BASE_WIDTH: usize = 160;
const BASE_HEIGHT: usize = 144;

const DOTS_PER_LINE: u16 = 456;
const LINES_PER_DRAW: u16 = 144;
const LINES_PER_FRAME: u16 = 154;

const DOTS_PER_OAM_SCAN: u16 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PPUMode {
    HorizontalBlank,
    VerticalBlank,
    OamScan,
    Drawing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White = 0xFF,
    LightGray = 0xAA,
    DarkGrey = 0x55,
    Black = 0x00,
    Transparent,
}
impl Color {
    pub fn color_from_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn minifb_color(&self) -> u32 {
        let v = *self as u8;
        Self::color_from_rgb(v, v, v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Palette(u8);
impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        Palette(value)
    }
}
impl Index<u8> for Palette {
    type Output = Color;

    fn index(&self, index: u8) -> &Self::Output {
        assert!(index <= 3, "Palette only indexes values 0-3");
        let shift = index * 2;
        let mask = 0b11 << shift;
        let bits = self.0 & mask;
        let code = bits >> shift;
        match code {
            0 => &Color::White,
            1 => &Color::LightGray,
            2 => &Color::DarkGrey,
            3 => &Color::Black,
            _ => unreachable!("Invalid color in palette"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pixel {}

#[derive(Debug)]
pub struct BasicPPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
    // Window data
    win: Window,
    scale: usize,
    width: usize,
    height: usize,
    // Draw state
    frame_buffer: Vec<u32>,
    mode: PPUMode,
    next_mode: PPUMode,
    current_line: u8,
    wait: u8,
    dots_this_line: u16,
    dots_this_mode: u16,
    bg_fifo: VecDeque<Pixel>,
    obj_fifo: VecDeque<Pixel>,
    // Palettes
    bg_palette: Palette,
    obj_palette_0: Palette,
    obj_palette_1: Palette,
    // LCD status and control
    io_lcdc: u8,
    io_stat: u8,
    compare_line: u8,
    viewport_x: u8,
    viewport_y: u8,
    window_xp7: u8,
    window_y: u8,
}

impl<M: MMU> PPU<M> for BasicPPU<M> {
    fn new(mmu: Rc<RefCell<M>>, scale: usize) -> Self {
        let width = BASE_WIDTH * scale;
        let height = BASE_HEIGHT * scale;
        let win = Window::new("gbemu", width, height, WindowOptions::default())
            .expect("Failed to create window");
        Self {
            mmu: mmu.clone(),
            win,
            scale,
            width,
            height,
            mode: PPUMode::OamScan,
            next_mode: PPUMode::OamScan,
            current_line: 0,
            compare_line: 0,
            wait: 0,
            dots_this_line: 0,
            dots_this_mode: 0,
            frame_buffer: vec![Color::DarkGrey.minifb_color(); width * height],
            io_lcdc: 0,
            io_stat: 0,
            viewport_x: 0,
            viewport_y: 0,
            window_xp7: 0,
            window_y: 0,
            bg_palette: 0.into(),
            obj_palette_0: 0.into(),
            obj_palette_1: 0.into(),
            bg_fifo: VecDeque::new(),
            obj_fifo: VecDeque::new(),
        }
    }

    fn step_dots(&mut self, dm: MTime) {
        // The number of dots passed is 4 times the CPU m-times passed
        let dots = dm * 4;

        // Load IO registers relevant to the PPU
        self.load_io();

        // Step an amount of time equal to the dots
        for _ in 0..dots {
            self.step();
        }

        // Write out the IO registers the PPU changed
        self.set_io();
    }

    fn should_terminate(&self) -> bool {
        !self.win.is_open()
    }
}

macro_rules! get_bit_flag {
    ($get_name:ident, $byte:ident, $bit_pos:expr) => {
        fn $get_name(&self) -> bool {
            const MASK: u8 = 1 << $bit_pos;
            self.$byte & MASK != 0
        }
    };
}

macro_rules! set_bit_flag {
    ($set_name:ident, $byte:ident, $bit_pos:expr) => {
        fn $set_name(&mut self, value: bool) {
            const MASK: u8 = 1 << $bit_pos;
            const INV_MASK: u8 = !MASK;
            if value {
                self.$byte |= MASK;
            } else {
                self.$byte &= INV_MASK;
            }
        }
    };
}

macro_rules! getset_bit_flag {
    ($get_name:ident, $set_name:ident, $byte:ident, $bit_pos:expr) => {
        get_bit_flag!($get_name, $byte, $bit_pos);
        set_bit_flag!($set_name, $byte, $bit_pos);
    };
}

impl<M: MMU> BasicPPU<M> {
    fn load_io(&mut self) {
        // Convenience
        let b_mmu = self.mmu.borrow();

        // Load IO registers
        self.io_lcdc = b_mmu.get(LCDC);
        self.io_stat = b_mmu.get(STAT);
        self.compare_line = b_mmu.get(LYC);
        self.bg_palette = b_mmu.get(BGP).into();
        self.obj_palette_0 = b_mmu.get(OBP0).into();
        self.obj_palette_1 = b_mmu.get(OBP1).into();
        self.viewport_y = b_mmu.get(SCY);
        self.viewport_x = b_mmu.get(SCX);
        self.window_y = b_mmu.get(WY);
        self.window_xp7 = b_mmu.get(WX);
    }

    fn set_io(&mut self) {
        // Set LYC == LY bit in IO_STAT
        self.set_lyc_eq_ly(self.current_line == self.compare_line);

        // Set mode in IO_STAT
        self.io_stat = (self.io_stat & 0b1111_1100) | (self.mode as u8);

        // Convenience
        let mut mb_mmu = self.mmu.borrow_mut();

        // Set registers
        mb_mmu.set(STAT, self.io_stat);
        mb_mmu.set(LY, self.current_line);
    }

    get_bit_flag!(get_enabled, io_lcdc, 7);
    get_bit_flag!(get_window_tile_map, io_lcdc, 6);
    get_bit_flag!(get_window_enabled, io_lcdc, 5);
    get_bit_flag!(get_bg_window_tiles, io_lcdc, 4);
    get_bit_flag!(get_bg_tile_map, io_lcdc, 3);
    get_bit_flag!(get_obj_size, io_lcdc, 2);
    get_bit_flag!(get_obj_enabled, io_lcdc, 1);
    get_bit_flag!(get_bg_window_enabled, io_lcdc, 0);

    get_bit_flag!(get_lyc_interrupt, io_stat, 6);
    get_bit_flag!(get_mode_2_interrupt, io_stat, 5);
    get_bit_flag!(get_mode_1_interrupt, io_stat, 4);
    get_bit_flag!(get_mode_0_interrupt, io_stat, 3);
    set_bit_flag!(set_lyc_eq_ly, io_stat, 2);

    fn step(&mut self) {
        // Increment line
        if self.dots_this_line == DOTS_PER_LINE {
            self.dots_this_line = 0;
            self.current_line += 1;
            if self.current_line as u16 == LINES_PER_FRAME {
                self.current_line = 0;
            }
        }

        // Change mode
        if self.mode != self.next_mode {
            self.mode = self.next_mode;
            self.dots_this_mode = 0;
        }

        self.dots_this_mode += 1;
        self.dots_this_line += 1;

        match self.mode {
            PPUMode::OamScan => {
                // At the beginning of OAM scan, block OAM.
                self.mmu.borrow_mut().block_range(OAM);

                // TODO: OAM scan

                // OAM scan is always 80 dots long
                if self.dots_this_line == DOTS_PER_OAM_SCAN {
                    self.next_mode = PPUMode::Drawing
                }
            }
            PPUMode::Drawing => {
                // At the beginning of draw, block VRAM.
                self.mmu.borrow_mut().block_range(VRAM);

                // TODO: Drawing mode

                // This is a dummy just to advance the mode
                if self.dots_this_mode == 200 {
                    self.next_mode = PPUMode::HorizontalBlank;
                }
            }
            PPUMode::HorizontalBlank => {
                // At the beginning of hblank, unblock all memory.
                if self.dots_this_mode == 1 {
                    let mut bm_mmu = self.mmu.borrow_mut();
                    bm_mmu.unblock_range(VRAM);
                    bm_mmu.unblock_range(OAM);
                }
                // At the end of the line, switch to...
                else if self.dots_this_line == DOTS_PER_LINE {
                    // ...vblank, if this is the last draw scanline
                    if self.current_line as u16 == LINES_PER_DRAW - 1 {
                        self.next_mode = PPUMode::VerticalBlank;
                    }
                    // ...OAM scan, if there are more draw scanlines
                    else {
                        self.next_mode = PPUMode::OamScan;
                    }
                }
                // Otherwise do nothing.
            }
            PPUMode::VerticalBlank => {
                // At the beginning of vblank, draw the frame buffer. Theoretically it isn't
                // necessary to unblock memory because hblank will right before this.
                if self.dots_this_mode == 1 {
                    self.win
                        .update_with_buffer(&self.frame_buffer, self.width, self.height)
                        .ok();
                }
                // At the end of the line, and if this is the last scanline, switch back to OAM scan
                else if self.dots_this_line == DOTS_PER_LINE
                    && self.current_line as u16 == LINES_PER_FRAME - 1
                {
                    self.next_mode = PPUMode::OamScan;
                }
                // Otherwise do nothing.
            }
        }
    }
}

impl<M: MMU> Display for BasicPPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
