use super::{RenderMode::*, GPU, LINES_PER_DRAW, LINES_PER_FRAME, OAM_TIME, SCANLINE_TIME};

impl GPU {
    pub(super) fn oam_scan(&mut self) {
        // OAM Scan always ends at the same time
        self.ds.end_mode_time = OAM_TIME;

        // Request the Mode 2 STAT interrupt if we've been asked to
        if self.get_mode_2_interrupt() {
            self.set_stat_interrupt(true);
        }

        // TODO: OAM scan
    }

    pub(super) fn oam_scan_next(&mut self) {
        // Mode on to Draw
        self.ds.mode = Draw;
    }

    pub(super) fn draw(&mut self) {
        // Temporary; just so the draw mode ends
        self.ds.end_mode_time += 200.into();

        // TODO: Drawing
    }

    pub(super) fn draw_next(&mut self) {
        // Move on to horizontal blank
        self.ds.mode = HBlank;
    }

    pub(super) fn hblank(&mut self) {
        self.ds.end_mode_time = SCANLINE_TIME;

        // Request the Mode 0 STAT interrupt if we've been asked to
        if self.get_mode_0_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Nothing happens during hblank
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

    pub(super) fn vblank(&mut self) {
        self.ds.end_mode_time = SCANLINE_TIME;

        // Request the VBlank interrupt
        self.set_vblank_interrupt(true);

        // Request the Mode 1 STAT interrupt if we've been asked to
        if self.get_mode_1_interrupt() {
            self.set_stat_interrupt(true);
        }

        // TODO: Vertical blank
    }

    pub(super) fn vblank_next(&mut self) {
        if self.ds.current_line >= LINES_PER_FRAME {
            // All the lines of vblank are done, go back to OAM scan
            self.ds.current_line = 0;
            self.ds.mode = OamScan;
        }
        // Otherwise, just stay in vblank
    }
}
