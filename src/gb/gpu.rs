use std::{
    cell::RefCell,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

use color_id::ColorID;
use draw_state::DrawState;
use log::debug;
use meta_palette::*;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use palette::Palette;
use RenderMode::*;

use crate::mem_region::regions::{OAM, VRAM};

use super::{mmu::MMU, time_types::TTime};

pub mod color_id;
pub mod draw_state;
pub mod meta_palette;
pub mod object;
pub mod palette;
pub mod tile;

const WINDOW_TITLE: &str = "GBEMU";
const BASE_SCREEN_WIDTH: usize = 160;
const BASE_SCREEN_HEIGHT: usize = 144;

pub const REAL_GB_FPS: f32 = 59.73;

const OAM_TIME: TTime = TTime::make(80);
const SCANLINE_TIME: TTime = TTime::make(456);
const FRAME_TIME: TTime = TTime::make(70224);
const LINES_PER_DRAW: u8 = 144;
const LINES_PER_FRAME: u8 = 154;

pub const OBJECTS_PER_LINE: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderMode {
    OamScan = 2,
    Draw = 3,
    HBlank = 0,
    VBlank = 1,
}

#[derive(Debug)]
pub struct GPU {
    // MMU ref
    mmu: Rc<RefCell<MMU>>,
    // Window stuff
    win: Window,
    scr_width: usize,
    scr_height: usize,
    scale: usize,
    frame_buffer: Vec<u32>,
    // Window control
    frames_per_second: f32,
    seconds_per_frame: Duration,
    last_frame_time: Instant,
    // State
    ds: DrawState,
    disabled_frame_time: TTime,
    // Palettes
    meta_palette: MetaPalette,
    bg_palette: Palette,
    obj_palette_0: Palette,
    obj_palette_1: Palette,
    // LCD status and control
    io_lcdc: u8,
    io_stat: u8,
    compare_line: u8,
    scroll_x: u8,
    scroll_y: u8,
    win_xp7: u8,
    win_y: u8,
    interrupt_requests: u8,
    // Control
    terminate: bool,
}
impl GPU {
    pub fn new(mmu: Rc<RefCell<MMU>>, fps: f32, window_scale: usize) -> Self {
        let scr_width = window_scale * BASE_SCREEN_WIDTH;
        let scr_height = window_scale * BASE_SCREEN_HEIGHT;
        let win = Window::new(
            WINDOW_TITLE,
            scr_width,
            scr_height,
            WindowOptions::default(),
        )
        .expect("Failed to create window");

        let mut this = Self {
            // MMU ref
            mmu,
            // Window stuff
            win,
            scr_width,
            scr_height,
            scale: window_scale,
            frame_buffer: vec![0; scr_width * scr_height],
            // Window control
            frames_per_second: fps,
            seconds_per_frame: Duration::from_secs_f32(1.0 / fps),
            last_frame_time: Instant::now(),
            // State
            ds: DrawState::new(),
            disabled_frame_time: 0.into(),
            // Palettes
            meta_palette: GAMEBOY_MPALETTE,
            bg_palette: 0.into(),
            obj_palette_0: 0.into(),
            obj_palette_1: 0.into(),
            // LCD status and control
            io_lcdc: 0,
            io_stat: 0,
            compare_line: 0,
            scroll_x: 0,
            scroll_y: 0,
            win_xp7: 0,
            win_y: 0,
            interrupt_requests: 0,
            // Control
            terminate: false,
        };

        this.reset_frame_buffer();
        this
    }

    pub fn should_terminate(&self) -> bool {
        !self.win.is_open() || self.win.is_key_pressed(Key::Escape, KeyRepeat::No)
    }

    pub fn step(&mut self, dt: TTime) {
        // Load hardware registers
        let last_enabled = self.get_enabled();
        self.load_regs();

        if last_enabled && !self.get_enabled() {
            debug!("LCD disabled.");

            self.ds = DrawState::new();
            self.reset_frame_buffer();
            self.frame(false);

            // Make sure memory is unblocked
            self.mmu.borrow_mut().unblock_region(OAM);
            self.mmu.borrow_mut().unblock_region(VRAM);
        } else if !last_enabled && self.get_enabled() {
            debug!("LCD enabled.");
            self.disabled_frame_time = 0.into();
            self.frame(false);
        }

        // If LCD is disabled...
        if !self.get_enabled() {
            self.disabled_frame_time += dt;
            if self.disabled_frame_time >= FRAME_TIME {
                self.disabled_frame_time %= FRAME_TIME;
            }
        }
        // If LCD is enabled...
        else {
            // Advance time
            self.ds.time_this_line += dt;

            // Execute the mode if it hasn't executed yet
            if self.ds.last_executed_mode != self.ds.mode {
                self.ds.last_executed_mode = self.ds.mode;
                match self.ds.mode {
                    OamScan => self.oam_scan(),
                    Draw => self.draw(),
                    HBlank => self.hblank(),
                    VBlank => self.vblank(),
                }
            }

            // When we pass end_mode_time, we should move on to the next mode
            if self.ds.time_this_line >= self.ds.end_mode_time {
                // Increment line and reset time_this_line if necessary
                if self.ds.time_this_line >= SCANLINE_TIME {
                    self.ds.time_this_line %= SCANLINE_TIME;
                    self.ds.current_line += 1;
                }

                // Next mode?
                match self.ds.mode {
                    OamScan => self.oam_scan_next(),
                    Draw => self.draw_next(),
                    HBlank => self.hblank_next(),
                    VBlank => self.vblank_next(),
                }
            }
        }

        // Write out hardware registers
        self.set_regs();
    }

    fn frame(&mut self, wait: bool) {
        let dt = self.last_frame_time.elapsed();
        self.last_frame_time = Instant::now();

        let mut wait_dur = Duration::ZERO;
        if wait {
            wait_dur = self.seconds_per_frame.saturating_sub(dt);
            sleep(wait_dur);
        }

        if self.get_enabled() {
            let fps = 1.0 / (dt + wait_dur).as_secs_f32();
            self.win.set_title(&format!(
                "{WINDOW_TITLE} | {fps:0.02} FPS | {:0>6.02}%",
                100.0 * fps / REAL_GB_FPS
            ));
        } else {
            self.win
                .set_title(&format!("{WINDOW_TITLE} | LCD disabled"));
        }

        self.win
            .update_with_buffer(&self.frame_buffer, self.scr_width, self.scr_height)
            .expect("Failed to update frame buffer!");
    }

    fn reset_frame_buffer(&mut self) {
        let c = self.meta_palette[ColorID::Color0];
        for i in 0..self.frame_buffer.len() {
            self.frame_buffer[i] = c;
        }
    }

    /* #region MMU convenience ================================================================== */

    fn mmu_get(&self, address: u16) -> u8 {
        self.mmu.borrow().get(address)
    }

    fn mmu_set(&self, address: u16, value: u8) {
        self.mmu.borrow_mut().set(address, value);
    }

    /* #endregion */
}

mod modes;
mod regs;
