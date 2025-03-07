use std::{cell::RefCell, fmt::Display, rc::Rc};

use minifb::{Key, Window, WindowOptions};

use crate::{cpu::MTime, mmu::MMU, ppu::PPU};

const BASE_WIDTH: usize = 160;
const BASE_HEIGHT: usize = 144;

const DOTS_PER_DRAW: u32 = 65664;
const DOTS_PER_VBLANK: u32 = 4560;
const DOTS_PER_FRAME: u32 = DOTS_PER_DRAW + DOTS_PER_VBLANK;

const IO_LCDC: u16 = 0xFF40;
const IO_STAT: u16 = 0xFF41;
const IO_SCY: u16 = 0xFF42;
const IO_SCX: u16 = 0xFF43;
const IO_LY: u16 = 0xFF44;
const IO_LYC: u16 = 0xFF44;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PPUMode {
    HORIZONTAL_BLANK,
    VERTICAL_BLANK,
    OAM_SCAN,
    DRAWING,
}

#[derive(Debug)]
pub struct BasicPPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
    // Window data
    win: Window,
    scale: usize,
    width: usize,
    height: usize,
    // Draw state
    last_executed_mode: PPUMode,
    mode: PPUMode,
    scanline: u8,
    wait: u8,
    dots_this_frame: u32,
    frame_buffer: Vec<u32>,
    // LCD status and control
    io_lcdc: u8,
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
            last_executed_mode: PPUMode::VERTICAL_BLANK,
            mode: PPUMode::OAM_SCAN,
            scanline: 0,
            wait: 0,
            dots_this_frame: 0,
            frame_buffer: vec![Self::from_u8_rgb(0, 0, 0); width * height],
            io_lcdc: 0,
        }
    }

    fn step_dots(&mut self, dm: MTime) {
        let dots = dm * 4;
        self.get_bg_window_enabled();
    }

    fn should_terminate(&self) -> bool {
        !self.win.is_open()
    }
}

macro_rules! get_lcdc_flag {
    ($get_name:ident, $mask:expr) => {
        fn $get_name(&self) -> bool {
            self.io_lcdc & $mask != 0
        }
    };
}

impl<M: MMU> BasicPPU<M> {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn load_io(&mut self) {
        self.io_lcdc = self.mmu.borrow().read_byte(IO_LCDC);
    }

    get_lcdc_flag!(get_enabled, 1 << 7);
    get_lcdc_flag!(get_window_tile_map, 1 << 6);
    get_lcdc_flag!(get_window_enabled, 1 << 5);
    get_lcdc_flag!(get_bg_window_tiles, 1 << 4);
    get_lcdc_flag!(get_bg_tile_map, 1 << 3);
    get_lcdc_flag!(get_obj_size, 1 << 2);
    get_lcdc_flag!(get_obj_enabled, 1 << 1);
    get_lcdc_flag!(get_bg_window_enabled, 1 << 0);

    fn do_oam_scan(&mut self) {
        // TODO: Do OAM scan
    }

    fn do_draw(&mut self) {
        // TODO: Do draw
    }
}

impl<M: MMU> Display for BasicPPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
