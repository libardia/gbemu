use std::{
    cell::RefCell,
    fmt::Display,
    ops::Index,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

use draw_state::{DrawState, Object, OBJECT_BYTE_SIZE};
use log::{debug, trace};
use minifb::{Window, WindowOptions};

use crate::{
    cpu::MTime,
    either::either,
    gb::DEFAULT_FPS,
    mem_region::{
        io_regs::*,
        regions::{OAM, VRAM},
    },
    mmu::MMU,
    ppu::PPU,
};

pub mod draw_state;

const BASE_WIDTH: usize = 160;
const BASE_HEIGHT: usize = 144;

const DOTS_PER_LINE: u16 = 456;
const LINES_PER_DRAW: u16 = 144;
const LINES_PER_FRAME: u16 = 154;

const DRAW_ON_DOT: u32 = 65664;
const DOTS_PER_FRAME: u32 = 70224;

const DOTS_PER_OAM_SCAN: u16 = 80;

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

const GRAYSCALE_PALETTE: MetaPalette = MetaPalette(WHITE, LIGHT_GRAY, DARK_GRAY, BLACK);
const GAMEBOY_PALETTE: MetaPalette = MetaPalette(GB_GREEN_0, GB_GREEN_1, GB_GREEN_2, GB_GREEN_3);

const NUM_OBJECTS: u16 = (OAM.end - OAM.begin + 1) / OBJECT_BYTE_SIZE;

/* #region Types =============================================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Color0,
    Color1,
    Color2,
    Color3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MetaPalette(u32, u32, u32, u32);
impl Index<Color> for MetaPalette {
    type Output = u32;

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::Color0 => &self.0,
            Color::Color1 => &self.1,
            Color::Color2 => &self.2,
            Color::Color3 => &self.3,
        }
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
            0 => &Color::Color0,
            1 => &Color::Color1,
            2 => &Color::Color2,
            3 => &Color::Color3,
            _ => unreachable!("Invalid color in palette"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PPUMode {
    HorizontalBlank,
    VerticalBlank,
    OamScan,
    Drawing,
}

/* #endregion */

#[derive(Debug)]
pub struct BasicPPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
    // Window data
    win: Window,
    w_scale: usize,
    w_width: usize,
    w_height: usize,
    meta_palette: MetaPalette,
    time_per_frame: Duration,
    last_frame_time: Instant,
    dots_this_frame: u32,
    // Draw state
    frame_buffer: Vec<u32>,
    ds: DrawState,
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
    interrupt_requests: u8,
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

        let mut new = Self {
            mmu: mmu.clone(),
            win,
            w_scale: scale,
            w_width: width,
            w_height: height,
            frame_buffer: Vec::new(),
            ds: DrawState::new(),
            compare_line: 0,
            last_frame_time: Instant::now(),
            time_per_frame: Duration::from_micros((1e6f32 / frame_rate).round() as u64),
            io_lcdc: 0,
            io_stat: 0,
            viewport_x: 0,
            viewport_y: 0,
            window_xp7: 0,
            window_y: 0,
            bg_palette: 0.into(),
            obj_palette_0: 0.into(),
            obj_palette_1: 0.into(),
            d_px: 0,
            d_py: 0,
            meta_palette: GAMEBOY_PALETTE,
            dots_this_frame: 0,
            interrupt_requests: 0,
        };

        new.reset_frame_buffer();
        new
    }

    fn step_dots(&mut self, dm: MTime) {
        // The number of dots passed is 4 times the CPU m-times passed
        let dots = dm * 4;

        // Load IO registers relevant to the PPU
        let last_enabled = self.get_enabled();
        self.load_io();

        // LCD was just enabled
        if !last_enabled && self.get_enabled() {
            debug!("LCD was enabled!");

            // Reset draw state
            self.reset_state();

            // Reset `dots_this_frame`
            self.dots_this_frame = 0;
        }
        // LCD was just disabled
        else if last_enabled && !self.get_enabled() {
            debug!("LCD was disabled!");

            // Reset frame buffer
            self.reset_frame_buffer();

            // Make sure memory is unblocked
            self.mmu.borrow_mut().unblock_range(OAM);
            self.mmu.borrow_mut().unblock_range(VRAM);
        }

        // Step an amount of time equal to the dots
        for _ in 0..dots {
            if self.get_enabled() {
                self.step();
            }

            if DRAW_ON_DOT == self.dots_this_frame {
                self.wait_and_present();
            }

            self.dots_this_frame = (self.dots_this_frame + 1) % DOTS_PER_FRAME;
        }

        // Write out the IO registers the PPU changed
        self.set_io();
    }

    fn should_terminate(&self) -> bool {
        !self.win.is_open()
    }
}

