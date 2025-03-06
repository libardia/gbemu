use std::{cell::RefCell, fmt::Display, rc::Rc};

use minifb::{Key, Window, WindowOptions};

use crate::{cpu::MTime, gpu::GPU, mmu::MMU};

const BASE_WIDTH: usize = 160;
const BASE_HEIGHT: usize = 144;

const DOTS_PER_DRAW: u32 = 65664;
const DOTS_PER_VBLANK: u32 = 4560;
const DOTS_PER_FRAME: u32 = DOTS_PER_DRAW + DOTS_PER_VBLANK;

#[derive(Debug)]
pub struct BasicGPU<M: MMU> {
    mmu: Rc<RefCell<M>>,
    win: Window,
    scale: usize,
    width: usize,
    height: usize,
    dots_this_frame: u32,
    frame_buffer: Vec<u32>,
}

impl<M: MMU> GPU<M> for BasicGPU<M> {
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
            dots_this_frame: 0,
            frame_buffer: vec![Self::from_u8_rgb(0, 0, 0); width * height],
        }
    }

    fn draw_dots(&mut self, dm: MTime) {
        let dots = dm * 4;
        for _ in 0..dots {
            self.draw_dot();
        }
    }

    fn should_terminate(&self) -> bool {
        !self.win.is_open()
    }
}

impl<M: MMU> BasicGPU<M> {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn draw_dot(&mut self) {
        // TODO: Draw one dot
        self.dots_this_frame += 1;

        if self.dots_this_frame >= DOTS_PER_DRAW {
            self.win
                .update_with_buffer(&self.frame_buffer, self.width, self.height)
                .ok();
        }
    }
}

impl<M: MMU> Display for BasicGPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
