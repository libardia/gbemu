use std::{cell::RefCell, fmt::Display, rc::Rc};

use minifb::{Key, Window, WindowOptions};

use crate::{cpu::MTime, mmu::MMU, ppu::PPU};

const BASE_WIDTH: usize = 160;
const BASE_HEIGHT: usize = 144;

const DOTS_PER_DRAW: u32 = 65664;
const DOTS_PER_VBLANK: u32 = 4560;
const DOTS_PER_FRAME: u32 = DOTS_PER_DRAW + DOTS_PER_VBLANK;

#[derive(Debug)]
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
    mode: PPUMode,
    scanline: u8,
    wait: u8,
    dots_this_frame: u32,
    frame_buffer: Vec<u32>,
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
            mode: PPUMode::OAM_SCAN,
            scanline: 0,
            wait: 0,
            dots_this_frame: 0,
            frame_buffer: vec![Self::from_u8_rgb(0, 0, 0); width * height],
        }
    }

    fn step_dots(&mut self, dm: MTime) {
        let dots = dm * 4;

        match self.mode {
            PPUMode::OAM_SCAN => {}
            PPUMode::DRAWING => todo!(),
            PPUMode::HORIZONTAL_BLANK => todo!(),
            PPUMode::VERTICAL_BLANK => todo!(),
        }

        // Draw a dot, or skip it if we're waiting
        if self.wait > 0 {
            self.wait -= 1;
        } else {
            self.draw_dot();
        }

        // One more dot passed this frame
        self.dots_this_frame += 1;

        // If we've drawn all scanlines, update the window
        if self.dots_this_frame >= DOTS_PER_DRAW {
            self.win
                .update_with_buffer(&self.frame_buffer, self.width, self.height)
                .ok();
        }
    }

    fn should_terminate(&self) -> bool {
        !self.win.is_open()
    }
}

impl<M: MMU> BasicPPU<M> {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn draw_dot(&mut self) {
        // TODO: Draw one dot
    }
}

impl<M: MMU> Display for BasicPPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
