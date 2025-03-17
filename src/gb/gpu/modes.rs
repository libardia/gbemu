use crate::{
    mem_region::regions::{OAM, VRAM},
    util::either,
};

use super::{
    color_id::ColorID,
    object::{Object, OBJECT_BYTE_SIZE},
    RenderMode::*,
    BASE_SCREEN_WIDTH, GPU, LINES_PER_DRAW, LINES_PER_FRAME, OAM_TIME, OBJECTS_PER_LINE,
    SCANLINE_TIME,
};

pub const NUM_OBJECTS: u16 = OAM.size() / OBJECT_BYTE_SIZE;

impl GPU {
    /* #region Mode execute methods ============================================================ */

    pub(super) fn oam_scan(&mut self) {
        // OAM Scan always ends at the same time
        self.ds.end_mode_time = OAM_TIME;

        // Block OAM memory
        self.mmu.borrow_mut().block_region(OAM);

        // Request the Mode 2 STAT interrupt if we've been asked to
        if self.get_mode_2_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Scan for objects that intersect this line
        for i in 0..NUM_OBJECTS {
            let a = OAM.begin() + i * OBJECT_BYTE_SIZE;
            let obj_y = (self.mmu_get(a) as i16) - 16;
            let obj_height = obj_y + either!(self.get_obj_size() => 16; 8);
            let line = self.ds.current_line as i16;
            if obj_y <= line && line < (obj_y + obj_height) {
                self.ds.selected_objects.push(Object {
                    y: obj_y,
                    x: self.mmu_get(a + 1) as i16 - 8,
                    tile_index: self.mmu_get(a + 2),
                    flags: self.mmu_get(a + 3),
                });
                if self.ds.selected_objects.len() >= OBJECTS_PER_LINE {
                    // Stop scanning for more objects; only 10 per line
                    break;
                }
            }
        }
    }

    pub(super) fn draw(&mut self) {
        const TILE_SIZE: usize = 8;
        const MAP_PX_SIZE: usize = 256;
        const MAP_SIZE: usize = MAP_PX_SIZE / TILE_SIZE;

        // Block VRAM
        self.mmu.borrow_mut().block_region(VRAM);

        let y = self.ds.current_line as usize;

        let bg_map_y = (self.scroll_y as usize + y) % MAP_PX_SIZE;
        let bg_tile_y = bg_map_y / TILE_SIZE;
        let bg_intile_y = bg_map_y % TILE_SIZE;

        let window_map_y = self.ds.window_y_counter as usize;
        let window_tile_y = window_map_y / TILE_SIZE;
        let window_intile_y = window_tile_y % TILE_SIZE;

        for x in 0..BASE_SCREEN_WIDTH {
            let mut pix_color = ColorID::Color0;

            if self.get_bg_window_enabled() {
                let bg_map_x = (self.scroll_x as usize + x) % MAP_PX_SIZE;
                let bg_tile_x = bg_map_x / TILE_SIZE;
                let bg_intile_x = bg_map_x % TILE_SIZE;
                let bg_tile_index = self.mmu.borrow().get_tile_index_at(
                    self.get_bg_tile_map(),
                    bg_tile_x,
                    bg_tile_y,
                );
                let tile_color = self
                    .mmu
                    .borrow()
                    .get_tile(self.get_bg_win_tile_data(), bg_tile_index)
                    .pix_at(bg_intile_x, bg_intile_y);
                pix_color = self.bg_palette[tile_color];

                if self.get_win_enabled() {
                    // TODO: draw window pixel
                    let window_map_x = x as isize - self.window_xp7 as isize - 7;
                }
            }

            if self.get_obj_enabled() {
                for obj in &self.ds.selected_objects {
                    // TODO: draw pixel of objects
                }
            }

            self.draw_scaled_pixel(x, pix_color);
        }

        // TODO: Calculate actual draw mode length
        self.ds.end_mode_time += 200.into();
    }

    fn draw_scaled_pixel(&mut self, x: usize, color_id: ColorID) {
        let x1 = x * self.scale;
        let x2 = x1 + self.scale;

        let y1 = self.ds.current_line as usize * self.scale;
        let y2 = y1 + self.scale;

        let c = self.meta_palette[color_id];

        for iy in y1..y2 {
            for ix in x1..x2 {
                self.frame_buffer[iy * self.scr_width + ix] = c;
            }
        }
    }

    pub(super) fn hblank(&mut self) {
        // HBlank lasts until the end of the current scanline
        self.ds.end_mode_time = SCANLINE_TIME;

        // Unblock memory
        self.mmu.borrow_mut().unblock_region(OAM);
        self.mmu.borrow_mut().unblock_region(VRAM);

        // Request the Mode 0 STAT interrupt if we've been asked to
        if self.get_mode_0_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Clear the selected objects for the line
        self.ds.selected_objects.clear();
    }

    pub(super) fn vblank(&mut self) {
        // VBlank doesn't necessarily end here, but we do need to check if it ends
        self.ds.end_mode_time = SCANLINE_TIME;

        // Request the VBlank interrupt
        self.set_vblank_interrupt(true);

        // Request the Mode 1 STAT interrupt if we've been asked to
        if self.get_mode_1_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Reset window y counter
        self.ds.window_y_counter = 0;

        // Present the frame and wait if necessary
        self.frame();
    }

    /* #endregion */

    /* #region Mode switch methods ============================================================= */

    pub(super) fn oam_scan_next(&mut self) {
        // Mode on to Draw
        self.ds.mode = Draw;
    }

    pub(super) fn draw_next(&mut self) {
        // Move on to horizontal blank
        self.ds.mode = HBlank;
    }

    pub(super) fn hblank_next(&mut self) {
        if self.ds.current_line >= LINES_PER_DRAW {
            // After all lines are drawn, move on to vertical blank
            self.ds.mode = VBlank;
        } else {
            // If there are lines left to draw, go back to OAM scan
            self.ds.mode = OamScan;
        }
    }

    pub(super) fn vblank_next(&mut self) {
        if self.ds.current_line >= LINES_PER_FRAME {
            // All the lines of vblank are done, go back to OAM scan
            self.ds.current_line = 0;
            self.ds.mode = OamScan;
        }
        // Otherwise, just stay in vblank
    }

    /* #endregion */
}
