use std::{cell::RefCell, rc::Rc};

use draw_state::DrawState;
use log::debug;
use palette::Palette;

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

const OAM_TIME: TTime = TTime::make(80);
const SCANLINE_TIME: TTime = TTime::make(456);
const LINES_PER_DRAW: u8 = 144;
const LINES_PER_FRAME: u8 = 154;

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
    // State
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
}
impl GPU {
    pub fn new(mmu: Rc<RefCell<MMU>>) -> Self {
        Self {
            mmu,
            ds: DrawState::new(),
            io_lcdc: 0,
            io_stat: 0,
            compare_line: 0,
            viewport_x: 0,
            viewport_y: 0,
            window_xp7: 0,
            window_y: 0,
            interrupt_requests: 0,
            bg_palette: 0.into(),
            obj_palette_0: 0.into(),
            obj_palette_1: 0.into(),
        }
    }

    pub fn step(&mut self, dt: MTime) {
        // Advance time
        self.ds.time_this_line += dt.into();

        // Load hardware registers
        let last_enabled = self.get_enabled();
        self.load_regs();

        if last_enabled && !self.get_enabled() {
            debug!("LCD was just disabled.");
            self.ds = DrawState::new();
            // TODO: Reset frame buffer
            // TODO: Unblock memory
        } else if !last_enabled && self.get_enabled() {
            debug!("LCD was just enabled.");
        }

        if self.get_enabled() {
            // Execute the mode if it hasn't executed yet
            use RenderMode::*;
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
            else if self.ds.time_this_line >= self.ds.end_mode_time {
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
