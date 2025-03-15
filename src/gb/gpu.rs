use std::{
    cell::RefCell,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

use draw_state::DrawState;
use log::debug;
use minifb::{Window, WindowOptions};
use palette::Palette;
use RenderMode::*;

use super::{
    mmu::MMU,
    time_types::{MTime, TTime},
};

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
    // Control
    pub terminate: bool,
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

        Self {
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
            bg_palette: 0.into(),
            obj_palette_0: 0.into(),
            obj_palette_1: 0.into(),
            // LCD status and control
            io_lcdc: 0,
            io_stat: 0,
            compare_line: 0,
            viewport_x: 0,
            viewport_y: 0,
            window_xp7: 0,
            window_y: 0,
            interrupt_requests: 0,
            // Control
            terminate: false,
        }
    }

    pub fn step(&mut self, dt: MTime) {
        // Load hardware registers
        let last_enabled = self.get_enabled();
        self.load_regs();

        if last_enabled && !self.get_enabled() {
            debug!("LCD was just disabled.");
            self.ds = DrawState::new();
            self.frame_buffer = vec![0; self.scr_width * self.scr_height];
            // TODO: Unblock memory
        } else if !last_enabled && self.get_enabled() {
            debug!("LCD was just enabled.");
            self.disabled_frame_time = 0.into();
        }

        // If LCD is disabled...
        if !self.get_enabled() {
            self.disabled_frame_time += dt.into();
            if self.disabled_frame_time >= FRAME_TIME {
                self.disabled_frame_time %= FRAME_TIME;
                self.frame();
            }
        }
        // If LCD is enabled...
        else {
            // Advance time
            self.ds.time_this_line += dt.into();

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

        // Ask to terminate if the window is closed
        self.terminate = !self.win.is_open();
    }

    fn frame(&mut self) {
        let dt = self.last_frame_time.elapsed();
        self.last_frame_time = Instant::now();

        let wait_dur = self.seconds_per_frame.saturating_sub(dt);
        sleep(wait_dur);

        if self.get_enabled() {
            let fps = 1.0 / (dt + wait_dur).as_secs_f32();
            self.win.set_title(&format!(
                "{WINDOW_TITLE} | {fps:0.02} FPS | {:0.02}%",
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

    /* #region MMU convenience ================================================================= */

    fn mmu_read_byte(&self, address: u16) -> u8 {
        self.mmu.borrow().read_byte(address)
    }

    fn mmu_write_byte(&self, address: u16, value: u8) {
        self.mmu.borrow_mut().write_byte(address, value);
    }

    fn mmu_read_word(&self, address: u16) -> u16 {
        self.mmu.borrow().read_word(address)
    }

    fn mmu_write_word(&self, address: u16, value: u16) {
        self.mmu.borrow_mut().write_word(address, value);
    }

    /* #endregion */
}

mod modes;
mod regs;