/* #region Macros ============================================================================== */

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

pub(crate) use get_bit_flag;
pub(crate) use getset_bit_flag;
pub(crate) use set_bit_flag;

/* #endregion */

impl<M: MMU> BasicPPU<M> {
    /* #region Helpers ========================================================================= */
    fn reset_frame_buffer(&mut self) {
        self.frame_buffer = vec![self.meta_palette[Color::Color0]; self.w_width * self.w_height];
    }

    fn reset_state(&mut self) {
        self.ds = DrawState::new();
    }

    fn draw_scaled_pixel(&mut self, color: Color, x: u8, y: u8) {
        let actual_x = x as usize * self.w_scale;
        let actual_y = y as usize * self.w_scale;
        let flattened = actual_y * self.w_width + actual_x;
        for dy in 0..self.w_scale {
            for dx in 0..self.w_scale {
                let i = flattened + (dy * self.w_width) + dx;
                self.frame_buffer[i] = self.meta_palette[color];
            }
        }
    }

    fn wait_and_present(&mut self) {
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
        self.io_lcdc = b_mmu.read_byte(REG_LCDC);
        self.io_stat = b_mmu.read_byte(REG_STAT);
        self.compare_line = b_mmu.read_byte(REG_LYC);
        self.bg_palette = b_mmu.read_byte(REG_BGP).into();
        self.obj_palette_0 = b_mmu.read_byte(REG_OBP0).into();
        self.obj_palette_1 = b_mmu.read_byte(REG_OBP1).into();
        self.viewport_y = b_mmu.read_byte(REG_SCY);
        self.viewport_x = b_mmu.read_byte(REG_SCX);
        self.window_y = b_mmu.read_byte(REG_WY);
        self.window_xp7 = b_mmu.read_byte(REG_WX);
        self.interrupt_requests = b_mmu.read_byte(REG_IF);
    }

