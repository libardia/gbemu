use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::Display,
    ops::Index,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

use minifb::{Window, WindowOptions};

use crate::{
    cpu::MTime,
    gb::DEFAULT_FPS,
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

fn color_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

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
    pub fn to_minifb_color(&self) -> u32 {
        let v = *self as u8;
        color_from_rgb(v, v, v)
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
    w_scale: usize,
    w_width: usize,
    w_height: usize,
    // Draw state
    last_frame_time: Instant,
    frame_rate: f32,
    time_per_frame: Duration,
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
    // Debug
    d_px: u8,
    d_py: u8,
}

impl<M: MMU> PPU<M> for BasicPPU<M> {
    fn new(mmu: Rc<RefCell<M>>, scale: usize, frame_rate: f32) -> Self {
        let width = BASE_WIDTH * scale;
        let height = BASE_HEIGHT * scale;
        let win = Window::new("gbemu", width, height, WindowOptions::default())
            .expect("Failed to create window");
        Self {
            mmu: mmu.clone(),
            win,
            w_scale: scale,
            w_width: width,
            w_height: height,
            mode: PPUMode::OamScan,
            next_mode: PPUMode::OamScan,
            current_line: 0,
            compare_line: 0,
            wait: 0,
            dots_this_line: 0,
            dots_this_mode: 0,
            last_frame_time: Instant::now(),
            time_per_frame: Duration::from_micros((1e6f32 / frame_rate).round() as u64),
            frame_rate,
            frame_buffer: vec![Color::White.to_minifb_color(); width * height],
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
            d_px: 0,
            d_py: 0,
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
    fn draw_scaled_pixel(&mut self, color: u32, x: u8, y: u8) {
        let actual_x = x as usize * self.w_scale;
        let actual_y = y as usize * self.w_scale;
        let flattened = actual_y * self.w_width + actual_x;
        for dy in 0..self.w_scale {
            for dx in 0..self.w_scale {
                let i = flattened + (dy * self.w_width) + dx;
                self.frame_buffer[i] = color;
            }
        }
    }

    fn present(&mut self) {
        // Wait for framerate
        let elapsed = self.last_frame_time.elapsed();
        let mut wait = Duration::new(0, 0);
        if elapsed < self.time_per_frame {
            wait = self.time_per_frame - elapsed;
            sleep(wait);
        }
        let fps = 1e6f32 / (elapsed + wait).as_micros() as f32;
        self.last_frame_time = Instant::now();
        self.win.set_title(&format!(
            "gbemu | FPS: {:0.2} | Speed: {:0.2}%",
            fps,
            fps * 100.0 / DEFAULT_FPS
        ));

        // Draw frame buffer
        self.win
            .update_with_buffer(&self.frame_buffer, self.w_width, self.w_height)
            .expect("Failed to update window");
    }

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
                if self.dots_this_mode == 1 {
                    self.mmu.borrow_mut().block_range(VRAM);
                    // for debug
                    if self.current_line as u16 == LINES_PER_DRAW - 1 {
                        // let mut rng = rand::rng();
                        // let color = color_from_rgb(rng.random(), rng.random(), rng.random());
                        self.draw_scaled_pixel(
                            Color::Black.to_minifb_color(),
                            self.d_px,
                            self.d_py,
                        );
                        self.d_px += 1;
                        if self.d_px as usize > BASE_WIDTH {
                            self.d_px = 0;
                            self.d_py = (self.d_py + 1) % BASE_HEIGHT as u8;
                        }
                    }
                }

                // TODO: Drawing mode

                // This is a dummy impl for draw mode
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
                    self.present();
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
