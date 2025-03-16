use crate::{mem_region::regions::OAM, util::either};

use super::{
    object::{Object, OBJECT_BYTE_SIZE},
    RenderMode::*,
    GPU, LINES_PER_DRAW, LINES_PER_FRAME, OAM_TIME, OBJECTS_PER_LINE, SCANLINE_TIME,
};

pub const NUM_OBJECTS: u16 = OAM.size() / OBJECT_BYTE_SIZE;

impl GPU {
    /* #region Mode execute methods ============================================================ */

    pub(super) fn oam_scan(&mut self) {
        // OAM Scan always ends at the same time
        self.ds.end_mode_time = OAM_TIME;

        // Request the Mode 2 STAT interrupt if we've been asked to
        if self.get_mode_2_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Scan for objects that intersect this line
        for i in 0..NUM_OBJECTS {
            let a = OAM.begin() + i * OBJECT_BYTE_SIZE;
            let obj_y = (self.mmu_read_byte(a) as i16) - 16;
            let obj_height = obj_y + either!(self.get_obj_size() => 16; 8);
            let line = self.ds.current_line as i16;
            if obj_y <= line && line < (obj_y + obj_height) {
                self.ds.selected_objects.push(Object {
                    y: obj_y,
                    x: self.mmu_read_byte(a + 1) as i16 - 8,
                    tile_index: self.mmu_read_byte(a + 2),
                    flags: self.mmu_read_byte(a + 3),
                });
                if self.ds.selected_objects.len() >= OBJECTS_PER_LINE {
                    // Stop scanning for more objects; only 10 per line
                    break;
                }
            }
        }
    }

    pub(super) fn draw(&mut self) {
        // TODO: Drawing

        // TODO: Calculate actual draw mode length
        self.ds.end_mode_time += 200.into();
    }

    pub(super) fn hblank(&mut self) {
        self.ds.end_mode_time = SCANLINE_TIME;

        // Request the Mode 0 STAT interrupt if we've been asked to
        if self.get_mode_0_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Clear the selected objects for the line, but otherwise nothing happens in hblank
        self.ds.selected_objects.clear();
    }

    pub(super) fn vblank(&mut self) {
        self.ds.end_mode_time = SCANLINE_TIME;

        // Request the VBlank interrupt
        self.set_vblank_interrupt(true);

        // Request the Mode 1 STAT interrupt if we've been asked to
        if self.get_mode_1_interrupt() {
            self.set_stat_interrupt(true);
        }

        self.frame();
        // TODO: Vertical blank
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