    fn set_io(&mut self) {
        // Set LYC == LY bit in IO_STAT
        self.set_lyc_eq_ly(self.ds.current_line == self.compare_line);

        // Set mode in IO_STAT (always report 0 when disabled)
        let v = either(self.get_enabled(), self.ds.mode as u8, 0);
        self.io_stat = (self.io_stat & 0b1111_1100) | v;

        // Convenience
        let mut mb_mmu = self.mmu.borrow_mut();

        // Set registers
        mb_mmu.write_byte(REG_STAT, self.io_stat);
        mb_mmu.write_byte(REG_LY, self.ds.current_line);
        mb_mmu.write_byte(REG_IF, self.interrupt_requests);
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
    set_bit_flag!(set_vblank_interrupt, interrupt_requests, 0);

    /* #endregion */

    /* #region Step ============================================================================ */

    fn step(&mut self) {
        // Increment line
        if self.ds.dots_this_line == DOTS_PER_LINE {
            self.ds.dots_this_line = 0;
            self.ds.current_line += 1;
            if self.ds.current_line as u16 == LINES_PER_FRAME {
                self.ds.current_line = 0;
            }
        }

        // Change mode
        if self.ds.mode != self.ds.next_mode {
            self.ds.mode = self.ds.next_mode;
            self.ds.dots_this_mode = 0;
        }

        self.ds.dots_this_mode += 1;
        self.ds.dots_this_line += 1;

        match self.ds.mode {
            PPUMode::OamScan => {
                // At the beginning of OAM scan, block OAM, then do the scan.
                if self.ds.dots_this_mode == 1 {
                    trace!("Beginning of OAM scan");

                    self.mmu.borrow_mut().block_range(OAM);
                    if self.get_obj_enabled() {
                        // Reset the selected objects
                        self.ds.selected_objects.clear();

                        // Convenience
                        let b_mmu = self.mmu.borrow();

                        for i in 0..NUM_OBJECTS {
                            // Prepare the addresses of the object's bytes; just here for readability
                            let ix4 = i * 4;
                            let addresses = (
                                OAM.begin + ix4 + 0,
                                OAM.begin + ix4 + 1,
                                OAM.begin + ix4 + 2,
                                OAM.begin + ix4 + 3,
                            );
                            let obj = Object {
                                y: b_mmu.read_blocked_byte(addresses.0),
                                x: b_mmu.read_blocked_byte(addresses.1),
                                tile_index: b_mmu.read_blocked_byte(addresses.2),
                                flags: b_mmu.read_blocked_byte(addresses.3),
                            };

                            // Get the upper and lower bounds for an object to be on this line
                            let lower;
                            let upper = obj.y;
                            if !self.get_obj_size() {
                                // Object size 8x8
                                lower = upper.wrapping_sub(8);
                            } else {
                                // Object size 8x16
                                lower = upper.wrapping_sub(16);
                            }

                            // lower <= scanline < upper
                            if lower <= self.ds.current_line && self.ds.current_line < upper {
                                self.ds.selected_objects.push(obj);
                                // Maximum of 10 objects per line
                                if self.ds.selected_objects.len() == 10 {
                                    break;
                                }
                            }
                        }
                        trace!("OAM scan found {} objects", self.ds.selected_objects.len());
                    } else {
                        trace!("OAM scan skipped: objects are disabled.");
                    }
                }
                // We don't do anything else in this mode, until it hits 80 dots.

                // OAM scan is always 80 dots long
                if self.ds.dots_this_line == DOTS_PER_OAM_SCAN {
                    self.ds.next_mode = PPUMode::Drawing
                }
            }
            PPUMode::Drawing => {
                // At the beginning of draw, block VRAM.
                if self.ds.dots_this_mode == 1 {
                    self.mmu.borrow_mut().block_range(VRAM);
                    // dummy, for debug
                    if self.ds.current_line as u16 == LINES_PER_DRAW - 1 {
                        // let mut rng = rand::rng();
                        // let color = color_from_rgb(rng.random(), rng.random(), rng.random());
                        self.draw_scaled_pixel(Color::Color3, self.d_px, self.d_py);
                        self.d_px += 1;
                        if self.d_px as usize >= BASE_WIDTH {
                            self.d_px = 0;
                            self.d_py = (self.d_py + 1) % BASE_HEIGHT as u8;
                        }
                    }
                }

                // TODO: Drawing mode

                // This is a dummy impl so draw mode ends
                if self.ds.dots_this_mode == 200 {
                    self.ds.next_mode = PPUMode::HorizontalBlank;
                }
            }
            PPUMode::HorizontalBlank => {
                // At the beginning of hblank, unblock all memory.
                if self.ds.dots_this_mode == 1 {
                    let mut bm_mmu = self.mmu.borrow_mut();
                    bm_mmu.unblock_range(VRAM);
                    bm_mmu.unblock_range(OAM);
                }
                // At the end of the line, switch to...
                else if self.ds.dots_this_line == DOTS_PER_LINE {
                    // ...vblank, if this is the last draw scanline
                    if self.ds.current_line as u16 == LINES_PER_DRAW - 1 {
                        self.ds.next_mode = PPUMode::VerticalBlank;
                    }
                    // ...OAM scan, if there are more draw scanlines
                    else {
                        self.ds.next_mode = PPUMode::OamScan;
                    }
                }
                // Otherwise do nothing.
            }
            PPUMode::VerticalBlank => {
                // At the beginning of vblank, request the vblank interrupt.
                if self.ds.dots_this_mode == 1 {
                    self.set_vblank_interrupt(true);
                }
                // At the end of the line, and if this is the last scanline, switch back to OAM scan
                else if self.ds.dots_this_line == DOTS_PER_LINE
                    && self.ds.current_line as u16 == LINES_PER_FRAME - 1
                {
                    self.ds.next_mode = PPUMode::OamScan;
                }
                // Otherwise do nothing.
            }
        }
    }

    /* #endregion */
}

impl<M: MMU> Display for BasicPPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
